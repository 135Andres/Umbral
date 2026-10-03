//! Physical identity is evidence, never identity. A6 in the acceptance criteria.

use umbral::identity::{
    compare_cross_path, compare_same_path, AmbiguityReason, IdentityMatch, PhysicalId,
};

fn id(dev: u64, ino: u64) -> Option<PhysicalId> {
    Some(PhysicalId { dev, ino })
}

#[test]
fn same_evidence_is_the_same_object() {
    assert_eq!(
        compare_same_path(id(1, 42), id(1, 42)),
        IdentityMatch::SameObject
    );
}

#[test]
fn changed_identity_at_the_same_path_is_ambiguous_not_a_verdict() {
    assert_eq!(
        compare_same_path(id(1, 42), id(1, 43)),
        IdentityMatch::Ambiguous(AmbiguityReason::IdentityChangedAtSamePath)
    );
}

#[test]
fn missing_evidence_is_ambiguous_not_identity() {
    assert_eq!(
        compare_same_path(None, id(1, 42)),
        IdentityMatch::Ambiguous(AmbiguityReason::MissingPhysicalEvidence)
    );
    assert_eq!(
        compare_same_path(id(1, 42), None),
        IdentityMatch::Ambiguous(AmbiguityReason::MissingPhysicalEvidence)
    );
    assert_eq!(
        compare_same_path(None, None),
        IdentityMatch::Ambiguous(AmbiguityReason::MissingPhysicalEvidence)
    );
}

#[test]
fn cross_path_links_only_on_equal_evidence() {
    assert_eq!(
        compare_cross_path(id(1, 7), id(1, 7)),
        Some(IdentityMatch::SameObject)
    );
    // Different evidence is not a negative verdict, just the absence of a candidate.
    assert_eq!(compare_cross_path(id(1, 7), id(1, 8)), None);
    assert_eq!(compare_cross_path(None, id(1, 7)), None);
    assert_eq!(compare_cross_path(id(1, 7), None), None);
}

#[test]
fn partial_evidence_never_builds_an_id() {
    assert_eq!(PhysicalId::from_parts(Some(1), None), None);
    assert_eq!(PhysicalId::from_parts(None, Some(2)), None);
    assert_eq!(PhysicalId::from_parts(None, None), None);
    assert_eq!(
        PhysicalId::from_parts(Some(1), Some(2)),
        Some(PhysicalId { dev: 1, ino: 2 })
    );
}

/// The reason strings are part of the output contract, so they are pinned. They are kebab-case,
/// like every other token of the output (`UD-031`).
#[test]
fn ambiguity_reasons_have_stable_names() {
    assert_eq!(
        AmbiguityReason::IdentityChangedAtSamePath.as_str(),
        "identity-changed-at-same-path"
    );
    assert_eq!(
        AmbiguityReason::MissingPhysicalEvidence.as_str(),
        "missing-physical-evidence"
    );
    assert_eq!(
        AmbiguityReason::ConflictingCandidates.as_str(),
        "conflicting-candidates"
    );
    assert_eq!(
        AmbiguityReason::DuplicateContentNotIdentity.as_str(),
        "duplicate-content-not-identity"
    );
    assert_eq!(
        AmbiguityReason::NoContentEvidence.as_str(),
        "no-content-evidence"
    );
}
