//! v0.2 slice 3 — the O(changes) skip (`docs/candidates/V0.2-SCOPE-PROPOSAL.md` §9.7, criteria
//! `A2-T3-*`, `UD-035`, `UD-036`).
//!
//! The decision is tested as a pure function over hand-built observations. Tests that touch a
//! real filesystem compute what they expect from what the filesystem reported — they never
//! assert that a filesystem behaves a particular way (F-6).

use std::path::{Path, PathBuf};

use umbral::acquisition::{content_state, AcquisitionState};
use umbral::content::Stability;
use umbral::log::sqlite::SqliteLog;
use umbral::log::{NewObservation, NewRun, Observation, ObservationLog};
use umbral::observe::{observe, Policy};
use umbral::reconcile::{reconcile, ObservationSet, ObservedPath};
use umbral::scan::{Entry, EntryKind};
use umbral::skip::{decide, Decision};
use umbral::workspace::Workspace;

// ---------------------------------------------------------------------------------------
// A2-T3-1 — the decision is the condition, exhaustively
// ---------------------------------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
enum Term {
    Equal,
    Different,
    AbsentBefore,
    AbsentNow,
}
const TERMS: [Term; 4] = [
    Term::Equal,
    Term::Different,
    Term::AbsentBefore,
    Term::AbsentNow,
];

fn pair<T: Copy>(t: Term, a: T, b: T) -> (Option<T>, Option<T>) {
    match t {
        Term::Equal => (Some(a), Some(a)),
        Term::Different => (Some(a), Some(b)),
        Term::AbsentBefore => (None, Some(a)),
        Term::AbsentNow => (Some(a), None),
    }
}

#[derive(Clone, Copy, Debug)]
enum Reading {
    Valid,
    Unstable,
    None,
}

fn previous(kind: EntryKind, reading: Reading, t: [Term; 5]) -> (Observation, Entry) {
    let (dev0, dev1) = pair(t[0], 1u64, 2);
    let (ino0, ino1) = pair(t[1], 10u64, 11);
    let (size0, size1) = pair(t[2], 4u64, 5);
    let (mtime0, mtime1) = pair(t[3], (1_000i64, 0u32), (1_001, 0));
    let (ctime0, ctime1) = pair(t[4], (2_000i64, 0u32), (2_001, 0));
    let prev = Observation {
        run_id: 1,
        path: PathBuf::from("a"),
        kind,
        dev: dev0,
        ino: ino0,
        size: size0,
        mtime: mtime0,
        ctime: ctime0,
        ctime_recorded: true,
        hash: match reading {
            Reading::None => None,
            _ => Some([7; 32]),
        },
        hashed_len: Some(4),
        stability: match reading {
            Reading::Valid => Some(Stability::Stable),
            Reading::Unstable => Some(Stability::Unstable),
            Reading::None => None,
        },
        hash_read_run: match reading {
            Reading::None => None,
            _ => Some(1),
        },
        deltas: Vec::new(),
        error: None,
        content_error: None,
    };
    let cur = Entry {
        path: PathBuf::from("a"),
        kind: EntryKind::File,
        dev: dev1,
        ino: ino1,
        size: size1,
        mtime: mtime1,
        ctime: ctime1,
    };
    (prev, cur)
}

#[test]
fn the_decision_is_exactly_the_condition() {
    let mut cases = 0;
    let mut skips = 0;
    for prev_kind in [EntryKind::File, EntryKind::Dir] {
        for cur_kind in [EntryKind::File, EntryKind::Symlink] {
            for reading in [Reading::Valid, Reading::Unstable, Reading::None] {
                for t0 in TERMS {
                    for t1 in TERMS {
                        for t2 in TERMS {
                            for t3 in TERMS {
                                for t4 in TERMS {
                                    let terms = [t0, t1, t2, t3, t4];
                                    let (prev, mut cur) = previous(prev_kind, reading, terms);
                                    cur.kind = cur_kind;
                                    let expected = prev_kind == EntryKind::File
                                        && cur_kind == EntryKind::File
                                        && matches!(reading, Reading::Valid)
                                        && terms.iter().all(|t| matches!(t, Term::Equal));
                                    let got = decide(Some(&prev), &cur);
                                    assert_eq!(
                                        got == Decision::Skip,
                                        expected,
                                        "{prev_kind:?} {cur_kind:?} {reading:?} {terms:?}"
                                    );
                                    cases += 1;
                                    skips += usize::from(expected);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    assert_eq!(cases, 2 * 2 * 3 * 4usize.pow(5));
    assert_eq!(skips, 1, "exactly one combination may skip");
}

#[test]
fn nothing_to_compare_means_read() {
    let (_, cur) = previous(EntryKind::File, Reading::Valid, [Term::Equal; 5]);
    assert_eq!(decide(None, &cur), Decision::Read);
}

#[test]
fn a_changed_ctime_alone_forces_a_read() {
    // P12: the ctime term can only remove skips. Everything else equal, a different ctime reads.
    let mut terms = [Term::Equal; 5];
    let (prev, cur) = previous(EntryKind::File, Reading::Valid, terms);
    assert_eq!(decide(Some(&prev), &cur), Decision::Skip);
    terms[4] = Term::Different;
    let (prev, cur) = previous(EntryKind::File, Reading::Valid, terms);
    assert_eq!(decide(Some(&prev), &cur), Decision::Read);
}

// ---------------------------------------------------------------------------------------
// Real directories
// ---------------------------------------------------------------------------------------

struct Tree {
    dir: tempfile::TempDir,
}

impl Tree {
    fn new() -> Self {
        Tree {
            dir: tempfile::TempDir::new().unwrap(),
        }
    }
    fn root(&self) -> &Path {
        self.dir.path()
    }
    fn write(&self, rel: &str, body: &[u8]) {
        let p = self.root().join(rel);
        if let Some(parent) = p.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(p, body).unwrap();
    }
    fn stat(&self, rel: &str) -> std::fs::Metadata {
        std::fs::symlink_metadata(self.root().join(rel)).unwrap()
    }
}

fn observe_into(log: &mut SqliteLog, tree: &Tree, policy: Policy) -> umbral::observe::Observed {
    observe(tree.root(), tree.root(), log, policy).unwrap()
}

#[cfg(unix)]
fn ctime_of(m: &std::fs::Metadata) -> (i64, i64) {
    use std::os::unix::fs::MetadataExt;
    (m.ctime(), m.ctime_nsec())
}

// ---------------------------------------------------------------------------------------
// A2-T3-2 — O(changes), shown by counters
// ---------------------------------------------------------------------------------------

#[cfg(unix)]
#[test]
fn counters_show_that_only_changes_are_read() {
    use std::os::unix::fs::PermissionsExt;

    let t = Tree::new();
    t.write("a.txt", b"alpha");
    t.write("b.txt", b"bravo");
    t.write("sub/c.txt", b"charlie");
    let mut log = SqliteLog::open_in_memory().unwrap();

    let first = observe_into(&mut log, &t, Policy::Skip);
    assert_eq!(first.counters.read_entries, 3);
    assert_eq!(first.counters.read_bytes, 5 + 5 + 7);

    // Unchanged tree: nothing is read.
    let second = observe_into(&mut log, &t, Policy::Skip);
    assert_eq!(
        (second.counters.read_entries, second.counters.read_bytes),
        (0, 0)
    );

    // One file modified: exactly that file, in full.
    t.write("a.txt", b"alpha, longer");
    let third = observe_into(&mut log, &t, Policy::Skip);
    assert_eq!(
        (third.counters.read_entries, third.counters.read_bytes),
        (1, 13)
    );

    // Atomic replacement: the new file is read in full.
    t.write("b.tmp", b"BRAVO!");
    std::fs::rename(t.root().join("b.tmp"), t.root().join("b.txt")).unwrap();
    let fourth = observe_into(&mut log, &t, Policy::Skip);
    assert_eq!(
        (fourth.counters.read_entries, fourth.counters.read_bytes),
        (1, 6)
    );

    // A rewrite that restores size and mtime: read exactly when the filesystem moved ctime.
    let before = t.stat("sub/c.txt");
    std::thread::sleep(std::time::Duration::from_millis(1100));
    t.write("sub/c.txt", b"CHARLIE");
    let mtime = filetime_of(&before);
    set_mtime(&t.root().join("sub/c.txt"), mtime);
    let after = t.stat("sub/c.txt");
    let moved = ctime_of(&before) != ctime_of(&after);
    let fifth = observe_into(&mut log, &t, Policy::Skip);
    assert_eq!(fifth.counters.read_entries, u64::from(moved));

    // chmod: no byte changed, but ctime moves — the cost `UD-018` accepted.
    let before = t.stat("a.txt");
    std::thread::sleep(std::time::Duration::from_millis(1100));
    std::fs::set_permissions(
        t.root().join("a.txt"),
        std::fs::Permissions::from_mode(0o600),
    )
    .unwrap();
    let moved = ctime_of(&before) != ctime_of(&t.stat("a.txt"));
    let sixth = observe_into(&mut log, &t, Policy::Skip);
    assert_eq!(sixth.counters.read_entries, u64::from(moved));

    // A rename: the new path has no previous observation, so it is read.
    std::fs::rename(t.root().join("sub/c.txt"), t.root().join("sub/d.txt")).unwrap();
    let seventh = observe_into(&mut log, &t, Policy::Skip);
    assert_eq!(
        (seventh.counters.read_entries, seventh.counters.read_bytes),
        (1, 7)
    );
}

#[cfg(unix)]
fn filetime_of(m: &std::fs::Metadata) -> (i64, i64) {
    use std::os::unix::fs::MetadataExt;
    (m.mtime(), m.mtime_nsec())
}

#[cfg(unix)]
fn set_mtime(path: &Path, (s, ns): (i64, i64)) {
    let t = std::time::UNIX_EPOCH
        + std::time::Duration::from_secs(s as u64)
        + std::time::Duration::from_nanos(ns as u64);
    let f = std::fs::File::options().write(true).open(path).unwrap();
    f.set_modified(t).unwrap();
}

// ---------------------------------------------------------------------------------------
// A2-T3-3 — a skip is never content verification
// ---------------------------------------------------------------------------------------

fn ws(root: &Path) -> Workspace {
    Workspace {
        root: root.to_path_buf(),
        canonical: root.to_path_buf(),
        id: "test".into(),
        state_dir: root.join(".state"),
    }
}

#[test]
fn a_reused_reading_is_shown_and_counted_as_reused_with_its_source() {
    let t = Tree::new();
    t.write("a.txt", b"alpha");
    let mut log = SqliteLog::open_in_memory().unwrap();
    observe_into(&mut log, &t, Policy::Skip);
    observe_into(&mut log, &t, Policy::Skip);
    observe_into(&mut log, &t, Policy::Skip);

    let stored = log.observations_for_path(Path::new("a.txt")).unwrap();
    assert_eq!(content_state(&stored[0]), Some(AcquisitionState::Fresh));
    for o in &stored[1..] {
        assert_eq!(content_state(o), Some(AcquisitionState::Reused));
        // The source is the observation that actually read the bytes — run 1 — not run 2.
        assert_eq!(o.hash_read_run, Some(1));
        assert_eq!(o.hash, stored[0].hash);
    }

    let show = umbral::report::render(
        &umbral::report::show(&ws(t.root()), &log, Path::new("a.txt")).unwrap(),
    );
    assert!(
        show.contains("observation=3:a.txt  hash=")
            && show.contains("metadata=fresh  content=reused  content-source=1:a.txt"),
        "got:\n{show}"
    );
    let status = umbral::report::render(&umbral::report::status(&ws(t.root()), &log).unwrap());
    assert!(
        status.contains("content-fresh=0  content-reused=1"),
        "got:\n{status}"
    );
}

// ---------------------------------------------------------------------------------------
// A2-T3-4 / A2-T3-5 — verdicts equal reading everything, wherever the evidence is the same
// ---------------------------------------------------------------------------------------

fn changes(log: &SqliteLog, root: &Path) -> String {
    umbral::report::render(&umbral::report::changes(&ws(root), log).unwrap())
}

#[cfg(unix)]
#[test]
fn skipping_gives_the_verdicts_of_reading_everything_over_the_mutation_matrix() {
    use std::os::unix::fs::PermissionsExt;

    let t = Tree::new();
    t.write("keep.txt", b"keep");
    t.write("edit.txt", b"edit");
    t.write("gone.txt", b"gone");
    t.write("move.txt", b"move");
    t.write("swap.txt", b"swap");
    t.write("link-a.txt", b"link");
    std::fs::hard_link(t.root().join("link-a.txt"), t.root().join("link-b.txt")).unwrap();
    let mut skip = SqliteLog::open_in_memory().unwrap();
    let mut all = SqliteLog::open_in_memory().unwrap();
    let both = |skip: &mut SqliteLog, all: &mut SqliteLog| {
        observe(t.root(), t.root(), skip, Policy::Skip).unwrap();
        observe(t.root(), t.root(), all, Policy::ReadAll).unwrap();
        assert_eq!(changes(skip, t.root()), changes(all, t.root()));
    };
    both(&mut skip, &mut all);
    both(&mut skip, &mut all); // unchanged
    t.write("edit.txt", b"edited");
    std::fs::remove_file(t.root().join("gone.txt")).unwrap();
    std::fs::rename(t.root().join("move.txt"), t.root().join("moved.txt")).unwrap();
    t.write("new.txt", b"new");
    both(&mut skip, &mut all);
    t.write("swap.tmp", b"SWAP");
    std::fs::rename(t.root().join("swap.tmp"), t.root().join("swap.txt")).unwrap();
    std::fs::set_permissions(
        t.root().join("keep.txt"),
        std::fs::Permissions::from_mode(0o600),
    )
    .unwrap();
    std::fs::remove_file(t.root().join("link-a.txt")).unwrap();
    both(&mut skip, &mut all);
    both(&mut skip, &mut all); // unchanged again, after reuse
}

/// The one admissible difference (`UD-021`, F-TD-4), shown on hand-built observations: a
/// change that keeps identity, size, mtime **and** ctime is skipped, and the carried reading
/// makes the verdict `unchanged`. This is `ctime`'s residual false negative, documented in the
/// version record — never a guarantee.
#[test]
fn the_residual_false_negative_is_real_on_hand_built_input() {
    let (prev, cur) = previous(EntryKind::File, Reading::Valid, [Term::Equal; 5]);
    assert_eq!(decide(Some(&prev), &cur), Decision::Skip);
    let as_path = |hash: u8| ObservedPath {
        path: PathBuf::from("a"),
        kind: EntryKind::File,
        dev: Some(1),
        ino: Some(10),
        size: Some(4),
        mtime: Some((1_000, 0)),
        valid_hash: Some([hash; 32]),
    };
    // Carried reading (skip): the same hash on both sides.
    let carried = reconcile(
        &ObservationSet::new(vec![as_path(7)], true),
        &ObservationSet::new(vec![as_path(7)], true),
    );
    // What a read would have found had the bytes changed.
    let read = reconcile(
        &ObservationSet::new(vec![as_path(7)], true),
        &ObservationSet::new(vec![as_path(8)], true),
    );
    assert_eq!(carried.mutations[0].kind, umbral::MutationKind::Unchanged);
    assert_eq!(read.mutations[0].kind, umbral::MutationKind::Modified);
}

/// A2-T3-5: a same-path pair whose identity differs is never skipped, so the inputs to the
/// ambiguity rules (phases 2–3) are the readings of this run, exactly as without the skip.
#[test]
fn identity_changes_are_never_skipped() {
    for t in [Term::Different, Term::AbsentBefore, Term::AbsentNow] {
        for which in [0, 1] {
            let mut terms = [Term::Equal; 5];
            terms[which] = t;
            let (prev, cur) = previous(EntryKind::File, Reading::Valid, terms);
            assert_eq!(decide(Some(&prev), &cur), Decision::Read);
        }
    }
}

// ---------------------------------------------------------------------------------------
// A2-T3-6 — the three states are distinguishable from the log
// ---------------------------------------------------------------------------------------

#[test]
fn every_row_states_how_its_reading_was_obtained() {
    let t = Tree::new();
    t.write("a.txt", b"alpha");
    t.write("b.txt", b"bravo");
    let mut log = SqliteLog::open_in_memory().unwrap();
    observe_into(&mut log, &t, Policy::Skip);
    t.write("b.txt", b"BRAVO, longer");
    observe_into(&mut log, &t, Policy::Skip);
    let run2 = log.observations_for_run(2).unwrap();
    let state = |p: &str| content_state(run2.iter().find(|o| o.path == Path::new(p)).unwrap());
    assert_eq!(state("a.txt"), Some(AcquisitionState::Reused));
    assert_eq!(state("b.txt"), Some(AcquisitionState::Fresh));
}

// ---------------------------------------------------------------------------------------
// A2-T3-9 — interruption
// ---------------------------------------------------------------------------------------

#[test]
fn a_run_that_fails_to_persist_leaves_nothing_and_the_next_run_reads() {
    let t = Tree::new();
    t.write("a.txt", b"alpha");
    let mut log = SqliteLog::open_in_memory().unwrap();
    observe_into(&mut log, &t, Policy::Skip);

    // A run whose persistence fails part-way (the second row violates the primary key).
    let entry = umbral::scan::scan(t.root()).unwrap().entries[0].clone();
    let row = || NewObservation {
        entry: entry.clone(),
        content: None,
        error: None,
        reused_from: Some(1),
    };
    let failed = log.append_run(NewRun {
        started_at_ns: 1,
        finished_at_ns: 2,
        root: t.root().to_path_buf(),
        observations: vec![row(), row()],
    });
    assert!(failed.is_err());
    assert_eq!(
        log.runs().unwrap().len(),
        1,
        "no row of the failed run remains"
    );

    // The next run decides against the last run that was persisted.
    let next = observe_into(&mut log, &t, Policy::Skip);
    assert_eq!(next.run_id, 2);
    let o = &log.observations_for_run(2).unwrap()[0];
    assert_eq!(o.hash_read_run, Some(1));
}
