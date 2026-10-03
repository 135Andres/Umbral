//! Reconciliation properties. Covers A6, and the F-6 lesson: no assertion here may depend
//! on what a filesystem does with inode numbers.
//!
//! Every filesystem-dependent scenario is exercised with observation sets built by hand, so
//! the assertions are deterministic in any environment. The filesystem's actual behaviour is
//! *recorded* separately (see `fs_matrix.rs`), never asserted.

use std::path::{Path, PathBuf};

use proptest::prelude::*;
use umbral::reconcile::{reconcile, MutationKind, ObservationSet, ObservedPath};
use umbral::scan::EntryKind;

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

fn set(paths: Vec<ObservedPath>, complete: bool) -> ObservationSet {
    ObservationSet::new(paths, complete)
}

fn kinds(
    r: &umbral::reconcile::Reconciliation,
    k: MutationKind,
) -> Vec<&umbral::reconcile::Mutation> {
    r.mutations.iter().filter(|m| m.kind == k).collect()
}

// ---------------------------------------------------------------------------------------
// Identity is never inferred from content
// ---------------------------------------------------------------------------------------

/// A6: identical content at different paths is NOT the same object.
#[test]
fn identical_content_at_different_paths_is_not_identity() {
    let prev = set(vec![mk("a.txt", 1, 10, Some(7))], true);
    let cur = set(
        vec![mk("a.txt", 1, 10, Some(7)), mk("copy.txt", 1, 11, Some(7))],
        true,
    );
    let r = reconcile(&prev, &cur);

    assert_eq!(kinds(&r, MutationKind::Unchanged).len(), 1);
    assert_eq!(kinds(&r, MutationKind::Created).len(), 1);
    let created = kinds(&r, MutationKind::Created)[0];
    assert_eq!(created.path, PathBuf::from("copy.txt"));
    // The duplicate content must not have been read as a rename or as the same object.
    assert_eq!(kinds(&r, MutationKind::RenamedOrMoved).len(), 0);
    assert_eq!(kinds(&r, MutationKind::Ambiguous).len(), 0);
}

// ---------------------------------------------------------------------------------------
// Inode reuse. This is the F-6 scenario, expressed so that it holds on any filesystem.
// ---------------------------------------------------------------------------------------

/// Same path, different `dev`+`ino`, DIFFERENT content → the content separates the sides.
#[test]
fn same_path_new_identity_with_different_content_is_recreated() {
    let prev = set(vec![mk("bye.txt", 1, 10, Some(1))], true);
    let cur = set(vec![mk("bye.txt", 1, 99, Some(2))], true);
    let r = reconcile(&prev, &cur);

    assert_eq!(kinds(&r, MutationKind::Recreated).len(), 1);
    assert_eq!(r.count(MutationKind::Deleted), 0);
    assert_eq!(r.count(MutationKind::Created), 0);
    assert_eq!(r.count(MutationKind::Unchanged), 0);
}

/// Same path, different `dev`+`ino`, IDENTICAL content → ambiguous. Duplicate content is not
/// identity, and the evidence does not say what happened.
#[test]
fn same_path_new_identity_with_identical_content_is_ambiguous() {
    let prev = set(vec![mk("bye.txt", 1, 10, Some(5))], true);
    let cur = set(vec![mk("bye.txt", 1, 99, Some(5))], true);
    let r = reconcile(&prev, &cur);

    let amb = kinds(&r, MutationKind::Ambiguous);
    assert_eq!(amb.len(), 1);
    assert_eq!(
        amb[0].evidence.reason,
        Some(umbral::identity::AmbiguityReason::DuplicateContentNotIdentity)
    );
    assert_eq!(r.count(MutationKind::Deleted), 0);
    assert_eq!(r.count(MutationKind::Created), 0);
}

/// Same path, different `dev`+`ino`, no content evidence at all → ambiguous, with the
/// reason saying the evidence is missing.
#[test]
fn same_path_new_identity_without_content_evidence_is_ambiguous() {
    let prev = set(vec![mk("x.txt", 1, 10, None)], true);
    let cur = set(vec![mk("x.txt", 1, 99, None)], true);
    let r = reconcile(&prev, &cur);

    let amb = kinds(&r, MutationKind::Ambiguous);
    assert_eq!(amb.len(), 1);
    assert_eq!(
        amb[0].evidence.reason,
        Some(umbral::identity::AmbiguityReason::NoContentEvidence)
    );
}

/// The F-6 property, stated so that it holds on every filesystem: a path that disappeared is
/// accounted for EXACTLY ONCE — as a `Deleted`, or as a rename away from that path — and
/// never as both, and never as neither.
///
/// This is the property the original V0 assertion lacked: it required one specific reading
/// and therefore only held where the filesystem did not reuse inodes.
#[test]
fn a_vanished_path_is_accounted_for_exactly_once() {
    // Reading A: the new file did NOT reuse the freed identity.
    let prev = set(vec![mk("bye.txt", 1, 10, Some(1))], true);
    let cur_no_reuse = set(vec![mk("new.txt", 1, 11, Some(2))], true);
    let r = reconcile(&prev, &cur_no_reuse);

    let deleted = r
        .mutations
        .iter()
        .filter(|m| m.kind == MutationKind::Deleted && m.path.as_path() == Path::new("bye.txt"))
        .count();
    let renamed_away = r
        .mutations
        .iter()
        .filter(|m| {
            m.kind == MutationKind::RenamedOrMoved
                && m.old_path.as_deref() == Some(std::path::Path::new("bye.txt"))
        })
        .count();
    assert_eq!(
        deleted + renamed_away,
        1,
        "bye.txt must be accounted for exactly once"
    );
    assert_eq!(deleted, 1);
    assert_eq!(renamed_away, 0);

    // Reading B: the new file REUSED the freed identity.
    let cur_reuse = set(vec![mk("new.txt", 1, 10, Some(2))], true);
    let r2 = reconcile(&prev, &cur_reuse);

    let deleted2 = r2
        .mutations
        .iter()
        .filter(|m| m.kind == MutationKind::Deleted && m.path.as_path() == Path::new("bye.txt"))
        .count();
    let renamed_away2 = r2
        .mutations
        .iter()
        .filter(|m| {
            m.kind == MutationKind::RenamedOrMoved
                && m.old_path.as_deref() == Some(std::path::Path::new("bye.txt"))
        })
        .count();
    assert_eq!(
        deleted2 + renamed_away2,
        1,
        "bye.txt must be accounted for exactly once"
    );
    assert_eq!(deleted2, 0);
    assert_eq!(renamed_away2, 1);

    // And in both readings the new path is accounted for exactly once.
    for r in [&r, &r2] {
        let seen = r
            .mutations
            .iter()
            .filter(|m| m.path.as_path() == Path::new("new.txt"))
            .count();
        assert_eq!(seen, 1);
    }
}

// ---------------------------------------------------------------------------------------
// Incompleteness
// ---------------------------------------------------------------------------------------

/// "Not seen" is not "deleted".
#[test]
fn an_incomplete_scan_never_produces_deleted() {
    let prev = set(
        vec![mk("a.txt", 1, 10, Some(1)), mk("b.txt", 1, 11, Some(2))],
        true,
    );
    let cur = set(vec![mk("a.txt", 1, 10, Some(1))], false);
    let r = reconcile(&prev, &cur);

    assert_eq!(r.count(MutationKind::Deleted), 0);
    assert_eq!(r.count(MutationKind::Unobserved), 1);
    assert!(!r.complete);
    let un = kinds(&r, MutationKind::Unobserved)[0];
    assert_eq!(un.path, PathBuf::from("b.txt"));
    assert!(!un.evidence.complete_scan);
}

#[test]
fn a_complete_scan_does_produce_deleted() {
    let prev = set(
        vec![mk("a.txt", 1, 10, Some(1)), mk("b.txt", 1, 11, Some(2))],
        true,
    );
    let cur = set(vec![mk("a.txt", 1, 10, Some(1))], true);
    let r = reconcile(&prev, &cur);

    assert_eq!(r.count(MutationKind::Deleted), 1);
    assert_eq!(r.count(MutationKind::Unobserved), 0);
}

// ---------------------------------------------------------------------------------------
// Renames, swaps, hard links
// ---------------------------------------------------------------------------------------

#[test]
fn a_rename_is_reported_with_its_previous_path() {
    let prev = set(vec![mk("old.txt", 1, 10, Some(1))], true);
    let cur = set(vec![mk("new.txt", 1, 10, Some(1))], true);
    let r = reconcile(&prev, &cur);

    assert_eq!(r.count(MutationKind::RenamedOrMoved), 1);
    let m = kinds(&r, MutationKind::RenamedOrMoved)[0];
    assert_eq!(m.path, PathBuf::from("new.txt"));
    assert_eq!(m.old_path, Some(PathBuf::from("old.txt")));
    assert_eq!(r.count(MutationKind::Deleted), 0);
    assert_eq!(r.count(MutationKind::Created), 0);
}

/// Swapping two paths is two renames, not two modifications.
#[test]
fn a_path_swap_is_two_renames() {
    let prev = set(
        vec![mk("a.txt", 1, 10, Some(1)), mk("b.txt", 1, 11, Some(2))],
        true,
    );
    let cur = set(
        vec![mk("a.txt", 1, 11, Some(2)), mk("b.txt", 1, 10, Some(1))],
        true,
    );
    let r = reconcile(&prev, &cur);

    assert_eq!(r.count(MutationKind::RenamedOrMoved), 2);
    assert_eq!(r.count(MutationKind::Modified), 0);
    assert_eq!(r.count(MutationKind::Unchanged), 0);
    // D-V01-6: counting the renames is not enough. The same-path pairs that phase 2 already
    // explained must not be classified again, so the swap is exactly two verdicts.
    assert_eq!(r.count(MutationKind::Recreated), 0);
    assert_eq!(r.count(MutationKind::Ambiguous), 0);
    assert_eq!(r.mutations.len(), 2, "got: {:?}", r.mutations);
}

/// D-V01-6, partial case: `a` moved to `c` and a new object took `a`. The previous `a` is
/// explained by the rename, and the new `a` is still classified against it — once.
#[test]
fn a_path_vacated_by_a_rename_and_refilled_is_classified_once() {
    let prev = set(vec![mk("a.txt", 1, 10, Some(1))], true);
    let cur = set(
        vec![mk("a.txt", 1, 11, Some(2)), mk("c.txt", 1, 10, Some(1))],
        true,
    );
    let r = reconcile(&prev, &cur);

    assert_eq!(r.count(MutationKind::RenamedOrMoved), 1);
    assert_eq!(r.count(MutationKind::Recreated), 1);
    assert_eq!(r.mutations.len(), 2, "got: {:?}", r.mutations);
}

/// Two hard links to one object: one physical id, two entries. When the pairing is
/// ambiguous, it is reported as ambiguous rather than resolved by picking one.
#[test]
fn several_candidates_for_one_identity_are_ambiguous() {
    let prev = set(
        vec![mk("l1", 1, 10, Some(1)), mk("l2", 1, 10, Some(1))],
        true,
    );
    let cur = set(
        vec![mk("m1", 1, 10, Some(1)), mk("m2", 1, 10, Some(1))],
        true,
    );
    let r = reconcile(&prev, &cur);

    // D-V01-8: every path in the conflicting group is reported, the previous-side ones
    // included. Before the fix `l1` and `l2` received no verdict at all.
    assert_eq!(
        r.count(MutationKind::Ambiguous),
        4,
        "got: {:?}",
        r.mutations
    );
    assert_eq!(r.count(MutationKind::RenamedOrMoved), 0);
    assert_eq!(r.count(MutationKind::Deleted), 0);
    for m in kinds(&r, MutationKind::Ambiguous) {
        assert_eq!(
            m.evidence.reason,
            Some(umbral::identity::AmbiguityReason::ConflictingCandidates)
        );
    }
    let paths: Vec<_> = r.mutations.iter().map(|m| m.path.clone()).collect();
    for p in ["l1", "l2", "m1", "m2"] {
        assert!(
            paths.contains(&PathBuf::from(p)),
            "{p} not reported: {paths:?}"
        );
    }
}

/// D-V01-8: a previous-side path of a conflicting group that is still present in the
/// current set is classified by its own same-path verdict, not reported a second time.
#[test]
fn a_conflicting_previous_path_still_present_is_reported_once() {
    let prev = set(
        vec![mk("l1", 1, 10, Some(1)), mk("l2", 1, 10, Some(1))],
        true,
    );
    let cur = set(
        vec![
            mk("l1", 1, 99, Some(7)),
            mk("m1", 1, 10, Some(1)),
            mk("m2", 1, 10, Some(1)),
        ],
        true,
    );
    let r = reconcile(&prev, &cur);

    let l1: Vec<_> = r
        .mutations
        .iter()
        .filter(|m| m.path.as_path() == std::path::Path::new("l1"))
        .collect();
    assert_eq!(l1.len(), 1, "got: {:?}", r.mutations);
    assert_eq!(l1[0].kind, MutationKind::Recreated);
    assert_eq!(r.mutations.len(), 4, "got: {:?}", r.mutations);
}

/// Deleting one of two hard links leaves the object alive, and that is reported.
#[test]
fn deleting_one_hard_link_reports_that_the_object_survives() {
    let prev = set(
        vec![mk("l1", 1, 10, Some(1)), mk("l2", 1, 10, Some(1))],
        true,
    );
    let cur = set(vec![mk("l1", 1, 10, Some(1))], true);
    let r = reconcile(&prev, &cur);

    let deleted = kinds(&r, MutationKind::Deleted);
    assert_eq!(deleted.len(), 1);
    assert_eq!(deleted[0].path, PathBuf::from("l2"));
    assert_eq!(deleted[0].evidence.object_survives, Some(true));
}

// ---------------------------------------------------------------------------------------
// Determinism and idempotence
// ---------------------------------------------------------------------------------------

#[test]
fn reconciling_a_set_with_itself_yields_only_unchanged() {
    let s = set(
        vec![mk("a.txt", 1, 10, Some(1)), mk("b.txt", 1, 11, Some(2))],
        true,
    );
    let r = reconcile(&s, &s);
    assert_eq!(r.count(MutationKind::Unchanged), 2);
    assert_eq!(r.mutations.len(), 2);
}

#[test]
fn reconcile_is_deterministic_regardless_of_input_order() {
    let a1 = mk("a.txt", 1, 10, Some(1));
    let a2 = mk("b.txt", 1, 11, Some(2));
    let b1 = mk("a.txt", 1, 10, Some(1));
    let b2 = mk("c.txt", 1, 12, Some(3));

    let r1 = reconcile(
        &set(vec![a1.clone(), a2.clone()], true),
        &set(vec![b1.clone(), b2.clone()], true),
    );
    let r2 = reconcile(
        &set(vec![a2.clone(), a1.clone()], true),
        &set(vec![b2.clone(), b1.clone()], true),
    );
    assert_eq!(r1.mutations, r2.mutations);
}

// ---------------------------------------------------------------------------------------
// Property: every path on both sides is accounted for exactly once.
// ---------------------------------------------------------------------------------------

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    /// With unique physical ids (no hard links), every previous path is consumed by exactly
    /// one mutation and every current path is named by exactly one current-side mutation.
    ///
    /// Current objects either keep their own path's identity, take a fresh one, or take the
    /// identity of another previous path (`perm`) — which generates renames, swaps and
    /// renames over an existing path. Before D-V01-6 the generator produced no renames at
    /// all, which is why a path swap classified twice went unnoticed.
    #[test]
    fn every_path_is_accounted_for_exactly_once(
        (spec, perm, fresh) in (1usize..8).prop_flat_map(|n| (
            prop::collection::vec((any::<bool>(), any::<bool>(), 0u8..4u8), n),
            Just((0..n).collect::<Vec<usize>>()).prop_shuffle(),
            prop::collection::vec(any::<bool>(), n),
        ))
    ) {
        let mut prev_paths = Vec::new();
        let mut cur_paths = Vec::new();
        for (i, (in_prev, in_cur, h)) in spec.iter().enumerate() {
            let name = format!("p{i}.txt");
            let prev_ino = 100 + i as u64;
            // `perm` is a permutation, so current identities stay unique.
            let cur_ino = if fresh[i] { 500 + i as u64 } else { 100 + perm[i] as u64 };
            if *in_prev {
                prev_paths.push(mk(&name, 1, prev_ino, Some(*h)));
            }
            if *in_cur {
                cur_paths.push(mk(&name, 1, cur_ino, Some(*h)));
            }
        }
        let prev_names: Vec<PathBuf> = prev_paths.iter().map(|p| p.path.clone()).collect();
        let cur_names: Vec<PathBuf> = cur_paths.iter().map(|p| p.path.clone()).collect();

        let r = reconcile(&set(prev_paths, true), &set(cur_paths, true));

        // P1: every previous path is accounted for, at most once as the source of a rename and
        // at most once by a same-path mutation. Both happen together in exactly one case: the
        // path's object moved away AND a new object took the path. Then the rename describes
        // the object and the same-path verdict describes the path — the frozen V0
        // experiment's rule, fixed by UD-029. Nothing else may account for a path twice.
        for p in &prev_names {
            let as_source = r.mutations.iter()
                .filter(|m| m.old_path.as_ref() == Some(p))
                .count();
            let same_path = r.mutations.iter()
                .filter(|m| m.old_path.is_none() && &m.path == p)
                .count();
            prop_assert!(as_source + same_path >= 1, "previous path {} not accounted for", p.display());
            prop_assert!(as_source <= 1, "previous path {} is the source of {} renames", p.display(), as_source);
            prop_assert!(same_path <= 1, "previous path {} has {} same-path verdicts", p.display(), same_path);
            if as_source == 1 && same_path == 1 {
                prop_assert!(
                    cur_names.contains(p),
                    "previous path {} renamed away and also given a same-path verdict, but no current entry holds it",
                    p.display()
                );
            }
        }

        // P2: every current path is named by exactly one current-side mutation. `deleted`
        // and `unobserved` are claims about the previous side: when an object is renamed
        // over an existing path, the object that was there is reported `deleted` under the
        // same path (V0's behaviour, kept by UD-029; OPEN-QUESTIONS Q26).
        for c in &cur_names {
            let n = r.mutations.iter()
                .filter(|m| m.path.as_path() == c.as_path())
                .filter(|m| !matches!(m.kind, MutationKind::Deleted | MutationKind::Unobserved))
                .count();
            prop_assert_eq!(n, 1, "current path {} named {} times", c.display(), n);
        }
    }

    /// No verdict may claim identity that the evidence does not support: a `RenamedOrMoved`
    /// requires both sides to carry the same physical id.
    #[test]
    fn renames_always_carry_matching_physical_evidence(
        spec in prop::collection::vec((0u8..3u8, 0u8..3u8), 1..6)
    ) {
        let mut prev_paths = Vec::new();
        let mut cur_paths = Vec::new();
        for (i, (a, b)) in spec.iter().enumerate() {
            let ino = 100 + i as u64;
            if *a == 1 { prev_paths.push(mk(&format!("old{i}"), 1, ino, Some(1))); }
            if *b == 1 { cur_paths.push(mk(&format!("new{i}"), 1, ino, Some(1))); }
        }
        let r = reconcile(&set(prev_paths, true), &set(cur_paths, true));
        for m in &r.mutations {
            if m.kind == MutationKind::RenamedOrMoved {
                prop_assert!(m.evidence.physical_identity.is_some());
                prop_assert_eq!(m.evidence.physical_identity, m.evidence.old_physical_identity);
                prop_assert!(m.old_path.is_some());
            }
        }
    }
}

/// D-V01-9: the reference's completeness travels with every verdict.
#[test]
fn every_verdict_carries_the_completeness_of_its_reference() {
    let prev = set(vec![mk("a.txt", 1, 10, Some(1))], false);
    let cur = set(
        vec![mk("a.txt", 1, 10, Some(1)), mk("b.txt", 1, 11, Some(2))],
        true,
    );
    let r = reconcile(&prev, &cur);
    let created = kinds(&r, MutationKind::Created);
    assert_eq!(created.len(), 1);
    assert!(!created[0].evidence.reference_complete);
    assert!(r.mutations.iter().all(|m| !m.evidence.reference_complete));

    let r = reconcile(&set(prev.paths.clone(), true), &cur);
    assert!(r.mutations.iter().all(|m| m.evidence.reference_complete));
}

/// D-V01-17 (E-TD-2/E-TD-3, `experiments/e-td-2-3/README.md` §5.3). Two valid readings of the
/// same object that differ are a content change, whatever the metadata says: a rewrite that
/// keeps size and mtime, or a delete-and-recreate that keeps the inode (1000/1000 on ext4).
/// Before the fix the verdict was `unchanged`, while the log held two different hashes.
#[test]
fn differing_valid_hashes_are_a_modification_even_with_equal_metadata() {
    let r = reconcile(
        &set(vec![mk("a.txt", 1, 10, Some(1))], true),
        &set(vec![mk("a.txt", 1, 10, Some(2))], true),
    );
    let m = kinds(&r, MutationKind::Modified);
    assert_eq!(m.len(), 1, "{:#?}", r.mutations);
    assert_eq!(m[0].evidence.content_changed, Some(true));
    assert!(kinds(&r, MutationKind::Unchanged).is_empty());

    // Equal valid readings with equal metadata: compared, and stated as compared.
    let r = reconcile(
        &set(vec![mk("a.txt", 1, 10, Some(1))], true),
        &set(vec![mk("a.txt", 1, 10, Some(1))], true),
    );
    let u = kinds(&r, MutationKind::Unchanged);
    assert_eq!(u[0].evidence.content_changed, Some(false));

    // No comparable reading: metadata alone, and nothing is claimed about the content.
    let r = reconcile(
        &set(vec![mk("a.txt", 1, 10, Some(1))], true),
        &set(vec![mk("a.txt", 1, 10, None)], true),
    );
    let u = kinds(&r, MutationKind::Unchanged);
    assert_eq!(u[0].evidence.content_changed, None);
}
