//! Pure reconciliation between two observation sets.
//!
//! This function is pure by construction: no syscalls, no directory walking, no database,
//! no clock. It consumes observations and returns classified transitions. That is what
//! makes it testable in every filesystem environment, and what keeps filesystem behaviour
//! out of the classification logic.
//!
//! # Matching rule, in priority order
//!
//! 1. **Same path, same physical identity** → `Unchanged` or `Modified`.
//!    `Modified` is decided by size + nanosecond mtime, refined by content hash: if the
//!    metadata differs but both sides carry a valid hash and the hashes are equal, the
//!    bytes are the same and the verdict is `Unchanged`.
//! 2. **Remaining entries matched globally by physical id** against paths whose object
//!    left them. Exactly one candidate on each side → `RenamedOrMoved`. Several →
//!    `Ambiguous(ConflictingCandidates)`, never an arbitrary pick. This is also what makes
//!    a path swap come out as two renames rather than two modifications.
//! 3. **Same path without shared identity**: content evidence separates `Recreated`
//!    (different content) from `Ambiguous` — identical content is *not* identity, and
//!    neither is missing evidence.
//! 4. **Genuinely new paths** → `Created`.
//! 5. **Vanished previous paths** → `Deleted` only when the current scan is complete;
//!    otherwise `Unobserved`. "Not seen" is not "deleted".
//!
//! # Filesystem independence
//!
//! Nothing here asks what a filesystem does with inode numbers. Phase 3 exists precisely
//! because a freed `dev`+`ino` may be handed to a new file: when it is, the same-path pair
//! carries no usable identity evidence and the outcome is `Ambiguous` (or `Recreated` when
//! content separates the sides) — never a fabricated `Deleted` + `Created` pair, and never
//! a fabricated `Unchanged`.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::content::HASH_LEN;
use crate::identity::{AmbiguityReason, IdentityMatch, PhysicalId};
use crate::scan::EntryKind;

/// One observed path: the observable state plus whatever content evidence exists.
/// `valid_hash` is `None` unless a stable hash was obtained — an unstable or errored read
/// is NO content evidence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservedPath {
    pub path: PathBuf,
    pub kind: EntryKind,
    pub dev: Option<u64>,
    pub ino: Option<u64>,
    pub size: Option<u64>,
    pub mtime: Option<(i64, u32)>,
    pub valid_hash: Option<[u8; HASH_LEN]>,
}

impl ObservedPath {
    pub fn physical_id(&self) -> Option<PhysicalId> {
        PhysicalId::from_parts(self.dev, self.ino)
    }

    pub fn has_physical_evidence(&self) -> bool {
        self.physical_id().is_some()
    }
}

/// One side of a reconciliation. `complete` is the scan's own evidence: it was true only if
/// that scan recorded no per-path errors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservationSet {
    pub paths: Vec<ObservedPath>,
    pub complete: bool,
}

impl ObservationSet {
    pub fn new(paths: Vec<ObservedPath>, complete: bool) -> Self {
        ObservationSet { paths, complete }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MutationKind {
    Unchanged,
    Modified,
    Created,
    Deleted,
    Unobserved,
    RenamedOrMoved,
    Recreated,
    Ambiguous,
}

impl MutationKind {
    pub fn as_str(self) -> &'static str {
        match self {
            MutationKind::Unchanged => "unchanged",
            MutationKind::Modified => "modified",
            MutationKind::Created => "created",
            MutationKind::Deleted => "deleted",
            MutationKind::Unobserved => "unobserved",
            MutationKind::RenamedOrMoved => "renamed-or-moved",
            MutationKind::Recreated => "recreated",
            MutationKind::Ambiguous => "ambiguous",
        }
    }
}

/// Why a verdict was reached. Kept structured so that every classification can be audited
/// rather than trusted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Evidence {
    pub physical_identity: Option<PhysicalId>,
    pub old_physical_identity: Option<PhysicalId>,
    /// `Some(true)` = content differs; `Some(false)` = content compared and equal;
    /// `None` = not compared or not comparable.
    pub content_changed: Option<bool>,
    pub complete_scan: bool,
    /// `Some(true)` = the object still exists elsewhere in the current set (a hard-link
    /// entry vanished but the content did not). Only meaningful for `Deleted`.
    pub object_survives: Option<bool>,
    pub reason: Option<AmbiguityReason>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mutation {
    pub kind: MutationKind,
    pub path: PathBuf,
    /// The previous path, when the verdict relates a path to another one.
    pub old_path: Option<PathBuf>,
    pub evidence: Evidence,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reconciliation {
    /// Sorted by (path, old_path) — deterministic regardless of input order.
    pub mutations: Vec<Mutation>,
    pub complete: bool,
}

enum SamePath {
    Same,
    Ambiguous(AmbiguityReason),
}

fn same_path_identity(prev: &ObservedPath, cur: &ObservedPath) -> SamePath {
    match crate::identity::compare_same_path(prev.physical_id(), cur.physical_id()) {
        IdentityMatch::SameObject => SamePath::Same,
        IdentityMatch::Ambiguous(r) => SamePath::Ambiguous(r),
    }
}

/// Whether the observable state is equal, given the same physical identity.
/// For regular files this means size + mtime; for other kinds the metadata that changes
/// with content is not meaningful, so identity plus kind is the whole statement.
fn same_observable(prev: &ObservedPath, cur: &ObservedPath) -> bool {
    if prev.kind != cur.kind {
        return false;
    }
    match prev.kind {
        EntryKind::File => prev.size == cur.size && prev.mtime == cur.mtime,
        _ => true,
    }
}

fn hashes_equal(prev: &ObservedPath, cur: &ObservedPath) -> bool {
    match (prev.valid_hash, cur.valid_hash) {
        (Some(a), Some(b)) => a == b,
        _ => false,
    }
}

fn hashes_differ(prev: &ObservedPath, cur: &ObservedPath) -> bool {
    match (prev.valid_hash, cur.valid_hash) {
        (Some(a), Some(b)) => a != b,
        _ => false,
    }
}

/// Classify the transitions between two observation sets.
pub fn reconcile(previous: &ObservationSet, current: &ObservationSet) -> Reconciliation {
    let mut mutations: Vec<Mutation> = Vec::new();
    let mut prev_matched = vec![false; previous.paths.len()];
    let mut cur_matched = vec![false; current.paths.len()];

    let mut cur_by_path: BTreeMap<&Path, usize> = BTreeMap::new();
    for (i, p) in current.paths.iter().enumerate() {
        cur_by_path.insert(p.path.as_path(), i);
    }

    // ---- Phase 1: same path, shared physical identity -------------------------------
    let mut deferred: Vec<(usize, usize)> = Vec::new();
    for (pi, prev) in previous.paths.iter().enumerate() {
        if let Some(&ci) = cur_by_path.get(prev.path.as_path()) {
            let cur = &current.paths[ci];
            match same_path_identity(prev, cur) {
                SamePath::Same => {
                    prev_matched[pi] = true;
                    cur_matched[ci] = true;

                    let (kind, content_changed) = if same_observable(prev, cur) {
                        (MutationKind::Unchanged, None)
                    } else if hashes_equal(prev, cur) {
                        // Metadata moved, bytes did not.
                        (MutationKind::Unchanged, Some(false))
                    } else {
                        (
                            MutationKind::Modified,
                            hashes_differ(prev, cur).then_some(true),
                        )
                    };

                    mutations.push(Mutation {
                        kind,
                        path: prev.path.clone(),
                        old_path: None,
                        evidence: Evidence {
                            physical_identity: cur.physical_id(),
                            old_physical_identity: prev.physical_id(),
                            content_changed,
                            complete_scan: current.complete,
                            object_survives: None,
                            reason: None,
                        },
                    });
                }
                // The reason is re-derived in phase 3 from the content evidence, so it is
                // not carried across phases.
                SamePath::Ambiguous(_reason) => deferred.push((pi, ci)),
            }
        }
    }

    // ---- Phase 2: global physical matching of the leftovers -------------------------
    let mut prev_by_pid: BTreeMap<PhysicalId, Vec<usize>> = BTreeMap::new();
    for (i, p) in previous.paths.iter().enumerate() {
        if !prev_matched[i] {
            if let Some(id) = p.physical_id() {
                prev_by_pid.entry(id).or_default().push(i);
            }
        }
    }
    let mut cur_by_pid: BTreeMap<PhysicalId, Vec<usize>> = BTreeMap::new();
    for (i, p) in current.paths.iter().enumerate() {
        if !cur_matched[i] {
            if let Some(id) = p.physical_id() {
                cur_by_pid.entry(id).or_default().push(i);
            }
        }
    }

    for (pid, prevs) in prev_by_pid.iter() {
        let Some(curs) = cur_by_pid.get(pid) else {
            continue;
        };
        if prevs.len() == 1 && curs.len() == 1 {
            let (pi, ci) = (prevs[0], curs[0]);
            prev_matched[pi] = true;
            cur_matched[ci] = true;
            mutations.push(Mutation {
                kind: MutationKind::RenamedOrMoved,
                path: current.paths[ci].path.clone(),
                old_path: Some(previous.paths[pi].path.clone()),
                evidence: Evidence {
                    physical_identity: Some(*pid),
                    old_physical_identity: Some(*pid),
                    content_changed: None,
                    complete_scan: current.complete,
                    object_survives: None,
                    reason: None,
                },
            });
        } else {
            // More than one candidate on at least one side. Never pick one.
            for &pi in prevs.iter() {
                prev_matched[pi] = true;
            }
            for &ci in curs.iter() {
                cur_matched[ci] = true;
                mutations.push(Mutation {
                    kind: MutationKind::Ambiguous,
                    path: current.paths[ci].path.clone(),
                    old_path: None,
                    evidence: Evidence {
                        physical_identity: Some(*pid),
                        old_physical_identity: Some(*pid),
                        content_changed: None,
                        complete_scan: current.complete,
                        object_survives: None,
                        reason: Some(AmbiguityReason::ConflictingCandidates),
                    },
                });
            }
        }
    }

    // ---- Phase 3: same path without shared identity ---------------------------------
    // A pair is classified here only when its current entry was not already explained by
    // phase 2. Otherwise a path swap would be reported as two renames AND two same-path
    // verdicts (D-V01-6). This is the frozen V0 experiment's rule. When only the previous
    // entry was consumed by phase 2 (its object moved away and a new one took the path), the
    // current entry is still classified against it here.
    for (pi, ci) in deferred {
        if cur_matched[ci] {
            continue;
        }
        let prev = &previous.paths[pi];
        let cur = &current.paths[ci];
        prev_matched[pi] = true;
        cur_matched[ci] = true;

        let (kind, content_changed, reason) = if hashes_differ(prev, cur) {
            (MutationKind::Recreated, Some(true), None)
        } else if hashes_equal(prev, cur) {
            (
                MutationKind::Ambiguous,
                Some(false),
                Some(AmbiguityReason::DuplicateContentNotIdentity),
            )
        } else {
            (
                MutationKind::Ambiguous,
                None,
                Some(AmbiguityReason::NoContentEvidence),
            )
        };

        mutations.push(Mutation {
            kind,
            path: prev.path.clone(),
            old_path: None,
            evidence: Evidence {
                physical_identity: cur.physical_id(),
                old_physical_identity: prev.physical_id(),
                content_changed,
                complete_scan: current.complete,
                object_survives: None,
                reason,
            },
        });
    }

    // ---- Phase 4: genuinely new paths ------------------------------------------------
    for (ci, cur) in current.paths.iter().enumerate() {
        if !cur_matched[ci] {
            cur_matched[ci] = true;
            mutations.push(Mutation {
                kind: MutationKind::Created,
                path: cur.path.clone(),
                old_path: None,
                evidence: Evidence {
                    physical_identity: cur.physical_id(),
                    old_physical_identity: None,
                    content_changed: None,
                    complete_scan: current.complete,
                    object_survives: None,
                    reason: None,
                },
            });
        }
    }

    // ---- Phase 5: vanished previous paths --------------------------------------------
    // Physical ids still present in the current set: used only to report that a vanished
    // hard-link entry did not take its content with it.
    let surviving_ids: std::collections::BTreeSet<PhysicalId> = current
        .paths
        .iter()
        .filter_map(|p| p.physical_id())
        .collect();

    for (pi, prev) in previous.paths.iter().enumerate() {
        if prev_matched[pi] {
            continue;
        }
        prev_matched[pi] = true;

        // The reason an entry is `Unobserved` rather than `Deleted` is the incompleteness of
        // the scan, which `evidence.complete_scan` already carries. No separate ambiguity
        // reason is invented for it.
        let kind = if current.complete {
            MutationKind::Deleted
        } else {
            MutationKind::Unobserved
        };
        let reason = None;

        let object_survives = match (kind, prev.physical_id()) {
            (MutationKind::Deleted, Some(id)) => Some(surviving_ids.contains(&id)),
            _ => None,
        };

        mutations.push(Mutation {
            kind,
            path: prev.path.clone(),
            old_path: None,
            evidence: Evidence {
                physical_identity: None,
                old_physical_identity: prev.physical_id(),
                content_changed: None,
                complete_scan: current.complete,
                object_survives,
                reason,
            },
        });
    }

    mutations.sort_by(|a, b| {
        a.path
            .cmp(&b.path)
            .then_with(|| a.old_path.cmp(&b.old_path))
            .then_with(|| a.kind.cmp(&b.kind))
    });

    Reconciliation {
        mutations,
        complete: current.complete,
    }
}

impl Reconciliation {
    pub fn count(&self, kind: MutationKind) -> usize {
        self.mutations.iter().filter(|m| m.kind == kind).count()
    }
}
