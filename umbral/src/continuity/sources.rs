//! Git and the working copy: the only two things a continuity record captures.

use std::ffi::OsStr;
use std::io::Write;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use super::{io_err, Error, GitState, HeadState, Reason};
use crate::content::{self, ContentError, Stability};

/// Run git with separate arguments, no shell, no optional locks and no prompt. No
/// subcommand used here (`rev-parse`, `check-ignore`) runs hooks or touches the network.
/// Reading the index runs `core.fsmonitor`, a program the observed repository names: it is
/// switched off on the command line, which outranks every config file (B-01).
fn git(root: &Path, args: &[&str], input: &[u8]) -> Result<std::process::Output, Error> {
    let mut child = Command::new("git")
        .args(["-c", "core.fsmonitor=false"])
        .args(args)
        .current_dir(root)
        .env("GIT_OPTIONAL_LOCKS", "0")
        .env("GIT_TERMINAL_PROMPT", "0")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| Error::NoGit(format!("cannot run git: {e}")))?;
    let mut stdin = child.stdin.take();
    // Written from another thread, so a full stdout pipe cannot block the write.
    std::thread::scope(|s| {
        s.spawn(move || {
            if let Some(pipe) = stdin.as_mut() {
                // A write error shows up as git's own failure.
                let _ = pipe.write_all(input);
            }
        });
        child.wait_with_output()
    })
    .map_err(|e| Error::NoGit(format!("cannot run git: {e}")))
}

/// The canonical root, which must be the toplevel of a git worktree.
pub(super) fn git_root(root: &Path) -> Result<PathBuf, Error> {
    let canonical = std::fs::canonicalize(root).map_err(|e| io_err("root", &e))?;
    let out = git(&canonical, &["rev-parse", "--show-toplevel"], b"")?;
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
    let out = git(root, &["rev-parse", "--verify", "-q", "HEAD"], b"")?;
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

/// Which of `paths` (raw bytes, relative to the root) Git ignores (contract P6). A tracked
/// file is not ignored: its content is already versioned.
pub(super) fn git_ignored(root: &Path, paths: &[Vec<u8>]) -> Result<Vec<Vec<u8>>, Error> {
    if paths.is_empty() {
        return Ok(Vec::new());
    }
    // `./` keeps a name such as `:(exclude)x` from being read as pathspec magic; git echoes
    // each ignored path as it was given.
    let input: Vec<u8> = paths
        .iter()
        .flat_map(|p| b"./".iter().chain(p).chain(&[0]))
        .copied()
        .collect();
    let out = git(root, &["check-ignore", "-z", "--stdin"], &input)?;
    // 0: some ignored, 1: none ignored, anything else: git failed.
    match out.status.code() {
        Some(0 | 1) => {}
        _ => {
            return Err(Error::Io(format!(
                "git check-ignore failed: {}",
                String::from_utf8_lossy(&out.stderr).trim()
            )))
        }
    }
    Ok(out
        .stdout
        .split(|b| *b == 0)
        .filter_map(|p| p.strip_prefix(b"./"))
        .map(<[u8]>::to_vec)
        .collect())
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
