//! Increment 1 — SCAN.
//!
//! Determinism contract (V0-IMPLEMENTATION-PLAN §3):
//!   A. observable state (path, kind, dev, ino, size, mtime) — must be equal
//!      across two scans of an unchanged tree;
//!   B. observation-event time (Scan::started_at) — expected to differ;
//!   C. enumeration order — never trusted; output is sorted by relative path.
//!
//! The scanner is read-only: it opens no files and modifies nothing.
//! Symlinks are observed as entries (lstat semantics), never followed, both
//! to avoid cycles and because following would observe a different tree than
//! the one on disk.

use crate::{Entry, EntryKind, PathError, Scan};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use walkdir::WalkDir;

#[cfg(unix)]
use std::os::unix::fs::MetadataExt as _;

#[derive(Debug)]
pub enum ScanError {
    RootMissing(PathBuf),
    RootNotADirectory(PathBuf),
    RootUnreadable(PathBuf, std::io::Error),
}

impl std::fmt::Display for ScanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScanError::RootMissing(p) => write!(f, "root does not exist: {}", p.display()),
            ScanError::RootNotADirectory(p) => {
                write!(f, "root is not a directory: {}", p.display())
            }
            ScanError::RootUnreadable(p, e) => write!(f, "root unreadable: {}: {}", p.display(), e),
        }
    }
}
impl std::error::Error for ScanError {}

/// Scan `root` recursively without following symlinks, producing a
/// deterministically ordered set of observation records.
pub fn scan(root: &Path) -> Result<Scan, ScanError> {
    let meta = match std::fs::symlink_metadata(root) {
        Ok(m) => m,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return Err(ScanError::RootMissing(root.to_path_buf()));
        }
        Err(e) => return Err(ScanError::RootUnreadable(root.to_path_buf(), e)),
    };
    if !meta.is_dir() {
        return Err(ScanError::RootNotADirectory(root.to_path_buf()));
    }

    let started_at = SystemTime::now();
    let mut entries = Vec::new();
    let mut errors = Vec::new();

    for item in WalkDir::new(root).follow_links(false).min_depth(1) {
        match item {
            Ok(dent) => {
                // lstat semantics: walkdir with follow_links(false) yields the
                // link itself, so symlink_metadata would be redundant — but we
                // go through fs::symlink_metadata on the path to keep the
                // observation independent of walkdir's internal behaviour.
                let sm = match std::fs::symlink_metadata(dent.path()) {
                    Ok(sm) => sm,
                    Err(e) => {
                        errors.push(PathError {
                            path: dent.path().to_path_buf(),
                            message: e.to_string(),
                        });
                        continue;
                    }
                };
                let kind = if sm.file_type().is_symlink() {
                    EntryKind::Symlink
                } else if sm.is_dir() {
                    EntryKind::Dir
                } else if sm.is_file() {
                    EntryKind::File
                } else {
                    EntryKind::Other
                };
                let mtime = sm.modified().ok().and_then(|t| {
                    t.duration_since(std::time::UNIX_EPOCH)
                        .ok()
                        .map(|d| (d.as_secs() as i64, d.subsec_nanos()))
                });
                entries.push(Entry {
                    path: dent
                        .path()
                        .strip_prefix(root)
                        .expect("walkdir path under root")
                        .to_path_buf(),
                    kind,
                    dev: dev_of(&sm),
                    ino: Some(sm.ino()),
                    size: Some(sm.len()),
                    mtime,
                });
            }
            Err(e) => {
                errors.push(PathError {
                    path: e.path().map(Path::to_path_buf).unwrap_or_default(),
                    message: e.to_string(),
                });
            }
        }
    }

    entries.sort_by(|a, b| a.path.cmp(&b.path));
    errors.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(Scan {
        root: root.to_path_buf(),
        started_at,
        entries,
        errors,
    })
}

#[cfg(unix)]
fn dev_of(sm: &std::fs::Metadata) -> Option<u64> {
    use std::os::unix::fs::MetadataExt;
    Some(sm.dev())
}

#[cfg(not(unix))]
fn dev_of(_sm: &std::fs::Metadata) -> Option<u64> {
    None // dev is not portable; absence is recorded, not invented
}
