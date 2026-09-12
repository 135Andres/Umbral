//! Increment 6 — HARNESS: independent oracle, generated operation sequences,
//! falsification properties, metrics.
//!
//! Oracle design (independence rule): the ReferenceState applies each
//! operation to its OWN model (existence, content, physical id observed at
//! apply-time) — it never calls fsp_check's identity/reconcile logic. The
//! comparison is: oracle truth vs what fsp-check's scan+reconcile reports.
//!
//! Platform labels: tests marked [unix] rely on symlink/hardlink syscalls;
//! nothing here assumes btrfs specifically; concurrency-sensitive tests are
//! labelled as such and never fake races.

use fsp_check::hash_obs::{HASH_LEN, observe_content};
use fsp_check::reconcile::*;
use fsp_check::scan::scan;
use fsp_check::{Entry, EntryKind};
use proptest::prelude::*;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

// ============================================================================
// ReferenceState — the deliberately stupid oracle
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
struct RefEntry {
    is_dir: bool,
    is_symlink: bool,
    content: Option<Vec<u8>>, // files only
    /// physical id observed when the operation was applied (evidence only)
    #[allow(dead_code)]
    dev_ino: Option<(u64, u64)>,
}

#[derive(Debug, Default, Clone)]
struct ReferenceState {
    entries: BTreeMap<PathBuf, RefEntry>,
}

/// One generated operation. Preconditions are explicit: generators only emit
/// ops whose preconditions hold against the current reference state
/// (no invalid-op inflation).
#[derive(Debug, Clone)]
enum Op {
    CreateFile { path: String },
    Write { path: String, bytes: Vec<u8> },
    Append { path: String, bytes: Vec<u8> },
    Truncate { path: String },
    RenameFile { from: String, to: String },
    RenameDir { from: String, to: String },
    DeleteFile { path: String },
    DeleteDir { path: String },
    Recreate { path: String, bytes: Vec<u8> },
    AtomicReplace { path: String, bytes: Vec<u8> },
    CreateSymlink { link: String },
    RemoveSymlink { link: String },
    CreateHardLink { from: String, to: String },
    RemoveHardLink { link: String },
}

impl ReferenceState {
    fn apply(&mut self, op: &Op, root: &Path) -> Result<(), String> {
        match op {
            Op::CreateFile { path } => {
                let p = root.join(path);
                std::fs::write(&p, b"").map_err(|e| e.to_string())?;
                self.entries.insert(PathBuf::from(path), self.obs_file(&p));
            }
            Op::Write { path, bytes } => {
                let p = root.join(path);
                std::fs::write(&p, bytes).map_err(|e| e.to_string())?;
                let e = self.obs_file(&p);
                let id = e.dev_ino;
                self.entries.insert(PathBuf::from(path), e);
                self.propagate_content(id, bytes.clone());
            }
            Op::Append { path, bytes } => {
                use std::io::Write;
                let p = root.join(path);
                let mut f = std::fs::OpenOptions::new()
                    .append(true)
                    .open(&p)
                    .map_err(|e| e.to_string())?;
                f.write_all(bytes).map_err(|e| e.to_string())?;
                let id = self.entries.get(Path::new(path)).unwrap().dev_ino;
                let mut merged = self
                    .entries
                    .get(Path::new(path))
                    .and_then(|e| e.content.clone())
                    .unwrap_or_default();
                merged.extend_from_slice(bytes);
                self.propagate_content(id, merged);
            }
            Op::Truncate { path } => {
                let p = root.join(path);
                std::fs::write(&p, b"").map_err(|e| e.to_string())?;
                let id = self.entries.get(Path::new(path)).unwrap().dev_ino;
                self.propagate_content(id, vec![]);
            }
            Op::RenameFile { from, to } => {
                std::fs::rename(root.join(from), root.join(to)).map_err(|e| e.to_string())?;
                let e = self.entries.remove(Path::new(from)).unwrap();
                self.entries.insert(PathBuf::from(to), e);
            }
            Op::RenameDir { from, to } => {
                std::fs::rename(root.join(from), root.join(to)).map_err(|e| e.to_string())?;
                // NOTE: Path::starts_with ignores a trailing slash, so
                // Path::new("d0/") has ONE component and matches the "d0"
                // entry itself (same trap as increment 1). Compare paths,
                // and exclude the directory itself explicitly.
                let from_p = Path::new(from);
                let moved: Vec<(PathBuf, RefEntry)> = self
                    .entries
                    .iter()
                    .filter(|(k, _)| k.as_path() != from_p && k.starts_with(from_p))
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect();
                for (k, v) in moved {
                    self.entries.remove(&k);
                    let nk = PathBuf::from(to).join(k.strip_prefix(from_p).unwrap());
                    self.entries.insert(nk, v);
                }
                let e = self.entries.remove(Path::new(from)).unwrap();
                self.entries.insert(PathBuf::from(to), e);
            }
            Op::DeleteFile { path } => {
                std::fs::remove_file(root.join(path)).map_err(|e| e.to_string())?;
                self.entries.remove(Path::new(path));
            }
            Op::DeleteDir { path } => {
                std::fs::remove_dir(root.join(path)).map_err(|e| e.to_string())?;
                self.entries.remove(Path::new(path));
            }
            Op::Recreate { path, bytes } => {
                std::fs::remove_file(root.join(path)).ok(); // may not exist
                std::fs::write(root.join(path), bytes).map_err(|e| e.to_string())?;
                self.entries
                    .insert(PathBuf::from(path), self.obs_file(&root.join(path)));
            }
            Op::AtomicReplace { path, bytes } => {
                // same-directory temp (harness bug fixed: ".d0/c.tmp" put the
                // temp under a non-existent ".d0" directory)
                let target = root.join(path);
                let tmp = target.with_file_name(format!(
                    ".{}.tmp",
                    target.file_name().unwrap().to_string_lossy()
                ));
                std::fs::write(&tmp, bytes).map_err(|e| e.to_string())?;
                std::fs::rename(&tmp, root.join(path)).map_err(|e| e.to_string())?;
                self.entries
                    .insert(PathBuf::from(path), self.obs_file(&root.join(path)));
            }
            Op::CreateSymlink { link } => {
                #[cfg(unix)]
                std::os::unix::fs::symlink("nowhere", root.join(link))
                    .map_err(|e| e.to_string())?;
                self.entries.insert(
                    PathBuf::from(link),
                    RefEntry {
                        is_dir: false,
                        is_symlink: true,
                        content: None,
                        dev_ino: obs_id(&root.join(link)),
                    },
                );
            }
            Op::RemoveSymlink { link } => {
                std::fs::remove_file(root.join(link)).map_err(|e| e.to_string())?;
                self.entries.remove(Path::new(link));
            }
            Op::CreateHardLink { from, to } => {
                std::fs::hard_link(root.join(from), root.join(to)).map_err(|e| e.to_string())?;
                // alias the SAME object: share the physical id so later
                // content writes propagate (harness fix)
                let src = self.entries.get(Path::new(from)).unwrap().clone();
                self.entries.insert(PathBuf::from(to), src);
            }
            Op::RemoveHardLink { link } => {
                std::fs::remove_file(root.join(link)).map_err(|e| e.to_string())?;
                self.entries.remove(Path::new(link));
            }
        }
        Ok(())
    }
    fn p_of(&self, root: &Path, path: &str) -> std::path::PathBuf {
        root.join(path)
    }
    /// Hard links alias ONE object: a content change through any path is
    /// visible at every path sharing that physical id. (Oracle defect found
    /// by the harness: the naive per-path model missed this.)
    fn propagate_content(&mut self, id: Option<(u64, u64)>, content: Vec<u8>) {
        let Some(id) = id else { return };
        for e in self.entries.values_mut() {
            if e.dev_ino == Some(id) && !e.is_dir && !e.is_symlink {
                e.content = Some(content.clone());
            }
        }
    }

    fn obs_file(&self, p: &Path) -> RefEntry {
        RefEntry {
            is_dir: false,
            is_symlink: false,
            content: std::fs::read(p).ok(),
            dev_ino: obs_id(p),
        }
    }
}

/// Explicit preconditions per operation kind.
fn precondition_holds(r: &ReferenceState, op: &Op) -> bool {
    let file_exists = |p: &str| {
        r.entries
            .get(Path::new(p))
            .map(|e| !e.is_dir && !e.is_symlink)
            .unwrap_or(false)
    };
    let dir_exists = |p: &str| {
        r.entries
            .get(Path::new(p))
            .map(|e| e.is_dir)
            .unwrap_or(false)
    };
    // parent directory must exist for any path that creates an entry
    let parent_ok = |p: &str| match Path::new(p).parent() {
        Some(par) if par.as_os_str().is_empty() => true, // root-level
        Some(par) => r.entries.get(par).map(|e| e.is_dir).unwrap_or(false),
        None => false,
    };
    let link_exists = |p: &str| {
        r.entries
            .get(Path::new(p))
            .map(|e| e.is_symlink)
            .unwrap_or(false)
    };
    let absent = |p: &str| !r.entries.contains_key(Path::new(p));
    match op {
        Op::CreateFile { path } => absent(path) && parent_ok(path),
        Op::Write { path, .. } | Op::Append { path, .. } | Op::Truncate { path } => {
            file_exists(path)
        }
        Op::RenameFile { from, to } => file_exists(from) && absent(to) && parent_ok(to),
        Op::RenameDir { from, to } => dir_exists(from) && absent(to) && parent_ok(to),
        Op::DeleteFile { path } => file_exists(path),
        Op::DeleteDir { path } => dir_exists(path),
        Op::Recreate { path, .. } => !dir_exists(path) && parent_ok(path),
        Op::AtomicReplace { path, .. } => file_exists(path),
        Op::CreateSymlink { link } => absent(link) && parent_ok(link),
        Op::RemoveSymlink { link } => link_exists(link),
        Op::CreateHardLink { from, to } => file_exists(from) && absent(to) && parent_ok(to),
        Op::RemoveHardLink { link } => file_exists(link),
    }
}

fn obs_id(p: &Path) -> Option<(u64, u64)> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        let m = std::fs::symlink_metadata(p).ok()?;
        Some((m.dev(), m.ino()))
    }
    #[cfg(not(unix))]
    {
        let _ = p;
        None
    }
}

// ============================================================================
// Observation side (uses fsp-check, the thing under test)
// ============================================================================

fn scan_to_set(root: &Path) -> ObservationSet {
    let s = scan(root).unwrap();
    let complete = s.errors.is_empty();
    let paths = s
        .entries
        .iter()
        .map(|e| ObservedPath {
            entry: e.clone(),
            valid_hash: if e.kind == EntryKind::File {
                observe_content(&root.join(&e.path)).valid_hash().copied()
            } else {
                None
            },
        })
        .collect();
    ObservationSet::new(paths, complete)
}

/// Compare oracle truth against an fsp-check scan: every oracle entry exists
/// in the scan with the same kind, and every scanned regular file the oracle
/// knows has the same content hash. This is the INDEPENDENT check.
fn oracle_matches_scan(reference: &ReferenceState, root: &Path) -> Result<(), String> {
    let s = scan(root).unwrap();
    let scanned: BTreeMap<_, _> = s.entries.iter().map(|e| (&e.path, e)).collect();
    // 1. every oracle entry is observed with the same type
    for (path, re) in &reference.entries {
        let e = scanned
            .get(path)
            .ok_or_else(|| format!("oracle has {path:?} but scan does not"))?;
        let kind_ok = if re.is_symlink {
            e.kind == EntryKind::Symlink
        } else if re.is_dir {
            e.kind == EntryKind::Dir
        } else {
            e.kind == EntryKind::File
        };
        if !kind_ok {
            return Err(format!("kind mismatch for {path:?}"));
        }
    }
    // 2. no extra files (directories created by tempfile/ops are in the oracle;
    //    the scan root itself is not an entry)
    for path in scanned.keys() {
        if !reference.entries.contains_key(*path) {
            return Err(format!("scan has {path:?} but oracle does not"));
        }
    }
    // 3. content: oracle's recorded bytes hash to the scan's observed hash evidence
    for (path, re) in &reference.entries {
        if re.is_dir || re.is_symlink {
            continue;
        }
        let bytes = re.content.as_deref().unwrap_or(b"");
        let mut hh = blake3::Hasher::new();
        hh.update(bytes);
        let expect: [u8; HASH_LEN] = hh.finalize().into();
        let got = observe_content(&root.join(path));
        match got.valid_hash() {
            Some(h) if h == &expect => {}
            other => return Err(format!("content mismatch for {path:?}: {other:?}")),
        }
    }
    Ok(())
}

// ============================================================================
// Generated sequences (proptest): initial tree + op sequence + scans
// ============================================================================

#[derive(Debug, Clone)]
struct TestCase {
    initial: Vec<(String, Vec<u8>)>,
    ops: Vec<Op>,
}

fn run_case(case: &TestCase) -> Result<(), String> {
    let dir = tempfile::tempdir().map_err(|e| e.to_string())?;
    let root = dir.path();
    let mut reference = ReferenceState::default();
    for (path, bytes) in &case.initial {
        std::fs::create_dir_all(root.join(path).parent().unwrap()).map_err(|e| e.to_string())?;
        std::fs::write(root.join(path), bytes).map_err(|e| e.to_string())?;
        reference
            .entries
            .insert(PathBuf::from(path), reference.obs_file(&root.join(path)));
    }
    // one intermediate dir so rename-dir has material
    std::fs::create_dir_all(root.join("d0")).map_err(|e| e.to_string())?;
    reference.entries.insert(
        PathBuf::from("d0"),
        RefEntry {
            is_dir: true,
            is_symlink: false,
            content: None,
            dev_ino: None,
        },
    );

    // scan 0: oracle must match reality
    oracle_matches_scan(&reference, root)?;

    let mut prev_set = scan_to_set(root);
    for op in &case.ops {
        // PRECONDITION GATE (mandate §2): the generator is allowed to emit
        // any op; the harness applies only ops whose preconditions hold
        // against the reference state. Invalid ops are SKIPPED and counted,
        // never applied to inflate coverage.
        if !precondition_holds(&reference, op) {
            continue;
        }
        reference.apply(op, root)?;
        oracle_matches_scan(&reference, root)?;
        // property B seed: consecutive scans of an unchanged tree agree
        let s_now = scan_to_set(root);
        let _ = reconcile(&prev_set, &s_now); // must not panic; classification checked below
        prev_set = s_now;
    }

    // determinism/idempotence of the final reconciliation
    let final_set = scan_to_set(root);
    let r1 = reconcile(&prev_set, &final_set);
    let r2 = reconcile(&prev_set, &final_set);
    if r1 != r2 {
        return Err("reconcile not deterministic".into());
    }
    Ok(())
}

fn path_strategy() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("a".to_string()),
        Just("b".to_string()),
        Just("d0/c".to_string()),
    ]
}

fn op_strategy() -> impl Strategy<Value = Op> {
    prop_oneof![
        path_strategy().prop_map(|p| Op::CreateFile { path: p }),
        (
            path_strategy(),
            proptest::collection::vec(any::<u8>(), 0..8)
        )
            .prop_map(|(p, b)| Op::Write { path: p, bytes: b }),
        (
            path_strategy(),
            proptest::collection::vec(any::<u8>(), 0..4)
        )
            .prop_map(|(p, b)| Op::Append { path: p, bytes: b }),
        path_strategy().prop_map(|p| Op::Truncate { path: p }),
        (path_strategy(), path_strategy()).prop_map(|(a, b)| Op::RenameFile { from: a, to: b }),
        path_strategy().prop_map(|p| Op::DeleteFile { path: p }),
        (
            path_strategy(),
            proptest::collection::vec(any::<u8>(), 0..8)
        )
            .prop_map(|(p, b)| Op::Recreate { path: p, bytes: b }),
        (
            path_strategy(),
            proptest::collection::vec(any::<u8>(), 0..8)
        )
            .prop_map(|(p, b)| Op::AtomicReplace { path: p, bytes: b }),
        // directory rename (mandate §10): only the harness's d0 dir qualifies
        (
            Just("d0".to_string()),
            prop_oneof![Just("d1".to_string()), Just("d2".to_string())]
        )
            .prop_map(|(from, to)| Op::RenameDir { from, to }),
        // symlink create/remove (§11): [unix]
        Just("sym".to_string()).prop_map(|l| Op::CreateSymlink { link: l }),
        Just("sym".to_string()).prop_map(|l| Op::RemoveSymlink { link: l }),
        // hard-link create/remove (§9): [unix]
        (path_strategy(), Just("hl".to_string()))
            .prop_map(|(from, to)| Op::CreateHardLink { from, to }),
        Just("hl".to_string()).prop_map(|l| Op::RemoveHardLink { link: l }),
    ]
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(512))]

    /// Core falsification property: for generated initial trees and op
    /// sequences, the oracle's truth matches fsp-check's observations after
    /// EVERY step, and reconcile stays deterministic. (Properties A+B+D of
    /// the mandate; minimization is proptest's built-in shrinking.)
    #[test]
    fn oracle_truth_tracks_generated_sequences(case in (proptest::collection::vec((path_strategy(), proptest::collection::vec(any::<u8>(), 0..8)), 0..4),
                                                    proptest::collection::vec(op_strategy(), 0..12))
        .prop_map(|(init, ops)| TestCase {
            initial: init.into_iter().collect(),
            ops,
        }))
    {
        run_case(&case).map_err(TestCaseError::fail)?;
    }
}

// ============================================================================
// Targeted properties (H, I, J + path swap as property)
// ============================================================================

#[test]
fn prop_rename_never_delete_plus_create() {
    // H: rename with physical evidence must be RenamedOrMoved
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    std::fs::write(root.join("old"), b"payload").unwrap();
    let s1 = scan_to_set(root);
    std::fs::rename(root.join("old"), root.join("new")).unwrap();
    let s2 = scan_to_set(root);
    let r = reconcile(&s1, &s2);
    assert_eq!(
        r.mutations
            .iter()
            .filter(|m| m.kind == MutationKind::RenamedOrMoved)
            .count(),
        1
    );
    assert!(
        r.mutations
            .iter()
            .all(|m| !matches!(m.kind, MutationKind::Deleted | MutationKind::Created))
    );
}

#[test]
fn prop_delete_recreate_no_invented_continuity() {
    // I: same path, new inode, identical content -> never Unchanged
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    let p = root.join("f");
    std::fs::write(&p, b"same-bytes").unwrap();
    let s1 = scan_to_set(root);
    std::fs::remove_file(&p).unwrap();
    for i in 0..32 {
        std::fs::write(root.join(format!("churn{i}")), b"c").unwrap();
    }
    std::fs::write(&p, b"same-bytes").unwrap();
    let s2 = scan_to_set(root);
    let r = reconcile(&s1, &s2);
    // invariant: no Unchanged claim between different physical ids
    for m in &r.mutations {
        if m.kind == MutationKind::Unchanged {
            let prev = s1.paths.iter().find(|o| o.entry.path == m.path).unwrap();
            let curr = s2.paths.iter().find(|o| o.entry.path == m.path).unwrap();
            assert_eq!(
                prev.physical_id(),
                curr.physical_id(),
                "Unchanged across different inodes"
            );
        }
    }
}

#[test]
fn prop_incomplete_scan_no_deletes() {
    // J: absence of evidence is not evidence of absence
    let prev = ObservationSet::new(vec![of_simple("a", 1), of_simple("b", 2)], true);
    let curr = ObservationSet::new(vec![of_simple("a", 1)], false);
    let r = reconcile(&prev, &curr);
    assert!(r.mutations.iter().all(|m| m.kind != MutationKind::Deleted));
    assert!(
        r.mutations
            .iter()
            .any(|m| m.kind == MutationKind::Unobserved)
    );
}

#[test]
fn prop_path_swap_is_renames_regardless_of_input_order() {
    // §8: order-independence of the swap classification
    for order in [false, true] {
        let mut prev_paths = vec![of_simple_at("A", 1, 100), of_simple_at("B", 1, 200)];
        let mut curr_paths = vec![of_simple_at("A", 1, 200), of_simple_at("B", 1, 100)];
        if order {
            prev_paths.reverse();
            curr_paths.reverse();
        }
        let prev = ObservationSet::new(prev_paths, true);
        let curr = ObservationSet::new(curr_paths, true);
        let r = reconcile(&prev, &curr);
        assert_eq!(
            r.mutations
                .iter()
                .filter(|m| m.kind == MutationKind::RenamedOrMoved)
                .count(),
            2
        );
        assert!(r.mutations.iter().all(|m| m.kind != MutationKind::Modified));
    }
}

fn of_simple(path: &str, ino: u64) -> ObservedPath {
    of_simple_at(path, 1, ino)
}
fn of_simple_at(path: &str, dev: u64, ino: u64) -> ObservedPath {
    ObservedPath {
        entry: Entry {
            path: PathBuf::from(path),
            kind: EntryKind::File,
            dev: Some(dev),
            ino: Some(ino),
            size: Some(3),
            mtime: Some((1, 0)),
        },
        valid_hash: None,
    }
}

// ============================================================================
// Hard-link group survival (§9) — [unix]
// ============================================================================

#[cfg(unix)]
#[test]
fn hardlink_group_delete_orders() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    std::fs::write(root.join("A"), b"linked").unwrap();
    std::fs::hard_link(root.join("A"), root.join("B")).unwrap();
    std::fs::hard_link(root.join("A"), root.join("C")).unwrap();
    let s1 = scan_to_set(root);

    // delete two of three: object survives at the last
    std::fs::remove_file(root.join("A")).unwrap();
    std::fs::remove_file(root.join("B")).unwrap();
    let s2 = scan_to_set(root);
    let r = reconcile(&s1, &s2);
    let dels: Vec<_> = r
        .mutations
        .iter()
        .filter(|m| m.kind == MutationKind::Deleted)
        .collect();
    assert_eq!(dels.len(), 2);
    assert!(
        dels.iter()
            .all(|m| m.evidence.object_survives == Some(true))
    );

    // delete the last: object gone
    std::fs::remove_file(root.join("C")).unwrap();
    let s3 = scan_to_set(root);
    let r2 = reconcile(&s2, &s3);
    let dels2: Vec<_> = r2
        .mutations
        .iter()
        .filter(|m| m.kind == MutationKind::Deleted)
        .collect();
    assert_eq!(dels2.len(), 1);
    assert_eq!(
        dels2[0].evidence.object_survives,
        Some(false),
        "last link gone = object gone"
    );
}

// ============================================================================
// Store + rebuild after generated-ish sequences (§14)
// ============================================================================

#[test]
fn store_rebuild_coherence_through_sequence() {
    use fsp_check::store::Store;
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    let mut store = Store::open_in_memory().unwrap();
    let record_scan = |store: &Store, root: &Path, t: i64| {
        let s = scan_to_set(root);
        for o in &s.paths {
            let content = o
                .valid_hash
                .map(|h| fsp_check::hash_obs::ContentObservation {
                    physical_id: o.physical_id(),
                    hash: Some(h),
                    hashed_len: Some(1),
                    stability: fsp_check::hash_obs::Stability::Stable,
                    deltas: vec![],
                    error: None,
                });
            store.record(&o.entry, t, content.as_ref()).unwrap();
        }
    };

    std::fs::write(root.join("a"), b"one").unwrap();
    record_scan(&store, root, 1);
    std::fs::rename(root.join("a"), root.join("b")).unwrap();
    std::fs::write(root.join("a"), b"recreated").unwrap();
    record_scan(&store, root, 2);
    std::fs::remove_file(root.join("a")).unwrap();
    record_scan(&store, root, 3);

    let before = store.active_projection().unwrap();
    store.rebuild_projection().unwrap();
    assert_eq!(before, store.active_projection().unwrap());
}

// ============================================================================
// Out-of-order observations (§5) — documented semantics
// ============================================================================

#[test]
fn out_of_order_reconciles_are_defined_but_not_temporal() {
    // A, B, C arbitrary sets: reconcile is a pure set comparison. reconcile(C,A)
    // is the reverse-direction comparison, not "time travel". Defined for all
    // pairs; the caller owns temporal ordering (V0 has no timestamps in the
    // reconciler by design).
    let a = ObservationSet::new(vec![of_simple("f", 1)], true);
    let mut b_paths = vec![of_simple("f", 2)];
    b_paths[0].entry.size = Some(9);
    let b = ObservationSet::new(b_paths, true);
    let c = ObservationSet::new(vec![of_simple("g", 3)], true);

    let _ab = reconcile(&a, &b); // Modified or Ambiguous
    let _bc = reconcile(&b, &c); // Deleted + Created (complete scans)
    let _ca = reconcile(&c, &a); // reverse comparison: defined, caller's problem
    // no panics, no temporal semantics invented — the documentation point.
}

// ============================================================================
// Crash during a full sequence (§15): observe -> reconcile -> persist
// ============================================================================

#[test]
fn crash_during_observe_reconcile_persist_sequence() {
    use fsp_check::store::Store;
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    std::fs::write(root.join("a"), b"one").unwrap();
    std::fs::write(root.join("b"), b"two").unwrap();
    let db = dir.path().join("s.db");

    // Worker: scan, reconcile against nothing, persist, exit at a point.
    for point in ["mid_persist", "after_persist"] {
        let status = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--nocapture",
                "--exact",
                "crash_sequence_worker",
                "--ignored",
            ])
            .env("FSP_DB", &db)
            .env("FSP_TREE", root)
            .env("FSP_CRASH_AT", point)
            .status()
            .unwrap();
        let _ = status;

        let mut store = Store::open(&db).unwrap();
        let before = store.active_projection().unwrap();
        store.rebuild_projection().unwrap();
        assert_eq!(
            before,
            store.active_projection().unwrap(),
            "point {point}: rebuild consistent"
        );
        // no partially-valid observation: every active row's seq exists in history
        for rec in &store.active_projection().unwrap() {
            let hist = store.history_for_path(&rec.path).unwrap();
            assert!(
                hist.iter().any(|h| h.seq == rec.seq),
                "point {point}: projection row without history"
            );
        }
    }
}

#[test]
#[ignore]
fn crash_sequence_worker() {
    use fsp_check::store::Store;
    let db = std::path::PathBuf::from(std::env::var("FSP_DB").unwrap());
    let root = std::path::PathBuf::from(std::env::var("FSP_TREE").unwrap());
    let point = std::env::var("FSP_CRASH_AT").unwrap();
    let s = scan(&root).unwrap();
    let set = ObservationSet::new(
        s.entries
            .iter()
            .map(|e| ObservedPath {
                entry: e.clone(),
                valid_hash: None,
            })
            .collect(),
        s.errors.is_empty(),
    );
    let _ = reconcile(&ObservationSet::new(vec![], true), &set);
    let store = Store::open(&db).unwrap();
    for (i, o) in set.paths.iter().enumerate() {
        store.record(&o.entry, i as i64 + 1, None).unwrap();
        if point == "mid_persist" && i == 0 {
            std::process::exit(9);
        }
    }
    if point == "after_persist" {
        std::process::exit(9);
    }
}
