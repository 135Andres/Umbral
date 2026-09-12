//! The v0.1 implementation of the persistence seam, on SQLite.
//!
//! # What is deliberately NOT claimed here
//!
//! This is one implementation behind [`ObservationLog`](super::ObservationLog). Choosing it
//! for v0.1 does **not** answer Q25 (SQLite-only vs SQLite + an external append-only log),
//! and it is not a statement about Umbral's persistence model. Q25 remains OPEN.
//!
//! WAL is deliberately **not** enabled and `synchronous` is deliberately not tuned. Tuning
//! durability here would be answering a question this version was told not to answer (Q25),
//! and v0.1's contract makes no durability claim: crash and power-loss durability are
//! outside its scope. SQLite's default rollback journal is used, which also lets the
//! read-only commands open the state file without any write capability at all.
//!
//! # Schema
//!
//! Two tables, and every column justifies its persistence:
//!
//! - `run` — one row per `observe` execution. `run_id`, when it started and finished, and
//!   the root it observed.
//! - `observation` — one row per observed entry per run. This is the only irreplaceable
//!   content: it records what was seen, and it cannot be recomputed from anything else.
//!
//! There is no projection table and no derived column. `entries`, `errors` and `complete`
//! on [`RunMeta`](super::RunMeta) are computed from `observation` on read.

use std::path::{Path, PathBuf};

use rusqlite::{Connection, OptionalExtension};

use super::{
    LogError, NewObservation, NewRun, Observation, ObservationLog, RunId, RunMeta, SCHEMA_VERSION,
};
use crate::content::{hex, GuardDelta, Stability};
use crate::scan::EntryKind;

pub struct SqliteLog {
    conn: Connection,
}

// Hand-written rather than derived: the connection handle is an implementation detail and
// has no useful representation to print.
impl std::fmt::Debug for SqliteLog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("SqliteLog")
    }
}

const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS schema_meta (
    key   TEXT PRIMARY KEY,
    value TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS run (
    run_id          INTEGER PRIMARY KEY AUTOINCREMENT,
    started_at_ns   INTEGER NOT NULL,
    finished_at_ns  INTEGER NOT NULL,
    root            BLOB    NOT NULL
);
CREATE TABLE IF NOT EXISTS observation (
    run_id          INTEGER NOT NULL REFERENCES run(run_id),
    path            BLOB    NOT NULL,   -- raw bytes; paths are not necessarily UTF-8
    kind            TEXT    NOT NULL,
    dev             INTEGER,
    ino             INTEGER,
    size            INTEGER,
    mtime_s         INTEGER,
    mtime_ns        INTEGER,
    hash            BLOB,
    hashed_len      INTEGER,
    hash_stability  TEXT,
    hash_deltas     TEXT,
    obs_error       TEXT,
    PRIMARY KEY (run_id, path),
    -- A stored hash without a stability verdict would be a hash whose validity is unknown.
    CHECK (hash IS NULL OR hash_stability IS NOT NULL)
);
CREATE INDEX IF NOT EXISTS idx_observation_path ON observation(path);
";

impl SqliteLog {
    pub fn open(path: &Path) -> Result<Self, LogError> {
        let conn = Connection::open(path)?;
        Self::from_conn(conn)
    }

    pub fn open_in_memory() -> Result<Self, LogError> {
        let conn = Connection::open_in_memory()?;
        Self::from_conn(conn)
    }

    fn from_conn(conn: Connection) -> Result<Self, LogError> {
        conn.execute_batch("PRAGMA foreign_keys=ON;")?;
        conn.execute_batch(SCHEMA)?;
        let stored: Option<String> = conn
            .query_row(
                "SELECT value FROM schema_meta WHERE key='schema_version'",
                [],
                |r| r.get(0),
            )
            .optional()?;
        match stored {
            None => {
                conn.execute(
                    "INSERT INTO schema_meta (key, value) VALUES ('schema_version', ?1)",
                    [SCHEMA_VERSION],
                )?;
            }
            Some(v) if v == SCHEMA_VERSION => {}
            Some(v) => return Err(LogError::UnknownSchema(v)),
        }
        Ok(SqliteLog { conn })
    }

    /// Open the state file with **no write capability at all**.
    ///
    /// This is what makes the read-only commands genuinely read-only: they cannot create a
    /// table, write a journal, or modify a row, because the connection is opened read-only.
    /// The schema is verified, never created.
    pub fn open_read_only(path: &Path) -> Result<Self, LogError> {
        let conn = Connection::open_with_flags(
            path,
            rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY | rusqlite::OpenFlags::SQLITE_OPEN_NO_MUTEX,
        )?;
        let stored: Option<String> = conn
            .query_row(
                "SELECT value FROM schema_meta WHERE key='schema_version'",
                [],
                |r| r.get(0),
            )
            .optional()?;
        match stored {
            Some(v) if v == SCHEMA_VERSION => Ok(SqliteLog { conn }),
            Some(v) => Err(LogError::UnknownSchema(v)),
            None => Err(LogError::UnknownSchema("<absent>".into())),
        }
    }
}

fn path_to_blob(p: &Path) -> Vec<u8> {
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStrExt;
        p.as_os_str().as_bytes().to_vec()
    }
    #[cfg(not(unix))]
    {
        p.to_string_lossy().as_bytes().to_vec()
    }
}

fn blob_to_path(b: &[u8]) -> PathBuf {
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStrExt;
        PathBuf::from(std::ffi::OsStr::from_bytes(b))
    }
    #[cfg(not(unix))]
    {
        PathBuf::from(String::from_utf8_lossy(b).into_owned())
    }
}

impl ObservationLog for SqliteLog {
    fn append_run(&mut self, run: NewRun) -> Result<RunId, LogError> {
        let tx = self.conn.transaction()?;
        tx.execute(
            "INSERT INTO run (started_at_ns, finished_at_ns, root) VALUES (?1, ?2, ?3)",
            rusqlite::params![
                run.started_at_ns,
                run.finished_at_ns,
                path_to_blob(&run.root)
            ],
        )?;
        let run_id = tx.last_insert_rowid();

        {
            let mut stmt = tx.prepare(
                "INSERT INTO observation
                 (run_id, path, kind, dev, ino, size, mtime_s, mtime_ns,
                  hash, hashed_len, hash_stability, hash_deltas, obs_error)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            )?;
            for obs in &run.observations {
                let NewObservation {
                    entry,
                    content,
                    error,
                } = obs;
                let (hash, hashed_len, stability, deltas) = match content {
                    Some(c) => (
                        c.hash.map(|h| h.to_vec()),
                        c.hashed_len.map(|v| v as i64),
                        c.stability.map(|s| s.as_str().to_string()),
                        c.deltas_str(),
                    ),
                    None => (None, None, None, None),
                };
                let (mtime_s, mtime_ns) = match entry.mtime {
                    Some((s, n)) => (Some(s), Some(n as i64)),
                    None => (None, None),
                };
                stmt.execute(rusqlite::params![
                    run_id,
                    path_to_blob(&entry.path),
                    entry.kind.as_str(),
                    entry.dev.map(|v| v as i64),
                    entry.ino.map(|v| v as i64),
                    entry.size.map(|v| v as i64),
                    mtime_s,
                    mtime_ns,
                    hash,
                    hashed_len,
                    stability,
                    deltas,
                    error,
                ])?;
            }
        }

        tx.commit()?;
        Ok(run_id)
    }

    fn runs(&self) -> Result<Vec<RunMeta>, LogError> {
        let mut stmt = self.conn.prepare(
            "SELECT r.run_id, r.started_at_ns, r.finished_at_ns, r.root,
                    (SELECT COUNT(*) FROM observation o WHERE o.run_id = r.run_id),
                    (SELECT COUNT(*) FROM observation o WHERE o.run_id = r.run_id AND o.obs_error IS NOT NULL)
             FROM run r ORDER BY r.run_id ASC",
        )?;
        let rows = stmt.query_map([], |r| {
            let root: Vec<u8> = r.get(3)?;
            let entries: i64 = r.get(4)?;
            let errors: i64 = r.get(5)?;
            Ok(RunMeta {
                id: r.get(0)?,
                started_at_ns: r.get(1)?,
                finished_at_ns: r.get(2)?,
                root: blob_to_path(&root),
                entries: entries as u64,
                errors: errors as u64,
            })
        })?;
        Ok(rows.collect::<Result<Vec<_>, _>>()?)
    }

    fn run(&self, id: RunId) -> Result<Option<RunMeta>, LogError> {
        Ok(self.runs()?.into_iter().find(|r| r.id == id))
    }

    fn latest_run(&self) -> Result<Option<RunMeta>, LogError> {
        Ok(self.runs()?.into_iter().next_back())
    }

    fn observations_for_run(&self, id: RunId) -> Result<Vec<Observation>, LogError> {
        let mut stmt = self.conn.prepare(
            "SELECT run_id, path, kind, dev, ino, size, mtime_s, mtime_ns,
                    hash, hashed_len, hash_stability, hash_deltas, obs_error
             FROM observation WHERE run_id = ?1 ORDER BY path ASC",
        )?;
        let rows = stmt.query_map([id], row_to_observation)?;
        Ok(rows.collect::<Result<Vec<_>, _>>()?)
    }

    fn observations_for_path(&self, path: &Path) -> Result<Vec<Observation>, LogError> {
        let mut stmt = self.conn.prepare(
            "SELECT run_id, path, kind, dev, ino, size, mtime_s, mtime_ns,
                    hash, hashed_len, hash_stability, hash_deltas, obs_error
             FROM observation WHERE path = ?1 ORDER BY run_id ASC",
        )?;
        let rows = stmt.query_map([path_to_blob(path)], row_to_observation)?;
        Ok(rows.collect::<Result<Vec<_>, _>>()?)
    }

    fn all_observations(&self) -> Result<Vec<Observation>, LogError> {
        let mut stmt = self.conn.prepare(
            "SELECT run_id, path, kind, dev, ino, size, mtime_s, mtime_ns,
                    hash, hashed_len, hash_stability, hash_deltas, obs_error
             FROM observation ORDER BY run_id ASC, path ASC",
        )?;
        let rows = stmt.query_map([], row_to_observation)?;
        Ok(rows.collect::<Result<Vec<_>, _>>()?)
    }

    fn counts(&self) -> Result<(u64, u64), LogError> {
        let runs: i64 = self
            .conn
            .query_row("SELECT COUNT(*) FROM run", [], |r| r.get(0))?;
        let obs: i64 = self
            .conn
            .query_row("SELECT COUNT(*) FROM observation", [], |r| r.get(0))?;
        Ok((runs as u64, obs as u64))
    }

    fn tables(&self) -> Result<Vec<String>, LogError> {
        let mut stmt = self.conn.prepare(
            "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' ORDER BY name",
        )?;
        let rows = stmt.query_map([], |r| r.get::<_, String>(0))?;
        Ok(rows.collect::<Result<Vec<_>, _>>()?)
    }
}

fn row_to_observation(r: &rusqlite::Row<'_>) -> rusqlite::Result<Observation> {
    let path: Vec<u8> = r.get(1)?;
    let kind: String = r.get(2)?;
    let dev: Option<i64> = r.get(3)?;
    let ino: Option<i64> = r.get(4)?;
    let size: Option<i64> = r.get(5)?;
    let mtime_s: Option<i64> = r.get(6)?;
    let mtime_ns: Option<i64> = r.get(7)?;
    let hash: Option<Vec<u8>> = r.get(8)?;
    let hashed_len: Option<i64> = r.get(9)?;
    let stability: Option<String> = r.get(10)?;
    let deltas: Option<String> = r.get(11)?;
    let error: Option<String> = r.get(12)?;

    let hash_arr = match hash {
        Some(v) if v.len() == 32 => {
            let mut a = [0u8; 32];
            a.copy_from_slice(&v);
            Some(a)
        }
        _ => None,
    };

    let mtime = match (mtime_s, mtime_ns) {
        (Some(s), Some(n)) => Some((s, n as u32)),
        _ => None,
    };

    Ok(Observation {
        run_id: r.get(0)?,
        path: blob_to_path(&path),
        kind: EntryKind::parse(&kind),
        dev: dev.map(|v| v as u64),
        ino: ino.map(|v| v as u64),
        size: size.map(|v| v as u64),
        mtime,
        hash: hash_arr,
        hashed_len: hashed_len.map(|v| v as u64),
        stability: stability.as_deref().and_then(Stability::parse),
        deltas: deltas
            .as_deref()
            .map(|s| s.split(',').filter_map(GuardDelta::parse).collect())
            .unwrap_or_default(),
        error,
    })
}

/// Full hex of a stored hash, for `show`.
pub fn hash_hex(h: &[u8; 32]) -> String {
    hex(h)
}
