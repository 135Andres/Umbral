//! The record store: `<root>/.umbral/continuity/tasks/<task>/<revision>.json`.
//!
//! Files are never rewritten. A revision is written to `tmp/` and hard-linked into place,
//! which fails rather than overwrite. Reads never create anything.

use std::collections::BTreeSet;
use std::io::{Read, Write};
use std::os::fd::OwnedFd;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use rustix::fs::{fsync, linkat, mkdirat, openat, unlinkat, AtFlags, Mode, OFlags, CWD};
use rustix::io::Errno;
use serde_json::Value;

use super::{io_err, Error, Revision, EDITION};

const STATE_DIR: &str = ".umbral";
const GITATTRIBUTES: &[u8] = b"* -text\n";
const GITIGNORE: &[u8] = b"continuity/tmp/\n";

pub(super) const PRIVACY_WARNING: &str = "Records in .umbral/ are versioned with Git and may be \
published. Do not write secrets or personal data in them. Umbral stores paths and hashes, never \
file contents.";

pub(super) fn revision_id(bytes: &[u8]) -> String {
    crate::content::hex(&blake3::hash(bytes).as_bytes()[..16])
}

/// Canonical serialization (contract §2.1): 2-space indent, the declared key order, LF.
pub(super) fn serialize(r: &Revision) -> Result<Vec<u8>, Error> {
    let mut bytes =
        serde_json::to_vec_pretty(r).map_err(|e| Error::Io(format!("serialize: {e}")))?;
    bytes.push(b'\n');
    Ok(bytes)
}

/// `Ok(true)` for a real directory, `Ok(false)` when absent. A symlink or anything else at
/// a state path is unsafe.
fn real_dir(p: &Path) -> Result<bool, Error> {
    match std::fs::symlink_metadata(p) {
        Ok(m) if m.is_dir() => Ok(true),
        Ok(_) => Err(Error::Unsafe(format!(
            "{} is not a directory (or is a symlink)",
            p.display()
        ))),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(e) => Err(io_err(&p.display().to_string(), &e)),
    }
}

pub(super) struct Store {
    root: PathBuf,
}

impl Store {
    /// `root` must already be canonical.
    pub fn new(root: PathBuf) -> Self {
        Store { root }
    }

    fn chain(&self) -> [PathBuf; 3] {
        let state = self.root.join(STATE_DIR);
        let cont = state.join("continuity");
        let tasks = cont.join("tasks");
        [state, cont, tasks]
    }

    /// The tasks directory, if the whole chain exists.
    fn tasks_dir(&self) -> Result<Option<PathBuf>, Error> {
        for d in self.chain() {
            if !real_dir(&d)? {
                return Ok(None);
            }
        }
        Ok(Some(self.chain()[2].clone()))
    }

    /// Every directory name under `tasks/`, sorted.
    pub fn task_names(&self) -> Result<Vec<String>, Error> {
        let Some(tasks) = self.tasks_dir()? else {
            return Ok(Vec::new());
        };
        let mut names = Vec::new();
        for entry in std::fs::read_dir(&tasks).map_err(|e| io_err("tasks", &e))? {
            let entry = entry.map_err(|e| io_err("tasks", &e))?;
            let name = entry.file_name();
            names.push(crate::contract::write_value(name.as_encoded_bytes()).text);
        }
        names.sort();
        Ok(names)
    }

    /// All revisions of a task, by id. `None` when the task has no directory.
    pub fn load_task(&self, task: &str) -> Result<Option<Vec<(String, Revision)>>, Error> {
        let Some(tasks) = self.tasks_dir()? else {
            return Ok(None);
        };
        let dir = tasks.join(task);
        if !real_dir(&dir)? {
            return Ok(None);
        }
        let mut names = Vec::new();
        for entry in std::fs::read_dir(&dir).map_err(|e| io_err(task, &e))? {
            names.push(entry.map_err(|e| io_err(task, &e))?.file_name());
        }
        names.sort();
        let mut revs = Vec::new();
        for name in names {
            let file = crate::contract::write_value(name.as_encoded_bytes()).text;
            let bad = |token, reason: String| Error::Record {
                token,
                task: task.to_string(),
                file: file.clone(),
                reason,
            };
            let Some(id) = file
                .strip_suffix(".json")
                .filter(|s| super::is_revision_id(s))
            else {
                return Err(bad("registro-invalido", "not a revision file name".into()));
            };
            let path = dir.join(&name);
            match std::fs::symlink_metadata(&path) {
                Ok(m) if m.is_file() => {}
                Ok(_) => {
                    return Err(Error::Unsafe(format!(
                        "{} is not a regular file",
                        path.display()
                    )))
                }
                Err(e) => return Err(io_err(&file, &e)),
            }
            let bytes = std::fs::read(&path).map_err(|e| io_err(&file, &e))?;
            if revision_id(&bytes) != id {
                return Err(bad("registro-alterado", altered(&bytes).into()));
            }
            let value: Value = serde_json::from_slice(&bytes)
                .map_err(|e| bad("registro-invalido", e.to_string()))?;
            match value.get("format").and_then(Value::as_str) {
                Some(EDITION) => {}
                Some(other) => return Err(bad("formato-desconocido", other.to_string())),
                None => return Err(bad("registro-invalido", "no format".into())),
            }
            let rev: Revision = serde_json::from_value(value)
                .map_err(|e| bad("registro-invalido", e.to_string()))?;
            super::ops::check_record(&rev).map_err(|e| bad("registro-invalido", e.to_string()))?;
            if rev.task != task {
                return Err(bad(
                    "registro-invalido",
                    "task does not match its directory".into(),
                ));
            }
            revs.push((id.to_string(), rev));
        }
        Ok(Some(revs))
    }

    /// The bytes of one revision file, checked against its name again: the file may have
    /// changed since the task was loaded.
    pub fn raw(&self, task: &str, id: &str) -> Result<Vec<u8>, Error> {
        let path = self.chain()[2].join(task).join(format!("{id}.json"));
        let bytes = std::fs::read(&path).map_err(|e| io_err(id, &e))?;
        if revision_id(&bytes) != id {
            return Err(Error::Record {
                token: "registro-alterado",
                task: task.to_string(),
                file: format!("{id}.json"),
                reason: altered(&bytes).into(),
            });
        }
        Ok(bytes)
    }

    /// Create the directories a save needs, each opened relative to its parent with
    /// O_NOFOLLOW, so a symlink swapped in after a check cannot redirect a write (B-03).
    /// `true` when this call created `.umbral/`, `continuity/` or one of the two Git files.
    fn ensure(&self, task: &str) -> Result<(OwnedFd, OwnedFd, bool), Error> {
        let root = openat(CWD, &self.root, OFlags::RDONLY | DIR, Mode::empty())
            .map_err(|e| errno("root", e))?;
        let mut created = false;
        let state = make_dir(&root, &self.root, STATE_DIR, &mut created)?;
        let state_path = self.root.join(STATE_DIR);
        // Written whenever missing, not only with `.umbral/` (B-08).
        for (name, body) in [(".gitattributes", GITATTRIBUTES), (".gitignore", GITIGNORE)] {
            match write_new(&state, name, body) {
                Ok(()) => created = true,
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {}
                Err(e) => return Err(io_err(name, &e)),
            }
        }
        let cont = make_dir(&state, &state_path, "continuity", &mut created)?;
        let cont_path = state_path.join("continuity");
        let mut ignored = false;
        let tasks = make_dir(&cont, &cont_path, "tasks", &mut ignored)?;
        let task_dir = make_dir(&tasks, &cont_path.join("tasks"), task, &mut ignored)?;
        let tmp = make_dir(&cont, &cont_path, "tmp", &mut ignored)?;
        Ok((task_dir, tmp, created))
    }

    /// Write one revision atomically. Returns its id and whether the state was created.
    pub fn write(&self, task: &str, bytes: &[u8]) -> Result<(String, bool), Error> {
        let (task_dir, tmp_dir, created) = self.ensure(task)?;
        let id = revision_id(bytes);
        let tmp = temp_name();
        write_new(&tmp_dir, &tmp, bytes).map_err(|e| io_err(&tmp, &e))?;
        let target = format!("{id}.json");
        let linked = linkat(&tmp_dir, &tmp, &task_dir, &target, AtFlags::empty());
        // A leftover temporary is only clutter in an ignored directory; the outcome is decided
        // by the link.
        let _ = unlinkat(&tmp_dir, &tmp, AtFlags::empty());
        match linked {
            Ok(()) => {}
            Err(Errno::EXIST) => {
                let mut existing = Vec::new();
                openat(
                    &task_dir,
                    &target,
                    OFlags::RDONLY | OFlags::NOFOLLOW | OFlags::CLOEXEC,
                    Mode::empty(),
                )
                .map(std::fs::File::from)
                .map_err(std::io::Error::from)
                .and_then(|mut f| f.read_to_end(&mut existing))
                .map_err(|e| io_err(&id, &e))?;
                if existing != bytes {
                    return Err(Error::Record {
                        token: "registro-alterado",
                        task: task.to_string(),
                        file: target,
                        reason: "hash-mismatch".into(),
                    });
                }
            }
            Err(e) => return Err(errno("link revision", e)),
        }
        fsync(&task_dir).map_err(|e| errno("sync task directory", e))?;
        Ok((id, created))
    }
}

/// A directory of the store opened for writing: never through a symlink.
const DIR: OFlags = OFlags::DIRECTORY.union(OFlags::CLOEXEC);

fn errno(what: &str, e: Errno) -> Error {
    io_err(what, &e.into())
}

/// Create `name` under `parent` if missing, then open it, refusing a symlink or a
/// non-directory. A directory created here is made durable in its parent.
fn make_dir(parent: &OwnedFd, at: &Path, name: &str, created: &mut bool) -> Result<OwnedFd, Error> {
    let path = at.join(name);
    match mkdirat(parent, name, Mode::from_raw_mode(0o777)) {
        Ok(()) => {
            *created = true;
            fsync(parent).map_err(|e| errno(&at.display().to_string(), e))?;
        }
        Err(Errno::EXIST) => {}
        Err(e) => return Err(errno(&path.display().to_string(), e)),
    }
    match openat(
        parent,
        name,
        OFlags::RDONLY | DIR | OFlags::NOFOLLOW,
        Mode::empty(),
    ) {
        Ok(fd) => Ok(fd),
        Err(Errno::LOOP | Errno::NOTDIR) => Err(Error::Unsafe(format!(
            "{} is not a directory (or is a symlink)",
            path.display()
        ))),
        Err(e) => Err(errno(&path.display().to_string(), e)),
    }
}

/// Why a file's bytes no longer match its name (contract §5.4). A canonical record has no
/// line that starts like a Git conflict marker.
fn altered(bytes: &[u8]) -> &'static str {
    let marker = bytes.split(|b| *b == b'\n').any(|l| {
        l.starts_with(b"<<<<<<< ") || l.starts_with(b">>>>>>> ") || l.starts_with(b"=======")
    });
    if marker {
        "conflict-markers"
    } else {
        "hash-mismatch"
    }
}

/// Create `name` under `dir`, failing if anything (a symlink included) is already there.
fn write_new(dir: &OwnedFd, name: &str, bytes: &[u8]) -> std::io::Result<()> {
    let flags = OFlags::WRONLY | OFlags::CREATE | OFlags::EXCL | OFlags::NOFOLLOW | OFlags::CLOEXEC;
    let mut f = std::fs::File::from(openat(dir, name, flags, Mode::from_raw_mode(0o666))?);
    f.write_all(bytes)?;
    f.sync_all()
}

fn temp_name() -> String {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    format!(
        "{}-{nanos}-{}",
        std::process::id(),
        COUNTER.fetch_add(1, Ordering::Relaxed)
    )
}

/// Revisions no other revision names as a parent, sorted. A named parent that is not
/// present is an altered record.
pub(super) fn heads(task: &str, revs: &[(String, Revision)]) -> Result<Vec<String>, Error> {
    let ids: BTreeSet<&str> = revs.iter().map(|(id, _)| id.as_str()).collect();
    let mut named = BTreeSet::new();
    for (id, r) in revs {
        for p in &r.parents {
            if !ids.contains(p.as_str()) {
                return Err(Error::Record {
                    token: "registro-alterado",
                    task: task.to_string(),
                    file: format!("{id}.json"),
                    reason: "missing-parent".into(),
                });
            }
            named.insert(p.as_str());
        }
    }
    Ok(ids
        .into_iter()
        .filter(|id| !named.contains(id))
        .map(str::to_string)
        .collect())
}
