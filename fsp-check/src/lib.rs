//! fsp-check — V0 observation prototype (increment 1: SCAN).
//!
//! Scope guard (H25): this store/prototype is NOT Project Reality.
//! Increment boundary: observation only. No identity resolution, no hashing,
//! no persistence, no reconcile — those belong to later increments.

pub mod scan;

/// A filesystem observation record. Pure data: the future reconciler consumes
/// this, never walkdir or syscalls (plan §12 portability boundary).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    /// Path relative to the scan root, as observed (never lossily converted).
    pub path: std::path::PathBuf,
    pub kind: EntryKind,
    /// Physical metadata for the next increments. `None` = could not be
    /// obtained reliably — recorded absence, never an invented value.
    pub dev: Option<u64>,
    pub ino: Option<u64>,
    pub size: Option<u64>,
    /// Modification time as (seconds, nanoseconds) since the Unix epoch.
    /// Belongs to the OBSERVED STATE (A), not to the observation event (B):
    /// two scans of an unchanged tree must carry equal mtimes.
    pub mtime: Option<(i64, u32)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryKind {
    File,
    Dir,
    Symlink,
    Other,
}

/// A path that could not be observed at all (permission, race, ...).
/// Represented explicitly; the scanner never drops it silently (INV-4 seed).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathError {
    pub path: std::path::PathBuf,
    pub message: String,
}

/// Result of one scan. `started_at` is observation-event time (B); it is kept
/// OUT of `Entry` so that deterministic equivalence of two scans of an
/// unchanged tree compares only observable state (A).
#[derive(Debug, Clone)]
pub struct Scan {
    pub root: std::path::PathBuf,
    pub started_at: std::time::SystemTime,
    /// Deterministically ordered by `path` (ascending, `PathBuf: Ord` —
    /// component-wise, total, lossless). The scanner does not rely on the
    /// order walkdir happens to return (plan §3-C).
    pub entries: Vec<Entry>,
    pub errors: Vec<PathError>,
}

impl Scan {
    /// Deterministic equivalence of two scans of the same tree: same entry
    /// set with the same observable state and the same order, and no
    /// difference in observed errors. Deliberately ignores `started_at`.
    pub fn equivalent_observable_state(&self, other: &Scan) -> bool {
        self.entries == other.entries && self.errors == other.errors
    }
}

#[cfg(unix)]
pub fn os_str_bytes(s: &std::ffi::OsStr) -> &[u8] {
    use std::os::unix::ffi::OsStrExt;
    s.as_bytes()
}

#[cfg(not(unix))]
pub fn os_str_bytes(s: &std::ffi::OsStr) -> Vec<u8> {
    s.as_encoded_bytes().to_vec()
}
