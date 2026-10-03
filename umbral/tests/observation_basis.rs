//! v0.2 slice 2a — the per-observation `basis`, and `show` naming its entry
//! (`docs/candidates/V0.2-SCOPE-PROPOSAL.md` §9.5, criteria `A2-T2a-*`, `UD-033`).
//!
//! Classification properties are tested against hand-built observations, never against what a
//! filesystem happens to do (F-6).

mod common;

use std::path::{Path, PathBuf};

use common::{assert_all_labelled, Sandbox};
use proptest::prelude::*;
use umbral::acquisition::{content_diagnostic, content_state, metadata_state, AcquisitionState};
use umbral::content::{ContentError, ContentObservation, GuardDelta, Stability};
use umbral::contract::{self, read_reference, write_reference, ReferenceError};
use umbral::log::sqlite::SqliteLog;
use umbral::log::{NewObservation, NewRun, Observation, ObservationLog};
use umbral::scan::{Entry, EntryKind};
use umbral::workspace::Workspace;

// ---------------------------------------------------------------------------------------
// Fixtures
// ---------------------------------------------------------------------------------------

fn ws() -> Workspace {
    Workspace {
        root: PathBuf::from("/tmp/root"),
        canonical: PathBuf::from("/tmp/root"),
        id: "test".into(),
        state_dir: PathBuf::from("/tmp/state"),
    }
}

fn entry(path: &str, kind: EntryKind, ino: u64) -> Entry {
    Entry {
        path: PathBuf::from(path),
        kind,
        dev: Some(1),
        ino: Some(ino),
        size: Some(4),
        mtime: Some((1_000, 0)),
        ctime: None,
    }
}

fn stable(byte: u8) -> ContentObservation {
    ContentObservation {
        hash: Some([byte; 32]),
        hashed_len: Some(4),
        stability: Some(Stability::Stable),
        deltas: Vec::new(),
        error: None,
    }
}

fn unstable() -> ContentObservation {
    ContentObservation {
        hash: None,
        hashed_len: None,
        stability: Some(Stability::Unstable),
        deltas: vec![GuardDelta::SizeChanged],
        error: None,
    }
}

fn errored(e: ContentError) -> ContentObservation {
    ContentObservation {
        hash: None,
        hashed_len: None,
        stability: None,
        deltas: Vec::new(),
        error: Some(e),
    }
}

/// One run holding a row for every case of the §9.5 table that a current build can write.
fn every_case() -> NewRun {
    let obs =
        |entry: Entry, content: Option<ContentObservation>, error: Option<&str>| NewObservation {
            entry,
            content,
            error: error.map(str::to_string),
            reused_from: None,
        };
    NewRun {
        started_at_ns: 1_000,
        finished_at_ns: 2_000,
        root: PathBuf::from("/tmp/root"),
        observations: vec![
            obs(
                entry("fresh.txt", EntryKind::File, 1),
                Some(stable(1)),
                None,
            ),
            obs(
                entry("moving.txt", EntryKind::File, 2),
                Some(unstable()),
                None,
            ),
            obs(
                entry("locked.txt", EntryKind::File, 3),
                Some(errored(ContentError::PermissionDenied)),
                None,
            ),
            obs(
                entry("vanished.txt", EntryKind::File, 4),
                Some(errored(ContentError::NotFound)),
                None,
            ),
            obs(
                entry("swapped.txt", EntryKind::File, 5),
                Some(errored(ContentError::NotARegularFile)),
                None,
            ),
            obs(
                entry("broken.txt", EntryKind::File, 6),
                Some(errored(ContentError::ReadError(
                    "Input/output error".into(),
                ))),
                None,
            ),
            obs(entry("dir", EntryKind::Dir, 7), None, None),
            obs(entry("link", EntryKind::Symlink, 8), None, None),
            obs(entry("fifo", EntryKind::Other, 9), None, None),
            obs(
                Entry {
                    path: PathBuf::from("gone"),
                    kind: EntryKind::Other,
                    dev: None,
                    ino: None,
                    size: None,
                    mtime: None,
                    ctime: None,
                },
                None,
                Some("No such file or directory (os error 2)"),
            ),
        ],
    }
}

fn log_with_every_case() -> SqliteLog {
    let mut log = SqliteLog::open_in_memory().unwrap();
    log.append_run(every_case()).unwrap();
    log
}

fn render(lines: &[umbral::report::Line]) -> String {
    umbral::report::render(lines)
}

fn show(log: &SqliteLog, path: &str) -> String {
    render(&umbral::report::show(&ws(), log, Path::new(path)).unwrap())
}

/// The value of `key=` on a rendered line, read through the contract reader.
fn field_of(output: &str, line_no: usize, key: &str) -> Option<Vec<u8>> {
    let parsed = contract::parse(output).unwrap();
    parsed[line_no].field(key).map(<[u8]>::to_vec)
}

// ---------------------------------------------------------------------------------------
// A2-T2a-1 — the observation reference
// ---------------------------------------------------------------------------------------

fn path_bytes() -> impl Strategy<Value = Vec<u8>> {
    // Bias toward the bytes that matter here — the separator, digits, the escape character,
    // spaces, controls, non-UTF-8 — while still reaching every byte.
    let interesting = prop::sample::select(vec![
        b':', b'0', b'1', b'9', b'\\', b' ', b'\n', b'\t', 0x00, 0x7F, 0xFF, 0xC3, b'x', b'/',
    ]);
    let any = any::<u8>();
    prop::collection::vec(prop_oneof![3 => interesting, 1 => any], 0..24)
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(2048))]

    #[test]
    fn a_reference_reads_back_to_the_same_run_and_bytes(
        run in prop_oneof![Just(0u64), Just(1u64), Just(i64::MAX as u64), 0u64..=i64::MAX as u64],
        path in path_bytes(),
    ) {
        let written = write_reference(run, &path);
        let value = contract::read_value(&written.text).unwrap();
        prop_assert_eq!(read_reference(&value), Ok((run, path)));
    }
}

#[test]
fn the_reference_generator_reaches_the_cases_that_matter() {
    // A round-trip property is only as strong as its generator (the slice 1 precedent).
    use proptest::strategy::ValueTree;
    use proptest::test_runner::TestRunner;
    let mut runner = TestRunner::deterministic();
    let (mut colon, mut leading_digit, mut escaped) = (false, false, false);
    for _ in 0..2048 {
        let p = path_bytes().new_tree(&mut runner).unwrap().current();
        colon |= p.contains(&b':');
        leading_digit |= p.first().is_some_and(u8::is_ascii_digit);
        escaped |= !write_reference(1, &p).classes.is_empty();
    }
    assert!(colon && leading_digit && escaped);
}

#[test]
fn a_reference_splits_at_the_first_separator() {
    assert_eq!(
        read_reference(b"12:a:b"),
        Ok((12, b"a:b".to_vec())),
        "a path may contain `:`; a run may not"
    );
    assert_eq!(read_reference(b"3:"), Ok((3, Vec::new())));
}

#[test]
fn a_malformed_reference_is_rejected_with_its_reason() {
    assert_eq!(read_reference(b"12"), Err(ReferenceError::MissingSeparator));
    assert_eq!(read_reference(b":a"), Err(ReferenceError::EmptyRun));
    assert_eq!(read_reference(b"x1:a"), Err(ReferenceError::NotDecimal));
    assert_eq!(read_reference(b"-1:a"), Err(ReferenceError::NotDecimal));
    assert_eq!(read_reference(b"+1:a"), Err(ReferenceError::NotDecimal));
    assert_eq!(read_reference(b"01:a"), Err(ReferenceError::LeadingZero));
    assert_eq!(
        read_reference(b"9223372036854775808:a"),
        Err(ReferenceError::OutOfRange)
    );
}

// ---------------------------------------------------------------------------------------
// A2-T2a-2 — every per-observation line of `show` names its observation
// ---------------------------------------------------------------------------------------

#[test]
fn every_line_of_show_names_the_observation_it_reports() {
    let mut log = log_with_every_case();
    log.append_run(NewRun {
        started_at_ns: 3_000,
        finished_at_ns: 4_000,
        root: PathBuf::from("/tmp/root"),
        observations: vec![NewObservation {
            entry: entry("fresh.txt", EntryKind::File, 1),
            content: Some(stable(2)),
            error: None,
            reused_from: None,
        }],
    })
    .unwrap();

    for path in [
        "fresh.txt",
        "moving.txt",
        "locked.txt",
        "dir",
        "link",
        "gone",
    ] {
        let out = show(&log, path);
        let stored = log.observations_for_path(Path::new(path)).unwrap();
        let parsed = contract::parse(&out).unwrap();
        for (i, line) in parsed.iter().enumerate() {
            // A verdict between two runs names its observations by side (`reference=`,
            // `compared=`; slice 2b, `tests/verdict_basis.rs`).
            if line.field("path").is_some() {
                continue;
            }
            let reference = line.field("observation").unwrap_or_else(|| {
                panic!("line {i} of `show {path}` names no observation:\n{out}")
            });
            let (run, bytes) = read_reference(reference).unwrap();
            assert_eq!(
                bytes,
                path.as_bytes(),
                "line {i} names another entry:\n{out}"
            );
            let matching: Vec<&Observation> =
                stored.iter().filter(|o| o.run_id == run as i64).collect();
            assert_eq!(
                matching.len(),
                1,
                "line {i} must resolve to exactly one observation:\n{out}"
            );
        }
    }
}

#[test]
fn each_line_of_show_reports_the_observation_it_names() {
    // Two runs of one entry with different sizes: each `observed` line's size must be the size
    // of the observation its own reference names — whatever order the lines come in.
    let mut log = SqliteLog::open_in_memory().unwrap();
    for (size, byte) in [(4u64, 1u8), (9, 2)] {
        log.append_run(NewRun {
            started_at_ns: 1_000,
            finished_at_ns: 2_000,
            root: PathBuf::from("/tmp/root"),
            observations: vec![NewObservation {
                entry: Entry {
                    size: Some(size),
                    ..entry("a.txt", EntryKind::File, 1)
                },
                content: Some(stable(byte)),
                error: None,
                reused_from: None,
            }],
        })
        .unwrap();
    }
    let out = show(&log, "a.txt");
    let mut parsed = contract::parse(&out).unwrap();
    parsed.reverse(); // attribution must not depend on order
    let stored = log.observations_for_path(Path::new("a.txt")).unwrap();
    let mut checked = 0;
    for line in &parsed {
        if line.label != umbral::report::Label::Observed {
            continue;
        }
        let (run, _) = read_reference(line.field("observation").unwrap()).unwrap();
        let o = stored.iter().find(|o| o.run_id == run as i64).unwrap();
        assert_eq!(
            line.field("size").unwrap(),
            o.size.unwrap().to_string().as_bytes()
        );
        checked += 1;
    }
    assert_eq!(checked, 2, "got:\n{out}");
}

#[test]
fn the_identification_field_is_not_an_observed_claim() {
    let out = show(&log_with_every_case(), "fresh.txt");
    assert!(
        out.lines()
            .any(|l| l.starts_with("observed") && l.contains("observation=1:fresh.txt")),
        "got:\n{out}"
    );
    assert!(
        umbral::report::label_contract_violations_in_text(&out).is_empty(),
        "{:?}",
        umbral::report::label_contract_violations_in_text(&out)
    );
    // ... and the allowance is for that field only.
    let other = vec![umbral::report::Line::observed("observation=1:a  run=1")];
    assert!(!umbral::report::label_contract_violations(&other).is_empty());
}

// ---------------------------------------------------------------------------------------
// A2-T2a-3 / A2-T2a-4 — states follow the table; nothing unread is shown as read
// ---------------------------------------------------------------------------------------

fn stored(path: &str) -> Observation {
    log_with_every_case()
        .observations_for_path(Path::new(path))
        .unwrap()
        .remove(0)
}

#[test]
fn states_follow_the_table() {
    use AcquisitionState::*;
    let cases: &[(
        &str,
        AcquisitionState,
        Option<AcquisitionState>,
        Option<&str>,
    )] = &[
        ("fresh.txt", Fresh, Some(Fresh), None),
        (
            "moving.txt",
            Fresh,
            Some(Failed),
            Some("unstable-observation"),
        ),
        ("locked.txt", Fresh, Some(Failed), Some("permission-denied")),
        ("vanished.txt", Fresh, Some(Failed), Some("not-found")),
        (
            "swapped.txt",
            Fresh,
            Some(Failed),
            Some("not-a-regular-file"),
        ),
        ("broken.txt", Fresh, Some(Failed), Some("read-error")),
        ("dir", Fresh, None, None),
        ("link", Fresh, None, None),
        ("fifo", Fresh, None, None),
        ("gone", Failed, Some(NotAttempted), None),
    ];
    for (path, metadata, content, diagnostic) in cases {
        let o = stored(path);
        assert_eq!(metadata_state(&o), *metadata, "{path}: metadata");
        assert_eq!(content_state(&o), *content, "{path}: content");
        assert_eq!(content_diagnostic(&o), *diagnostic, "{path}: diagnostic");
    }
}

#[test]
fn a_row_without_its_recorded_reason_is_not_recorded() {
    let mut o = stored("locked.txt");
    o.content_error = Some("not-recorded".into());
    assert_eq!(content_state(&o), Some(AcquisitionState::NotRecorded));
    assert_eq!(content_diagnostic(&o), None);
}

#[test]
fn nothing_unread_is_shown_as_read() {
    // A hash without a stable reading is not a reading.
    let mut o = stored("fresh.txt");
    o.stability = Some(Stability::Unstable);
    assert_eq!(content_state(&o), Some(AcquisitionState::Failed));

    let log = log_with_every_case();
    for path in ["fresh.txt", "moving.txt", "locked.txt", "gone"] {
        let out = show(&log, path);
        let fresh = out.lines().any(|l| l.contains("content=fresh"));
        assert_eq!(fresh, path == "fresh.txt", "`show {path}`:\n{out}");
        assert!(!out.contains("reused"), "no skip exists in 2a:\n{out}");
    }
}

#[test]
fn show_prints_both_components_and_content_only_where_it_exists() {
    let log = log_with_every_case();
    let file = show(&log, "fresh.txt");
    assert!(
        file.contains("derived   observation=1:fresh.txt  hash=010101010101  stability=stable  metadata=fresh  content=fresh"),
        "got:\n{file}"
    );
    let dir = show(&log, "dir");
    assert!(dir.contains("metadata=fresh"), "got:\n{dir}");
    assert!(
        !dir.contains("content="),
        "a directory has no content component:\n{dir}"
    );
    let gone = show(&log, "gone");
    assert!(
        gone.contains("metadata=failed  content=not-attempted"),
        "got:\n{gone}"
    );
    let moving = show(&log, "moving.txt");
    assert!(
        moving.contains(
            "unknown   observation=1:moving.txt  reason=unstable-observation  deltas=size-changed"
        ),
        "got:\n{moving}"
    );
}

// ---------------------------------------------------------------------------------------
// A2-T2a-5 — counts are consistent and scoped
// ---------------------------------------------------------------------------------------

fn count(output: &str, key: &str) -> u64 {
    let parsed = contract::parse(output).unwrap();
    let values: Vec<u64> = parsed
        .iter()
        .filter_map(|l| l.field(key))
        .map(|v| std::str::from_utf8(v).unwrap().parse().unwrap())
        .collect();
    assert_eq!(
        values.len(),
        1,
        "`{key}` must appear exactly once:\n{output}"
    );
    values[0]
}

const COUNT_KEYS: &[&str] = &[
    "entries",
    "files",
    "dirs",
    "symlinks",
    "other",
    "kind-unknown",
    "metadata-fresh",
    "metadata-failed",
    "content-fresh",
    "content-reused",
    "content-failed",
    "content-not-attempted",
    "content-not-recorded",
    "unstable-observation",
    "not-found",
    "permission-denied",
    "not-a-regular-file",
    "read-error",
];

fn assert_counts_consistent(out: &str) {
    let c = |k| count(out, k);
    assert_eq!(
        c("metadata-fresh") + c("metadata-failed"),
        c("entries"),
        "{out}"
    );
    assert_eq!(
        c("content-fresh")
            + c("content-reused")
            + c("content-failed")
            + c("content-not-attempted")
            + c("content-not-recorded"),
        c("files") + c("kind-unknown"),
        "{out}"
    );
    assert_eq!(
        c("unstable-observation")
            + c("not-found")
            + c("permission-denied")
            + c("not-a-regular-file")
            + c("read-error"),
        c("content-failed"),
        "{out}"
    );
    // Every count names the run it summarises.
    for line in contract::parse(out).unwrap() {
        if COUNT_KEYS.iter().any(|k| line.field(k).is_some()) {
            assert!(
                line.field("run").is_some(),
                "a count line without `run=`:\n{out}"
            );
        }
    }
}

#[test]
fn status_counts_every_case_by_state() {
    let log = log_with_every_case();
    let out = render(&umbral::report::status(&ws(), &log).unwrap());
    assert_counts_consistent(&out);
    let c = |k| count(&out, k);
    assert_eq!((c("entries"), c("files"), c("kind-unknown")), (10, 6, 1));
    assert_eq!((c("metadata-fresh"), c("metadata-failed")), (9, 1));
    assert_eq!(
        (
            c("content-fresh"),
            c("content-reused"),
            c("content-failed"),
            c("content-not-attempted"),
            c("content-not-recorded")
        ),
        (1, 0, 5, 1, 0)
    );
    for k in [
        "unstable-observation",
        "not-found",
        "permission-denied",
        "not-a-regular-file",
        "read-error",
    ] {
        assert_eq!(c(k), 1, "{k}");
    }
    for old in [
        "content-verified=",
        "content-not-verified=",
        "content-verification-not-applicable=",
    ] {
        assert!(!out.contains(old), "`{old}` is removed (`UD-033`):\n{out}");
    }
}

#[test]
fn observe_and_status_agree_for_the_same_run() {
    let s = Sandbox::new();
    s.write("a.txt", "alpha");
    s.write("sub/b.txt", "beta");
    #[cfg(unix)]
    std::os::unix::fs::symlink("a.txt", s.root().join("link")).unwrap();
    let observe = s.init_and_observe();
    let r = s.root().to_string_lossy().to_string();
    let status = s.run_ok(&["status", r.as_str()]);
    assert_all_labelled(&observe);
    assert_counts_consistent(&observe);
    assert_counts_consistent(&status);
    for k in COUNT_KEYS {
        assert_eq!(count(&observe, k), count(&status, k), "`{k}`");
    }
    assert_eq!(count(&observe, "content-fresh"), 2);
}

// ---------------------------------------------------------------------------------------
// A2-T2a-6 — closed vocabulary
// ---------------------------------------------------------------------------------------

#[test]
fn the_state_vocabulary_is_the_closed_set_of_ud_031() {
    let names: Vec<&str> = AcquisitionState::ALL.iter().map(|s| s.as_str()).collect();
    assert_eq!(
        names,
        ["fresh", "reused", "failed", "not-attempted", "not-recorded"]
    );
    let log = log_with_every_case();
    for path in ["fresh.txt", "moving.txt", "locked.txt", "dir", "gone"] {
        let out = show(&log, path);
        for line in contract::parse(&out).unwrap() {
            for key in ["metadata", "content"] {
                if let Some(v) = line.field(key) {
                    let v = std::str::from_utf8(v).unwrap();
                    assert!(names.contains(&v), "`{key}={v}` is outside UD-031's set");
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------------------
// A2-T2a-8 — real outputs over hostile names are read by the contract reader
// ---------------------------------------------------------------------------------------

#[cfg(unix)]
#[test]
fn real_outputs_with_hostile_names_read_back_and_resolve() {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;

    let s = Sandbox::new();
    let names: [&[u8]; 5] = [
        b"12:34.txt",
        b"a  b.txt",
        b"line\nbreak.txt",
        b"back\\slash.txt",
        b"bytes-\xFF.txt",
    ];
    for n in names {
        std::fs::write(s.root().join(OsStr::from_bytes(n)), b"x").unwrap();
    }
    let observe = s.init_and_observe();
    contract::parse(&observe).unwrap();
    let status = s.run_ok(&["status", s.root().to_str().unwrap()]);
    contract::parse(&status).unwrap();

    for n in names {
        let out = s.run_os_ok(&[
            OsStr::new("show"),
            s.root().as_os_str(),
            OsStr::from_bytes(n),
        ]);
        let parsed = contract::parse(&out).unwrap();
        assert!(!parsed.is_empty());
        for (i, line) in parsed.iter().enumerate() {
            let (run, bytes) = read_reference(line.field("observation").unwrap()).unwrap();
            assert_eq!(run, 1, "line {i}");
            assert_eq!(bytes, n, "line {i} names another entry:\n{out}");
        }
        // The `observed` line still states exactly what was observed.
        assert!(
            field_of(&out, 1, "kind").as_deref() == Some(b"file".as_slice()),
            "got:\n{out}"
        );
    }
}
