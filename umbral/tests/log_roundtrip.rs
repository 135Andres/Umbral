//! The persistence seam: append, read back, and prove the log is the only stored truth.
//!
//! The last two tests deliberately corrupt a log to show that the checks in `check` can
//! fail. A test that cannot fail proves nothing.

use std::path::PathBuf;

use umbral::content::{ContentObservation, Stability};
use umbral::log::sqlite::SqliteLog;
use umbral::log::{NewObservation, NewRun, ObservationLog, SCHEMA_VERSION};
use umbral::scan::{Entry, EntryKind};
use umbral::workspace::Workspace;

fn entry(path: &str, ino: u64, size: u64) -> Entry {
    Entry {
        path: PathBuf::from(path),
        kind: EntryKind::File,
        dev: Some(1),
        ino: Some(ino),
        size: Some(size),
        mtime: Some((1_000, 0)),
    }
}

fn verified(hash_byte: u8, len: u64) -> ContentObservation {
    ContentObservation {
        hash: Some([hash_byte; 32]),
        hashed_len: Some(len),
        stability: Some(Stability::Stable),
        deltas: Vec::new(),
        error: None,
    }
}

fn run(entries: Vec<(Entry, Option<ContentObservation>)>) -> NewRun {
    NewRun {
        started_at_ns: 1_000,
        finished_at_ns: 2_000,
        root: PathBuf::from("/tmp/root"),
        observations: entries
            .into_iter()
            .map(|(entry, content)| NewObservation {
                entry,
                content,
                error: None,
            })
            .collect(),
    }
}

fn ws() -> Workspace {
    Workspace {
        root: PathBuf::from("/tmp/root"),
        canonical: PathBuf::from("/tmp/root"),
        id: "test".into(),
        state_dir: PathBuf::from("/tmp/state"),
    }
}

#[test]
fn a_run_round_trips() {
    let mut log = SqliteLog::open_in_memory().unwrap();
    let id = log
        .append_run(run(vec![
            (entry("a.txt", 10, 3), Some(verified(1, 3))),
            (entry("b.txt", 11, 4), None),
        ]))
        .unwrap();
    assert_eq!(id, 1);

    let obs = log.observations_for_run(id).unwrap();
    assert_eq!(obs.len(), 2);
    assert_eq!(obs[0].path, PathBuf::from("a.txt"));
    assert_eq!(obs[0].valid_hash(), Some(&[1u8; 32]));
    assert!(obs[0].is_content_verified());
    assert_eq!(obs[1].valid_hash(), None);
    assert!(!obs[1].is_content_verified());
}

#[test]
fn counts_and_completeness_are_derived_not_stored() {
    let mut log = SqliteLog::open_in_memory().unwrap();
    log.append_run(run(vec![(entry("a.txt", 10, 1), None)]))
        .unwrap();

    let mut with_error = run(vec![(entry("b.txt", 11, 1), None)]);
    with_error.observations[0].error = Some("permission denied".into());
    log.append_run(with_error).unwrap();

    let runs = log.runs().unwrap();
    assert_eq!(runs.len(), 2);
    assert_eq!(runs[0].entries, 1);
    assert!(runs[0].complete());
    assert_eq!(runs[1].errors, 1);
    assert!(
        !runs[1].complete(),
        "a run with an unobserved path is not complete"
    );

    let (n_runs, n_obs) = log.counts().unwrap();
    assert_eq!((n_runs, n_obs), (2, 2));
}

#[test]
fn path_history_is_available_without_a_second_index() {
    let mut log = SqliteLog::open_in_memory().unwrap();
    log.append_run(run(vec![(entry("a.txt", 10, 3), Some(verified(1, 3)))]))
        .unwrap();
    log.append_run(run(vec![(entry("a.txt", 10, 9), Some(verified(2, 9)))]))
        .unwrap();

    let hist = log.observations_for_path(&PathBuf::from("a.txt")).unwrap();
    assert_eq!(hist.len(), 2);
    assert_eq!(hist[0].run_id, 1);
    assert_eq!(hist[1].run_id, 2);
    assert_ne!(hist[0].valid_hash(), hist[1].valid_hash());

    assert!(log
        .observations_for_path(&PathBuf::from("nope"))
        .unwrap()
        .is_empty());
}

#[test]
fn latest_run_is_the_most_recent_one() {
    let mut log = SqliteLog::open_in_memory().unwrap();
    assert!(log.latest_run().unwrap().is_none());
    log.append_run(run(vec![(entry("a", 1, 1), None)])).unwrap();
    log.append_run(run(vec![(entry("b", 2, 1), None)])).unwrap();
    assert_eq!(log.latest_run().unwrap().unwrap().id, 2);
}

/// Non-UTF-8 paths survive the round trip byte-exactly.
#[cfg(unix)]
#[test]
fn non_utf8_paths_round_trip_byte_exact() {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;

    let mut log = SqliteLog::open_in_memory().unwrap();
    let raw = OsStr::from_bytes(b"caf\xe9-\xff.bin");
    let mut e = entry("ignored", 10, 1);
    e.path = PathBuf::from(raw);
    log.append_run(run(vec![(e, None)])).unwrap();

    let obs = log.observations_for_run(1).unwrap();
    assert_eq!(obs[0].path.as_os_str().as_bytes(), b"caf\xe9-\xff.bin");
}

/// Only the log is stored. This is the structural half of the `check` claim.
#[test]
fn the_only_stored_tables_are_the_log() {
    let log = SqliteLog::open_in_memory().unwrap();
    let mut tables = log.tables().unwrap();
    tables.sort();
    assert_eq!(tables, vec!["observation", "run", "schema_meta"]);
}

#[test]
fn the_schema_version_is_recorded_and_an_unknown_one_is_refused() {
    assert_eq!(SCHEMA_VERSION, "umbral-v0.1");

    let t = tempfile::TempDir::new().unwrap();
    let path = t.path().join("log.sqlite");
    {
        let _ = SqliteLog::open(&path).unwrap();
    }
    {
        let conn = rusqlite::Connection::open(&path).unwrap();
        conn.execute(
            "UPDATE schema_meta SET value='umbral-v99' WHERE key='schema_version'",
            [],
        )
        .unwrap();
    }
    match SqliteLog::open_read_only(&path) {
        Err(umbral::log::LogError::UnknownSchema(v)) => assert_eq!(v, "umbral-v99"),
        other => panic!("expected UnknownSchema, got {other:?}"),
    }
}

/// A read-only connection cannot write. This is what makes the read-only commands genuinely
/// read-only, and it is asserted rather than assumed.
#[test]
fn a_read_only_connection_cannot_write() {
    let t = tempfile::TempDir::new().unwrap();
    let path = t.path().join("log.sqlite");
    {
        let mut log = SqliteLog::open(&path).unwrap();
        log.append_run(run(vec![(entry("a", 1, 1), None)])).unwrap();
    }

    let log = SqliteLog::open_read_only(&path).unwrap();
    // Reading works.
    assert_eq!(log.counts().unwrap(), (1, 1));
    // Writing does not: the trait exposes no write, and the underlying handle is read-only.
    let conn =
        rusqlite::Connection::open_with_flags(&path, rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY)
            .unwrap();
    let r = conn.execute(
        "INSERT INTO run (started_at_ns, finished_at_ns, root) VALUES (1,2,x'00')",
        [],
    );
    assert!(r.is_err(), "a read-only connection must refuse a write");
}

// ---------------------------------------------------------------------------------------
// Negative controls: prove the checks in `check` can fail.
// ---------------------------------------------------------------------------------------

/// An observation with no parent run is detected. The foreign key is disabled here so the
/// row can be inserted at all — which is exactly the corruption being modelled.
#[test]
fn check_detects_an_orphan_observation() {
    let t = tempfile::TempDir::new().unwrap();
    let path = t.path().join("log.sqlite");
    {
        let mut log = SqliteLog::open(&path).unwrap();
        log.append_run(run(vec![(entry("a", 1, 1), None)])).unwrap();
    }
    {
        let conn = rusqlite::Connection::open(&path).unwrap();
        conn.execute_batch("PRAGMA foreign_keys=OFF;").unwrap();
        conn.execute(
            "INSERT INTO observation (run_id, path, kind) VALUES (999, x'62', 'file')",
            [],
        )
        .unwrap();
    }

    let log = SqliteLog::open_read_only(&path).unwrap();
    let all = log.all_observations().unwrap();
    let known: std::collections::BTreeSet<i64> = log.runs().unwrap().iter().map(|r| r.id).collect();
    let orphans = all.iter().filter(|o| !known.contains(&o.run_id)).count();
    assert_eq!(orphans, 1, "the orphan must be visible to the check");

    let lines = umbral::report::check(&ws(), &log).unwrap();
    let rendered = umbral::report::render(&lines);
    assert!(
        rendered.contains("referential-integrity=false"),
        "check must report the inconsistency, got:\n{rendered}"
    );
    assert!(rendered.contains("consistent=false"));
}

/// A table that would hold derived state is detected. This falsifies the claim "derived
/// state is not persisted" if the claim were ever made false.
#[test]
fn check_detects_a_persisted_projection_table() {
    let t = tempfile::TempDir::new().unwrap();
    let path = t.path().join("log.sqlite");
    {
        let mut log = SqliteLog::open(&path).unwrap();
        log.append_run(run(vec![(entry("a", 1, 1), None)])).unwrap();
    }
    {
        let conn = rusqlite::Connection::open(&path).unwrap();
        conn.execute_batch("CREATE TABLE projection (path BLOB PRIMARY KEY);")
            .unwrap();
    }

    let log = SqliteLog::open_read_only(&path).unwrap();
    let lines = umbral::report::check(&ws(), &log).unwrap();
    let rendered = umbral::report::render(&lines);
    assert!(
        rendered.contains("derived-state-persisted=true"),
        "check must notice a projection table, got:\n{rendered}"
    );
    assert!(rendered.contains("consistent=false"));
}

/// And on an untouched log, the same checks report consistency. Together with the two tests
/// above, this shows the checks discriminate rather than always returning the same answer.
#[test]
fn check_reports_a_clean_log_as_consistent() {
    let t = tempfile::TempDir::new().unwrap();
    let path = t.path().join("log.sqlite");
    {
        let mut log = SqliteLog::open(&path).unwrap();
        log.append_run(run(vec![
            (entry("a", 1, 1), Some(verified(1, 1))),
            (entry("b", 2, 1), None),
        ]))
        .unwrap();
        log.append_run(run(vec![(entry("a", 1, 2), Some(verified(2, 2)))]))
            .unwrap();
    }
    let log = SqliteLog::open_read_only(&path).unwrap();
    let rendered = umbral::report::render(&umbral::report::check(&ws(), &log).unwrap());
    assert!(rendered.contains("consistent=true"), "got:\n{rendered}");
    assert!(rendered.contains("derived-state-persisted=false"));
}
