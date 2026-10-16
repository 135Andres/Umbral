//! The record store: `<root>/.umbral/continuity/tasks/<task>/<revision>.json`.
//!
//! Files are never rewritten. A revision is written to `tmp/` and hard-linked into place,
//! which fails rather than overwrite. Reads never create anything.

use std::collections::BTreeSet;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

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
                return Err(bad("registro-alterado", "hash-mismatch".into()));
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

    /// Create the directories a save needs. `true` when `.umbral/` itself was created.
    fn ensure(&self, task: &str) -> Result<(PathBuf, PathBuf, bool), Error> {
        let [state, cont, tasks] = self.chain();
        let task_dir = tasks.join(task);
        let tmp = cont.join("tmp");
        let mut created = false;
        for d in [&state, &cont, &tasks, &task_dir, &tmp] {
            if !real_dir(d)? {
                match std::fs::create_dir(d) {
                    Ok(()) => created |= *d == state,
                    Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                        real_dir(d)?;
                    }
                    Err(e) => return Err(io_err(&d.display().to_string(), &e)),
                }
            }
        }
        if created {
            write_new(&state.join(".gitattributes"), GITATTRIBUTES)?;
            write_new(&state.join(".gitignore"), GITIGNORE)?;
        }
        Ok((task_dir, tmp, created))
    }

    /// Write one revision atomically. Returns its id and whether `.umbral/` was created.
    pub fn write(&self, task: &str, bytes: &[u8]) -> Result<(String, bool), Error> {
        let (task_dir, tmp_dir, created) = self.ensure(task)?;
        let id = revision_id(bytes);
        let tmp = temp_path(&tmp_dir);
        write_new(&tmp, bytes)?;
        let target = task_dir.join(format!("{id}.json"));
        let linked = std::fs::hard_link(&tmp, &target);
        // A leftover temporary is only clutter in an ignored directory; the outcome is decided
        // by the link.
        let _ = std::fs::remove_file(&tmp);
        match linked {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                let existing = std::fs::read(&target).map_err(|e| io_err(&id, &e))?;
                if existing != bytes {
                    return Err(Error::Record {
                        token: "registro-alterado",
                        task: task.to_string(),
                        file: format!("{id}.json"),
                        reason: "hash-mismatch".into(),
                    });
                }
            }
            Err(e) => return Err(io_err("link revision", &e)),
        }
        std::fs::File::open(&task_dir)
            .and_then(|d| d.sync_all())
            .map_err(|e| io_err("sync task directory", &e))?;
        Ok((id, created))
    }
}

fn write_new(path: &Path, bytes: &[u8]) -> Result<(), Error> {
    let mut f = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| io_err(&path.display().to_string(), &e))?;
    f.write_all(bytes)
        .and_then(|()| f.sync_all())
        .map_err(|e| io_err(&path.display().to_string(), &e))
}

fn temp_path(dir: &Path) -> PathBuf {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    dir.join(format!(
        "{}-{nanos}-{}",
        std::process::id(),
        COUNTER.fetch_add(1, Ordering::Relaxed)
    ))
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
