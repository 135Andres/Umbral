//! Increment 2 — IDENTITY as physical evidence.
//!
//! Physical identity ≠ semantic identity. `dev+ino` names a filesystem
//! object on a given system at a given time; it is NOT "what this project
//! thing is", it is NOT eternal (inodes are reused after delete), and it is
//! NOT portable (non-Unix yields no dev at all). Everything here is
//! EVIDENCE for reconciliation — insufficient evidence must produce
//! `Ambiguous`, never an invented continuity (INV-8).

use crate::Entry;
use std::collections::HashMap;

/// A physical object as seen in one observation of one entry.
/// `None` = the evidence was not obtainable — treated as missing, never as 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PhysicalId {
    pub dev: Option<u64>,
    pub ino: Option<u64>,
}

impl Entry {
    /// The physical identity of THIS entry (the link itself for symlinks —
    /// the target's identity is a different object and is not observed).
    /// Hard links share one PhysicalId by design: one filesystem object,
    /// several directory entries.
    pub fn physical_id(&self) -> PhysicalId {
        PhysicalId {
            dev: self.dev,
            ino: self.ino,
        }
    }

    /// True when we hold usable physical identity evidence for this entry.
    /// On platforms/filesystems where dev+ino are unavailable, comparison
    /// falls back to weaker evidence and must stay conservative.
    pub fn has_physical_evidence(&self) -> bool {
        self.ino.is_some() && self.dev.is_some()
    }
}

/// Result of comparing one previous entry against one current entry.
///
/// Comparison-level states (rename/modified are properties of a PAIR of
/// observations, never of a single entry). `matches` names the evidence that
/// justifies the verdict so the reconciler can audit WHY.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdentityMatch {
    /// Same physical object, state untouched (dev+ino equal, size+mtime equal).
    SamePhysicalObjectUnchanged,
    /// Same physical object, state changed (dev+ino equal, size/mtime moved).
    SamePhysicalObjectModified,
    /// Same physical object at a DIFFERENT path (rename/move within the same
    /// filesystem): dev+ino equal, path differs.
    RenamedOrMoved,
    /// The two observations carry the same dev+ino. This is a property of
    /// hard links: two directory entries, one filesystem object. Not a claim
    /// about project semantics.
    SameObjectViaHardLink,
    /// The previous object is gone and nothing at that path: deleted.
    Deleted,
    /// Physical evidence contradicts or is insufficient:
    /// - same path but different dev+ino (delete+recreate, or inode reuse);
    /// - missing dev/ino on either side.
    ///
    /// Path alone NEVER produces a positive identity.
    Ambiguous { reason: AmbiguityReason },
    /// No previous observation at this path.
    NewObject,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AmbiguityReason {
    /// Same path, different dev+ino: could be delete+recreate OR inode reuse.
    IdentityChangedAtSamePath,
    /// Physical identity evidence is absent on one or both sides.
    MissingEvidence,
}

/// Compare one previous entry with one current entry at the SAME path.
/// Path is context, never evidence of identity.
pub fn compare_same_path(previous: &Entry, current: &Entry) -> IdentityMatch {
    match (
        previous.has_physical_evidence(),
        current.has_physical_evidence(),
    ) {
        (true, true) => {
            if previous.ino == current.ino && previous.dev == current.dev {
                if previous.size == current.size && previous.mtime == current.mtime {
                    IdentityMatch::SamePhysicalObjectUnchanged
                } else {
                    IdentityMatch::SamePhysicalObjectModified
                }
            } else {
                IdentityMatch::Ambiguous {
                    reason: AmbiguityReason::IdentityChangedAtSamePath,
                }
            }
        }
        _ => IdentityMatch::Ambiguous {
            reason: AmbiguityReason::MissingEvidence,
        },
    }
}

/// Cross-path identity: does the object previously at `old` still exist at
/// `new`? Renames preserve dev+ino on Linux (rename(2) never changes the
/// inode); this is observed behaviour AND man-page-documented, but the
/// guarantee is filesystem-family-local, not universal — callers must treat
/// a negative here as Ambiguous/absent, never as proof of a new object.
pub fn compare_cross_path(old: &Entry, new: &Entry) -> Option<IdentityMatch> {
    if !old.has_physical_evidence() || !new.has_physical_evidence() {
        return Some(IdentityMatch::Ambiguous {
            reason: AmbiguityReason::MissingEvidence,
        });
    }
    if old.ino == new.ino && old.dev == new.dev {
        if old.size == new.size && old.mtime == new.mtime {
            Some(IdentityMatch::RenamedOrMoved)
        } else {
            // same object, moved AND modified between observations
            Some(IdentityMatch::SamePhysicalObjectModified)
        }
    } else {
        None
    }
}

/// Index a scan's entries by physical identity (for cross-path matching,
/// e.g. detecting renames without content hashing). Multiple entries may
/// share one identity — that is the hard-link case, kept explicitly.
pub fn index_by_physical_id(entries: &[Entry]) -> HashMap<PhysicalId, Vec<&Entry>> {
    let mut map: HashMap<PhysicalId, Vec<&Entry>> = HashMap::new();
    for e in entries {
        map.entry(e.physical_id()).or_default().push(e);
    }
    map
}
