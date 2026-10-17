//! The record store: `<root>/.umbral/continuity/tasks/<task>/<revision>.json`.
//!
//! Files are never rewritten. A revision is written to `tmp/` and hard-linked into place,
//! which fails rather than overwrite. Reads never create anything.

use std::collections::BTreeSet;
use std::ffi::OsString;
use std::io::{Read, Write};
use std::os::fd::OwnedFd;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use rustix::fs::{
    fsync, linkat, mkdirat, openat, statat, unlinkat, AtFlags, Dir, FileType, Mode, OFlags, CWD,
};
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

pub(super) struct Store {
    root: PathBuf,
}

impl Store {
    /// `root` must already be canonical.
    pub fn new(root: PathBuf) -> Self {
        Store { root }
    }

    /// Open every component relative to its parent; a replaced pathname cannot redirect
    /// later reads through a symlink. Reads never create missing directories.
    fn tasks_dir(&self) -> Result<Option<OwnedFd>, Error> {
        let mut dir = openat(
            CWD,
            &self.root,
            OFlags::RDONLY | DIR | OFlags::NOFOLLOW,
            Mode::empty(),
        )
        .map_err(|e| errno("root", e))?;
        for name in [STATE_DIR, "continuity", "tasks"] {
            let Some(next) = read_dir_at(&dir, name)? else {
                return Ok(None);
            };
            dir = next;
        }
        Ok(Some(dir))
    }

    fn task_dir(&self, task: &str) -> Result<Option<OwnedFd>, Error> {
        let Some(tasks) = self.tasks_dir()? else {
            return Ok(None);
        };
        read_dir_at(&tasks, task)
    }

    /// Every name under `tasks/`, sorted (including damaged entries for list integrity).
    pub fn task_names(&self) -> Result<Vec<String>, Error> {
        let Some(tasks) = self.tasks_dir()? else {
            return Ok(Vec::new());
        };
        let mut names: Vec<_> = directory_names(&tasks)?
            .into_iter()
            .map(|name| crate::contract::write_value(name.as_encoded_bytes()).text)
            .collect();
        names.sort();
        Ok(names)
    }

    /// All revisions of a task, by id. `None` when the task has no directory.
    pub fn load_task(&self, task: &str) -> Result<Option<Vec<(String, Revision)>>, Error> {
        let Some(dir) = self.task_dir(task)? else {
            return Ok(None);
        };
        let mut names = directory_names(&dir)?;
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
            let bytes = read_record(&dir, &name, task)?;
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
            let rev: Revision = serde_json::from_slice(&bytes)
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
        let dir = self
            .task_dir(task)?
            .ok_or_else(|| Error::NoSuchRevision(id.into()))?;
        let bytes = read_record(&dir, std::ffi::OsStr::new(&format!("{id}.json")), task)?;
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
                let existing = read_record(&task_dir, std::ffi::OsStr::new(&target), task)?;
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

/// Missing is an empty state; symlinks and non-directories are unsafe.
fn read_dir_at(parent: &OwnedFd, name: &str) -> Result<Option<OwnedFd>, Error> {
    match openat(
        parent,
        name,
        OFlags::RDONLY | DIR | OFlags::NOFOLLOW,
        Mode::empty(),
    ) {
        Ok(fd) => Ok(Some(fd)),
        Err(Errno::NOENT) => Ok(None),
        Err(Errno::LOOP | Errno::NOTDIR) => Err(Error::Unsafe(format!(
            "{name} is not a directory (or is a symlink)"
        ))),
        Err(e) => Err(errno(name, e)),
    }
}

fn directory_names(dir: &OwnedFd) -> Result<Vec<OsString>, Error> {
    let entries = Dir::read_from(dir).map_err(|e| errno("read directory", e))?;
    let mut names = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|e| errno("read directory entry", e))?;
        let name = entry.file_name().to_bytes();
        if name != b"." && name != b".." {
            names.push(std::ffi::OsStr::from_bytes(name).to_owned());
        }
    }
    Ok(names)
}

/// Bound actual bytes read, including growth after fstat. The extra byte detects excess;
/// it is never parsed or exported as a truncated record.
fn bounded_bytes(reader: impl Read) -> std::io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    reader
        .take(super::ops::MAX_FILE as u64 + 1)
        .read_to_end(&mut bytes)?;
    Ok(bytes)
}

fn read_record(dir: &OwnedFd, name: &std::ffi::OsStr, task: &str) -> Result<Vec<u8>, Error> {
    let file_name = crate::contract::write_value(name.as_bytes()).text;
    // Avoid opening known special files (devices may have effects on open). This is
    // only an early rejection: the opened descriptor is still checked after the race.
    let before = statat(dir, name, AtFlags::SYMLINK_NOFOLLOW).map_err(|e| errno(&file_name, e))?;
    if FileType::from_raw_mode(before.st_mode) != FileType::RegularFile {
        return Err(Error::Unsafe(format!("{file_name} is not a regular file")));
    }
    // NONBLOCK prevents a hostile FIFO from blocking before we can check its type.
    let fd = openat(
        dir,
        name,
        OFlags::RDONLY | OFlags::NOFOLLOW | OFlags::CLOEXEC | OFlags::NONBLOCK,
        Mode::empty(),
    )
    .map_err(|e| match e {
        Errno::LOOP | Errno::NXIO => {
            Error::Unsafe(format!("{file_name} cannot be opened as a regular file"))
        }
        _ => errno(&file_name, e),
    })?;
    let file = std::fs::File::from(fd);
    let metadata = file.metadata().map_err(|e| io_err(&file_name, &e))?;
    if !metadata.is_file() {
        return Err(Error::Unsafe(format!("{file_name} is not a regular file")));
    }
    let too_large = || Error::Record {
        token: "registro-invalido",
        task: task.into(),
        file: file_name.clone(),
        reason: "larger than 512 KiB".into(),
    };
    if metadata.len() > super::ops::MAX_FILE as u64 {
        return Err(too_large());
    }
    let bytes = bounded_bytes(file).map_err(|e| io_err(&file_name, &e))?;
    if bytes.len() > super::ops::MAX_FILE {
        return Err(too_large());
    }
    Ok(bytes)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_reads_are_bounded_even_if_a_reader_outgrows_its_initial_size() {
        let limit = super::super::ops::MAX_FILE;
        let exact = vec![b'x'; limit];
        assert_eq!(bounded_bytes(exact.as_slice()).unwrap(), exact);
        let mut endless = std::io::repeat(b'x');
        assert_eq!(bounded_bytes(&mut endless).unwrap().len(), limit + 1);
    }

    #[test]
    fn an_open_directory_cannot_be_redirected_by_replacing_its_name() {
        let root = tempfile::TempDir::new().unwrap();
        let store = Store::new(root.path().to_owned());
        let (id, _) = store.write("t", b"{}\n").unwrap();
        let dir = store.task_dir("t").unwrap().unwrap();
        let path = root.path().join(".umbral/continuity/tasks/t");
        let parked = path.with_file_name("parked");
        std::fs::rename(&path, &parked).unwrap();
        let outside = tempfile::TempDir::new().unwrap();
        let name = format!("{id}.json");
        std::fs::write(outside.path().join(&name), b"outside").unwrap();
        std::os::unix::fs::symlink(outside.path(), &path).unwrap();
        assert_eq!(
            read_record(&dir, std::ffi::OsStr::new(&name), "t").unwrap(),
            b"{}\n"
        );
        assert_eq!(store.raw("t", &id).unwrap_err().token(), "estado-inseguro");
        assert_eq!(store.load_task("t").unwrap_err().token(), "estado-inseguro");
        std::fs::remove_file(path).unwrap();
        std::fs::rename(parked, root.path().join(".umbral/continuity/tasks/t")).unwrap();
    }

    #[test]
    fn raw_and_collision_reads_refuse_oversize_links_and_special_files() {
        let root = tempfile::TempDir::new().unwrap();
        let store = Store::new(root.path().to_owned());
        let body = b"{}\n";
        let (id, _) = store.write("t", body).unwrap();
        let file = root
            .path()
            .join(format!(".umbral/continuity/tasks/t/{id}.json"));
        // Exactly the boundary is allowed by the reader (the hash still differs).
        std::fs::write(&file, vec![b'x'; super::super::ops::MAX_FILE]).unwrap();
        assert_eq!(
            store.raw("t", &id).unwrap_err().token(),
            "registro-alterado"
        );
        std::fs::write(&file, vec![b'x'; super::super::ops::MAX_FILE + 1]).unwrap();
        assert_eq!(
            store.raw("t", &id).unwrap_err().token(),
            "registro-invalido"
        );
        assert_eq!(
            store.write("t", body).unwrap_err().token(),
            "registro-invalido"
        );
        std::fs::remove_file(&file).unwrap();
        std::os::unix::fs::symlink("/dev/zero", &file).unwrap();
        assert_eq!(store.raw("t", &id).unwrap_err().token(), "estado-inseguro");
        assert_eq!(
            store.write("t", body).unwrap_err().token(),
            "estado-inseguro"
        );
        std::fs::remove_file(&file).unwrap();
        let dir = store.task_dir("t").unwrap().unwrap();
        rustix::fs::mknodat(
            &dir,
            std::ffi::OsStr::new(&format!("{id}.json")),
            rustix::fs::FileType::Fifo,
            Mode::from_raw_mode(0o600),
            0,
        )
        .unwrap();
        assert_eq!(store.raw("t", &id).unwrap_err().token(), "estado-inseguro");
        assert_eq!(
            store.write("t", body).unwrap_err().token(),
            "estado-inseguro"
        );
    }

    /// Contract §5.3 (A M-1, mutant M13): writing a revision that is already there with the
    /// same bytes is the same revision; other bytes under its name are an altered record.
    #[test]
    fn writing_the_same_revision_twice_is_idempotent() {
        let dir = tempfile::TempDir::new().unwrap();
        let store = Store::new(std::fs::canonicalize(dir.path()).unwrap());
        let (id, created) = store.write("t", b"{}\n").unwrap();
        assert!(created);
        assert_eq!(store.write("t", b"{}\n").unwrap(), (id.clone(), false));
        let file = dir
            .path()
            .join(format!(".umbral/continuity/tasks/t/{id}.json"));
        std::fs::remove_file(&file).unwrap();
        std::fs::write(&file, b"other\n").unwrap();
        let err = store.write("t", b"{}\n").unwrap_err();
        assert_eq!(err.token(), "registro-alterado");
        assert_eq!(std::fs::read(&file).unwrap(), b"other\n");
        assert_eq!(
            std::fs::read_dir(dir.path().join(".umbral/continuity/tmp"))
                .unwrap()
                .count(),
            0
        );
    }
}
