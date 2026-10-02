//! Workspace resolution: one observed root plus its local record.
//!
//! # Where state lives
//!
//! `$XDG_DATA_HOME/umbral/ws-<id>/`, falling back to `~/.local/share/umbral/ws-<id>/`.
//! Never inside the observed tree. The state directory is the only thing this program ever
//! writes to.
//!
//! # How the identifier is derived, exactly
//!
//! 1. The root is canonicalised with `std::fs::canonicalize`, which makes it absolute and
//!    resolves symlinks.
//! 2. The canonical path's **raw OS bytes** are taken (on Unix, `OsStr::as_bytes` — no lossy
//!    UTF-8 conversion, so a path with non-UTF-8 bytes still yields a stable identifier).
//! 3. `BLAKE3` is computed over exactly those bytes.
//! 4. The first 8 bytes of the digest are rendered as 16 lowercase hex characters, and the
//!    directory is named `ws-<those 16 characters>`.
//!
//! Consequences, stated plainly:
//!
//! - The same canonical root always produces the same identifier.
//! - Moving or renaming the root produces a different identifier, so it becomes a different
//!   workspace with an empty history. This is a deliberate consequence of not writing
//!   anything into the user's tree: there is no marker file to follow.
//! - Nothing is written inside the root, not even to identify it.
//!
//! # What this is NOT
//!
//! This is v0.1's mechanism for locating a workspace's local record. It is **not** the
//! definitive global identity of a workspace in Umbral, it is not a content identifier, and
//! it is replaceable. It is not an architecture decision.

use std::path::{Path, PathBuf};

pub const TOOL_VERSION: &str = env!("CARGO_PKG_VERSION");

/// The version of the workspace record format (`workspace.json`). Versioned separately from
/// the log schema: the log moved to `umbral-v0.1.1` without the record changing, and one
/// number must not serve two formats.
pub const RECORD_VERSION: &str = "umbral-v0.1";
const WORKSPACE_FILE: &str = "workspace.json";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Workspace {
    /// The root as the user gave it.
    pub root: PathBuf,
    /// The canonicalised root — the basis of the identifier.
    pub canonical: PathBuf,
    /// The 16-hex-character identifier.
    pub id: String,
    /// Where this workspace's record lives. Outside `root`, always.
    pub state_dir: PathBuf,
}

#[derive(Debug)]
pub enum WorkspaceError {
    RootMissing(PathBuf),
    NotADirectory(PathBuf),
    AlreadyInitialised(PathBuf),
    NotInitialised(PathBuf),
    UnknownSchema(String),
    Io(std::io::Error),
    Json(String),
}

impl std::fmt::Display for WorkspaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkspaceError::RootMissing(p) => write!(f, "root does not exist: {}", p.display()),
            WorkspaceError::NotADirectory(p) => {
                write!(f, "root is not a directory: {}", p.display())
            }
            WorkspaceError::AlreadyInitialised(p) => {
                write!(f, "workspace already initialised: {}", p.display())
            }
            WorkspaceError::NotInitialised(p) => {
                write!(f, "workspace is not initialised: {}", p.display())
            }
            WorkspaceError::UnknownSchema(v) => {
                write!(f, "unknown state schema version: {v}")
            }
            WorkspaceError::Io(e) => write!(f, "io: {e}"),
            WorkspaceError::Json(m) => write!(f, "malformed workspace record: {m}"),
        }
    }
}

impl std::error::Error for WorkspaceError {}

impl From<std::io::Error> for WorkspaceError {
    fn from(e: std::io::Error) -> Self {
        WorkspaceError::Io(e)
    }
}

/// `$XDG_DATA_HOME/umbral`, or `~/.local/share/umbral`.
pub fn state_root() -> Result<PathBuf, WorkspaceError> {
    if let Some(dir) = std::env::var_os("XDG_DATA_HOME") {
        if !dir.is_empty() {
            return Ok(PathBuf::from(dir).join("umbral"));
        }
    }
    let home = std::env::var_os("HOME")
        .ok_or_else(|| WorkspaceError::Io(std::io::Error::other("HOME is not set")))?;
    Ok(PathBuf::from(home)
        .join(".local")
        .join("share")
        .join("umbral"))
}

/// The canonical bytes of a root. Raw OS bytes, never lossily converted.
fn canonical_bytes(canonical: &Path) -> Vec<u8> {
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStrExt;
        canonical.as_os_str().as_bytes().to_vec()
    }
    #[cfg(not(unix))]
    {
        canonical.to_string_lossy().as_bytes().to_vec()
    }
}

/// The 16-hex-character workspace identifier for a canonical root.
pub fn workspace_id(canonical: &Path) -> String {
    let digest = blake3::hash(&canonical_bytes(canonical));
    digest.as_bytes()[..8]
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

pub fn state_dir_for(canonical: &Path) -> Result<PathBuf, WorkspaceError> {
    Ok(state_root()?.join(format!("ws-{}", workspace_id(canonical))))
}

fn canonicalise(root: &Path) -> Result<PathBuf, WorkspaceError> {
    let meta = std::fs::symlink_metadata(root).map_err(|e| match e.kind() {
        std::io::ErrorKind::NotFound => WorkspaceError::RootMissing(root.to_path_buf()),
        _ => WorkspaceError::Io(e),
    })?;
    if !meta.is_dir() {
        return Err(WorkspaceError::NotADirectory(root.to_path_buf()));
    }
    Ok(std::fs::canonicalize(root)?)
}

/// The JSON record written at `init`. Deliberately minimal: it exists so `workspaces` can
/// list what exists, and so `open` can detect a version it does not understand.
///
/// `canonical_hex` carries the canonical root's raw bytes, so a root that is not valid UTF-8
/// is listed exactly; the text fields are for a human reading the file (D-V01-12). Records
/// written before that field existed are still read, from the unescaped text.
fn render_record(ws: &Workspace, created_at_ns: i64) -> String {
    format!(
        "{{\n  \"root\": \"{}\",\n  \"canonical\": \"{}\",\n  \"canonical_hex\": \"{}\",\n  \"id\": \"{}\",\n  \"created_at_ns\": {},\n  \"tool_version\": \"{}\",\n  \"schema_version\": \"{}\"\n}}\n",
        json_escape(&ws.root.to_string_lossy()),
        json_escape(&ws.canonical.to_string_lossy()),
        crate::content::hex(&canonical_bytes(&ws.canonical)),
        ws.id,
        created_at_ns,
        TOOL_VERSION,
        RECORD_VERSION
    )
}

/// JSON string escaping: quote, backslash and every control character.
fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

fn bytes_from_hex(h: &str) -> Option<Vec<u8>> {
    if !h.len().is_multiple_of(2) {
        return None;
    }
    (0..h.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(h.get(i..i + 2)?, 16).ok())
        .collect()
}

fn path_from_bytes(b: Vec<u8>) -> PathBuf {
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStringExt;
        PathBuf::from(std::ffi::OsString::from_vec(b))
    }
    #[cfg(not(unix))]
    {
        PathBuf::from(String::from_utf8_lossy(&b).into_owned())
    }
}

fn json_int_field(text: &str, field: &str) -> Option<i64> {
    let key = format!("\"{field}\"");
    let start = text.find(&key)? + key.len();
    let rest = &text[start..];
    let colon = rest.find(':')? + 1;
    let rest = rest[colon..].trim_start();
    let digits: String = rest
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == '-')
        .collect();
    digits.parse().ok()
}

/// The value of a string field, unescaped. Reads to the closing quote, not to the first
/// quote character, so an escaped `\"` inside the value does not end it (D-V01-12).
fn json_string_field(text: &str, field: &str) -> Option<String> {
    let key = format!("\"{field}\"");
    let start = text.find(&key)? + key.len();
    let rest = &text[start..];
    let colon = rest.find(':')? + 1;
    let rest = rest[colon..].trim_start();
    let rest = rest.strip_prefix('"')?;
    let mut out = String::new();
    let mut chars = rest.chars();
    loop {
        match chars.next()? {
            '"' => return Some(out),
            '\\' => match chars.next()? {
                'n' => out.push('\n'),
                'r' => out.push('\r'),
                't' => out.push('\t'),
                'u' => {
                    let code: String = chars.by_ref().take(4).collect();
                    out.push(char::from_u32(u32::from_str_radix(&code, 16).ok()?)?);
                }
                other => out.push(other),
            },
            c => out.push(c),
        }
    }
}

/// Create a workspace record. Refuses to overwrite an existing one — re-initialising in
/// silence would discard a history without saying so.
pub fn init(root: &Path) -> Result<Workspace, WorkspaceError> {
    let canonical = canonicalise(root)?;
    let state_dir = state_dir_for(&canonical)?;
    if state_dir.join(WORKSPACE_FILE).exists() {
        return Err(WorkspaceError::AlreadyInitialised(root.to_path_buf()));
    }
    std::fs::create_dir_all(&state_dir)?;

    let ws = Workspace {
        root: root.to_path_buf(),
        canonical: canonical.clone(),
        id: workspace_id(&canonical),
        state_dir,
    };

    let now_ns = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as i64)
        .unwrap_or(0);

    std::fs::write(
        ws.state_dir.join(WORKSPACE_FILE),
        render_record(&ws, now_ns),
    )?;
    Ok(ws)
}

/// Open an existing workspace. Never creates one.
pub fn open(root: &Path) -> Result<Workspace, WorkspaceError> {
    let canonical = canonicalise(root)?;
    let state_dir = state_dir_for(&canonical)?;
    let record_path = state_dir.join(WORKSPACE_FILE);
    if !record_path.exists() {
        return Err(WorkspaceError::NotInitialised(root.to_path_buf()));
    }
    let text = std::fs::read_to_string(&record_path)?;
    let schema = json_string_field(&text, "schema_version")
        .ok_or_else(|| WorkspaceError::Json("missing schema_version".into()))?;
    if schema != RECORD_VERSION {
        return Err(WorkspaceError::UnknownSchema(schema));
    }
    Ok(Workspace {
        root: root.to_path_buf(),
        canonical: canonical.clone(),
        id: workspace_id(&canonical),
        state_dir,
    })
}

impl Workspace {
    /// The log file for this workspace.
    pub fn log_path(&self) -> PathBuf {
        self.state_dir.join("observations.sqlite")
    }

    /// The workspace record file.
    pub fn record_path(&self) -> PathBuf {
        self.state_dir.join(WORKSPACE_FILE)
    }
}

/// A workspace that exists on disk, as reported by `workspaces`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceEntry {
    pub id: String,
    pub canonical: PathBuf,
    /// `None` when the record carries no creation time — absence, not 1970.
    pub created_at_ns: Option<i64>,
    pub tool_version: String,
}

/// Every workspace record under the state root. Read-only.
pub fn list() -> Result<Vec<WorkspaceEntry>, WorkspaceError> {
    let root = state_root()?;
    if !root.exists() {
        return Ok(Vec::new());
    }
    let mut out: Vec<WorkspaceEntry> = Vec::new();
    for entry in std::fs::read_dir(&root)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().to_string();
        let Some(id) = name.strip_prefix("ws-") else {
            continue;
        };
        let record = entry.path().join(WORKSPACE_FILE);
        if !record.exists() {
            continue;
        }
        let text = std::fs::read_to_string(&record)?;
        let canonical =
            match json_string_field(&text, "canonical_hex").and_then(|h| bytes_from_hex(&h)) {
                Some(bytes) => path_from_bytes(bytes),
                None => match json_string_field(&text, "canonical") {
                    Some(c) => PathBuf::from(c),
                    None => continue,
                },
            };
        let created_at_ns = json_int_field(&text, "created_at_ns");
        let tool_version = json_string_field(&text, "tool_version").unwrap_or_default();
        out.push(WorkspaceEntry {
            id: id.to_string(),
            canonical,
            created_at_ns,
            tool_version,
        });
    }
    out.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(out)
}
