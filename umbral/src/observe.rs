//! One observation run: scan, decide per file whether to read its bytes, persist.
//!
//! In the library rather than the binary so that a test can run it against a log and a
//! directory, with either policy, and compare the two (A2-T3-4).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::content;
use crate::log::{LogError, NewObservation, NewRun, Observation, ObservationLog, RunId};
use crate::scan::{self, Entry, EntryKind, ScanError};
use crate::skip::{decide, Decision};

/// Whether a file may be skipped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Policy {
    /// Skip a file when the condition holds (`crate::skip::decide`).
    Skip,
    /// Read every file — the behaviour of v0.1. The reference the skip is compared against.
    ReadAll,
}

/// The content work one run did: what a skip saves, counted rather than timed (A2-V1).
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Counters {
    /// Regular files whose bytes were read in this run, successfully or not.
    pub read_entries: u64,
    /// Every byte passed to the hasher, retries included.
    pub read_bytes: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Observed {
    pub run_id: RunId,
    pub counters: Counters,
}

#[derive(Debug)]
pub enum ObserveError {
    Scan(ScanError),
    Log(LogError),
    /// The system clock is outside the range the log can record (D-V01-15).
    Clock,
}

impl std::fmt::Display for ObserveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ObserveError::Scan(e) => write!(f, "{e}"),
            ObserveError::Log(e) => write!(f, "{e}"),
            ObserveError::Clock => {
                f.write_str("the system clock is outside the range the log can record")
            }
        }
    }
}

impl std::error::Error for ObserveError {}

/// The current time for a run's timestamps. A run is never recorded at an invented time.
fn now_ns() -> Result<i64, ObserveError> {
    crate::report::unix_ns(std::time::SystemTime::now()).ok_or(ObserveError::Clock)
}

/// Observe `scan_root` and append one run to `log`, recorded under `recorded_root`.
pub fn observe(
    scan_root: &Path,
    recorded_root: &Path,
    log: &mut dyn ObservationLog,
    policy: Policy,
) -> Result<Observed, ObserveError> {
    let started_at_ns = now_ns()?;
    let scan = scan::scan(scan_root).map_err(ObserveError::Scan)?;

    // The skip compares each path only with the previous run's observation of that same path
    // (`UD-036`): a renamed file has none, and is read.
    let previous: BTreeMap<PathBuf, Observation> =
        match (policy, log.latest_run().map_err(ObserveError::Log)?) {
            (Policy::Skip, Some(run)) => log
                .observations_for_run(run.id)
                .map_err(ObserveError::Log)?
                .into_iter()
                .map(|o| (o.path.clone(), o))
                .collect(),
            _ => BTreeMap::new(),
        };

    let mut counters = Counters::default();
    let mut observations: Vec<NewObservation> = Vec::with_capacity(scan.entries.len());
    for entry in &scan.entries {
        let mut row = NewObservation {
            entry: entry.clone(),
            content: None,
            error: None,
            reused_from: None,
            traversal: None,
        };
        if entry.kind == EntryKind::File {
            let prev = previous.get(&entry.path);
            match decide(prev, entry) {
                Decision::Skip => {
                    // Carried, never re-verified: the reading keeps naming the run that read
                    // it (`UD-017`, `UD-021`).
                    let p = prev.expect("a skip always has a previous observation");
                    row.content = Some(content::ContentObservation {
                        hash: p.hash,
                        hashed_len: p.hashed_len,
                        stability: p.stability,
                        deltas: Vec::new(),
                        error: None,
                    });
                    row.reused_from = p.hash_read_run;
                }
                Decision::Read => {
                    let (c, bytes) = content::observe_content_counted(&scan_root.join(&entry.path));
                    counters.read_entries += 1;
                    counters.read_bytes += bytes;
                    row.content = Some(c);
                }
            }
        }
        observations.push(row);
    }

    // Paths that could not be observed at all are recorded as evidence, not dropped.
    //
    // A path can be BOTH an entry and an error: a directory whose metadata is readable but
    // whose contents are not yields an entry (it exists) and an error (it could not be
    // descended into). One path has one row per run, so the error is attached to the
    // existing observation rather than duplicated into a second row.
    let mut by_path: BTreeMap<PathBuf, usize> = BTreeMap::new();
    for (i, o) in observations.iter().enumerate() {
        by_path.insert(o.entry.path.clone(), i);
    }
    for err in &scan.errors {
        match by_path.get(&err.path) {
            Some(&i) => {
                observations[i].error = Some(err.message.clone());
                observations[i].traversal = Some(err.class);
            }
            None => {
                by_path.insert(err.path.clone(), observations.len());
                observations.push(NewObservation {
                    entry: Entry {
                        path: err.path.clone(),
                        kind: EntryKind::Other,
                        dev: None,
                        ino: None,
                        size: None,
                        mtime: None,
                        ctime: None,
                    },
                    content: None,
                    error: Some(err.message.clone()),
                    reused_from: None,
                    traversal: Some(err.class),
                });
            }
        }
    }

    let finished_at_ns = now_ns()?;
    let run_id = log
        .append_run(NewRun {
            started_at_ns,
            finished_at_ns,
            root: recorded_root.to_path_buf(),
            observations,
            root_error: scan.root_error.clone(),
        })
        .map_err(ObserveError::Log)?;
    Ok(Observed { run_id, counters })
}
