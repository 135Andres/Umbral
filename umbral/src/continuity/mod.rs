//! Task continuity records, edition `umbral-continuity/1` (UD-040).
//!
//! A record is one immutable JSON file per revision, inside the repository under
//! `.umbral/continuity/`. What the emitter declares is kept apart from what Umbral captured,
//! and nothing declared is ever verified. This is the only part of the crate that writes
//! inside the project, and it writes nowhere but `.umbral/`.

mod ops;
mod sources;
mod store;

pub use ops::{export, list, resume, save, Expected, SaveInput};

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::{json, Value};

pub const EDITION: &str = "umbral-continuity/1";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Revision {
    pub format: String,
    pub task: String,
    pub parents: Vec<String>,
    pub saved_at: String,
    pub emitter: Emitter,
    pub goal: String,
    pub next_step: String,
    pub blockers: Vec<String>,
    pub declarations: Vec<Declaration>,
    pub git: GitState,
    pub sources: Vec<Source>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Channel {
    Cli,
    Mcp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Emitter {
    pub channel: Channel,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Kind {
    Constraint,
    Decision,
    Check,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Origin {
    Agent,
    HumanStated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ClaimedResult {
    Passed,
    Failed,
    NotRun,
    Unknown,
}

/// `command`, `claimed_result` and `evidence` exist only on a `check`; there `command` and
/// `evidence` may be `null`. The outer `Option` is presence of the key, the inner one `null`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Declaration {
    pub id: String,
    pub kind: Kind,
    pub claimed_origin: Origin,
    pub text: String,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    pub command: Option<Option<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claimed_result: Option<ClaimedResult>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "present"
    )]
    pub evidence: Option<Option<String>>,
    pub depends_on: Vec<String>,
}

fn present<'de, D: Deserializer<'de>, T: Deserialize<'de>>(d: D) -> Result<Option<T>, D::Error> {
    T::deserialize(d).map(Some)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HeadState {
    Present,
    Unborn,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GitState {
    pub head_state: HeadState,
    pub head: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Capture {
    Captured,
    Unknown,
}

/// Why a source could not be compared (contract §4). Also the reason a capture is `unknown`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Reason {
    PermissionDenied,
    UnstableRead,
    NotARegularFile,
    Symlink,
    OutsideRoot,
    ReadError,
    NoRecordedHash,
}

impl Reason {
    pub fn as_str(self) -> &'static str {
        match self {
            Reason::PermissionDenied => "permission-denied",
            Reason::UnstableRead => "unstable-read",
            Reason::NotARegularFile => "not-a-regular-file",
            Reason::Symlink => "symlink",
            Reason::OutsideRoot => "outside-root",
            Reason::ReadError => "read-error",
            Reason::NoRecordedHash => "no-recorded-hash",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Source {
    pub path: String,
    pub capture: Capture,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<Reason>,
    pub captured_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    InvalidInput(String),
    LimitExceeded {
        field: String,
        limit: usize,
        actual: usize,
    },
    /// Each rejected source and why.
    SourceRejected(Vec<(String, &'static str)>),
    RevisionConflict {
        heads: Vec<String>,
    },
    /// More than one head. `revision` is the one just written, when a save caused it.
    Fork {
        revision: Option<String>,
        heads: Vec<(String, String)>,
        /// The save that forked also created the state: the warning is still owed (B-07).
        privacy_warning: bool,
    },
    Unsafe(String),
    NoGit(String),
    /// `registro-alterado`, `registro-invalido` or `formato-desconocido`.
    Record {
        token: &'static str,
        task: String,
        file: String,
        reason: String,
    },
    NoSuchRevision(String),
    Io(String),
}

impl Error {
    pub fn token(&self) -> &'static str {
        match self {
            Error::InvalidInput(_) => "entrada-invalida",
            Error::LimitExceeded { .. } => "limite-excedido",
            Error::SourceRejected(_) => "fuente-rechazada",
            Error::RevisionConflict { .. } => "conflicto-revision",
            Error::Fork { .. } => "bifurcacion",
            Error::Unsafe(_) => "estado-inseguro",
            Error::NoGit(_) => "sin-git",
            Error::Record { token, .. } => token,
            Error::NoSuchRevision(_) => "revision-inexistente",
            Error::Io(_) => "io",
        }
    }

    pub fn exit_code(&self) -> u8 {
        match self {
            Error::Io(_) | Error::Unsafe(_) | Error::NoGit(_) => 1,
            Error::InvalidInput(_) | Error::LimitExceeded { .. } | Error::SourceRejected(_) => 2,
            Error::RevisionConflict { .. } | Error::Fork { .. } => 4,
            Error::Record { .. } | Error::NoSuchRevision(_) => 5,
        }
    }

    /// The error document written to stdout.
    pub fn to_json(&self) -> Value {
        let mut v =
            json!({ "contract": EDITION, "error": self.token(), "message": self.to_string() });
        let extra = match self {
            Error::LimitExceeded {
                field,
                limit,
                actual,
            } => json!({ "field": field, "limit": limit, "actual": actual }),
            Error::SourceRejected(list) => json!({ "sources": list
                .iter()
                .map(|(p, r)| json!({ "path": p, "reason": r }))
                .collect::<Vec<_>>() }),
            Error::RevisionConflict { heads } => json!({ "heads": heads }),
            Error::Fork {
                revision,
                heads,
                privacy_warning,
            } => {
                let mut v = json!({
                    "revision": revision,
                    "heads": heads
                        .iter()
                        .map(|(r, t)| json!({ "revision": r, "saved_at": t }))
                        .collect::<Vec<_>>(),
                });
                if *privacy_warning {
                    v["privacy_warning"] = json!(store::PRIVACY_WARNING);
                }
                v
            }
            Error::Record {
                task, file, reason, ..
            } => json!({ "task": task, "file": file, "reason": reason }),
            _ => json!({}),
        };
        if let (Some(o), Value::Object(e)) = (v.as_object_mut(), extra) {
            o.extend(e);
        }
        v
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidInput(m) => write!(f, "invalid input: {m}"),
            Error::LimitExceeded {
                field,
                limit,
                actual,
            } => write!(f, "{field} exceeds its limit: {actual} > {limit}"),
            Error::SourceRejected(list) => {
                write!(f, "source(s) rejected:")?;
                for (p, r) in list {
                    write!(f, " {p} ({r})")?;
                }
                Ok(())
            }
            Error::RevisionConflict { heads } => write!(
                f,
                "expected revision is not the current head; head(s): {}",
                heads.join(", ")
            ),
            Error::Fork {
                revision, heads, ..
            } => {
                if let Some(r) = revision {
                    write!(f, "revision {r} was written, but ")?;
                }
                write!(f, "the task has {} heads; none is chosen", heads.len())
            }
            Error::Unsafe(m) => write!(f, "unsafe state: {m}"),
            Error::NoGit(m) => write!(f, "no git repository: {m}"),
            Error::Record {
                token,
                task,
                file,
                reason,
            } => write!(f, "{token}: task {task}, {file}: {reason}"),
            Error::NoSuchRevision(r) => write!(f, "no such revision: {r}"),
            Error::Io(m) => write!(f, "i/o error: {m}"),
        }
    }
}

pub(crate) fn io_err(what: &str, e: &std::io::Error) -> Error {
    Error::Io(format!("{what}: {e}"))
}

/// `^[a-z0-9][a-z0-9-]{0,max-1}$`
pub(crate) fn is_slug(s: &str, max: usize) -> bool {
    let b = s.as_bytes();
    !b.is_empty()
        && b.len() <= max
        && b[0] != b'-'
        && b.iter()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || *c == b'-')
}

pub fn is_task_id(s: &str) -> bool {
    is_slug(s, 64)
}

pub fn is_revision_id(s: &str) -> bool {
    s.len() == 32
        && s.bytes()
            .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
}
