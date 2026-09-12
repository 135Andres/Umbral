//! Content observation with a stability guard.
//!
//! The one normative statement this module implements:
//!
//! > A persisted hash represents exactly the bytes Umbral read during one valid
//! > observation.
//!
//! It does **not** claim the file still holds those bytes afterwards. Umbral observes a
//! live filesystem; it does not freeze the reality it observes.
//!
//! How that is enforced:
//!
//! - Streaming read (64 KiB buffer, never the whole file in memory).
//! - `stat` before / read / `stat` after, comparing size, nanosecond mtime and `dev`+`ino`.
//! - On a detected change the read is retried once. Still changing → `Unstable`, with no
//!   hash and the deltas that fired.
//! - Only `Stable` observations expose a hash via [`ContentObservation::valid_hash`].
//!
//! Documented blind spot, inherited knowingly: a rewrite that restores both size and exact
//! mtime inside the read window is invisible to any `stat`-based guard. Comparing content
//! hashes across observations is the partial backstop.

use std::io::Read;
use std::path::Path;

use crate::identity::{mtime_of, physical_id_of};

pub const HASH_LEN: usize = 32;
const READ_BUF: usize = 64 * 1024;

/// How many times a read is attempted before the observation is declared unstable.
pub const MAX_GUARD_ATTEMPTS: u32 = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stability {
    /// The before/after guard observed no relevant change.
    Stable,
    /// The guard observed a change on every attempt. No hash is reported.
    Unstable,
}

impl Stability {
    pub fn as_str(self) -> &'static str {
        match self {
            Stability::Stable => "stable",
            Stability::Unstable => "unstable",
        }
    }

    pub fn parse(s: &str) -> Option<Stability> {
        match s {
            "stable" => Some(Stability::Stable),
            "unstable" => Some(Stability::Unstable),
            _ => None,
        }
    }
}

/// What the guard saw change. Recorded as evidence, never smoothed over.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuardDelta {
    SizeChanged,
    MtimeChanged,
    IdentityChanged,
    Vanished,
}

impl GuardDelta {
    pub fn as_str(self) -> &'static str {
        match self {
            GuardDelta::SizeChanged => "size-changed",
            GuardDelta::MtimeChanged => "mtime-changed",
            GuardDelta::IdentityChanged => "identity-changed",
            GuardDelta::Vanished => "vanished",
        }
    }

    pub fn parse(s: &str) -> Option<GuardDelta> {
        match s {
            "size-changed" => Some(GuardDelta::SizeChanged),
            "mtime-changed" => Some(GuardDelta::MtimeChanged),
            "identity-changed" => Some(GuardDelta::IdentityChanged),
            "vanished" => Some(GuardDelta::Vanished),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentError {
    NotFound,
    PermissionDenied,
    /// A directory, symlink or special file. Never resolved to a target's content.
    NotARegularFile,
    ReadError(String),
}

impl ContentError {
    pub fn as_str(&self) -> &'static str {
        match self {
            ContentError::NotFound => "not-found",
            ContentError::PermissionDenied => "permission-denied",
            ContentError::NotARegularFile => "not-a-regular-file",
            ContentError::ReadError(_) => "read-error",
        }
    }
}

/// The result of one content observation. Absence of a hash is recorded as absence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentObservation {
    pub hash: Option<[u8; HASH_LEN]>,
    /// Bytes actually read. `None` when no complete read happened.
    pub hashed_len: Option<u64>,
    pub stability: Option<Stability>,
    pub deltas: Vec<GuardDelta>,
    pub error: Option<ContentError>,
}

impl ContentObservation {
    fn error(err: ContentError) -> Self {
        ContentObservation {
            hash: None,
            hashed_len: None,
            stability: None,
            deltas: Vec::new(),
            error: Some(err),
        }
    }

    /// The hash, ONLY for a stable observation. This is the single gate that keeps an
    /// unstable or errored read from being persisted as content evidence.
    pub fn valid_hash(&self) -> Option<&[u8; HASH_LEN]> {
        match self.stability {
            Some(Stability::Stable) => self.hash.as_ref(),
            _ => None,
        }
    }

    pub fn is_content_verified(&self) -> bool {
        self.valid_hash().is_some()
    }

    pub fn deltas_str(&self) -> Option<String> {
        if self.deltas.is_empty() {
            None
        } else {
            Some(
                self.deltas
                    .iter()
                    .map(|d| d.as_str())
                    .collect::<Vec<_>>()
                    .join(","),
            )
        }
    }
}

fn content_error_from(e: &std::io::Error) -> ContentError {
    match e.kind() {
        std::io::ErrorKind::NotFound => ContentError::NotFound,
        std::io::ErrorKind::PermissionDenied => ContentError::PermissionDenied,
        _ => ContentError::ReadError(e.to_string()),
    }
}

/// Observe the content of a regular file, guarded against concurrent modification.
pub fn observe_content(path: &Path) -> ContentObservation {
    let before = match std::fs::symlink_metadata(path) {
        Ok(m) => m,
        Err(e) => return ContentObservation::error(content_error_from(&e)),
    };
    if !before.file_type().is_file() {
        return ContentObservation::error(ContentError::NotARegularFile);
    }

    let mut last_deltas: Vec<GuardDelta> = Vec::new();

    for _attempt in 0..MAX_GUARD_ATTEMPTS {
        let before_id = physical_id_of(&before);
        let before_size = before.len();
        let before_mtime = mtime_of(&before);

        let mut hasher = blake3::Hasher::new();
        let mut read_total: u64 = 0;
        let mut read_error: Option<std::io::Error> = None;

        match std::fs::File::open(path) {
            Ok(mut f) => {
                let mut buf = vec![0u8; READ_BUF];
                loop {
                    match f.read(&mut buf) {
                        Ok(0) => break,
                        Ok(n) => {
                            hasher.update(&buf[..n]);
                            read_total += n as u64;
                        }
                        Err(e) => {
                            read_error = Some(e);
                            break;
                        }
                    }
                }
            }
            Err(e) => return ContentObservation::error(content_error_from(&e)),
        }
        if let Some(e) = read_error {
            return ContentObservation::error(content_error_from(&e));
        }

        let after = match std::fs::symlink_metadata(path) {
            Ok(m) => m,
            Err(e) => return ContentObservation::error(content_error_from(&e)),
        };

        let mut deltas: Vec<GuardDelta> = Vec::new();
        if physical_id_of(&after) != before_id {
            deltas.push(GuardDelta::IdentityChanged);
        }
        if after.len() != before_size {
            deltas.push(GuardDelta::SizeChanged);
        }
        if mtime_of(&after) != before_mtime {
            deltas.push(GuardDelta::MtimeChanged);
        }

        if deltas.is_empty() {
            let hash = *hasher.finalize().as_bytes();
            return ContentObservation {
                hash: Some(hash),
                hashed_len: Some(read_total),
                stability: Some(Stability::Stable),
                deltas: Vec::new(),
                error: None,
            };
        }
        last_deltas = deltas;
    }

    ContentObservation {
        hash: None,
        hashed_len: None,
        stability: Some(Stability::Unstable),
        deltas: last_deltas,
        error: None,
    }
}

/// Hex rendering of a hash, full length. Used by the report and by tests.
pub fn hex(h: &[u8]) -> String {
    h.iter().map(|b| format!("{b:02x}")).collect()
}

/// Short hex rendering for terminal display. Never used for comparison.
pub fn hex_short(h: &[u8]) -> String {
    h.iter().take(6).map(|b| format!("{b:02x}")).collect()
}
