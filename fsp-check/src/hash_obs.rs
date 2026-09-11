//! Increment 3 — HASH: content evidence with a re-read guard.
//!
//! INV-1 (normative formulation, user correction):
//!   A persisted hash represents exactly the bytes FSP read during one valid
//!   observation. It says NOTHING about the bytes the file has afterwards.
//!   FSP observes a LIVE filesystem; it does not control or freeze it.
//!
//! Design: an invalidatable observation must never surface as a valid hash.
//! The guard is stat-before / read+hash / stat-after; a detected change makes
//! the result Unstable — it is EVIDENCE of instability, not a hash of unknown
//! bytes. No magic values, no fake hashes (INV-3-family rule: errors and
//! instability are represented as states, never as bytes).

use crate::identity::PhysicalId;
use std::io::Read;
use std::path::Path;

/// Hash length in bytes (BLAKE3 default: 32).
pub const HASH_LEN: usize = 32;

/// Explicit policy, not a magic number: try the guarded read at most
/// `MAX_GUARD_ATTEMPTS` times; a file that keeps changing under us is
/// reported Unstable rather than retried forever. Value chosen as the
/// smallest number that distinguishes "one transient change" from "this file
/// is being actively written"; it is a V0 instrument policy, not an FSP
/// architecture decision (recorded in the plan).
pub const MAX_GUARD_ATTEMPTS: u32 = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stability {
    /// guard before == after (relevant properties), over a completed read.
    Stable,
    /// the object changed during/at the read, even after the last retry.
    Unstable,
}

/// Why the guard declared instability. More than one can apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuardDelta {
    SizeChanged,
    MtimeChanged,
    PhysicalIdChanged,
}

/// Content observation of ONE regular file.
/// `hash: None` means no content evidence exists (error or never readable) —
/// absence is represented, never faked with a magic value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentObservation {
    pub physical_id: PhysicalId,
    pub hash: Option<[u8; HASH_LEN]>,
    /// bytes actually hashed for the SUCCESSFUL attempt
    pub hashed_len: Option<u64>,
    pub stability: Stability,
    /// which guard properties moved (empty when Stable or on plain errors)
    pub deltas: Vec<GuardDelta>,
    /// None when a valid hash was obtained.
    pub error: Option<ContentError>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentError {
    NotFound,
    PermissionDenied,
    /// exists but is not a regular file (dir, fifo, socket, device...)
    NotARegularFile,
    ReadError(String),
}

impl ContentObservation {
    /// INV-1 accessor: only a Stable observation yields content evidence.
    /// Everything else (Unstable, any error) is explicitly NOT a valid hash.
    pub fn valid_hash(&self) -> Option<&[u8; HASH_LEN]> {
        if self.stability == Stability::Stable {
            self.hash.as_ref()
        } else {
            None
        }
    }
}

/// Which stat properties the guard compares. mtime uses ns resolution where
/// the platform provides it; a same-size rewrite that restores both size and
/// mtime to the exact previous values is undetectable by any stat-guard —
/// that residual limitation is documented, not hidden (see module tests).
fn guard_properties(sm: &std::fs::Metadata) -> (Option<u64>, Option<(i64, u32)>, Option<u64>) {
    let mtime = sm.modified().ok().and_then(|t| {
        t.duration_since(std::time::UNIX_EPOCH)
            .ok()
            .map(|d| (d.as_secs() as i64, d.subsec_nanos()))
    });
    (
        Some(sm.len()),
        mtime,
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt as _;
            Some(sm.ino())
        },
        #[cfg(not(unix))]
        {
            None
        },
    )
}

/// `observe_content(path)`: guarded, streaming BLAKE3 of a regular file.
/// Follows the scanner's contract: symlinks are observed as themselves —
/// hashing follows `symlink_metadata`, so a symlink path is NotARegularFile
/// and is never resolved to its target's content (target identity must not
/// become symlink identity).
pub fn observe_content(path: &Path) -> ContentObservation {
    let mut last_deltas: Vec<GuardDelta> = Vec::new();
    let mut last_error: Option<ContentError> = None;
    let mut last_pid = PhysicalId {
        dev: None,
        ino: None,
    };

    for _attempt in 0..MAX_GUARD_ATTEMPTS {
        // -- stat before (lstat semantics: never traverse symlinks)
        let sm_before = match std::fs::symlink_metadata(path) {
            Ok(m) => m,
            Err(e) => return error_obs(last_pid, map_io(e)),
        };
        if !sm_before.is_file() {
            // symlink or directory or special: no content observation in V0
            return error_obs(
                PhysicalId::from_metadata(&sm_before),
                ContentError::NotARegularFile,
            );
        }
        let pid = PhysicalId::from_metadata(&sm_before);
        let before = guard_properties(&sm_before);

        // -- read + hash, streaming; never load the whole file
        let mut hasher = blake3::Hasher::new();
        let mut hashed_len: u64 = 0;
        let read_result = (|| -> std::io::Result<()> {
            let mut f = std::fs::File::open(path)?;
            let mut buf = vec![0u8; 64 * 1024];
            loop {
                let n = f.read(&mut buf)?;
                if n == 0 {
                    break;
                }
                hasher.update(&buf[..n]);
                hashed_len += n as u64;
            }
            Ok(())
        })();

        // -- stat after
        let sm_after = std::fs::symlink_metadata(path);
        match (&read_result, &sm_after) {
            (Err(e), _) => {
                // read failed; a NotFound mid-read is a stable negative answer
                let ce = map_io_ref(e);
                if matches!(ce, ContentError::NotFound) {
                    return error_obs(pid, ce.clone());
                }
                last_error = Some(ce.clone());
                last_pid = pid;
                continue; // retry once; a persistent read error ends Unstable-with-error
            }
            (Ok(()), Err(e)) => {
                // object vanished after we read it: the bytes we hashed were
                // real but the object is gone — Unstable, hash withheld.
                let _ = e;
                last_error = Some(ContentError::NotFound);
                last_pid = pid;
                last_deltas = vec![GuardDelta::PhysicalIdChanged];
                continue;
            }
            (Ok(()), Ok(sm_a)) => {
                let pid_after = PhysicalId::from_metadata(sm_a);
                let after = guard_properties(sm_a);
                let mut deltas = Vec::new();
                if before.0 != after.0 {
                    deltas.push(GuardDelta::SizeChanged);
                }
                if before.1 != after.1 {
                    deltas.push(GuardDelta::MtimeChanged);
                }
                if before.2 != after.2 {
                    deltas.push(GuardDelta::PhysicalIdChanged);
                }
                if deltas.is_empty() {
                    return ContentObservation {
                        physical_id: pid,
                        hash: Some(hasher.finalize().into()),
                        hashed_len: Some(hashed_len),
                        stability: Stability::Stable,
                        deltas: vec![],
                        error: None,
                    };
                }
                last_deltas = deltas;
                last_pid = if pid_after == pid { pid } else { pid_after };
                last_error = None;
                // fall through to next attempt
            }
        }
    }

    ContentObservation {
        physical_id: last_pid,
        hash: None,
        hashed_len: None,
        stability: Stability::Unstable,
        deltas: last_deltas,
        error: last_error,
    }
}

fn map_io(e: std::io::Error) -> ContentError {
    map_io_ref(&e).clone()
}
fn map_io_ref(e: &std::io::Error) -> ContentError {
    match e.kind() {
        std::io::ErrorKind::NotFound => ContentError::NotFound,
        std::io::ErrorKind::PermissionDenied => ContentError::PermissionDenied,
        _ => ContentError::ReadError(e.to_string()),
    }
}
fn error_obs(pid: PhysicalId, e: ContentError) -> ContentObservation {
    ContentObservation {
        physical_id: pid,
        hash: None,
        hashed_len: None,
        stability: Stability::Unstable,
        deltas: vec![],
        error: Some(e),
    }
}

impl PhysicalId {
    pub fn from_metadata(sm: &std::fs::Metadata) -> PhysicalId {
        #[cfg(unix)]
        {
            use std::os::unix::fs::MetadataExt as _;
            PhysicalId {
                dev: Some(sm.dev()),
                ino: Some(sm.ino()),
            }
        }
        #[cfg(not(unix))]
        {
            let _ = sm;
            PhysicalId {
                dev: None,
                ino: None,
            }
        }
    }
}
