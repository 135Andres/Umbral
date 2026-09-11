//! Increment 4 — STORE: durable observation history + active projection (SQLite).
//!
//! H25 guard: this store records OBSERVATIONS. It is not Project Reality;
//! it holds no authority, standing, currency or relations.
//!
//! Model:
//!   observations table = the only source of truth (append-only history);
//!   projection table   = derived cache, rebuildable by replay (INV-2 seed).
//! One write = one transaction covering both, so a crash leaves either the
//! old state or the new one, never a half-observation presented as valid
//! (INV-7).
//!
//! Q25 evidence gathered here: crash tests A-H below exercise whether
//! SQLite alone gives durability + rebuild + rollback for V0's needs.

use crate::hash_obs::{ContentError, ContentObservation, HASH_LEN, Stability};
use crate::identity::PhysicalId;
use crate::{Entry, EntryKind};
use rusqlite::{Connection, Row, Transaction, params};
use std::path::{Path, PathBuf};

pub struct Store {
    conn: Connection,
}

/// One persisted observation. `kind`/`identity`/`hash` mirror the scan/hash
/// model; absence (None) is recorded, never faked (INV-8 family).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservationRecord {
    pub seq: i64,
    pub observed_at_unix_ns: i64,
    pub path: PathBuf,
    pub kind: EntryKind,
    pub dev: Option<u64>,
    pub ino: Option<u64>,
    pub size: Option<u64>,
    pub mtime: Option<(i64, u32)>,
    pub hash: Option<[u8; HASH_LEN]>,
    pub hashed_len: Option<u64>,
    pub hash_stability: Option<Stability>,
    pub hash_deltas: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug)]
pub enum StoreError {
    Sqlite(rusqlite::Error),
    Io(std::io::Error),
}

impl From<rusqlite::Error> for StoreError {
    fn from(e: rusqlite::Error) -> Self {
        StoreError::Sqlite(e)
    }
}
impl From<std::io::Error> for StoreError {
    fn from(e: std::io::Error) -> Self {
        StoreError::Io(e)
    }
}
impl std::fmt::Display for StoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StoreError::Sqlite(e) => write!(f, "sqlite: {e}"),
            StoreError::Io(e) => write!(f, "io: {e}"),
        }
    }
}
impl std::error::Error for StoreError {}

/// V0 WAL + synchronous mode: WAL for crash-consistency and concurrent
/// readers; synchronous=FULL because INV-7 testing is this increment's
/// point — power-loss durability is out of scope, process-kill durability
/// is what we assert (see tests).
fn configure(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch(
        "PRAGMA journal_mode=WAL;
         PRAGMA synchronous=FULL;
         PRAGMA foreign_keys=ON;",
    )
}

fn kind_to_str(k: EntryKind) -> &'static str {
    match k {
        EntryKind::File => "file",
        EntryKind::Dir => "dir",
        EntryKind::Symlink => "symlink",
        EntryKind::Other => "other",
    }
}
fn kind_from_str(s: &str) -> EntryKind {
    match s {
        "file" => EntryKind::File,
        "dir" => EntryKind::Dir,
        "symlink" => EntryKind::Symlink,
        _ => EntryKind::Other,
    }
}

/// Persisted content-evidence tuple (kept as a type alias for readability).
type ContentTuple = (
    Option<Vec<u8>>,
    Option<u64>,
    Option<&'static str>,
    Option<String>,
    Option<String>,
);

/// The schema. Every column justifies its persistence:
///   observations: the raw history — the only irreplaceable truth
///     (derivable from nothing; loss means losing what fsp-check saw);
///   projection: derived cache — every column rebuildable from observations
///     (loss recoverable; kept to make the active state queryable cheaply
///     and to give INV-7's "old or new" a second half to protect).
const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS observations (
    seq             INTEGER PRIMARY KEY AUTOINCREMENT,
    observed_at_ns  INTEGER NOT NULL,
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
    CHECK (hash IS NULL OR hash_stability IS NOT NULL)
);
CREATE INDEX IF NOT EXISTS idx_obs_path ON observations(path);
CREATE TABLE IF NOT EXISTS projection (
    path        BLOB PRIMARY KEY,
    kind        TEXT NOT NULL,
    last_seq    INTEGER NOT NULL REFERENCES observations(seq),
    dev         INTEGER,
    ino         INTEGER,
    size        INTEGER,
    mtime_s     INTEGER,
    mtime_ns    INTEGER,
    hash        BLOB,
    hash_stability TEXT
);
";

impl Store {
    pub fn open(path: &Path) -> Result<Store, StoreError> {
        let conn = Connection::open(path)?;
        configure(&conn)?;
        conn.execute_batch(SCHEMA)?;
        Ok(Store { conn })
    }

    pub fn open_in_memory() -> Result<Store, StoreError> {
        let conn = Connection::open_in_memory()?;
        configure(&conn)?;
        conn.execute_batch(SCHEMA)?;
        Ok(Store { conn })
    }

    /// Record one observation atomically: history append + projection update
    /// in a single transaction. A crash before COMMIT leaves the previous
    /// state fully intact; after COMMIT both halves are present.
    pub fn record(
        &self,
        entry: &Entry,
        observed_at_unix_ns: i64,
        content: Option<&ContentObservation>,
    ) -> Result<i64, StoreError> {
        let tx = self.conn.unchecked_transaction()?;
        let seq = insert_observation(&tx, entry, observed_at_unix_ns, content)?;
        tx.execute(
            "INSERT INTO projection (path, kind, last_seq, dev, ino, size, mtime_s, mtime_ns, hash, hash_stability)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
             ON CONFLICT(path) DO UPDATE SET
               kind=excluded.kind, last_seq=excluded.last_seq, dev=excluded.dev,
               ino=excluded.ino, size=excluded.size, mtime_s=excluded.mtime_s,
               mtime_ns=excluded.mtime_ns, hash=excluded.hash,
               hash_stability=excluded.hash_stability",
            params![
                path_bytes(&entry.path),
                kind_to_str(entry.kind),
                seq,
                i64_of(entry.dev),
                i64_of(entry.ino),
                i64_of(entry.size),
                entry.mtime.map(|m| m.0),
                entry.mtime.map(|m| m.1),
                content.and_then(|c| c.hash).map(hx_to_blob),
                content
                    .and_then(|c| c.hash)
                    .and_then(|_| content.map(|c| stability_str(c.stability))),
            ],
        )?;
        tx.commit()?;
        Ok(seq)
    }

    /// INV-2: the projection is a pure function of the observation history.
    /// Drops the projection table contents and replays the history in seq
    /// order. Returns the number of observations replayed.
    pub fn rebuild_projection(&mut self) -> Result<usize, StoreError> {
        self.conn.execute("DELETE FROM projection", [])?;
        let mut stmt = self.conn.prepare(
            "SELECT seq, observed_at_ns, path, kind, dev, ino, size, mtime_s, mtime_ns,
                    hash, hashed_len, hash_stability, hash_deltas, obs_error
             FROM observations ORDER BY seq",
        )?;
        let rows = stmt.query_map([], row_to_record)?;
        let mut n = 0usize;
        let replay: Vec<ObservationRecord> = rows.collect::<Result<_, _>>()?;
        drop(stmt);
        for rec in replay {
            let entry = rec.to_entry();
            let content = rec.to_content_observation();
            let tx = self.conn.unchecked_transaction()?;
            tx.execute(
                "INSERT INTO projection (path, kind, last_seq, dev, ino, size, mtime_s, mtime_ns, hash, hash_stability)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
                 ON CONFLICT(path) DO UPDATE SET
                   kind=excluded.kind, last_seq=excluded.last_seq, dev=excluded.dev,
                   ino=excluded.ino, size=excluded.size, mtime_s=excluded.mtime_s,
                   mtime_ns=excluded.mtime_ns, hash=excluded.hash,
                   hash_stability=excluded.hash_stability",
                params![
                    path_bytes(&entry.path),
                    kind_to_str(entry.kind),
                    rec.seq,
                    i64_of(entry.dev),
                    i64_of(entry.ino),
                    i64_of(entry.size),
                    entry.mtime.map(|m| m.0),
                    entry.mtime.map(|m| m.1),
                    content.as_ref().and_then(|c| c.hash).map(hx_to_blob),
                    content
                        .as_ref()
                        .and_then(|c| c.hash)
                        .and_then(|_| content.as_ref().map(|c| stability_str(c.stability))),
                ],
            )?;
            tx.commit()?;
            n += 1;
        }
        Ok(n)
    }

    /// The active state: latest observation per path.
    pub fn active_projection(&self) -> Result<Vec<ObservationRecord>, StoreError> {
        let mut stmt = self.conn.prepare(
            "SELECT o.seq, o.observed_at_ns, o.path, o.kind, o.dev, o.ino, o.size,
                    o.mtime_s, o.mtime_ns, o.hash, o.hashed_len, o.hash_stability,
                    o.hash_deltas, o.obs_error
             FROM projection p JOIN observations o ON o.seq = p.last_seq
             ORDER BY o.path",
        )?;
        let rows = stmt.query_map([], row_to_record)?;
        let mut v = Vec::new();
        for r in rows {
            v.push(r?);
        }
        Ok(v)
    }

    /// Test seam: start an explicit transaction the caller may drop to force
    /// a rollback (the worker binary also uses it for its own tx needs).
    pub fn begin_tx(&self) -> rusqlite::Transaction<'_> {
        self.conn.unchecked_transaction().unwrap()
    }

    /// Test/corruption-recovery seam: drop the derived projection WITHOUT
    /// touching history. Deliberately narrow — the history table is never
    /// exposed.
    pub fn clear_projection_cache(&self) -> Result<(), StoreError> {
        self.conn.execute("DELETE FROM projection", [])?;
        Ok(())
    }

    pub fn observation_count(&self) -> Result<usize, StoreError> {
        let n: i64 = self
            .conn
            .query_row("SELECT COUNT(*) FROM observations", [], |r| r.get(0))?;
        Ok(n as usize)
    }

    /// Full history for one path, oldest first.
    pub fn history_for_path(&self, path: &Path) -> Result<Vec<ObservationRecord>, StoreError> {
        let mut stmt = self.conn.prepare(
            "SELECT seq, observed_at_ns, path, kind, dev, ino, size, mtime_s, mtime_ns,
                    hash, hashed_len, hash_stability, hash_deltas, obs_error
             FROM observations WHERE path = ?1 ORDER BY seq",
        )?;
        let rows = stmt.query_map([path_bytes(path)], row_to_record)?;
        let mut v = Vec::new();
        for r in rows {
            v.push(r?);
        }
        Ok(v)
    }
}

fn path_bytes(p: &Path) -> Vec<u8> {
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStrExt;
        p.as_os_str().as_bytes().to_vec()
    }
    #[cfg(not(unix))]
    {
        p.as_os_str().as_encoded_bytes().to_vec()
    }
}
fn bytes_path(b: &[u8]) -> PathBuf {
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStrExt;
        PathBuf::from(std::ffi::OsStr::from_bytes(b))
    }
    #[cfg(not(unix))]
    {
        PathBuf::from(std::ffi::OsString::from_encoded_bytes_shared(b.to_vec()))
    }
}
fn i64_of(v: Option<u64>) -> Option<i64> {
    v.map(|x| x as i64)
}
fn hx_to_blob(h: [u8; HASH_LEN]) -> Vec<u8> {
    h.to_vec()
}
fn stability_str(s: Stability) -> &'static str {
    match s {
        Stability::Stable => "stable",
        Stability::Unstable => "unstable",
    }
}

fn insert_observation(
    tx: &Transaction,
    entry: &Entry,
    observed_at_unix_ns: i64,
    content: Option<&ContentObservation>,
) -> Result<i64, StoreError> {
    // INV-1 negative half enforced at rest: only a Stable content observation
    // may store a hash. Unstable/errored observations persist their ERROR
    // and stability evidence, never bytes pretending to be a hash.
    let (hash, hashed_len, stability, deltas, error): ContentTuple = match content {
        Some(c) if c.stability == Stability::Stable && c.hash.is_some() => (
            Some(hx_to_blob(c.hash.unwrap())),
            c.hashed_len,
            Some("stable"),
            None,
            None,
        ),
        Some(c) => (
            None,
            None,
            Some(stability_str(c.stability)),
            if c.deltas.is_empty() {
                None
            } else {
                Some(format!("{:?}", c.deltas))
            },
            Some(match &c.error {
                Some(e) => format!("{e:?}"),
                None => "unstable".to_string(),
            }),
        ),
        None => (None, None, None, None, None),
    };
    tx.execute(
        "INSERT INTO observations (observed_at_ns, path, kind, dev, ino, size,
                                   mtime_s, mtime_ns, hash, hashed_len,
                                   hash_stability, hash_deltas, obs_error)
         VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13)",
        params![
            observed_at_unix_ns,
            path_bytes(&entry.path),
            kind_to_str(entry.kind),
            i64_of(entry.dev),
            i64_of(entry.ino),
            i64_of(entry.size),
            entry.mtime.map(|m| m.0),
            entry.mtime.map(|m| m.1),
            hash,
            hashed_len.map(|v| v as i64),
            stability,
            deltas,
            error,
        ],
    )?;
    Ok(tx.last_insert_rowid())
}

fn row_to_record(r: &Row) -> rusqlite::Result<ObservationRecord> {
    let hash: Option<Vec<u8>> = r.get(9)?;
    let hash = match hash {
        Some(b) if b.len() == HASH_LEN => {
            let mut a = [0u8; HASH_LEN];
            a.copy_from_slice(&b);
            Some(a)
        }
        _ => None,
    };
    let mtime_s: Option<i64> = r.get(7)?;
    let mtime_ns: Option<i64> = r.get(8)?;
    let opt_u64 = |i: usize| -> rusqlite::Result<Option<u64>> {
        Ok(r.get::<_, Option<i64>>(i)?.map(|v| v as u64))
    };
    Ok(ObservationRecord {
        seq: r.get(0)?,
        observed_at_unix_ns: r.get(1)?,
        path: {
            let b: Vec<u8> = match r.get(2) {
                Ok(v) => v,
                Err(_) => {
                    let s: String = r.get(2)?;
                    s.into_bytes()
                }
            };
            bytes_path(&b)
        },
        kind: kind_from_str(&r.get::<_, String>(3)?),
        dev: opt_u64(4)?,
        ino: opt_u64(5)?,
        size: opt_u64(6)?,
        mtime: mtime_s.map(|s| (s, mtime_ns.unwrap_or(0) as u32)),
        hash,
        hashed_len: opt_u64(10)?,
        hash_stability: match r.get::<_, Option<String>>(11)? {
            Some(s) if s == "stable" => Some(Stability::Stable),
            Some(_) => Some(Stability::Unstable),
            None => None,
        },
        hash_deltas: r.get(12)?,
        error: r.get(13)?,
    })
}

impl ObservationRecord {
    pub fn to_entry(&self) -> Entry {
        Entry {
            path: self.path.clone(),
            kind: self.kind,
            dev: self.dev,
            ino: self.ino,
            size: self.size,
            mtime: self.mtime,
        }
    }
    pub fn to_content_observation(&self) -> Option<ContentObservation> {
        self.hash_stability.map(|st| ContentObservation {
            physical_id: PhysicalId {
                dev: self.dev,
                ino: self.ino,
            },
            hash: self.hash,
            hashed_len: self.hashed_len,
            stability: st,
            deltas: Vec::new(),
            error: self.error.as_ref().map(|e| match e.as_str() {
                "NotFound" => ContentError::NotFound,
                "PermissionDenied" => ContentError::PermissionDenied,
                "NotARegularFile" => ContentError::NotARegularFile,
                other => ContentError::ReadError(other.to_string()),
            }),
        })
    }
}
