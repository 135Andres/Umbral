//! v0.2 slice 4 — traversal facts, the rules of each run, and the cross-cutting checks
//! (`docs/candidates/V0.2-SCOPE-PROPOSAL.md` §9.8, criteria `A2-T4-*`, `UD-037`).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use umbral::log::sqlite::SqliteLog;
use umbral::log::{ObservationLog, SCHEMA_VERSION};
use umbral::observe::{observe, Policy};
use umbral::scan::{classify, Entry, EntryKind, TraversalClass, SCOPE};
use umbral::workspace::Workspace;

fn ws(root: &Path) -> Workspace {
    Workspace {
        root: root.to_path_buf(),
        canonical: root.to_path_buf(),
        id: "test".into(),
        state_dir: root.join(".state"),
    }
}

fn render(lines: &[umbral::report::Line]) -> String {
    umbral::report::render(lines)
}

/// Permission checks do not apply to root; a test that relies on them says so instead of
/// passing for the wrong reason.
#[cfg(unix)]
fn permissions_apply(dir: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    let probe = dir.join(".probe");
    std::fs::create_dir(&probe).unwrap();
    std::fs::set_permissions(&probe, std::fs::Permissions::from_mode(0o000)).unwrap();
    let blocked = std::fs::read_dir(&probe).is_err();
    std::fs::set_permissions(&probe, std::fs::Permissions::from_mode(0o755)).unwrap();
    std::fs::remove_dir(&probe).unwrap();
    blocked
}

// ---------------------------------------------------------------------------------------
// A2-T4-1 — classes decided at observation time
// ---------------------------------------------------------------------------------------

fn entry(path: &str, kind: EntryKind) -> Entry {
    Entry {
        path: PathBuf::from(path),
        kind,
        dev: Some(1),
        ino: Some(1),
        size: Some(0),
        mtime: Some((1, 0)),
        ctime: Some((1, 0)),
    }
}

#[test]
fn a_failure_on_an_observed_directory_is_not_descended_and_any_other_is_metadata_failed() {
    let entries = vec![
        entry("docs", EntryKind::Dir),
        entry("a.txt", EntryKind::File),
    ];
    assert_eq!(
        classify(Path::new("docs"), &entries),
        TraversalClass::NotDescended
    );
    // Listed, then gone before its `lstat`: no entry was obtained for it.
    assert_eq!(
        classify(Path::new("vanished.txt"), &entries),
        TraversalClass::MetadataFailed
    );
    // A file whose entry exists is not a directory that could not be listed.
    assert_eq!(
        classify(Path::new("a.txt"), &entries),
        TraversalClass::MetadataFailed
    );
}

#[cfg(unix)]
#[test]
fn an_unreadable_directory_is_recorded_as_not_descended() {
    use std::os::unix::fs::PermissionsExt;
    let t = tempfile::TempDir::new().unwrap();
    if !permissions_apply(t.path()) {
        eprintln!("NOT EXERCISED: permissions do not apply to this user");
        return;
    }
    std::fs::create_dir(t.path().join("locked")).unwrap();
    std::fs::write(t.path().join("locked/inside.txt"), b"x").unwrap();
    std::fs::write(t.path().join("a.txt"), b"a").unwrap();
    std::fs::set_permissions(
        t.path().join("locked"),
        std::fs::Permissions::from_mode(0o000),
    )
    .unwrap();

    let mut log = SqliteLog::open_in_memory().unwrap();
    let observed = observe(t.path(), t.path(), &mut log, Policy::Skip);
    std::fs::set_permissions(
        t.path().join("locked"),
        std::fs::Permissions::from_mode(0o755),
    )
    .unwrap();
    let id = observed.unwrap().run_id;

    let rows = log.observations_for_run(id).unwrap();
    let locked = rows.iter().find(|o| o.path == Path::new("locked")).unwrap();
    assert_eq!(locked.traversal.as_deref(), Some("not-descended"));
    assert!(rows
        .iter()
        .all(|o| o.path != Path::new("locked/inside.txt")));

    let show = render(&umbral::report::show(&ws(t.path()), &log, Path::new("locked")).unwrap());
    assert!(
        show.contains("observation=1:locked  traversal=not-descended  observation-error="),
        "got:\n{show}"
    );
    let status = render(&umbral::report::status(&ws(t.path()), &log).unwrap());
    assert!(
        status.contains(
            "run=1  traversal-complete=false  traversal-not-descended=1  traversal-metadata-failed=0  traversal-not-recorded=0"
        ),
        "got:\n{status}"
    );
    assert!(
        !status.contains("unobservable-paths"),
        "replaced (`UD-037`):\n{status}"
    );
}

#[cfg(unix)]
#[test]
fn an_unreadable_root_is_a_fact_of_the_run_not_a_row() {
    use std::os::unix::fs::PermissionsExt;
    let t = tempfile::TempDir::new().unwrap();
    if !permissions_apply(t.path()) {
        eprintln!("NOT EXERCISED: permissions do not apply to this user");
        return;
    }
    let root = t.path().join("root");
    std::fs::create_dir(&root).unwrap();
    std::fs::write(root.join("a.txt"), b"a").unwrap();

    let mut log = SqliteLog::open_in_memory().unwrap();
    observe(&root, &root, &mut log, Policy::Skip).unwrap();
    std::fs::set_permissions(&root, std::fs::Permissions::from_mode(0o000)).unwrap();
    let second = observe(&root, &root, &mut log, Policy::Skip);
    std::fs::set_permissions(&root, std::fs::Permissions::from_mode(0o755)).unwrap();
    let id = second.unwrap().run_id;

    assert!(
        log.observations_for_run(id).unwrap().is_empty(),
        "no row for the root"
    );
    let run = log.run(id).unwrap().unwrap();
    assert!(run.root_error.is_some());
    assert!(
        !run.complete(),
        "a run that could not list its root saw nothing"
    );

    // A2-T4-3: the earlier file is not called deleted.
    let changes = render(&umbral::report::changes(&ws(&root), &log).unwrap());
    assert!(changes.contains("count  deleted=0"), "got:\n{changes}");
    assert!(changes.contains("count  unobserved=1"), "got:\n{changes}");
    let status = render(&umbral::report::status(&ws(&root), &log).unwrap());
    assert!(
        status.contains("run=2  traversal-complete=false  traversal-not-descended=0  traversal-metadata-failed=0  traversal-not-recorded=0  root-not-descended=true"),
        "got:\n{status}"
    );
}

// ---------------------------------------------------------------------------------------
// A2-T4-2 / A2-T4-4 — counts, and the rules of each run
// ---------------------------------------------------------------------------------------

#[test]
fn every_new_run_names_its_tool_version_and_scope() {
    let t = tempfile::TempDir::new().unwrap();
    std::fs::write(t.path().join("a.txt"), b"a").unwrap();
    let mut log = SqliteLog::open_in_memory().unwrap();
    let id = observe(t.path(), t.path(), &mut log, Policy::Skip)
        .unwrap()
        .run_id;
    let run = log.run(id).unwrap().unwrap();
    assert_eq!(run.tool_version.as_deref(), Some(env!("CARGO_PKG_VERSION")));
    assert_eq!(run.scope.as_deref(), Some(SCOPE));
    assert_eq!(SCOPE, "recursive,symlinks-not-followed,no-exclusions");

    let status = render(&umbral::report::status(&ws(t.path()), &log).unwrap());
    assert!(
        status.contains(&format!(
            "run=1  tool-version={}  scope={SCOPE}",
            env!("CARGO_PKG_VERSION")
        )),
        "got:\n{status}"
    );
    assert!(
        status.contains("run=1  traversal-complete=true  traversal-not-descended=0  traversal-metadata-failed=0  traversal-not-recorded=0  root-not-descended=false"),
        "got:\n{status}"
    );
}

// ---------------------------------------------------------------------------------------
// A2-T4-5 / A2-T4-6 — migration, and emissions are never rewritten (A2-V8)
// ---------------------------------------------------------------------------------------

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
    hash, hashed_len, hash_stability, obs_error) VALUES
    (1, X'6F6B', 'file', 1, 1, 1, 1000, 0, X'0101010101010101010101010101010101010101010101010101010101010101', 1, 'stable', NULL),
    (1, X'6C6F636B6564', 'dir', 1, 2, 1, 1000, 0, NULL, NULL, NULL, 'Permission denied (os error 13)');
";

fn schema_v0_1_1() -> String {
    SCHEMA_V0_1
        .replace(
            "hash_deltas TEXT, obs_error TEXT,",
            "hash_deltas TEXT, obs_error TEXT, content_error TEXT,",
        )
        .replace("'umbral-v0.1')", "'umbral-v0.1.1')")
}

fn schema_v0_2() -> String {
    schema_v0_1_1()
        .replace(
            "content_error TEXT,",
            "content_error TEXT, hash_read_run INTEGER, ctime_s INTEGER, ctime_ns INTEGER,",
        )
        .replace(
            "'umbral-v0.1.1')",
            "'umbral-v0.2'), ('ctime_recorded_from_run', '1')",
        )
}

fn version(path: &Path) -> String {
    rusqlite::Connection::open(path)
        .unwrap()
        .query_row(
            "SELECT value FROM schema_meta WHERE key='schema_version'",
            [],
            |r| r.get(0),
        )
        .unwrap()
}

#[test]
fn every_earlier_schema_migrates_and_reports_what_it_did_not_record() {
    for (name, schema) in [
        ("v0.1", SCHEMA_V0_1.to_string()),
        ("v0.1.1", schema_v0_1_1()),
        ("v0.2", schema_v0_2()),
    ] {
        let t = tempfile::TempDir::new().unwrap();
        let path = t.path().join("log.sqlite");
        rusqlite::Connection::open(&path)
            .unwrap()
            .execute_batch(&schema)
            .unwrap();
        let before = version(&path);
        {
            let log = SqliteLog::open_read_only(&path).unwrap();
            let locked = &log.observations_for_path(Path::new("locked")).unwrap()[0];
            assert_eq!(locked.traversal.as_deref(), Some("not-recorded"), "{name}");
            assert_eq!(log.run(1).unwrap().unwrap().tool_version, None, "{name}");
        }
        assert_eq!(version(&path), before, "{name}: a read-only open wrote");

        let log = SqliteLog::open(&path).unwrap();
        assert_eq!(version(&path), SCHEMA_VERSION, "{name}");
        assert_eq!(SCHEMA_VERSION, "umbral-v0.2.1");
        let locked = &log.observations_for_path(Path::new("locked")).unwrap()[0];
        assert_eq!(locked.traversal.as_deref(), Some("not-recorded"), "{name}");
        let ok = &log.observations_for_path(Path::new("ok")).unwrap()[0];
        assert_eq!(ok.traversal, None, "{name}: no failure, no class");
        let run = log.run(1).unwrap().unwrap();
        assert_eq!(
            (run.tool_version, run.scope, run.root_error),
            (None, None, None)
        );

        let status = render(&umbral::report::status(&ws(Path::new("/tmp/root")), &log).unwrap());
        assert!(
            status.contains("unknown   run=1  fields=tool-version,scope  reason=not-recorded"),
            "{name}:\n{status}"
        );
        assert!(
            status.contains(
                "traversal-not-descended=0  traversal-metadata-failed=0  traversal-not-recorded=1"
            ),
            "{name}:\n{status}"
        );
    }
}

type Snapshot = BTreeMap<(String, String), BTreeMap<String, String>>;

/// Every stored value, keyed by (table, row key) and column.
fn snapshot(path: &Path) -> Snapshot {
    let conn = rusqlite::Connection::open(path).unwrap();
    let mut out = Snapshot::new();
    for (table, key) in [
        ("run", "run_id"),
        ("observation", "run_id || ':' || hex(path)"),
    ] {
        let mut stmt = conn
            .prepare(&format!("SELECT {key}, * FROM {table}"))
            .unwrap();
        let names: Vec<String> = stmt.column_names().iter().map(|s| s.to_string()).collect();
        let mut rows = stmt.query([]).unwrap();
        while let Some(r) = rows.next().unwrap() {
            let k: String = r
                .get::<_, rusqlite::types::Value>(0)
                .map(|v| format!("{v:?}"))
                .unwrap();
            let mut cols = BTreeMap::new();
            for (i, n) in names.iter().enumerate().skip(1) {
                let v: rusqlite::types::Value = r.get(i).unwrap();
                cols.insert(n.clone(), format!("{v:?}"));
            }
            out.insert((table.to_string(), k), cols);
        }
    }
    out
}

/// What a migration may add to an earlier row (A2-V8, with `UD-036`'s explicit exception).
fn addition_allowed(column: &str, value: &str, cols: &BTreeMap<String, String>) -> bool {
    match column {
        "content_error" => value == "Null" || value == "Text(\"not-recorded\")",
        "traversal" => value == "Null" || value == "Text(\"not-recorded\")",
        // `UD-036`: a reading stored by a build without a skip was read in its own run.
        "hash_read_run" => value == "Null" || cols.get("run_id") == Some(&value.to_string()),
        "ctime_s" | "ctime_ns" | "tool_version" | "scope" | "root_error" => value == "Null",
        _ => false,
    }
}

#[cfg(unix)]
#[test]
fn no_later_step_rewrites_an_earlier_emission() {
    let t = tempfile::TempDir::new().unwrap();
    let path = t.path().join("log.sqlite");
    rusqlite::Connection::open(&path)
        .unwrap()
        .execute_batch(SCHEMA_V0_1)
        .unwrap();
    let tree = t.path().join("tree");
    std::fs::create_dir(&tree).unwrap();
    std::fs::write(tree.join("a.txt"), b"alpha").unwrap();
    std::fs::write(tree.join("b.txt"), b"bravo").unwrap();

    let mut history: Vec<Snapshot> = vec![snapshot(&path)];
    let steps: Vec<Box<dyn Fn()>> = vec![
        Box::new(|| {}),
        Box::new(|| std::fs::write(tree.join("a.txt"), b"alpha, edited").unwrap()),
        Box::new(|| std::fs::remove_file(tree.join("b.txt")).unwrap()),
        Box::new(|| {}),
    ];
    for step in &steps {
        step();
        let mut log = SqliteLog::open(&path).unwrap(); // the first open migrates
        observe(&tree, &tree, &mut log, Policy::Skip).unwrap();
        drop(log);
        let now = snapshot(&path);
        let earlier = history.last().unwrap();
        for (key, old_cols) in earlier {
            let new_cols = now
                .get(key)
                .unwrap_or_else(|| panic!("row {key:?} removed"));
            for (col, new_value) in new_cols {
                match old_cols.get(col) {
                    Some(old) => assert_eq!(old, new_value, "{key:?}.{col} rewritten"),
                    None => assert!(
                        addition_allowed(col, new_value, new_cols),
                        "{key:?}.{col} added as {new_value}"
                    ),
                }
            }
        }
        history.push(now);
    }
    assert_eq!(
        history
            .last()
            .unwrap()
            .keys()
            .filter(|(t, _)| t == "run")
            .count(),
        5
    );
}

/// E-TD-8 (`V0.2-TECHNICAL-DESIGN.md` §H.4): an incomplete run that also skips a file and loses
/// another. The skipped file keeps its attributed reading, the lost file is `unobserved` — never
/// `deleted` — and the run says why it is incomplete.
#[cfg(unix)]
#[test]
fn an_incomplete_run_with_a_skip_and_a_vanished_file() {
    use std::os::unix::fs::PermissionsExt;
    let t = tempfile::TempDir::new().unwrap();
    if !permissions_apply(t.path()) {
        eprintln!("NOT EXERCISED: permissions do not apply to this user");
        return;
    }
    std::fs::write(t.path().join("kept.txt"), b"kept").unwrap();
    std::fs::write(t.path().join("gone.txt"), b"gone").unwrap();
    std::fs::create_dir(t.path().join("locked")).unwrap();
    let mut log = SqliteLog::open_in_memory().unwrap();
    observe(t.path(), t.path(), &mut log, Policy::Skip).unwrap();

    std::fs::remove_file(t.path().join("gone.txt")).unwrap();
    std::fs::set_permissions(
        t.path().join("locked"),
        std::fs::Permissions::from_mode(0o000),
    )
    .unwrap();
    let second = observe(t.path(), t.path(), &mut log, Policy::Skip);
    std::fs::set_permissions(
        t.path().join("locked"),
        std::fs::Permissions::from_mode(0o755),
    )
    .unwrap();
    let second = second.unwrap();
    assert_eq!(second.counters.read_entries, 0, "kept.txt is skipped");

    let kept = &log.observations_for_path(Path::new("kept.txt")).unwrap()[1];
    assert_eq!(kept.hash_read_run, Some(1));
    let changes = render(&umbral::report::changes(&ws(t.path()), &log).unwrap());
    assert!(changes.contains("count  deleted=0"), "{changes}");
    assert!(changes.contains("unobserved  path=gone.txt"), "{changes}");
    assert!(changes.contains("compared-complete=false"), "{changes}");
    let status = render(&umbral::report::status(&ws(t.path()), &log).unwrap());
    assert!(status.contains("traversal-not-descended=1"), "{status}");
}
