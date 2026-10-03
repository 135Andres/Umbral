//! The persistence seam: append, read back, and prove the log is the only stored truth.
//!
//! The last two tests deliberately corrupt a log to show that the checks in `check` can
//! fail. A test that cannot fail proves nothing.

use std::path::PathBuf;

use umbral::content::{ContentError, ContentObservation, GuardDelta, Stability};
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
        ctime: None,
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
                reused_from: None,
                traversal: None,
            })
            .collect(),
        root_error: None,
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
    assert_eq!(SCHEMA_VERSION, "umbral-v0.2.1");

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

/// D-V01-9. `created` is a claim relative to the reference run. When that reference was
/// incomplete — some path could not be observed — the entry may have existed and simply not
/// been seen, so the line must say so. Before the fix the reference's completeness was not
/// part of the evidence at all.
#[test]
fn created_states_whether_its_reference_was_complete() {
    let t = tempfile::TempDir::new().unwrap();
    let path = t.path().join("log.sqlite");
    {
        let mut log = SqliteLog::open(&path).unwrap();
        let mut incomplete = run(vec![(entry("a", 1, 1), Some(verified(1, 1)))]);
        incomplete.observations.push(NewObservation {
            entry: Entry {
                path: PathBuf::from("locked"),
                kind: EntryKind::Other,
                dev: None,
                ino: None,
                size: None,
                mtime: None,
                ctime: None,
            },
            content: None,
            error: Some("permission denied".into()),
            reused_from: None,
            traversal: None,
        });
        log.append_run(incomplete).unwrap();
        log.append_run(run(vec![
            (entry("a", 1, 1), Some(verified(1, 1))),
            (entry("b", 2, 1), Some(verified(2, 1))),
        ]))
        .unwrap();
        log.append_run(run(vec![
            (entry("a", 1, 1), Some(verified(1, 1))),
            (entry("b", 2, 1), Some(verified(2, 1))),
            (entry("c", 3, 1), Some(verified(3, 1))),
        ]))
        .unwrap();
    }

    // Runs 2 -> 3: the reference (run 2) was complete.
    let log = SqliteLog::open_read_only(&path).unwrap();
    let rendered = umbral::report::render(&umbral::report::changes(&ws(), &log).unwrap());
    assert!(
        rendered.contains(
            "created  path=c  reference-absent=2  compared=3:c  compared-fields=dev,ino  reference-complete=true"
        ),
        "got:\n{rendered}"
    );

    // Runs 1 -> 2: the reference (run 1) was incomplete.
    let t2 = tempfile::TempDir::new().unwrap();
    let path2 = t2.path().join("log.sqlite");
    {
        let src = rusqlite::Connection::open(&path).unwrap();
        src.execute_batch(&format!("VACUUM INTO '{}'; ", path2.display()))
            .unwrap();
        let dst = rusqlite::Connection::open(&path2).unwrap();
        dst.execute_batch(
            "DELETE FROM observation WHERE run_id = 3; DELETE FROM run WHERE run_id = 3;",
        )
        .unwrap();
    }
    let log2 = SqliteLog::open_read_only(&path2).unwrap();
    let rendered = umbral::report::render(&umbral::report::changes(&ws(), &log2).unwrap());
    assert!(
        rendered.contains(
            "created  path=b  reference-absent=1  compared=2:b  compared-fields=dev,ino  reference-complete=false"
        ),
        "got:\n{rendered}"
    );
}

/// The `umbral-v0.1` schema exactly as it was before D-V01-10, to prove that a log written by
/// an earlier build is still read and is migrated in place.
const SCHEMA_V0_1: &str = "
CREATE TABLE schema_meta (key TEXT PRIMARY KEY, value TEXT NOT NULL);
CREATE TABLE run (
    run_id INTEGER PRIMARY KEY AUTOINCREMENT, started_at_ns INTEGER NOT NULL,
    finished_at_ns INTEGER NOT NULL, root BLOB NOT NULL);
CREATE TABLE observation (
    run_id INTEGER NOT NULL REFERENCES run(run_id), path BLOB NOT NULL, kind TEXT NOT NULL,
    dev INTEGER, ino INTEGER, size INTEGER, mtime_s INTEGER, mtime_ns INTEGER, hash BLOB,
    hashed_len INTEGER, hash_stability TEXT, hash_deltas TEXT, obs_error TEXT,
    PRIMARY KEY (run_id, path), CHECK (hash IS NULL OR hash_stability IS NOT NULL));
CREATE INDEX idx_observation_path ON observation(path);
INSERT INTO schema_meta VALUES ('schema_version', 'umbral-v0.1');
INSERT INTO run VALUES (1, 1000, 2000, X'2F746D702F726F6F74');
INSERT INTO observation (run_id, path, kind, dev, ino, size, mtime_s, mtime_ns,
    hash, hashed_len, hash_stability) VALUES
    (1, X'6F6B', 'file', 1, 1, 1, 1000, 0, X'0101010101010101010101010101010101010101010101010101010101010101', 1, 'stable'),
    (1, X'6C6F636B6564', 'file', 1, 2, 1, 1000, 0, NULL, NULL, NULL);
";

/// D-V01-10. A content-acquisition error is evidence about how the content was (not)
/// obtained, and it must survive persistence (`UD-023` §4). Before the fix it was dropped by
/// `append_run`: a file that could not be read left no trace of why.
#[test]
fn a_content_acquisition_error_survives_the_log() {
    let mut log = SqliteLog::open_in_memory().unwrap();
    let denied = ContentObservation {
        hash: None,
        hashed_len: None,
        stability: None,
        deltas: Vec::new(),
        error: Some(ContentError::PermissionDenied),
    };
    let failed = ContentObservation {
        error: Some(ContentError::ReadError(
            "Input/output error (os error 5)".into(),
        )),
        ..denied.clone()
    };
    let id = log
        .append_run(run(vec![
            (entry("a", 1, 1), Some(verified(1, 1))),
            (entry("b", 2, 1), Some(denied)),
            (entry("c", 3, 1), Some(failed)),
        ]))
        .unwrap();

    let obs = log.observations_for_run(id).unwrap();
    assert_eq!(obs[0].content_error, None);
    assert_eq!(obs[1].content_error.as_deref(), Some("permission-denied"));
    assert_eq!(
        obs[2].content_error.as_deref(),
        Some("read-error: Input/output error (os error 5)")
    );

    let lines = umbral::report::show(&ws(), &log, std::path::Path::new("b")).unwrap();
    let rendered = umbral::report::render(&lines);
    assert!(
        rendered.contains("unknown   observation=1:b  content-error=permission-denied"),
        "got:\n{rendered}"
    );
}

/// `UD-031`. A content reading that kept changing yields no value, so it is `unknown` — a value
/// that is not determinable — and not `ambiguous`, which is reserved for a classification the
/// evidence leaves open between several outcomes. The deltas are its diagnostic.
#[test]
fn an_unstable_content_reading_is_shown_as_unknown() {
    let mut log = SqliteLog::open_in_memory().unwrap();
    let unstable = ContentObservation {
        hash: None,
        hashed_len: None,
        stability: Some(Stability::Unstable),
        deltas: vec![GuardDelta::SizeChanged, GuardDelta::MtimeChanged],
        error: None,
    };
    log.append_run(run(vec![(entry("a", 1, 1), Some(unstable))]))
        .unwrap();

    let lines = umbral::report::show(&ws(), &log, std::path::Path::new("a")).unwrap();
    let rendered = umbral::report::render(&lines);
    assert!(
        rendered.contains(
            "unknown   observation=1:a  reason=unstable-observation  deltas=size-changed,mtime-changed"
        ),
        "got:\n{rendered}"
    );
    assert!(
        !rendered.lines().any(|l| l.starts_with("ambiguous")),
        "no classification is open here, so nothing is ambiguous:\n{rendered}"
    );
}

/// D-V01-10, compatibility. A log written before the column existed is read without being
/// modified, and a file row that has no content result says that its reason was not
/// recorded — rather than claiming there was no error.
#[test]
fn a_v0_1_log_is_read_and_its_missing_diagnostics_are_marked_not_recorded() {
    let t = tempfile::TempDir::new().unwrap();
    let path = t.path().join("log.sqlite");
    rusqlite::Connection::open(&path)
        .unwrap()
        .execute_batch(SCHEMA_V0_1)
        .unwrap();

    let log = SqliteLog::open_read_only(&path).unwrap();
    let obs = log.observations_for_run(1).unwrap();
    let locked = obs
        .iter()
        .find(|o| o.path.as_path() == std::path::Path::new("locked"))
        .unwrap();
    assert_eq!(locked.content_error.as_deref(), Some("not-recorded"));
    let ok = obs
        .iter()
        .find(|o| o.path.as_path() == std::path::Path::new("ok"))
        .unwrap();
    assert_eq!(ok.content_error, None);
    drop(log);

    let version: String = rusqlite::Connection::open(&path)
        .unwrap()
        .query_row(
            "SELECT value FROM schema_meta WHERE key='schema_version'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(version, "umbral-v0.1", "a read-only open must not migrate");
}

/// D-V01-10, migration. Opening a `umbral-v0.1` log for writing migrates it in place,
/// additively: earlier rows are kept, marked as not recorded where they lack a content
/// result, and new runs record their diagnostics.
#[test]
fn a_v0_1_log_is_migrated_in_place_when_opened_for_writing() {
    let t = tempfile::TempDir::new().unwrap();
    let path = t.path().join("log.sqlite");
    rusqlite::Connection::open(&path)
        .unwrap()
        .execute_batch(SCHEMA_V0_1)
        .unwrap();

    let mut log = SqliteLog::open(&path).unwrap();
    let id = log
        .append_run(run(vec![(
            entry("new", 9, 1),
            Some(ContentObservation {
                hash: None,
                hashed_len: None,
                stability: None,
                deltas: Vec::new(),
                error: Some(ContentError::NotFound),
            }),
        )]))
        .unwrap();
    assert_eq!(
        log.observations_for_run(id).unwrap()[0]
            .content_error
            .as_deref(),
        Some("not-found")
    );
    let old = log.observations_for_run(1).unwrap();
    assert_eq!(old.len(), 2);
    assert_eq!(
        old.iter()
            .find(|o| o.path.as_path() == std::path::Path::new("locked"))
            .unwrap()
            .content_error
            .as_deref(),
        Some("not-recorded")
    );
    drop(log);

    let version: String = rusqlite::Connection::open(&path)
        .unwrap()
        .query_row(
            "SELECT value FROM schema_meta WHERE key='schema_version'",
            [],
            |r| r.get(0),
        )
        .unwrap();
    assert_eq!(version, "umbral-v0.2.1");
}

/// D-V01-11. `check` must look at the stored values, not at a normalised reading of them:
/// before the fix a hash of the wrong length read back as "no hash", an unknown stability as
/// "none" and an unknown kind as `other`, so a corrupted row was invisible to `check`.
#[test]
fn check_detects_stored_values_this_build_cannot_interpret() {
    let t = tempfile::TempDir::new().unwrap();
    let path = t.path().join("log.sqlite");
    {
        let mut log = SqliteLog::open(&path).unwrap();
        log.append_run(run(vec![
            (entry("a", 1, 1), Some(verified(1, 1))),
            (entry("b", 2, 1), Some(verified(2, 1))),
        ]))
        .unwrap();
    }
    {
        let conn = rusqlite::Connection::open(&path).unwrap();
        conn.execute_batch(
            "UPDATE observation SET hash = X'0102030405' WHERE path = X'61';
             UPDATE observation SET hash_stability = 'sturdy', kind = 'fifo-ish',
                                    mtime_ns = 2000000000 WHERE path = X'62';",
        )
        .unwrap();
    }

    let log = SqliteLog::open_read_only(&path).unwrap();
    let rendered = umbral::report::render(&umbral::report::check(&ws(), &log).unwrap());
    assert!(rendered.contains("consistent=false"), "got:\n{rendered}");
    assert!(
        rendered.contains("row-values-valid=false  invalid-values=4"),
        "got:\n{rendered}"
    );
    for field in ["hash", "hash_stability", "kind", "mtime_ns"] {
        assert!(
            rendered.contains(&format!("field={field}")),
            "{field} not reported:\n{rendered}"
        );
    }
}

/// D-V01-11. Every verification `check` prints must be one that can fail. These three could
/// not: a per-run count compared with itself, the same table read twice through the same
/// parser, and duplicates the primary key already forbids.
#[test]
fn check_prints_no_verification_that_cannot_fail() {
    let log = SqliteLog::open_in_memory().unwrap();
    let rendered = umbral::report::render(&umbral::report::check(&ws(), &log).unwrap());
    for gone in [
        "run-counts-agree",
        "derived-state-recomputed",
        "duplicate-entries",
    ] {
        assert!(
            !rendered.contains(gone),
            "{gone} still printed:\n{rendered}"
        );
    }
    assert!(
        rendered.contains("row-values-valid=true  invalid-values=0"),
        "got:\n{rendered}"
    );
    assert!(rendered.contains("consistent=true"), "got:\n{rendered}");
}

fn table_names(path: &std::path::Path) -> Vec<String> {
    let conn = rusqlite::Connection::open(path).unwrap();
    let mut stmt = conn
        .prepare("SELECT name FROM sqlite_master WHERE type='table' ORDER BY name")
        .unwrap();
    stmt.query_map([], |r| r.get(0))
        .unwrap()
        .map(|r| r.unwrap())
        .collect()
}

/// D-V01-13. A log this build does not understand is refused before anything is written to it.
/// Before the fix, `open` created its tables first and checked the version afterwards, so a log
/// written by a later build was modified by the build that then refused it.
#[test]
fn a_log_of_an_unknown_version_is_refused_without_being_modified() {
    let t = tempfile::TempDir::new().unwrap();
    let path = t.path().join("future.sqlite");
    rusqlite::Connection::open(&path)
        .unwrap()
        .execute_batch(
            "CREATE TABLE schema_meta (key TEXT PRIMARY KEY, value TEXT NOT NULL);
             INSERT INTO schema_meta VALUES ('schema_version', 'umbral-v9');",
        )
        .unwrap();
    let before = table_names(&path);

    let err = SqliteLog::open(&path).unwrap_err();
    assert!(matches!(err, umbral::log::LogError::UnknownSchema(ref v) if v == "umbral-v9"));
    assert_eq!(table_names(&path), before, "the refused log was modified");
}

/// D-V01-13. A database that is not an umbral log is not adopted as one.
#[test]
fn a_database_without_a_schema_version_is_not_adopted() {
    let t = tempfile::TempDir::new().unwrap();
    let path = t.path().join("other.sqlite");
    rusqlite::Connection::open(&path)
        .unwrap()
        .execute_batch("CREATE TABLE notes (body TEXT);")
        .unwrap();

    assert!(matches!(
        SqliteLog::open(&path).unwrap_err(),
        umbral::log::LogError::UnknownSchema(_)
    ));
    assert_eq!(table_names(&path), vec!["notes".to_string()]);
}

/// D-V01-16. A path whose metadata could not be obtained at all is stored with a placeholder
/// kind. Before the fix `show` printed that placeholder as `observed kind=other`: a value the
/// filesystem never stated, presented as if it had.
#[test]
fn a_path_whose_metadata_failed_is_not_shown_as_observed() {
    let mut log = SqliteLog::open_in_memory().unwrap();
    log.append_run(NewRun {
        started_at_ns: 1_000,
        finished_at_ns: 2_000,
        root: PathBuf::from("/tmp/root"),
        observations: vec![NewObservation {
            entry: Entry {
                path: PathBuf::from("gone"),
                kind: EntryKind::Other,
                dev: None,
                ino: None,
                size: None,
                mtime: None,
                ctime: None,
            },
            content: None,
            error: Some("No such file or directory (os error 2)".into()),
            reused_from: None,
            traversal: None,
        }],
        root_error: None,
    })
    .unwrap();

    let lines = umbral::report::show(&ws(), &log, std::path::Path::new("gone")).unwrap();
    let rendered = umbral::report::render(&lines);
    assert!(
        !rendered.lines().any(|l| l.starts_with("observed")),
        "nothing was observed:\n{rendered}"
    );
    assert!(
        rendered.contains("fields=kind,size,mtime,physical-identity"),
        "got:\n{rendered}"
    );

    let status = umbral::report::render(&umbral::report::status(&ws(), &log).unwrap());
    assert!(
        status.contains("other=0"),
        "an unobserved kind is not `other`:\n{status}"
    );
    assert!(status.contains("kind-unknown=1"), "got:\n{status}");
}

// ---------------------------------------------------------------------------------------
// v0.2 slice 3 — `hash_read_run` and `ctime` (A2-T3-6, A2-T3-7, A2-T3-8; `UD-036`)
// ---------------------------------------------------------------------------------------

/// `umbral-v0.1.1`: the v0.1 schema plus `content_error`, exactly as that build wrote it.
fn schema_v0_1_1() -> String {
    SCHEMA_V0_1
        .replace(
            "hashed_len INTEGER, hash_stability TEXT, hash_deltas TEXT, obs_error TEXT,",
            "hashed_len INTEGER, hash_stability TEXT, hash_deltas TEXT, obs_error TEXT, content_error TEXT,",
        )
        .replace("'umbral-v0.1')", "'umbral-v0.1.1')")
}

fn stored_version(path: &std::path::Path) -> String {
    rusqlite::Connection::open(path)
        .unwrap()
        .query_row(
            "SELECT value FROM schema_meta WHERE key='schema_version'",
            [],
            |r| r.get(0),
        )
        .unwrap()
}

/// A2-T3-7. Both earlier schemas migrate in one step to `umbral-v0.2`. A reading stored by those
/// builds was read in its own run — they had no skip — so its `hash_read_run` is its run; their
/// `ctime` was never recorded, and is reported as such, not as "not obtainable".
#[test]
fn earlier_logs_migrate_with_their_readings_attributed_and_ctime_not_recorded() {
    for (name, schema) in [
        ("v0.1", SCHEMA_V0_1.to_string()),
        ("v0.1.1", schema_v0_1_1()),
    ] {
        let t = tempfile::TempDir::new().unwrap();
        let path = t.path().join("log.sqlite");
        rusqlite::Connection::open(&path)
            .unwrap()
            .execute_batch(&schema)
            .unwrap();

        // Read-only first: the same values, derived, and nothing written.
        {
            let log = SqliteLog::open_read_only(&path).unwrap();
            let ok = &log
                .observations_for_path(std::path::Path::new("ok"))
                .unwrap()[0];
            assert_eq!(ok.hash_read_run, Some(1), "{name}");
            assert!(!ok.ctime_recorded, "{name}");
        }
        assert_ne!(
            stored_version(&path),
            "umbral-v0.2.1",
            "{name}: a read-only open wrote"
        );

        let mut log = SqliteLog::open(&path).unwrap();
        assert_eq!(stored_version(&path), "umbral-v0.2.1", "{name}");
        let old = log.observations_for_run(1).unwrap();
        let ok = old
            .iter()
            .find(|o| o.path == std::path::Path::new("ok"))
            .unwrap();
        let locked = old
            .iter()
            .find(|o| o.path == std::path::Path::new("locked"))
            .unwrap();
        assert_eq!(ok.hash_read_run, Some(1), "{name}");
        assert_eq!(
            locked.hash_read_run, None,
            "{name}: no reading, no reading's run"
        );
        assert_eq!((ok.ctime, ok.ctime_recorded), (None, false), "{name}");

        let id = log
            .append_run(run(vec![(entry("new", 9, 1), Some(verified(3, 1)))]))
            .unwrap();
        let new = &log.observations_for_run(id).unwrap()[0];
        assert!(
            new.ctime_recorded,
            "{name}: runs after the migration record ctime"
        );
        assert_eq!(new.hash_read_run, Some(id), "{name}");

        let show = umbral::report::render(
            &umbral::report::show(&ws(), &log, std::path::Path::new("ok")).unwrap(),
        );
        assert!(
            show.contains("unknown   observation=1:ok  fields=ctime  reason=not-recorded"),
            "{name}:\n{show}"
        );
    }
}

fn corrupt_and_check(sql: &str) -> String {
    let t = tempfile::TempDir::new().unwrap();
    let path = t.path().join("log.sqlite");
    {
        let mut log = SqliteLog::open(&path).unwrap();
        log.append_run(run(vec![(entry("a", 1, 1), Some(verified(1, 1)))]))
            .unwrap();
        let mut reused = run(vec![(entry("a", 1, 1), Some(verified(1, 1)))]);
        reused.observations[0].reused_from = Some(1);
        log.append_run(reused).unwrap();
    }
    let clean = {
        let log = SqliteLog::open_read_only(&path).unwrap();
        umbral::report::render(&umbral::report::check(&ws(), &log).unwrap())
    };
    assert!(
        clean.contains("readings-consistent=true  invalid-readings=0"),
        "{clean}"
    );
    rusqlite::Connection::open(&path)
        .unwrap()
        .execute_batch(sql)
        .unwrap();
    let log = SqliteLog::open_read_only(&path).unwrap();
    umbral::report::render(&umbral::report::check(&ws(), &log).unwrap())
}

/// A2-T3-8. Every violation of the reading invariant is reported, and makes the log
/// inconsistent.
#[test]
fn check_reports_every_broken_reading_attribution() {
    for (sql, reason) in [
        (
            "UPDATE observation SET hash_read_run = 7 WHERE run_id = 2;",
            "read-run-after-own-run",
        ),
        (
            "UPDATE observation SET hash = X'0202020202020202020202020202020202020202020202020202020202020202' WHERE run_id = 2;",
            "differs-from-source",
        ),
        (
            "UPDATE observation SET hash_read_run = 2 WHERE run_id = 1;",
            "read-run-after-own-run",
        ),
        (
            "DELETE FROM observation WHERE run_id = 1;",
            "source-missing",
        ),
        (
            "UPDATE observation SET hash_read_run = NULL WHERE run_id = 2;",
            "reading-without-read-run",
        ),
    ] {
        let out = corrupt_and_check(sql);
        assert!(out.contains(&format!("reason={reason}")), "{sql}\n{out}");
        assert!(out.contains("consistent=false"), "{sql}\n{out}");
    }
}
