//! Physical identity evidence.
//!
//! `dev`+`ino` is **evidence about a filesystem object**, never semantic identity. It is
//! not eternal: inode numbers are reused after deletion, and this module is written so
//! that no caller can accidentally treat a matching pair as proof of "the same file".
//!
//! Two rules this module exists to enforce:
//!
//! 1. A path alone never yields a positive identity.
//! 2. Missing evidence is never identity — it produces [`AmbiguityReason::MissingPhysicalEvidence`].
//!
//! Hard links share one [`PhysicalId`] (one object, several directory entries). A symlink's
//! identity is the link itself, never its target.

/// A physical filesystem object, as observed. Evidence, not identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct PhysicalId {
    pub dev: u64,
    pub ino: u64,
}

impl PhysicalId {
    /// Build from raw observed parts. `None` when either part was not obtainable —
    /// absence is recorded as absence, never filled in.
    pub fn from_parts(dev: Option<u64>, ino: Option<u64>) -> Option<Self> {
        match (dev, ino) {
            (Some(dev), Some(ino)) => Some(PhysicalId { dev, ino }),
            _ => None,
        }
    }
}

/// Why a comparison could not produce a positive verdict.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AmbiguityReason {
    /// One or both sides lacked `dev`/`ino`. No identity can be claimed.
    MissingPhysicalEvidence,
    /// Same path, different `dev`+`ino`. The evidence does not say whether this is a
    /// recreation, a replacement, or inode reuse — so it does not say anything.
    IdentityChangedAtSamePath,
    /// Several objects share one physical id in a way that admits more than one pairing.
    /// Never resolved by picking one.
    ConflictingCandidates,
    /// Content is identical, so content cannot separate the two sides. Duplicate content
    /// is not identity.
    DuplicateContentNotIdentity,
    /// No valid content evidence exists to separate the two sides.
    NoContentEvidence,
}

impl AmbiguityReason {
    pub fn as_str(self) -> &'static str {
        match self {
            AmbiguityReason::MissingPhysicalEvidence => "MissingPhysicalEvidence",
            AmbiguityReason::IdentityChangedAtSamePath => "IdentityChangedAtSamePath",
            AmbiguityReason::ConflictingCandidates => "ConflictingCandidates",
            AmbiguityReason::DuplicateContentNotIdentity => "DuplicateContentNotIdentity",
            AmbiguityReason::NoContentEvidence => "NoContentEvidence",
        }
    }
}

/// Verdict of comparing the physical evidence of two observations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IdentityMatch {
    /// Both sides carry the same `dev`+`ino`. The same filesystem object is the best
    /// available reading; it is still not a claim about meaning.
    SameObject,
    /// The evidence is insufficient or in conflict. A first-class outcome, not an error.
    Ambiguous(AmbiguityReason),
}

/// Compare two observations of the same path.
///
/// Takes already-derived [`PhysicalId`] evidence, so that a caller cannot accidentally pass
/// raw integers where an absence is meaningful.
pub fn compare_same_path(prev: Option<PhysicalId>, cur: Option<PhysicalId>) -> IdentityMatch {
    match (prev, cur) {
        (Some(a), Some(b)) if a == b => IdentityMatch::SameObject,
        (Some(_), Some(_)) => IdentityMatch::Ambiguous(AmbiguityReason::IdentityChangedAtSamePath),
        _ => IdentityMatch::Ambiguous(AmbiguityReason::MissingPhysicalEvidence),
    }
}

/// Compare two observations of *different* paths. Returns `None` when the evidence does not
/// link them — which is not a verdict, just the absence of a candidate.
pub fn compare_cross_path(
    old: Option<PhysicalId>,
    new: Option<PhysicalId>,
) -> Option<IdentityMatch> {
    match (old, new) {
        (Some(a), Some(b)) if a == b => Some(IdentityMatch::SameObject),
        _ => None,
    }
}

// ---------------------------------------------------------------------------------------
// Metadata helpers. Kept here because they are the only place that reads platform
// metadata layout; every other module consumes the resulting Option values.
// ---------------------------------------------------------------------------------------

/// `dev`+`ino` of an observed metadata record, or `None` where the platform does not
/// expose a stable pair.
#[cfg(unix)]
pub fn physical_id_of(m: &std::fs::Metadata) -> Option<PhysicalId> {
    use std::os::unix::fs::MetadataExt;
    Some(PhysicalId {
        dev: m.dev(),
        ino: m.ino(),
    })
}

#[cfg(not(unix))]
pub fn physical_id_of(_m: &std::fs::Metadata) -> Option<PhysicalId> {
    None
}

/// Modification time as (seconds, nanoseconds) since the Unix epoch, or `None` if the
/// platform cannot supply it. Nanosecond resolution is used where available so that two
/// writes inside one second remain distinguishable.
#[cfg(unix)]
pub fn mtime_of(m: &std::fs::Metadata) -> Option<(i64, u32)> {
    use std::os::unix::fs::MetadataExt;
    Some((m.mtime(), m.mtime_nsec() as u32))
}

#[cfg(not(unix))]
pub fn mtime_of(m: &std::fs::Metadata) -> Option<(i64, u32)> {
    m.modified()
        .ok()
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| (d.as_secs() as i64, d.subsec_nanos()))
}
