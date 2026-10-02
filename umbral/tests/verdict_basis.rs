//! v0.2 slice 2b — the `basis` of a `changes` verdict (`docs/candidates/V0.2-SCOPE-PROPOSAL.md`
//! §9.6, criteria `A2-T2b-*`, `UD-034`).
//!
//! The table of §9.6 is checked row by row against hand-built observation sets (F-6): what the
//! rules consulted is a property of the rules, not of a filesystem.

mod common;

use std::path::{Path, PathBuf};

use common::Sandbox;
use umbral::content::{ContentObservation, Stability};
use umbral::contract::{self, read_reference, Item};
use umbral::log::sqlite::SqliteLog;
use umbral::log::{NewObservation, NewRun, ObservationLog};
use umbral::reconcile::{
    reconcile, Mutation, MutationKind, ObservationSet, ObservedPath, Side, SideBasis,
};
use umbral::scan::{Entry, EntryKind};
use umbral::workspace::Workspace;

// ---------------------------------------------------------------------------------------
// Hand-built observation sets
// ---------------------------------------------------------------------------------------

fn mk(path: &str, dev: u64, ino: u64, hash: Option<u8>) -> ObservedPath {
    ObservedPath {
        path: PathBuf::from(path),
        kind: EntryKind::File,
        dev: Some(dev),
        ino: Some(ino),
        size: Some(10),
        mtime: Some((1_000, 0)),
        valid_hash: hash.map(|b| [b; 32]),
    }
}

fn no_id(path: &str) -> ObservedPath {
    ObservedPath {
        dev: None,
        ino: None,
        ..mk(path, 0, 0, Some(1))
    }
}

fn set(paths: Vec<ObservedPath>, complete: bool) -> ObservationSet {
    ObservationSet::new(paths, complete)
}

fn only<'a>(
    r: &'a umbral::reconcile::Reconciliation,
    path: &str,
    kind: MutationKind,
) -> &'a Mutation {
    let found: Vec<&Mutation> = r
        .mutations
        .iter()
        .filter(|m| m.path == Path::new(path) && m.kind == kind)
        .collect();
    assert_eq!(found.len(), 1, "{path} {kind:?} in {:#?}", r.mutations);
    found[0]
}

fn related(path: &str, fields: &[&'static str]) -> SideBasis {
    SideBasis::Related {
        path: PathBuf::from(path),
        fields: fields.to_vec(),
    }
}

const ID: &[&str] = &["dev", "ino"];
const FILE_META: &[&str] = &["dev", "ino", "kind", "size", "mtime"];
const FILE_META_HASH: &[&str] = &["dev", "ino", "kind", "size", "mtime", "hash"];
const ID_HASH: &[&str] = &["dev", "ino", "hash"];

// ---------------------------------------------------------------------------------------
// A2-T2b-2 — the fields follow the table, branch by branch
// ---------------------------------------------------------------------------------------

#[test]
fn unchanged_by_metadata_consults_identity_kind_and_file_metadata() {
    let r = reconcile(
        &set(vec![mk("a", 1, 10, Some(1))], true),
        &set(vec![mk("a", 1, 10, Some(1))], true),
    );
    let m = only(&r, "a", MutationKind::Unchanged);
    assert_eq!(m.evidence.content_changed, None);
    assert_eq!(m.evidence.basis.reference, related("a", FILE_META));
    assert_eq!(m.evidence.basis.compared, related("a", FILE_META));
    assert!(m.evidence.basis.counterparts.is_empty());
}

#[test]
fn unchanged_directory_consults_identity_and_kind_only() {
    let dir = |p| ObservedPath {
        kind: EntryKind::Dir,
        valid_hash: None,
        ..mk(p, 1, 10, None)
    };
    let r = reconcile(&set(vec![dir("d")], true), &set(vec![dir("d")], true));
    let m = only(&r, "d", MutationKind::Unchanged);
    assert_eq!(
        m.evidence.basis.reference,
        related("d", &["dev", "ino", "kind"])
    );
}

#[test]
fn unchanged_by_bytes_and_modified_consult_the_hash_too() {
    let moved = ObservedPath {
        mtime: Some((2_000, 0)),
        ..mk("a", 1, 10, Some(1))
    };
    let r = reconcile(
        &set(vec![mk("a", 1, 10, Some(1))], true),
        &set(vec![moved], true),
    );
    let m = only(&r, "a", MutationKind::Unchanged);
    assert_eq!(m.evidence.content_changed, Some(false));
    assert_eq!(m.evidence.basis.compared, related("a", FILE_META_HASH));

    let edited = ObservedPath {
        size: Some(11),
        ..mk("a", 1, 10, Some(2))
    };
    let r = reconcile(
        &set(vec![mk("a", 1, 10, Some(1))], true),
        &set(vec![edited], true),
    );
    let m = only(&r, "a", MutationKind::Modified);
    assert_eq!(m.evidence.content_changed, Some(true));
    assert_eq!(m.evidence.basis.reference, related("a", FILE_META_HASH));
}

#[test]
fn modified_without_comparable_hashes_still_lists_the_hash_it_consulted() {
    let edited = ObservedPath {
        size: Some(11),
        ..mk("a", 1, 10, None)
    };
    let r = reconcile(
        &set(vec![mk("a", 1, 10, Some(1))], true),
        &set(vec![edited], true),
    );
    let m = only(&r, "a", MutationKind::Modified);
    assert_eq!(m.evidence.content_changed, None);
    assert_eq!(m.evidence.basis.compared, related("a", FILE_META_HASH));
}

#[test]
fn a_kind_change_does_not_list_file_metadata() {
    let became_dir = ObservedPath {
        kind: EntryKind::Dir,
        valid_hash: None,
        ..mk("a", 1, 10, None)
    };
    let r = reconcile(
        &set(vec![mk("a", 1, 10, Some(1))], true),
        &set(vec![became_dir], true),
    );
    let m = only(&r, "a", MutationKind::Modified);
    assert_eq!(
        m.evidence.basis.reference,
        related("a", &["dev", "ino", "kind", "hash"])
    );
}

#[test]
fn a_rename_relates_the_old_path_and_the_new_one_by_identity() {
    let r = reconcile(
        &set(vec![mk("bye.txt", 1, 10, Some(1))], true),
        &set(vec![mk("hi.txt", 1, 10, Some(1))], true),
    );
    let m = only(&r, "hi.txt", MutationKind::RenamedOrMoved);
    assert_eq!(m.evidence.basis.reference, related("bye.txt", ID));
    assert_eq!(m.evidence.basis.compared, related("hi.txt", ID));
    assert!(m.evidence.basis.counterparts.is_empty());
}

#[test]
fn conflicting_candidates_name_every_other_member_of_the_group() {
    // Two hard links in the reference, one entry with the same identity in the comparison.
    let r = reconcile(
        &set(vec![mk("a", 1, 10, Some(1)), mk("b", 1, 10, Some(1))], true),
        &set(vec![mk("c", 1, 10, Some(1))], true),
    );
    let a = only(&r, "a", MutationKind::Ambiguous);
    assert_eq!(a.evidence.basis.reference, related("a", ID));
    assert_eq!(a.evidence.basis.compared, SideBasis::NotRelated);
    assert_eq!(
        a.evidence.basis.counterparts,
        vec![
            (Side::Reference, PathBuf::from("b")),
            (Side::Compared, PathBuf::from("c"))
        ]
    );
    assert_eq!(a.evidence.basis.counterpart_fields, ID.to_vec());

    let c = only(&r, "c", MutationKind::Ambiguous);
    assert_eq!(c.evidence.basis.reference, SideBasis::NotRelated);
    assert_eq!(c.evidence.basis.compared, related("c", ID));
    assert_eq!(
        c.evidence.basis.counterparts,
        vec![
            (Side::Reference, PathBuf::from("a")),
            (Side::Reference, PathBuf::from("b"))
        ]
    );
}

#[test]
fn same_path_verdicts_without_shared_identity_consult_identity_and_hash() {
    for (prev_hash, cur_hash, kind) in [
        (Some(1), Some(2), MutationKind::Recreated),
        (Some(1), Some(1), MutationKind::Ambiguous),
        (None, None, MutationKind::Ambiguous),
    ] {
        let r = reconcile(
            &set(vec![mk("a", 1, 10, prev_hash)], true),
            &set(vec![mk("a", 1, 11, cur_hash)], true),
        );
        let m = only(&r, "a", kind);
        assert_eq!(m.evidence.basis.reference, related("a", ID_HASH));
        assert_eq!(m.evidence.basis.compared, related("a", ID_HASH));
    }
    // Missing identity on one side: the identity was still consulted, and found absent.
    let r = reconcile(
        &set(vec![no_id("a")], true),
        &set(vec![mk("a", 1, 10, Some(1))], true),
    );
    let m = only(&r, "a", MutationKind::Ambiguous);
    assert_eq!(m.evidence.basis.reference, related("a", ID_HASH));
}

#[test]
fn created_rests_on_absence_from_the_reference() {
    let r = reconcile(
        &set(vec![], true),
        &set(vec![mk("a", 1, 10, Some(1))], true),
    );
    let m = only(&r, "a", MutationKind::Created);
    assert_eq!(m.evidence.basis.reference, SideBasis::Absent);
    assert_eq!(m.evidence.basis.compared, related("a", ID));

    let r = reconcile(&set(vec![], true), &set(vec![no_id("a")], true));
    let m = only(&r, "a", MutationKind::Created);
    assert_eq!(m.evidence.basis.compared, related("a", &[]));
}

#[test]
fn deleted_names_every_surviving_entry() {
    let r = reconcile(
        &set(vec![mk("a", 1, 10, Some(1))], true),
        &set(vec![], true),
    );
    let m = only(&r, "a", MutationKind::Deleted);
    assert_eq!(m.evidence.basis.reference, related("a", ID));
    assert_eq!(m.evidence.basis.compared, SideBasis::Absent);
    assert_eq!(m.evidence.object_survives, Some(false));
    assert!(m.evidence.basis.counterparts.is_empty());

    // Three hard links; one vanishes, two remain: both are named.
    let r = reconcile(
        &set(
            vec![
                mk("a", 1, 10, Some(1)),
                mk("b", 1, 10, Some(1)),
                mk("c", 1, 10, Some(1)),
            ],
            true,
        ),
        &set(vec![mk("b", 1, 10, Some(1)), mk("c", 1, 10, Some(1))], true),
    );
    let m = only(&r, "a", MutationKind::Deleted);
    assert_eq!(m.evidence.object_survives, Some(true));
    assert_eq!(
        m.evidence.basis.counterparts,
        vec![
            (Side::Compared, PathBuf::from("b")),
            (Side::Compared, PathBuf::from("c"))
        ]
    );
    assert_eq!(m.evidence.basis.counterpart_fields, ID.to_vec());

    let r = reconcile(&set(vec![no_id("a")], true), &set(vec![], true));
    let m = only(&r, "a", MutationKind::Deleted);
    assert_eq!(m.evidence.basis.reference, related("a", &[]));
}

#[test]
fn unobserved_rests_on_absence_from_an_incomplete_comparison() {
    let r = reconcile(
        &set(vec![mk("a", 1, 10, Some(1))], true),
        &set(vec![], false),
    );
    let m = only(&r, "a", MutationKind::Unobserved);
    assert_eq!(m.evidence.basis.reference, related("a", &[]));
    assert_eq!(m.evidence.basis.compared, SideBasis::Absent);
}

// ---------------------------------------------------------------------------------------
// A2-T2b-3 — absence is stated only when it is true (the Q26 case)
// ---------------------------------------------------------------------------------------

#[test]
fn a_path_renamed_over_is_not_called_absent() {
    // `a` is renamed over `b`. `b`'s earlier object gets a `deleted` verdict (Q26, open), but
    // the path `b` is present in the comparison: its observation is named, not called absent.
    let r = reconcile(
        &set(vec![mk("a", 1, 10, Some(1)), mk("b", 1, 20, Some(2))], true),
        &set(vec![mk("b", 1, 10, Some(1))], true),
    );
    let m = only(&r, "b", MutationKind::Deleted);
    assert_eq!(m.evidence.basis.compared, related("b", &[]));
    let rename = only(&r, "b", MutationKind::RenamedOrMoved);
    assert_eq!(rename.evidence.basis.reference, related("a", ID));
}

// ---------------------------------------------------------------------------------------
// The rendered verdict lines
// ---------------------------------------------------------------------------------------

fn ws() -> Workspace {
    Workspace {
        root: PathBuf::from("/tmp/root"),
        canonical: PathBuf::from("/tmp/root"),
        id: "test".into(),
        state_dir: PathBuf::from("/tmp/state"),
    }
}

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

fn stable(byte: u8) -> Option<ContentObservation> {
    Some(ContentObservation {
        hash: Some([byte; 32]),
        hashed_len: Some(4),
        stability: Some(Stability::Stable),
        deltas: Vec::new(),
        error: None,
    })
}

fn append(log: &mut SqliteLog, obs: Vec<(Entry, Option<ContentObservation>)>, errors: &[&str]) {
    let mut observations: Vec<NewObservation> = obs
        .into_iter()
        .map(|(entry, content)| NewObservation {
            entry,
            content,
            error: None,
        })
        .collect();
    // A directory that could not be read makes the run incomplete.
    for e in errors {
        observations.push(NewObservation {
            entry: Entry {
                kind: EntryKind::Dir,
                ..entry(e, 99, 0)
            },
            content: None,
            error: Some("Permission denied (os error 13)".into()),
        });
    }
    log.append_run(NewRun {
        started_at_ns: 1_000,
        finished_at_ns: 2_000,
        root: PathBuf::from("/tmp/root"),
        observations,
    })
    .unwrap();
}

/// Reference run (incomplete: `locked` could not be read): `a.txt`, hard links `b` and `c`,
/// `old`. Compared run (complete): `a.txt` edited, `c`, new `faq`, `old` renamed to `new`.
fn two_runs() -> SqliteLog {
    let mut log = SqliteLog::open_in_memory().unwrap();
    append(
        &mut log,
        vec![
            (entry("a.txt", 1, 4), stable(1)),
            (entry("b", 7, 4), stable(2)),
            (entry("c", 7, 4), stable(2)),
            (entry("old", 5, 4), stable(3)),
        ],
        &["locked"],
    );
    append(
        &mut log,
        vec![
            (entry("a.txt", 1, 9), stable(4)),
            (entry("c", 7, 4), stable(2)),
            (entry("faq", 6, 4), stable(5)),
            (entry("new", 5, 4), stable(3)),
            (
                Entry {
                    kind: EntryKind::Dir,
                    ..entry("locked", 99, 0)
                },
                None,
            ),
        ],
        &[],
    );
    log
}

fn changes(log: &SqliteLog) -> String {
    umbral::report::render(&umbral::report::changes(&ws(), log).unwrap())
}

const REFERENCE_KEYS: &[&str] = &["reference", "compared", "counterpart"];
const STORED_FIELDS: &[&str] = &["dev", "ino", "kind", "size", "mtime", "hash"];

fn run_of(parsed_header: &contract::ParsedLine, key: &str) -> u64 {
    std::str::from_utf8(parsed_header.field(key).unwrap())
        .unwrap()
        .parse()
        .unwrap()
}

#[test]
fn every_reference_on_a_verdict_resolves_and_every_listed_field_is_stored() {
    let log = two_runs();
    let out = changes(&log);
    let parsed = contract::parse(&out).unwrap();
    let header = &parsed[0];
    let (r_run, c_run) = (
        run_of(header, "reference-run"),
        run_of(header, "compared-run"),
    );
    assert_eq!((r_run, c_run), (1, 2), "got:\n{out}");

    let mut references = 0;
    for line in parsed.iter().filter(|l| l.field("path").is_some()) {
        for item in &line.items {
            let Item::Field { key, value } = item else {
                continue;
            };
            if REFERENCE_KEYS.contains(&key.as_str()) {
                let (run, path) = read_reference(value).unwrap();
                match key.as_str() {
                    "reference" => assert_eq!(run, r_run, "{out}"),
                    "compared" => assert_eq!(run, c_run, "{out}"),
                    _ => assert!(run == r_run || run == c_run, "{out}"),
                }
                let stored = log.observations_for_run(run as i64).unwrap();
                let path = PathBuf::from(String::from_utf8(path).unwrap());
                assert_eq!(
                    stored.iter().filter(|o| o.path == path).count(),
                    1,
                    "`{key}` must resolve to exactly one observation:\n{out}"
                );
                references += 1;
            }
            if key.ends_with("-fields") {
                let v = std::str::from_utf8(value).unwrap();
                if v != "none" {
                    for f in v.split(',') {
                        assert!(STORED_FIELDS.contains(&f), "`{f}` is not a stored field");
                    }
                }
            }
            if let Some(side) = key.strip_suffix("-absent") {
                let run = std::str::from_utf8(value).unwrap().parse::<i64>().unwrap();
                let subject =
                    PathBuf::from(std::str::from_utf8(line.field("path").unwrap()).unwrap());
                assert!(
                    log.observations_for_run(run)
                        .unwrap()
                        .iter()
                        .all(|o| o.path != subject),
                    "`{side}-absent` for a present path:\n{out}"
                );
            }
        }
    }
    assert!(
        references >= 7,
        "too few references checked ({references}):\n{out}"
    );
}

#[test]
fn verdict_lines_carry_the_basis_of_the_table() {
    let out = changes(&two_runs());
    let has = |needle: &str| {
        assert!(out.contains(needle), "missing `{needle}`:\n{out}");
    };
    has("derived   compared  reference-run=1  compared-run=2  reference-complete=false  compared-complete=true\n");
    has("derived   modified  path=a.txt  reference=1:a.txt  reference-fields=dev,ino,kind,size,mtime,hash  compared=2:a.txt  compared-fields=dev,ino,kind,size,mtime,hash  content-changed=true  reference-complete=false  compared-complete=true\n");
    has("derived   created  path=faq  reference-absent=1  compared=2:faq  compared-fields=dev,ino  reference-complete=false  compared-complete=true\n");
    has("derived   deleted  path=b  reference=1:b  reference-fields=dev,ino  compared-absent=2  counterpart=2:c  counterpart-fields=dev,ino  object-survives=true  reference-complete=false  compared-complete=true\n");
    has("derived   renamed-or-moved  path=new  reference=1:old  reference-fields=dev,ino  compared=2:new  compared-fields=dev,ino  reference-complete=false  compared-complete=true\n");
    for gone in ["old-path=", "from-run=", "to-run=", "scan-complete="] {
        assert!(
            !out.contains(gone),
            "`{gone}` is removed (`UD-034`):\n{out}"
        );
    }
}

/// A2-T2b-5. Both completeness fields on every verdict line, equal to each run's own.
#[test]
fn every_verdict_states_both_sides_completeness() {
    let log = two_runs();
    let runs = log.runs().unwrap();
    let (r, c) = (runs[0].complete(), runs[1].complete());
    assert_ne!(r, c, "the fixture must tell the two sides apart");
    let out = changes(&log);
    for line in contract::parse(&out).unwrap() {
        if line.field("path").is_none() && line.field("reference-run").is_none() {
            continue;
        }
        assert_eq!(
            line.field("reference-complete"),
            Some(r.to_string().as_bytes()),
            "{out}"
        );
        assert_eq!(
            line.field("compared-complete"),
            Some(c.to_string().as_bytes()),
            "{out}"
        );
    }
}

#[test]
fn show_writes_the_comparison_as_the_same_verdict_line() {
    let log = two_runs();
    let out =
        umbral::report::render(&umbral::report::show(&ws(), &log, Path::new("a.txt")).unwrap());
    assert!(!out.contains("->"), "got:\n{out}");
    assert!(
        out.contains("derived   modified  path=a.txt  reference=1:a.txt  reference-fields=dev,ino,kind,size,mtime,hash  compared=2:a.txt"),
        "got:\n{out}"
    );
}

// ---------------------------------------------------------------------------------------
// A2-T2b-7 — the contract reads it, repeated keys in order, hostile names
// ---------------------------------------------------------------------------------------

#[test]
fn a_repeated_key_is_kept_in_order() {
    let out = "derived   contract=umbral-output/1\nderived   deleted  path=a  counterpart=2:z  counterpart=2:b\n";
    let parsed = contract::parse(out).unwrap();
    let values: Vec<&[u8]> = parsed[0]
        .items
        .iter()
        .filter_map(|i| match i {
            Item::Field { key, value } if key == "counterpart" => Some(value.as_slice()),
            _ => None,
        })
        .collect();
    assert_eq!(values, vec![b"2:z".as_slice(), b"2:b".as_slice()]);
}

#[cfg(unix)]
#[test]
fn real_changes_with_hostile_names_read_back() {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;

    let s = Sandbox::new();
    let first: &[u8] = b"1:colon\nline.txt";
    let second: &[u8] = b"2:renamed  \xFF.txt";
    let link: &[u8] = b"link:to";
    std::fs::write(s.root().join(OsStr::from_bytes(first)), b"x").unwrap();
    std::fs::hard_link(
        s.root().join(OsStr::from_bytes(first)),
        s.root().join(OsStr::from_bytes(link)),
    )
    .unwrap();
    s.init_and_observe();
    // Rename one hard link: the other survives under its own name.
    std::fs::rename(
        s.root().join(OsStr::from_bytes(first)),
        s.root().join(OsStr::from_bytes(second)),
    )
    .unwrap();
    let r = s.root().to_string_lossy().to_string();
    s.run_ok(&["observe", r.as_str()]);
    let out = s.run_ok(&["changes", r.as_str()]);
    let parsed = contract::parse(&out).unwrap();
    let mut seen = Vec::new();
    for line in &parsed {
        for item in &line.items {
            if let Item::Field { key, value } = item {
                if REFERENCE_KEYS.contains(&key.as_str()) {
                    seen.push(read_reference(value).unwrap().1);
                }
            }
        }
    }
    assert!(seen.contains(&first.to_vec()), "got:\n{out}");
    assert!(seen.contains(&second.to_vec()), "got:\n{out}");
}
