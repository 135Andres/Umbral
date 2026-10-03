//! Deterministic, read-only filesystem observation.
//!
//! Guarantees this module exists to provide:
//!
//! - **Deterministic order.** Entries are sorted by relative path (`PathBuf: Ord`,
//!   component-wise, total, lossless). The order the walker happens to return is never
//!   relied upon.
//! - **Read-only.** Nothing here creates, writes, renames or deletes anything.
//! - **Symlinks are observed, not traversed.** A symlink is an entry; its target is not
//!   visited through it.
//! - **Errors are represented, never dropped.** A path that could not be observed appears
//!   in [`Scan::errors`]. Its presence is what makes a scan *incomplete*, and an incomplete
//!   scan is what prevents "not seen" from being reported as "deleted".
//!
//! Observation-event time ([`Scan::started_at`]) is deliberately kept OUT of [`Entry`], so
//! that comparing two scans of an unchanged tree compares observable state only.

use std::path::{Path, PathBuf};
use std::time::SystemTime;

use walkdir::WalkDir;

use crate::identity::{ctime_of, mtime_of, physical_id_of};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EntryKind {
    File,
    Dir,
    Symlink,
    Other,
}

impl EntryKind {
    pub fn as_str(self) -> &'static str {
        match self {
            EntryKind::File => "file",
            EntryKind::Dir => "dir",
            EntryKind::Symlink => "symlink",
            EntryKind::Other => "other",
        }
    }

    pub fn parse(s: &str) -> EntryKind {
        match s {
            "file" => EntryKind::File,
            "dir" => EntryKind::Dir,
            "symlink" => EntryKind::Symlink,
            _ => EntryKind::Other,
        }
    }
}

/// One observed entry. Every field is what the filesystem reported at observation time;
/// `None` means it was not obtainable, and is recorded as absence rather than filled in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entry {
    /// Path relative to the scan root, as observed (raw bytes; never lossily converted).
    pub path: PathBuf,
    pub kind: EntryKind,
    pub dev: Option<u64>,
    pub ino: Option<u64>,
    pub size: Option<u64>,
    /// (seconds, nanoseconds) since the Unix epoch — part of the OBSERVED STATE, so two
    /// scans of an unchanged tree carry equal mtimes.
    pub mtime: Option<(i64, u32)>,
    /// (seconds, nanoseconds) of the inode's last status change — the one timestamp a writer
    /// cannot set (`UD-018`). Part of the skip condition (`UD-035`). `None` where the platform
    /// does not report it.
    pub ctime: Option<(i64, u32)>,
}

/// The scope rules this build applies to every run, recorded with each run (`UD-037`): the
/// whole tree below the root, symlinks observed as entries and never followed, nothing
/// excluded.
pub const SCOPE: &str = "recursive,symlinks-not-followed,no-exclusions";

/// Which part of the scope a traversal failure leaves unobserved (`UD-025`, `UD-037`). Decided
/// when the failure happens, never inferred later from its message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TraversalClass {
    /// A directory whose contents could not be listed: everything below it is unobserved.
    NotDescended,
    /// An entry that could not be `lstat`ed: only that entry is unobserved.
    MetadataFailed,
}

impl TraversalClass {
    pub fn as_str(self) -> &'static str {
        match self {
            TraversalClass::NotDescended => "not-descended",
            TraversalClass::MetadataFailed => "metadata-failed",
        }
    }
}

/// The class of a failure at `path`: `NotDescended` when the scan observed a directory there
/// (it was listed, then could not be read), `MetadataFailed` otherwise.
pub fn classify(path: &Path, entries: &[Entry]) -> TraversalClass {
    if entries
        .iter()
        .any(|e| e.path == path && e.kind == EntryKind::Dir)
    {
        TraversalClass::NotDescended
    } else {
        TraversalClass::MetadataFailed
    }
}

/// A path that could not be observed at all. Represented explicitly; never dropped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathError {
    pub path: PathBuf,
    pub message: String,
    pub class: TraversalClass,
}

/// The result of one scan. The root itself is not an entry.
#[derive(Debug, Clone)]
pub struct Scan {
    pub root: PathBuf,
    pub started_at: SystemTime,
    pub entries: Vec<Entry>,
    pub errors: Vec<PathError>,
    /// The root itself could not be listed: nothing below it was observed. A fact of the run,
    /// not of an entry.
    pub root_error: Option<String>,
}

impl Scan {
    /// A scan is complete when every path it tried to observe was observed. Any per-path
    /// error makes it incomplete — deliberately conservative: a partially-seen tree must
    /// not be used to conclude that something was deleted.
    pub fn complete(&self) -> bool {
        self.errors.is_empty() && self.root_error.is_none()
    }

    /// Deterministic equivalence of two scans of the same tree: same entries with the same
    /// observable state and the same order, and the same observed errors. Ignores
    /// `started_at` on purpose.
    pub fn equivalent_observable_state(&self, other: &Scan) -> bool {
        self.entries == other.entries
            && self.errors == other.errors
            && self.root_error == other.root_error
    }
}

#[derive(Debug)]
pub enum ScanError {
    /// The root does not exist.
    RootMissing(PathBuf),
    /// The root exists but is not a directory.
    NotADirectory(PathBuf),
    Io(std::io::Error),
}

impl std::fmt::Display for ScanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScanError::RootMissing(p) => write!(f, "root does not exist: {}", p.display()),
            ScanError::NotADirectory(p) => write!(f, "root is not a directory: {}", p.display()),
            ScanError::Io(e) => write!(f, "io: {e}"),
        }
    }
}

impl std::error::Error for ScanError {}

/// Observe `root` and everything below it.
///
/// The root is not included as an entry: entries are the contents of the root, so an empty
/// directory yields zero entries and a complete scan.
pub fn scan(root: &Path) -> Result<Scan, ScanError> {
    let meta = std::fs::symlink_metadata(root).map_err(|e| match e.kind() {
        std::io::ErrorKind::NotFound => ScanError::RootMissing(root.to_path_buf()),
        _ => ScanError::Io(e),
    })?;
    if !meta.is_dir() {
        return Err(ScanError::NotADirectory(root.to_path_buf()));
    }

    let started_at = SystemTime::now();
    let mut entries: Vec<Entry> = Vec::new();
    let mut errors: Vec<PathError> = Vec::new();
    // Failures reported by the walker are classified once every entry is known: the directory
    // a listing failure belongs to may be reported before or after it.
    let mut walk_failures: Vec<(PathBuf, String)> = Vec::new();
    let mut root_error: Option<String> = None;

    for item in WalkDir::new(root).follow_links(false).min_depth(1) {
        match item {
            Ok(de) => {
                let full = de.path();
                let rel = full
                    .strip_prefix(root)
                    .map(Path::to_path_buf)
                    .unwrap_or_else(|_| full.to_path_buf());
                // `symlink_metadata` (not `metadata`): a symlink is observed as itself.
                match std::fs::symlink_metadata(full) {
                    Ok(m) => entries.push(entry_from(rel, &m)),
                    Err(e) => errors.push(PathError {
                        path: rel,
                        message: e.to_string(),
                        class: TraversalClass::MetadataFailed,
                    }),
                }
            }
            Err(e) => {
                let rel = e
                    .path()
                    .and_then(|p| p.strip_prefix(root).ok())
                    .map(Path::to_path_buf)
                    .unwrap_or_default();
                if rel.as_os_str().is_empty() {
                    root_error = Some(e.to_string());
                } else {
                    walk_failures.push((rel, e.to_string()));
                }
            }
        }
    }

    for (path, message) in walk_failures {
        let class = classify(&path, &entries);
        errors.push(PathError {
            path,
            message,
            class,
        });
    }
    entries.sort_by(|a, b| a.path.cmp(&b.path));
    errors.sort_by(|a, b| a.path.cmp(&b.path));

    Ok(Scan {
        root: root.to_path_buf(),
        started_at,
        entries,
        errors,
        root_error,
    })
}

fn entry_from(path: PathBuf, m: &std::fs::Metadata) -> Entry {
    let ft = m.file_type();
    let kind = if ft.is_symlink() {
        EntryKind::Symlink
    } else if ft.is_dir() {
        EntryKind::Dir
    } else if ft.is_file() {
        EntryKind::File
    } else {
        EntryKind::Other
    };

    let pid = physical_id_of(m);

    Entry {
        path,
        kind,
        dev: pid.map(|p| p.dev),
        ino: pid.map(|p| p.ino),
        size: Some(m.len()),
        mtime: mtime_of(m),
        ctime: ctime_of(m),
    }
}
