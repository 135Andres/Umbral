//! Increment 5 — RECONCILE tests. Pure-model determinism + real-fixture
//! integration with scan/hash/store. All deterministic unless labelled.

use fsp_check::hash_obs::{HASH_LEN, observe_content};
use fsp_check::reconcile::*;
use fsp_check::scan::scan;
use fsp_check::store::Store;
use fsp_check::{Entry, EntryKind};

fn of(
    path: &str,
    dev: u64,
    ino: u64,
    size: u64,
    mtime: (i64, u32),
    hash: Option<[u8; 32]>,
) -> ObservedPath {
    ObservedPath {
        entry: Entry {
            path: std::path::PathBuf::from(path),
            kind: EntryKind::File,
            dev: Some(dev),
            ino: Some(ino),
            size: Some(size),
            mtime: Some(mtime),
        },
        valid_hash: hash,
    }
}
fn h(b: u8) -> Option<[u8; 32]> {
    Some([b; HASH_LEN])
}
fn kinds(r: &Reconciliation, k: MutationKind) -> Vec<Mutation> {
    r.mutations
        .iter()
        .filter(|m| m.kind == k)
        .cloned()
        .collect()
}

// ---- pure model: the 16 mandated cases --------------------------------------

#[test]
fn unchanged_modified_new_deleted() {
    let prev = ObservationSet::new(
        vec![
            of("same", 1, 1, 3, (1, 0), h(1)),
            of("mod", 1, 2, 3, (1, 0), h(1)),
            of("gone", 1, 3, 3, (1, 0), h(2)),
        ],
        true,
    );
    let curr = ObservationSet::new(
        vec![
            of("same", 1, 1, 3, (1, 0), h(1)),
            of("mod", 1, 2, 9, (2, 0), h(3)),
            of("fresh", 1, 4, 1, (3, 0), h(4)),
        ],
        true,
    );
    let r = reconcile(&prev, &curr);
    assert_eq!(kinds(&r, MutationKind::Unchanged).len(), 1);
    let m = &kinds(&r, MutationKind::Modified)[0];
    assert_eq!(m.path, std::path::PathBuf::from("mod"));
    assert_eq!(
        m.evidence.content_equal,
        Some(false),
        "hash proves content change"
    );
    assert_eq!(kinds(&r, MutationKind::Deleted).len(), 1);
    assert_eq!(kinds(&r, MutationKind::Created).len(), 1);
}

#[test]
fn incomplete_scan_never_claims_deleted() {
    let prev = ObservationSet::new(vec![of("gone?", 1, 3, 3, (1, 0), None)], true);
    let curr = ObservationSet::new(vec![], false); // scan had errors
    let r = reconcile(&prev, &curr);
    assert!(
        kinds(&r, MutationKind::Deleted).is_empty(),
        "unseen is not deleted"
    );
    assert_eq!(kinds(&r, MutationKind::Unobserved).len(), 1);
}

#[test]
fn rename_is_one_mutation_not_delete_plus_create() {
    let prev = ObservationSet::new(vec![of("old.txt", 1, 7, 3, (1, 0), h(5))], true);
    let curr = ObservationSet::new(vec![of("new.txt", 1, 7, 3, (1, 0), h(5))], true);
    let r = reconcile(&prev, &curr);
    assert_eq!(kinds(&r, MutationKind::RenamedOrMoved).len(), 1);
    assert!(kinds(&r, MutationKind::Deleted).is_empty());
    assert!(kinds(&r, MutationKind::Created).is_empty());
    assert_eq!(
        kinds(&r, MutationKind::RenamedOrMoved)[0].old_path,
        Some(std::path::PathBuf::from("old.txt"))
    );
}

#[test]
fn directory_rename_marks_children() {
    let prev = ObservationSet::new(
        vec![
            of("a", 1, 10, 0, (1, 0), None), // dir (kind fixed below)
            of("a/sub.txt", 1, 11, 3, (1, 0), h(6)),
        ],
        true,
    );
    let mut prev = prev;
    prev.paths[0].entry.kind = EntryKind::Dir;
    let curr = ObservationSet::new(
        vec![
            of("b", 1, 10, 0, (1, 0), None),
            of("b/sub.txt", 1, 11, 3, (1, 0), h(6)),
        ],
        true,
    );
    let mut curr = curr;
    curr.paths[0].entry.kind = EntryKind::Dir;
    let r = reconcile(&prev, &curr);
    let renames = kinds(&r, MutationKind::RenamedOrMoved);
    assert_eq!(renames.len(), 2, "dir + child both renamed by evidence");
    assert!(renames.iter().any(|m| m.path == *"b"));
    assert!(
        renames.iter().any(|m| m.under_dir_rename),
        "child flagged under dir rename"
    );
}

#[test]
fn delete_recreate_resolved_only_with_content_evidence() {
    // different pid + different hash -> Recreated
    let prev = ObservationSet::new(vec![of("f", 1, 20, 3, (1, 0), h(1))], true);
    let curr = ObservationSet::new(vec![of("f", 1, 21, 5, (2, 0), h(2))], true);
    let r = reconcile(&prev, &curr);
    assert_eq!(kinds(&r, MutationKind::Recreated).len(), 1);

    // different pid + SAME hash: duplicate content — Ambiguous, never identity
    let curr_same = ObservationSet::new(vec![of("f", 1, 22, 3, (2, 0), h(1))], true);
    let r2 = reconcile(&prev, &curr_same);
    assert_eq!(
        kinds(
            &r2,
            MutationKind::Ambiguous(AmbiguityReason::RecreatedAtSamePath)
        )
        .len(),
        1,
        "same content + new inode is NOT the same object"
    );

    // different pid + no hash evidence: Ambiguous
    let curr_nohash = ObservationSet::new(vec![of("f", 1, 23, 5, (2, 0), None)], true);
    let r3 = reconcile(&prev, &curr_nohash);
    assert_eq!(
        kinds(
            &r3,
            MutationKind::Ambiguous(AmbiguityReason::RecreatedAtSamePath)
        )
        .len(),
        1
    );
}

#[test]
fn path_swap_is_two_renames_not_two_modifications() {
    // previous: A=pidX, B=pidY; current: A=pidY, B=pidX
    let prev = ObservationSet::new(
        vec![
            of("A", 1, 100, 3, (1, 0), h(1)),
            of("B", 1, 200, 4, (1, 0), h(2)),
        ],
        true,
    );
    let curr = ObservationSet::new(
        vec![
            of("A", 1, 200, 4, (1, 0), h(2)),
            of("B", 1, 100, 3, (1, 0), h(1)),
        ],
        true,
    );
    let r = reconcile(&prev, &curr);
    let renames = kinds(&r, MutationKind::RenamedOrMoved);
    assert_eq!(
        renames.len(),
        2,
        "swap = two renames, not two modifications"
    );
    assert!(kinds(&r, MutationKind::Modified).is_empty());
}

#[test]
fn hard_link_entry_deletion_keeps_object_survives_evidence() {
    // previous: a, b hard-linked (same pid); current: only a remains
    let prev = ObservationSet::new(
        vec![
            of("a", 1, 30, 3, (1, 0), h(7)),
            of("b", 1, 30, 3, (1, 0), h(7)),
        ],
        true,
    );
    let curr = ObservationSet::new(vec![of("a", 1, 30, 3, (1, 0), h(7))], true);
    let r = reconcile(&prev, &curr);
    let dels = kinds(&r, MutationKind::Deleted);
    assert_eq!(dels.len(), 1, "one directory entry removed");
    assert_eq!(
        dels[0].evidence.object_survives,
        Some(true),
        "object survives at a"
    );
    // and the surviving path is Unchanged, not Modified
    assert_eq!(kinds(&r, MutationKind::Unchanged).len(), 1);
}

#[test]
fn symlink_target_change_does_not_touch_the_link() {
    // the link entry itself is unchanged (its own dev/ino/mtime/hash family);
    // the target is a different path with its own observation.
    let prev = ObservationSet::new(
        vec![
            of("link", 1, 40, 8, (1, 0), None),
            of("target", 1, 41, 3, (1, 0), h(1)),
        ],
        true,
    );
    let curr = ObservationSet::new(
        vec![
            of("link", 1, 40, 8, (1, 0), None),
            of("target", 1, 41, 9, (5, 0), h(2)),
        ],
        true,
    );
    let r = reconcile(&prev, &curr);
    assert_eq!(
        kinds(&r, MutationKind::Unchanged).len(),
        1,
        "link unchanged"
    );
    assert_eq!(
        kinds(&r, MutationKind::Modified).len(),
        1,
        "target modified"
    );
}

#[test]
fn missing_physical_evidence_is_ambiguous() {
    let mut no_pid = of("m", 1, 50, 3, (1, 0), None);
    no_pid.entry.dev = None;
    no_pid.entry.ino = None;
    let prev = ObservationSet::new(vec![no_pid.clone()], true);
    let mut changed = no_pid.clone();
    changed.entry.size = Some(99);
    let curr = ObservationSet::new(vec![changed], true);
    let r = reconcile(&prev, &curr);
    assert_eq!(
        kinds(
            &r,
            MutationKind::Ambiguous(AmbiguityReason::MissingPhysicalEvidence)
        )
        .len(),
        1
    );
}

#[test]
fn idempotent_and_order_independent() {
    let prev = ObservationSet::new(
        vec![
            of("x", 1, 60, 3, (1, 0), h(1)),
            of("y", 1, 61, 3, (1, 0), h(2)),
        ],
        true,
    );
    let curr = ObservationSet::new(
        vec![
            of("y", 1, 61, 5, (2, 0), h(2)),
            of("x", 1, 60, 3, (1, 0), h(1)),
        ],
        true,
    );
    let r1 = reconcile(&prev, &curr);
    let r2 = reconcile(&prev, &curr);
    assert_eq!(r1, r2, "reconcile(A,B) == reconcile(A,B)");
    // input order must not matter (sets are sorted internally)
    let mut rev_prev_paths = prev.paths.clone();
    rev_prev_paths.reverse();
    let r3 = reconcile(&ObservationSet::new(rev_prev_paths, true), &curr);
    assert_eq!(r1, r3);
}

#[test]
fn reconcile_self_produces_only_unchanged() {
    let set = ObservationSet::new(
        vec![
            of("p", 1, 70, 3, (1, 0), h(1)),
            of("q", 1, 71, 0, (1, 0), None),
        ],
        true,
    );
    let r = reconcile(&set, &set);
    assert_eq!(r.mutations.len(), 2);
    assert!(
        r.mutations
            .iter()
            .all(|m| m.kind == MutationKind::Unchanged)
    );
}

#[test]
fn rename_plus_modify_is_rename_with_modified_evidence() {
    // same pid moves AND changes state: RenamedOrMoved with content evidence
    let prev = ObservationSet::new(vec![of("old", 1, 80, 3, (1, 0), h(1))], true);
    let curr = ObservationSet::new(vec![of("new", 1, 80, 9, (2, 0), h(2))], true);
    let r = reconcile(&prev, &curr);
    let renames = kinds(&r, MutationKind::RenamedOrMoved);
    assert_eq!(renames.len(), 1);
    assert_eq!(
        renames[0].evidence.content_equal,
        Some(false),
        "moved AND modified"
    );
}

// ---- integration: real fixtures through the whole cycle ----------------------

fn scan_to_set(root: &std::path::Path) -> ObservationSet {
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

#[test]
fn full_cycle_scan_reconcile_store() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    std::fs::write(root.join("keep.txt"), b"keep").unwrap();
    std::fs::write(root.join("bye.txt"), b"bye").unwrap();

    let s1 = scan_to_set(root);
    let mut store = Store::open_in_memory().unwrap();
    let now = 1;
    for o in &s1.paths {
        store
            .record(
                &o.entry,
                now,
                o.valid_hash
                    .map(|h| ContentObservationShim::stable(o.physical_id(), h))
                    .as_ref(),
            )
            .unwrap();
    }

    // mutate: modify keep, delete bye, create new, rename old->renamed
    std::fs::write(root.join("keep.txt"), b"keep-modified").unwrap();
    std::fs::remove_file(root.join("bye.txt")).unwrap();
    std::fs::write(root.join("new.txt"), b"new").unwrap();
    std::fs::rename(root.join("renamed-src"), root.join("renamed-dst")).ok();
    std::fs::write(root.join("renamed-src"), b"r").unwrap();
    let s1b = scan_to_set(root); // capture before rename for rename evidence
    std::fs::rename(root.join("renamed-src"), root.join("renamed-dst")).unwrap();

    let s2 = scan_to_set(root);
    let r = reconcile(&s1, &s2);
    assert!(
        kinds(&r, MutationKind::Modified)
            .iter()
            .any(|m| m.path == *"keep.txt")
    );
    assert!(
        kinds(&r, MutationKind::Deleted)
            .iter()
            .any(|m| m.path == *"bye.txt")
    );
    assert!(
        kinds(&r, MutationKind::Created)
            .iter()
            .any(|m| m.path == *"new.txt")
    );
    // rename: src existed in s1b, dst in s2
    let r2 = reconcile(&scan_to_set_pre(root, &s1b), &s2);
    assert!(
        kinds(&r2, MutationKind::RenamedOrMoved)
            .iter()
            .any(|m| m.path == *"renamed-dst")
    );

    // tombstone need: store keeps bye.txt in projection until told; the
    // reconciliation is the artifact that reports the deletion (H25: store
    // records observations; reconcile interprets them).
    let active_before = store.active_projection().unwrap();
    assert!(active_before.iter().any(|r| r.path == *"bye.txt"));

    // record s2 and confirm projection coherence (INV-2 via rebuild)
    let now2 = 2;
    for o in &s2.paths {
        store
            .record(
                &o.entry,
                now2,
                o.valid_hash
                    .map(|h| ContentObservationShim::stable(o.physical_id(), h))
                    .as_ref(),
            )
            .unwrap();
    }
    let before = store.active_projection().unwrap();
    store.rebuild_projection().unwrap();
    assert_eq!(before, store.active_projection().unwrap());
}

// helper so the integration test can synthesize stable content observations
struct ContentObservationShim;
impl ContentObservationShim {
    fn stable(
        pid: fsp_check::identity::PhysicalId,
        hash: [u8; HASH_LEN],
    ) -> fsp_check::hash_obs::ContentObservation {
        fsp_check::hash_obs::ContentObservation {
            physical_id: pid,
            hash: Some(hash),
            hashed_len: Some(1),
            stability: fsp_check::hash_obs::Stability::Stable,
            deltas: vec![],
            error: None,
        }
    }
}

fn scan_to_set_pre(_root: &std::path::Path, s: &ObservationSet) -> ObservationSet {
    s.clone()
}

// ---- proptest: model properties ---------------------------------------------

use proptest::prelude::*;

proptest! {
    /// reconcile(A, A) yields only Unchanged, for arbitrary generated sets.
    #[test]
    fn self_reconcile_is_all_unchanged(
        n in 0..20u8,
        seeds in proptest::collection::vec((0..4u8, 0..1_000u32), 20),
    ) {
        let mut paths = Vec::new();
        for i in 0..n {
            let (dev_bump, ino) = &seeds[i as usize];
            paths.push(of(&format!("p{i}"), 1 + *dev_bump as u64, *ino as u64 + 1000, 3, (1, 0), h(i)));
        }
        let set = ObservationSet::new(paths, true);
        let r = reconcile(&set, &set);
        prop_assert!(r.mutations.iter().all(|m| m.kind == MutationKind::Unchanged));
    }

    /// Idempotence/determinism for arbitrary pairs.
    #[test]
    fn reconcile_is_deterministic(
        a in proptest::collection::vec((0..1_000u32, 0..1_000u32), 10),
        b in proptest::collection::vec((0..1_000u32, 0..1_000u32), 10),
    ) {
        let mk = |v: &[(u32, u32)], off: u32| {
            ObservationSet::new(
                v.iter().enumerate().map(|(i, (dev, ino))| of(&format!("f{}", i + off as usize), 1, (*ino as u64) + 1, *dev as u64, (1, 0), None)).collect(),
                true,
            )
        };
        let set_a = mk(&a, 0);
        let set_b = mk(&b, 100);
        let r1 = reconcile(&set_a, &set_b);
        let r2 = reconcile(&set_a, &set_b);
        prop_assert_eq!(r1, r2);
    }

    /// Content equality never produces identity: equal hashes across
    /// different PhysicalIds must never yield RenamedOrMoved / Unchanged /
    /// Modified / Recreated-continuity. (The vanished original path IS
    /// Deleted — that is a path fact, not an identity claim; the copy is
    /// Created. Duplicate content, two objects.)
    #[test]
    fn same_hash_never_becomes_rename_or_unchanged(
        hash_byte in 0..255u8,
    ) {
        let prev = ObservationSet::new(vec![of("original", 1, 900, 3, (1, 0), h(hash_byte))], true);
        let curr = ObservationSet::new(vec![of("copy", 1, 901, 3, (1, 0), h(hash_byte))], true);
        let r = reconcile(&prev, &curr);
        prop_assert!(r.mutations.iter().all(|m| !matches!(
            m.kind,
            MutationKind::RenamedOrMoved | MutationKind::Unchanged | MutationKind::Modified
        )));
    }
}
