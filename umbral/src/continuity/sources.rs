//! Git and the working copy: the only two things a continuity record captures.

use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use super::{io_err, Error, GitState, HeadState, Reason};
use crate::content::{self, ContentError, Stability};

/// Run git with separate arguments, no shell, no optional locks and no prompt. Neither
/// subcommand used here runs hooks or touches the network.
fn git(root: &Path, args: &[&str]) -> Result<std::process::Output, Error> {
    Command::new("git")
        .args(args)
        .current_dir(root)
        .env("GIT_OPTIONAL_LOCKS", "0")
        .env("GIT_TERMINAL_PROMPT", "0")
        .stdin(Stdio::null())
        .output()
        .map_err(|e| Error::NoGit(format!("cannot run git: {e}")))
}

/// The canonical root, which must be the toplevel of a git worktree.
pub(super) fn git_root(root: &Path) -> Result<PathBuf, Error> {
    let canonical = std::fs::canonicalize(root).map_err(|e| io_err("root", &e))?;
    let out = git(&canonical, &["rev-parse", "--show-toplevel"])?;
    if !out.status.success() {
        return Err(Error::NoGit(format!(
            "{} is not inside a git repository",
            canonical.display()
        )));
    }
    let top = out.stdout.strip_suffix(b"\n").unwrap_or(&out.stdout);
    let top = std::fs::canonicalize(OsStr::from_bytes(top)).map_err(|e| io_err("toplevel", &e))?;
    if top != canonical {
        return Err(Error::InvalidInput(format!(
            "the root must be the repository toplevel ({})",
            top.display()
        )));
    }
    Ok(canonical)
}

pub(super) fn git_head(root: &Path) -> Result<GitState, Error> {
    let out = git(root, &["rev-parse", "--verify", "-q", "HEAD"])?;
    if !out.status.success() {
        // The root is already known to be a repository: a HEAD that does not resolve is unborn.
        return Ok(GitState {
            head_state: HeadState::Unborn,
            head: None,
        });
    }
    let head = String::from_utf8_lossy(&out.stdout).trim().to_string();
    if !matches!(head.len(), 40 | 64) || !head.bytes().all(|c| c.is_ascii_hexdigit()) {
        return Err(Error::Io(format!("unexpected HEAD from git: {head}")));
    }
    Ok(GitState {
        head_state: HeadState::Present,
        head: Some(head),
    })
}

/// A relative source path as raw bytes, checked against contract §3.5.
pub(super) fn check_path(bytes: &[u8]) -> Result<(), String> {
    if bytes.is_empty() || bytes[0] == b'/' || bytes.contains(&0) {
        return Err("empty, absolute or containing NUL".into());
    }
    let parts: Vec<&[u8]> = bytes.split(|b| *b == b'/').collect();
    if parts
        .iter()
        .any(|p| p.is_empty() || *p == b"." || *p == b"..")
    {
        return Err("empty, '.' or '..' component".into());
    }
    if parts[0] == b".git" || parts[0] == b".umbral" {
        return Err("under .git/ or .umbral/".into());
    }
    Ok(())
}

/// What one read of a source found.
pub(super) enum Seen {
    Hash {
        hash: String,
        size: u64,
    },
    Absent,
    /// Not a readable regular file inside the root. Rejected at save; `unknown` at resume.
    Refused(Reason),
    Unknown {
        reason: Reason,
        deltas: Option<String>,
        detail: Option<String>,
    },
}

pub(super) fn inspect(root: &Path, rel: &[u8]) -> Seen {
    let parts: Vec<&[u8]> = rel.split(|b| *b == b'/').collect();
    let mut path = root.to_path_buf();
    for (i, part) in parts.iter().enumerate() {
        path.push(OsStr::from_bytes(part));
        if i + 1 == parts.len() {
            break;
        }
        match std::fs::symlink_metadata(&path) {
            Ok(m) if m.file_type().is_symlink() => return Seen::Refused(Reason::OutsideRoot),
            Ok(m) if !m.is_dir() => return Seen::Absent,
            Ok(_) => {}
            Err(e) => return from_error(&content_error(&e)),
        }
    }
    match std::fs::symlink_metadata(&path) {
        Ok(m) if m.file_type().is_symlink() => return Seen::Refused(Reason::Symlink),
        Ok(m) if !m.is_file() => return Seen::Refused(Reason::NotARegularFile),
        Ok(_) => {}
        Err(e) => return from_error(&content_error(&e)),
    }
    let obs = content::observe_content(&path);
    if let (Some(h), Some(size)) = (obs.valid_hash(), obs.hashed_len) {
        return Seen::Hash {
            hash: format!("blake3:{}", content::hex(h)),
            size,
        };
    }
    if obs.stability == Some(Stability::Unstable) {
        return Seen::Unknown {
            reason: Reason::UnstableRead,
            deltas: obs.deltas_str(),
            detail: None,
        };
    }
    match &obs.error {
        Some(e) => from_error(e),
        None => Seen::Unknown {
            reason: Reason::ReadError,
            deltas: None,
            detail: None,
        },
    }
}

fn content_error(e: &std::io::Error) -> ContentError {
    match e.kind() {
        std::io::ErrorKind::NotFound => ContentError::NotFound,
        std::io::ErrorKind::PermissionDenied => ContentError::PermissionDenied,
        _ => ContentError::ReadError(e.to_string()),
    }
}

fn from_error(e: &ContentError) -> Seen {
    let unknown = |reason, detail| Seen::Unknown {
        reason,
        deltas: None,
        detail,
    };
    match e {
        ContentError::NotFound => Seen::Absent,
        ContentError::NotARegularFile => Seen::Refused(Reason::NotARegularFile),
        ContentError::PermissionDenied => unknown(Reason::PermissionDenied, None),
        ContentError::ReadError(m) => unknown(Reason::ReadError, Some(m.clone())),
    }
}
