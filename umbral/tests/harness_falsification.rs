//! Falsification harness: an independent model of the tree, compared against what the tool
//! observes and classifies.
//!
//! # What makes this an independent oracle
//!
//! [`Model`] maintains the truth directly: it separates **objects** (inodes) from **paths**,
//! so hard-link aliasing is represented correctly — an oracle that stores content per path
//! is wrong, not conservative. It never calls `reconcile`, never reads the tool's output and
//! never uses the tool's types to decide what is true. It is compared *against* them.
//!
//! # Coverage obligation carried over from V0
//!
//! The frozen V0 harness never generated directory deletion, so directory deletion was never
//! falsified (finding F-5). [`Op::RemoveDir`] is therefore part of the generator from the
//! start, and its share of generated operations is asserted to be non-zero.
//!
//! # Preconditions are explicit
//!
//! An operation whose precondition does not hold is SKIPPED, never applied to inflate
//! coverage.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use proptest::prelude::*;
use tempfile::TempDir;
use umbral::content::observe_content;
use umbral::reconcile::{reconcile, MutationKind, ObservationSet, ObservedPath};
use umbral::scan::{scan, EntryKind};

const DIRS: [&str; 1] = ["d0"];
const FILES: [&str; 5] = ["f0", "f1", "f2", "d0/g0", "d0/g1"];

// ---------------------------------------------------------------------------------------
// The independent model
// ---------------------------------------------------------------------------------------

#[derive(Default, Clone, Debug)]
struct Model {
    /// inode -> content. Objects exist independently of the paths that name them.
    objects: BTreeMap<u64, Vec<u8>>,
    /// path -> what that path names.
    paths: BTreeMap<String, PathKind>,
    next_ino: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum PathKind {
    File(u64),
    Dir,
}

impl Model {
    fn new() -> Self {
        Model {
            objects: BTreeMap::new(),
            paths: BTreeMap::new(),
            next_ino: 1,
        }
    }

    fn fresh_ino(&mut self) -> u64 {
        let i = self.next_ino;
        self.next_ino += 1;
        i
    }

    fn is_file(&self, p: &str) -> bool {
        matches!(self.paths.get(p), Some(PathKind::File(_)))
    }

    fn content_of(&self, p: &str) -> Option<&Vec<u8>> {
        match self.paths.get(p) {
            Some(PathKind::File(ino)) => self.objects.get(ino),
            _ => None,
        }
    }

    /// Every path the model says is a regular file.
    fn file_paths(&self) -> BTreeSet<String> {
        self.paths
            .iter()
            .filter(|(_, k)| matches!(k, PathKind::File(_)))
            .map(|(p, _)| p.clone())
            .collect()
    }

    /// Write bytes through `p`. Because content lives on the object, every path sharing the
    /// inode sees the change — which is the aliasing behaviour a per-path oracle gets wrong.
    fn write_through(&mut self, p: &str, bytes: Vec<u8>) {
        if let Some(PathKind::File(ino)) = self.paths.get(p).cloned() {
            self.objects.insert(ino, bytes);
        }
    }

    /// Creating a path below a directory requires that directory to exist, so the model must
    /// record the ancestors it implicitly brings into being. Without this the oracle claims a
    /// directory does not exist while reality — and the tool — say it does. Found by this
    /// harness: the first failure it produced was in the oracle, not in the implementation.
    fn ensure_parents(&mut self, p: &str) {
        if let Some((parent, _)) = p.rsplit_once('/') {
            if !parent.is_empty() {
                self.paths
                    .entry(parent.to_string())
                    .or_insert(PathKind::Dir);
            }
        }
    }

    fn create_file(&mut self, p: &str, bytes: Vec<u8>) {
        self.ensure_parents(p);
        let ino = self.fresh_ino();
        self.objects.insert(ino, bytes);
        self.paths.insert(p.to_string(), PathKind::File(ino));
    }

    fn remove(&mut self, p: &str) {
        if let Some(PathKind::File(ino)) = self.paths.remove(p) {
            // The object survives while any path still names it.
            let still_named = self
                .paths
                .values()
                .any(|k| matches!(k, PathKind::File(i) if *i == ino));
            if !still_named {
                self.objects.remove(&ino);
            }
        }
    }

    fn remove_dir(&mut self, dir: &str) {
        let prefix = format!("{dir}/");
        let victims: Vec<String> = self
            .paths
            .keys()
            .filter(|p| *p == dir || p.starts_with(&prefix))
            .cloned()
            .collect();
        for v in victims {
            self.remove(&v);
        }
    }
}

// ---------------------------------------------------------------------------------------
// Operations, each with an explicit precondition
// ---------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
enum Op {
    CreateFile { idx: usize, byte: u8 },
    WriteFile { idx: usize, byte: u8 },
    DeleteFile { idx: usize },
    RenameFile { from: usize, to: usize },
    HardLink { from: usize, to: usize },
    RemoveDir { idx: usize },
    AtomicReplace { idx: usize, byte: u8 },
}

fn op_strategy() -> impl Strategy<Value = Op> {
    prop_oneof![
        (0usize..FILES.len(), any::<u8>()).prop_map(|(idx, byte)| Op::CreateFile { idx, byte }),
        (0usize..FILES.len(), any::<u8>()).prop_map(|(idx, byte)| Op::WriteFile { idx, byte }),
        (0usize..FILES.len()).prop_map(|idx| Op::DeleteFile { idx }),
        (0usize..FILES.len(), 0usize..FILES.len())
            .prop_map(|(from, to)| Op::RenameFile { from, to }),
        (0usize..FILES.len(), 0usize..FILES.len()).prop_map(|(from, to)| Op::HardLink { from, to }),
        (0usize..DIRS.len()).prop_map(|idx| Op::RemoveDir { idx }),
        (0usize..FILES.len(), any::<u8>()).prop_map(|(idx, byte)| Op::AtomicReplace { idx, byte }),
    ]
}

fn body(byte: u8) -> Vec<u8> {
    vec![byte; 16 + (byte as usize % 32)]
}

/// Apply an operation to the real tree and the model together. Returns `false` when the
/// precondition did not hold and the operation was skipped.
fn apply(op: &Op, root: &Path, model: &mut Model) -> bool {
    match op {
        Op::CreateFile { idx, byte } => {
            let p = FILES[*idx];
            if model.paths.contains_key(p) {
                return false;
            }
            let full = root.join(p);
            if let Some(parent) = full.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            if std::fs::write(&full, body(*byte)).is_err() {
                return false;
            }
            model.create_file(p, body(*byte));
            true
        }

        Op::WriteFile { idx, byte } => {
            let p = FILES[*idx];
            if !model.is_file(p) {
                return false;
            }
            if std::fs::write(root.join(p), body(*byte)).is_err() {
                return false;
            }
            model.write_through(p, body(*byte));
            true
        }

        Op::DeleteFile { idx } => {
            let p = FILES[*idx];
            if !model.is_file(p) {
                return false;
            }
            if std::fs::remove_file(root.join(p)).is_err() {
                return false;
            }
            model.remove(p);
            true
        }

        Op::RenameFile { from, to } => {
            let (f, t) = (FILES[*from], FILES[*to]);
            if !model.is_file(f) || model.paths.contains_key(t) {
                return false;
            }
            let full_t = root.join(t);
            if let Some(parent) = full_t.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            if std::fs::rename(root.join(f), &full_t).is_err() {
                return false;
            }
            let kind = model.paths.remove(f).unwrap();
            model.ensure_parents(t);
            model.paths.insert(t.to_string(), kind);
            true
        }

        Op::HardLink { from, to } => {
            let (f, t) = (FILES[*from], FILES[*to]);
            if !model.is_file(f) || model.paths.contains_key(t) {
                return false;
            }
            let full_t = root.join(t);
            if let Some(parent) = full_t.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            if std::fs::hard_link(root.join(f), &full_t).is_err() {
                return false;
            }
            let kind = model.paths.get(f).cloned().unwrap();
            model.ensure_parents(t);
            model.paths.insert(t.to_string(), kind);
            true
        }

        Op::RemoveDir { idx } => {
            let d = DIRS[*idx];
            if !model.paths.contains_key(d) {
                return false;
            }
            if std::fs::remove_dir_all(root.join(d)).is_err() {
                return false;
            }
            model.remove_dir(d);
            true
        }

        Op::AtomicReplace { idx, byte } => {
            let p = FILES[*idx];
            if !model.is_file(p) {
                return false;
            }
            let full = root.join(p);
            let Some(parent) = full.parent() else {
                return false;
            };
            let tmp = parent.join(format!(".{}.tmp", p.replace('/', "_")));
            if std::fs::write(&tmp, body(*byte)).is_err() {
                return false;
            }
            if std::fs::rename(&tmp, &full).is_err() {
                return false;
            }
            // Same path, new object: the model gives it a fresh inode. Its parent must
            // already be modelled, because the path existed before this operation.
            debug_assert!(model.paths.contains_key(p));
            let ino = model.fresh_ino();
            model.objects.insert(ino, body(*byte));
            model.paths.insert(p.to_string(), PathKind::File(ino));
            true
        }
    }
}

// ---------------------------------------------------------------------------------------
// Observing the real tree, and comparing against the model
// ---------------------------------------------------------------------------------------

struct Observed {
    set: ObservationSet,
    hashes: BTreeMap<String, [u8; 32]>,
}

fn observe_tree(root: &Path) -> Observed {
    let s = scan(root).expect("scan failed");
    let complete = s.complete();
    let mut paths = Vec::new();
    let mut hashes = BTreeMap::new();

    for e in &s.entries {
        let valid_hash = if e.kind == EntryKind::File {
            let c = observe_content(&root.join(&e.path));
            let h = c.valid_hash().copied();
            if let Some(h) = h {
                hashes.insert(e.path.to_string_lossy().into_owned(), h);
            }
            h
        } else {
            None
        };
        paths.push(ObservedPath {
            path: e.path.clone(),
            kind: e.kind,
            dev: e.dev,
            ino: e.ino,
            size: e.size,
            mtime: e.mtime,
            valid_hash,
        });
    }
    Observed {
        set: ObservationSet::new(paths, complete),
        hashes,
    }
}

fn expected_hash(bytes: &[u8]) -> [u8; 32] {
    *blake3::hash(bytes).as_bytes()
}

// ---------------------------------------------------------------------------------------
// The falsification test
// ---------------------------------------------------------------------------------------

proptest! {
    #![proptest_config(ProptestConfig::with_cases(96))]

    #[test]
    fn the_tool_agrees_with_an_independent_model(ops in prop::collection::vec(op_strategy(), 1..14)) {
        let t = TempDir::new().unwrap();
        let root = t.path();
        let mut model = Model::new();
        std::fs::create_dir(root.join(DIRS[0])).unwrap();
        model.paths.insert(DIRS[0].to_string(), PathKind::Dir);

        let mut prev: Option<ObservationSet> = None;
        let mut remove_dir_applied = 0usize;
        let mut applied = 0usize;

        for op in &ops {
            let was_remove_dir = matches!(op, Op::RemoveDir { .. });
            if !apply(op, root, &mut model) {
                continue;
            }
            applied += 1;
            if was_remove_dir {
                remove_dir_applied += 1;
            }

            let observed = observe_tree(root);

            // 1. The set of regular files matches the model exactly.
            let observed_files: BTreeSet<String> = observed
                .set
                .paths
                .iter()
                .filter(|p| p.kind == EntryKind::File)
                .map(|p| p.path.to_string_lossy().into_owned())
                .collect();
            prop_assert_eq!(
                &observed_files,
                &model.file_paths(),
                "the observed file set must equal the model's"
            );

            // 2. Every observed hash equals the hash of the model's content for that path.
            for (path, got) in &observed.hashes {
                let want_bytes = model
                    .content_of(path)
                    .expect("the model must know every observed file");
                prop_assert_eq!(
                    got,
                    &expected_hash(want_bytes),
                    "hash mismatch at {}", path
                );
            }

            // 3. Reconciliation properties, checked against the model rather than against
            //    the implementation's own idea of what happened.
            if let Some(previous) = &prev {
                let r = reconcile(previous, &observed.set);

                for m in &r.mutations {
                    let path = m.path.to_string_lossy().into_owned();
                    match m.kind {
                        MutationKind::Deleted => {
                            prop_assert!(
                                !model.paths.contains_key(&path),
                                "deleted path {} still exists in the model", path
                            );
                        }
                        MutationKind::Created => {
                            prop_assert!(
                                !previous.paths.iter().any(|p| p.path == m.path),
                                "created path {} was already observed", path
                            );
                        }
                        MutationKind::RenamedOrMoved => {
                            let old = m.old_path.as_ref().unwrap().to_string_lossy().into_owned();
                            prop_assert!(
                                !model.paths.contains_key(&old),
                                "rename source {} still exists in the model", old
                            );
                            prop_assert!(
                                model.paths.contains_key(&path),
                                "rename target {} does not exist in the model", path
                            );
                        }
                        MutationKind::Unchanged | MutationKind::Modified => {
                            prop_assert!(
                                model.paths.contains_key(&path),
                                "{} path {} does not exist in the model",
                                m.kind.as_str(), path
                            );
                        }
                        _ => {}
                    }
                }

                // 4. A deletion is only ever produced from a complete scan.
                if r.count(MutationKind::Deleted) > 0 {
                    prop_assert!(observed.set.complete);
                }
                // 5. An incomplete scan never produces a deletion.
                if !observed.set.complete {
                    prop_assert_eq!(r.count(MutationKind::Deleted), 0);
                }
            }

            prev = Some(observed.set);
        }

        // The generator must actually exercise directory deletion (F-5 obligation), so the
        // suite cannot silently stop covering it.
        let had_remove_dir_op = ops.iter().any(|o| matches!(o, Op::RemoveDir { .. }));
        if had_remove_dir_op && applied > 0 {
            prop_assert!(
                remove_dir_applied > 0,
                "a RemoveDir was generated but never applied"
            );
        }
    }
}

/// The generator emits every operation kind, including directory deletion. Without this the
/// coverage claim would be unverified — which is exactly how F-5 happened in V0.
///
/// This test **samples the real strategy**. An earlier version enumerated the seven `Op`
/// variants by hand in a `match i % 7` and asserted their names were present, which proved only
/// that the enum has seven variants whose names the test already knew. Deleting `RemoveDir`
/// from `op_strategy()` left it passing — the test named for guarding F-5 could not see F-5
/// return.
///
/// The names below are read off the values the generator actually produced, so the assertion
/// cannot be satisfied without the generator emitting them.
#[test]
fn the_generator_covers_every_operation_kind() {
    use proptest::strategy::{Strategy, ValueTree};
    use proptest::test_runner::{Config, TestRunner};

    const DRAWS: u32 = 4000;
    const EXPECTED: [&str; 7] = [
        "CreateFile",
        "WriteFile",
        "DeleteFile",
        "RenameFile",
        "HardLink",
        "RemoveDir",
        "AtomicReplace",
    ];

    let mut runner = TestRunner::new(Config::default());
    let mut counts: BTreeMap<String, u32> = BTreeMap::new();

    for _ in 0..DRAWS {
        let tree = op_strategy()
            .new_tree(&mut runner)
            .expect("the operation strategy must produce a value");
        let op = tree.current();
        // The name comes from the value the generator produced, not from a list written here.
        let name = format!("{op:?}")
            .split_whitespace()
            .next()
            .unwrap_or("?")
            .to_string();
        *counts.entry(name).or_insert(0) += 1;
    }

    for kind in EXPECTED {
        assert!(
            counts.contains_key(kind),
            "op_strategy() never emitted {kind} in {DRAWS} draws; it produced: {:?}",
            counts.keys().collect::<Vec<_>>()
        );
    }

    // A kind the generator emits but this test does not know about means the list above is
    // stale, and a coverage claim resting on a stale list is not a coverage claim.
    assert_eq!(
        counts.len(),
        EXPECTED.len(),
        "the generator emits a kind this test does not name: {:?}",
        counts.keys().collect::<Vec<_>>()
    );

    // F-5: directory deletion must not merely be reachable — it must have a non-zero share of
    // what the generator produces, which is what "must be generated" means.
    assert!(
        counts.get("RemoveDir").copied().unwrap_or(0) > 0,
        "directory deletion must be generated (F-5)"
    );
}

/// The model itself must distinguish hard links from copies. If it did not, the comparison
/// above would be vacuous for that case — the mistake the frozen V0 oracle made.
#[test]
fn the_model_models_hard_link_aliasing() {
    let mut m = Model::new();
    m.create_file("a", b"one".to_vec());
    let ino = match m.paths.get("a") {
        Some(PathKind::File(i)) => *i,
        _ => panic!("expected a file"),
    };
    m.paths.insert("b".to_string(), PathKind::File(ino));

    m.write_through("a", b"two".to_vec());
    assert_eq!(
        m.content_of("b"),
        Some(&b"two".to_vec()),
        "aliasing must be modelled"
    );

    m.remove("a");
    assert!(
        m.content_of("b").is_some(),
        "the object survives while another path names it"
    );
    m.remove("b");
    assert!(
        m.objects.is_empty(),
        "the object goes when the last name goes"
    );
}

/// And the model's directory removal takes the subtree with it.
#[test]
fn the_model_removes_a_subtree() {
    let mut m = Model::new();
    m.paths.insert("d0".into(), PathKind::Dir);
    m.create_file("d0/g0", b"x".to_vec());
    m.create_file("outside", b"y".to_vec());
    m.remove_dir("d0");

    assert!(m.content_of("d0/g0").is_none());
    assert!(!m.paths.contains_key("d0"));
    assert!(m.content_of("outside").is_some());
}

/// A path that is both an entry and an error must not break the store, and the run must be
/// reported as incomplete. This is the duplicate-key defect found during development,
/// pinned so it cannot come back.
#[cfg(unix)]
#[test]
fn a_directory_that_is_both_an_entry_and_an_error_is_handled() {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    let t = TempDir::new().unwrap();
    let root = t.path();
    fs::create_dir(root.join("blocked")).unwrap();
    fs::write(root.join("blocked/hidden.txt"), "h").unwrap();
    fs::set_permissions(root.join("blocked"), fs::Permissions::from_mode(0o000)).unwrap();

    let observed = observe_tree(root);
    fs::set_permissions(root.join("blocked"), fs::Permissions::from_mode(0o755)).unwrap();

    assert!(!observed.set.complete);
    // The entry is present AND the scan is incomplete: both facts, one path.
    assert!(observed
        .set
        .paths
        .iter()
        .any(|p| p.path.as_path() == Path::new("blocked")));
}

/// Determinism: the same operation sequence produces the same observation set twice.
#[test]
fn observing_the_same_tree_twice_is_deterministic() {
    let t = TempDir::new().unwrap();
    let root = t.path();
    let mut model = Model::new();
    std::fs::create_dir(root.join("d0")).unwrap();
    model.paths.insert("d0".to_string(), PathKind::Dir);

    for op in [
        Op::CreateFile { idx: 0, byte: 1 },
        Op::CreateFile { idx: 3, byte: 2 },
        Op::HardLink { from: 0, to: 1 },
        Op::RemoveDir { idx: 0 },
        Op::CreateFile { idx: 2, byte: 3 },
    ] {
        apply(&op, root, &mut model);
    }

    let a = observe_tree(root);
    let b = observe_tree(root);
    assert_eq!(a.set, b.set);
}
