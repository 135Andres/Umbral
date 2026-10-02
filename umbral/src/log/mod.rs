//! The persistence seam.
//!
//! # Why this is a trait and not a concrete store
//!
//! The project has an OPEN question (Q25) about whether SQLite alone is the right
//! persistence substrate or whether an external append-only log is also needed. v0.1 does
//! **not** answer it. The trait exists so that the question stays open by construction: the
//! rest of the crate talks to [`ObservationLog`], and a second implementation would slot in
//! behind it without changing anything above.
//!
//! Nothing in this module is a claim about Umbral's persistence model. The shape chosen
//! here is the minimum the v0.1 contract needs, and it is documented as such.
//!
//! # The shape: runs
//!
//! The unit of observation is a **run**: one execution of `observe` over one root. Every
//! observation belongs to exactly one run. This is what makes "when was this verified?"
//! answerable per entry, and it is why a re-observation with no changes still records a
//! run rather than being collapsed into "no changes" — collapsing it would make
//! "verified just now" indistinguishable from "not verified since Tuesday".
//!
//! # One home per fact
//!
//! The log stores observations and the run that produced them. It stores **no** derived
//! state: there is no projection table, no cached "current state", and no stored
//! reconciliation result. Everything derived is recomputed on read, so there is nothing
//! that can drift out of agreement with the log.

use std::path::{Path, PathBuf};

use crate::content::{ContentObservation, Stability};
use crate::scan::{Entry, EntryKind};

pub mod sqlite;

pub type RunId = i64;

/// One run of `observe`. Counts and completeness are DERIVED from the observations, never
/// stored, so they cannot contradict the evidence they summarise.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunMeta {
    pub id: RunId,
    pub started_at_ns: i64,
    pub finished_at_ns: i64,
    pub root: PathBuf,
    /// Number of entries observed in this run.
    pub entries: u64,
    /// Number of entries that could not be observed.
    pub errors: u64,
}

impl RunMeta {
    /// A run is complete when every path it tried to observe was observed.
    pub fn complete(&self) -> bool {
        self.errors == 0
    }
}

/// One persisted observation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Observation {
    pub run_id: RunId,
    pub path: PathBuf,
    pub kind: EntryKind,
    pub dev: Option<u64>,
    pub ino: Option<u64>,
    pub size: Option<u64>,
    pub mtime: Option<(i64, u32)>,
    pub hash: Option<[u8; 32]>,
    pub hashed_len: Option<u64>,
    pub stability: Option<Stability>,
    pub deltas: Vec<crate::content::GuardDelta>,
    pub error: Option<String>,
    /// Why no content result was obtained, as recorded at observation time
    /// ([`ContentError::record`](crate::content::ContentError::record)). `not-recorded` marks a
    /// file row written by a `umbral-v0.1` build, which did not persist the reason.
    pub content_error: Option<String>,
}

impl Observation {
    /// The hash, only when the observation was stable. Same gate as
    /// [`ContentObservation::valid_hash`], applied to persisted rows.
    pub fn valid_hash(&self) -> Option<&[u8; 32]> {
        match self.stability {
            Some(Stability::Stable) => self.hash.as_ref(),
            _ => None,
        }
    }

    pub fn is_content_verified(&self) -> bool {
        self.valid_hash().is_some()
    }
}

/// An observation on its way into the log.
#[derive(Debug, Clone)]
pub struct NewObservation {
    pub entry: Entry,
    pub content: Option<ContentObservation>,
    pub error: Option<String>,
}

/// A run on its way into the log.
#[derive(Debug, Clone)]
pub struct NewRun {
    pub started_at_ns: i64,
    pub finished_at_ns: i64,
    pub root: PathBuf,
    pub observations: Vec<NewObservation>,
}

#[derive(Debug)]
pub enum LogError {
    Sqlite(rusqlite::Error),
    Io(std::io::Error),
    /// The stored schema version is not one this build understands.
    UnknownSchema(String),
    /// A structural inconsistency found by `check`.
    Inconsistent(String),
}

impl std::fmt::Display for LogError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogError::Sqlite(e) => write!(f, "sqlite: {e}"),
            LogError::Io(e) => write!(f, "io: {e}"),
            LogError::UnknownSchema(v) => write!(f, "unknown state schema version: {v}"),
            LogError::Inconsistent(m) => write!(f, "inconsistent state: {m}"),
        }
    }
}

impl std::error::Error for LogError {}

impl From<rusqlite::Error> for LogError {
    fn from(e: rusqlite::Error) -> Self {
        LogError::Sqlite(e)
    }
}

impl From<std::io::Error> for LogError {
    fn from(e: std::io::Error) -> Self {
        LogError::Io(e)
    }
}

/// The log schema version this build writes.
///
/// `umbral-v0.1.1` adds `observation.content_error` (D-V01-10). A `umbral-v0.1` log is still
/// read, and is migrated in place, additively, the first time it is opened for writing.
pub const SCHEMA_VERSION: &str = "umbral-v0.1.1";

/// The previous log schema version, read and migrated by this build.
pub const SCHEMA_VERSION_V0_1: &str = "umbral-v0.1";

/// The persistence seam.
pub trait ObservationLog {
    /// Append one run and its observations atomically: either the whole run is recorded,
    /// or none of it is.
    fn append_run(&mut self, run: NewRun) -> Result<RunId, LogError>;

    /// All runs, oldest first.
    fn runs(&self) -> Result<Vec<RunMeta>, LogError>;

    fn run(&self, id: RunId) -> Result<Option<RunMeta>, LogError>;

    /// The most recent run, if any.
    fn latest_run(&self) -> Result<Option<RunMeta>, LogError>;

    /// Observations of one run, ordered by path.
    fn observations_for_run(&self, id: RunId) -> Result<Vec<Observation>, LogError>;

    /// Every observation of one path, oldest run first. This is what makes a path's history
    /// answerable without any separate index.
    fn observations_for_path(&self, path: &Path) -> Result<Vec<Observation>, LogError>;

    /// Every observation in the log, ordered by (run, path). Used by `check`, which
    /// recomputes derived state from the raw rows rather than from any cached form.
    fn all_observations(&self) -> Result<Vec<Observation>, LogError>;

    /// Count of runs and observations. Cheap enough for `status`.
    fn counts(&self) -> Result<(u64, u64), LogError>;

    /// Names of the stored tables. A structural query, not a data read; `check` uses it to
    /// demonstrate that no derived state is persisted.
    fn tables(&self) -> Result<Vec<String>, LogError>;
}
