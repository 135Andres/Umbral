//! Increment 5 — RECONCILE: compare two observation sets, classify mutations.
//!
//! Pure model: no syscalls, no walkdir, no SQLite, no clock. Deterministic
//! and idempotent by construction (sorted inputs, no hidden state).
//!
//! Evidence priority (documented rule):
//!   A. physical identity (dev+ino, both sides)  — strongest
//!   B. content hash (valid/stable only)          — content evidence, NEVER identity
//!   C. path                                      — context only, never evidence
//!   D. nothing sufficient                        -> Ambiguous
//!
//! same hash ≠ same object; same path ≠ same object. Renames are transitions
//! proven by physical identity, never emitted as delete+create pairs.
//!
//! Scan completeness: Deleted is claimed ONLY when the current observation
//! set is complete (no per-path errors). "Did not appear in an incomplete
//! scan" is Unobserved — absence of evidence, not evidence of absence.

use crate::identity::PhysicalId;
use crate::{Entry, EntryKind};
use std::collections::HashMap;
use std::path::PathBuf;

/// One observed path: scanner entry + valid content evidence (if any).
/// `valid_hash` is None unless a Stable BLAKE3 was obtained — an unstable
/// or errored hash is NO content evidence (INV-1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservedPath {
    pub entry: Entry,
    pub valid_hash: Option<[u8; 32]>,
}

impl ObservedPath {
    pub fn physical_id(&self) -> PhysicalId {
        self.entry.physical_id()
    }
    pub fn has_physical_evidence(&self) -> bool {
        self.entry.has_physical_evidence()
    }
}

/// One side of a reconciliation. `complete` = the scan that produced these
/// entries had zero per-path errors (the scanner's own evidence).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObservationSet {
    pub paths: Vec<ObservedPath>,
    pub complete: bool,
}

impl ObservationSet {
    pub fn new(paths: Vec<ObservedPath>, complete: bool) -> ObservationSet {
        let mut paths = paths;
        paths.sort_by(|a, b| a.entry.path.cmp(&b.entry.path));
        ObservationSet { paths, complete }
    }
}

/// Why an Ambiguous verdict was produced.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AmbiguityReason {
    /// same path, different dev+ino, and content evidence cannot separate
    /// recreate from inode-reuse (identical content) — or is absent.
    RecreatedAtSamePath,
    /// no usable physical identity on one or both sides.
    MissingPhysicalEvidence,
    /// more than one equally-plausible match candidate.
    ConflictingCandidates,
}

/// Mutation classification. States (File/Dir/Symlink) live on entries;
/// everything here is a TRANSITION between two observation sets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MutationKind {
    /// same physical object, same observable state (size+mtime equal, and
    /// hashes equal when both exist).
    Unchanged,
    /// same physical object, observable state changed.
    Modified,
    /// new path with no continuity evidence.
    Created,
    /// previous path absent AND current scan complete.
    Deleted,
    /// previous path absent but current scan INCOMPLETE: not seen is not
    /// deleted.
    Unobserved,
    /// same physical object at a different path (proven by dev+ino).
    RenamedOrMoved,
    /// same path, different physical id, different valid content: a new
    /// object replaced the old one (delete+recreate resolved by evidence).
    Recreated,
    /// insufficient or conflicting evidence; the reason is recorded.
    Ambiguous(AmbiguityReason),
}

/// Why the classifier believes this mutation. Kept structured so tests and
/// the future store can audit every verdict.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Evidence {
    /// dev+ino matched across the pair
    pub physical_match: bool,
    /// valid hashes compared; None = not comparable
    pub content_equal: Option<bool>,
    /// size and/or mtime changed on a same-object pair
    pub state_changed: Option<bool>,
    /// for Deleted: the physical object still exists elsewhere (hard links)
    pub object_survives: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mutation {
    pub kind: MutationKind,
    /// current path; for Deleted/Unobserved, the vanished path
    pub path: PathBuf,
    /// for RenamedOrMoved: where the object was
    pub old_path: Option<PathBuf>,
    pub physical_id: Option<PhysicalId>,
    pub evidence: Evidence,
    /// RenamedOrMoved under a renamed directory (informational, V0)
    pub under_dir_rename: bool,
}

/// Result of one reconciliation. Sorted deterministically (path, old_path).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reconciliation {
    pub mutations: Vec<Mutation>,
}

/// reconcile(previous, current) — pure, deterministic, idempotent.
///
/// Phases (documented matching rule):
///   1. same-path pairs with equal physical identity -> Unchanged/Modified;
///   2. remaining entries: global physical matching against "vacated"
///      previous entries (their object left that path). Exactly one
///      candidate -> RenamedOrMoved (this is what makes path swaps two
///      renames); several -> Ambiguous(ConflictingCandidates) — never an
///      arbitrary pick;
///   3. still-unmatched same-path pairs: content evidence separates
///      Recreated (different content) from Ambiguous (identical content or
///      no evidence — duplicate content is not identity);
///   4. genuinely new paths -> Created;
///   5. unmatched previous paths -> Deleted (complete scan) / Unobserved
///      (incomplete scan).
pub fn reconcile(previous: &ObservationSet, current: &ObservationSet) -> Reconciliation {
    let prev_map: HashMap<&PathBuf, &ObservedPath> =
        previous.paths.iter().map(|o| (&o.entry.path, o)).collect();
    let _curr_paths: std::collections::HashSet<&PathBuf> =
        current.paths.iter().map(|o| &o.entry.path).collect();
    let curr_by_pid: HashMap<PhysicalId, usize> = {
        let mut m: HashMap<PhysicalId, usize> = HashMap::new();
        for o in &current.paths {
            if o.has_physical_evidence() {
                *m.entry(o.physical_id()).or_default() += 1;
            }
        }
        m
    };

    let mut mutations: Vec<Mutation> = Vec::new();
    let mut matched_curr: std::collections::HashSet<&PathBuf> = std::collections::HashSet::new();
    let mut matched_prev: std::collections::HashSet<&PathBuf> = std::collections::HashSet::new();

    // ---- PHASE 1: same-path, same physical identity ----
    for curr in &current.paths {
        let prev = match prev_map.get(&curr.entry.path) {
            Some(p) => *p,
            None => continue,
        };
        if !(prev.has_physical_evidence() && curr.has_physical_evidence()) {
            continue;
        }
        if prev.physical_id() != curr.physical_id() {
            continue;
        }
        let state_changed =
            prev.entry.size != curr.entry.size || prev.entry.mtime != curr.entry.mtime;
        let content_equal = content_compare(Some(curr), Some(prev));
        let kind = if !state_changed && content_equal != Some(false) {
            MutationKind::Unchanged
        } else {
            MutationKind::Modified
        };
        mutations.push(Mutation {
            kind,
            path: curr.entry.path.clone(),
            old_path: None,
            physical_id: Some(curr.physical_id()),
            evidence: Evidence {
                physical_match: true,
                content_equal,
                state_changed: Some(state_changed),
                object_survives: None,
            },
            under_dir_rename: false,
        });
        matched_curr.insert(&curr.entry.path);
        matched_prev.insert(&curr.entry.path);
    }

    // ---- PHASE 2: global physical matching over the leftovers ----
    // "vacated": previous paths whose object is no longer there.
    let vacated: Vec<&ObservedPath> = previous
        .paths
        .iter()
        .filter(|p| !matched_prev.contains(&p.entry.path))
        .filter(|p| p.has_physical_evidence())
        .collect();
    for curr in &current.paths {
        if matched_curr.contains(&curr.entry.path) || !curr.has_physical_evidence() {
            continue;
        }
        let candidates: Vec<&ObservedPath> = vacated
            .iter()
            .copied()
            .filter(|p| p.physical_id() == curr.physical_id())
            .collect();
        match candidates.len() {
            1 => {
                let old = candidates[0];
                mutations.push(Mutation {
                    kind: MutationKind::RenamedOrMoved,
                    path: curr.entry.path.clone(),
                    old_path: Some(old.entry.path.clone()),
                    physical_id: Some(curr.physical_id()),
                    evidence: Evidence {
                        physical_match: true,
                        content_equal: content_compare(Some(curr), Some(old)),
                        state_changed: None,
                        object_survives: None,
                    },
                    under_dir_rename: false,
                });
                matched_curr.insert(&curr.entry.path);
                matched_prev.insert(&old.entry.path);
            }
            0 => {}
            _ => {
                mutations.push(Mutation {
                    kind: MutationKind::Ambiguous(AmbiguityReason::ConflictingCandidates),
                    path: curr.entry.path.clone(),
                    old_path: None,
                    physical_id: Some(curr.physical_id()),
                    evidence: Evidence {
                        physical_match: false,
                        content_equal: None,
                        state_changed: None,
                        object_survives: None,
                    },
                    under_dir_rename: false,
                });
                matched_curr.insert(&curr.entry.path);
                // all candidates consumed by the ambiguity — never pick one
                for c in &candidates {
                    matched_prev.insert(&c.entry.path);
                }
            }
        }
    }

    // ---- PHASE 3: same-path pairs without shared identity ----
    for curr in &current.paths {
        if matched_curr.contains(&curr.entry.path) {
            continue;
        }
        let prev = prev_map.get(&curr.entry.path);
        let prev = match prev {
            Some(p) => *p,
            None => continue, // phase 4
        };
        let content_equal = content_compare(Some(curr), Some(prev));
        let (kind, reason) = if prev.has_physical_evidence() && curr.has_physical_evidence() {
            match content_equal {
                Some(false) => (MutationKind::Recreated, None),
                _ => (
                    MutationKind::Ambiguous(AmbiguityReason::RecreatedAtSamePath),
                    Some(AmbiguityReason::RecreatedAtSamePath),
                ),
            }
        } else {
            (
                MutationKind::Ambiguous(AmbiguityReason::MissingPhysicalEvidence),
                Some(AmbiguityReason::MissingPhysicalEvidence),
            )
        };
        mutations.push(Mutation {
            kind,
            path: curr.entry.path.clone(),
            old_path: prev
                .has_physical_evidence()
                .then(|| curr.entry.path.clone()),
            physical_id: curr.has_physical_evidence().then(|| curr.physical_id()),
            evidence: Evidence {
                physical_match: false,
                content_equal,
                state_changed: None,
                object_survives: None,
            },
            under_dir_rename: false,
        });
        let _ = reason;
        matched_curr.insert(&curr.entry.path);
        matched_prev.insert(&curr.entry.path);
    }

    // ---- PHASE 4: new paths ----
    for curr in &current.paths {
        if matched_curr.contains(&curr.entry.path) {
            continue;
        }
        mutations.push(Mutation {
            kind: MutationKind::Created,
            path: curr.entry.path.clone(),
            old_path: None,
            physical_id: curr.has_physical_evidence().then(|| curr.physical_id()),
            evidence: Evidence {
                physical_match: false,
                content_equal: None,
                state_changed: None,
                object_survives: None,
            },
            under_dir_rename: false,
        });
        matched_curr.insert(&curr.entry.path);
    }

    // ---- PHASE 5: vanished previous paths ----
    for prev in &previous.paths {
        if matched_prev.contains(&prev.entry.path) {
            continue;
        }
        let pid = prev.physical_id();
        let object_survives =
            prev.has_physical_evidence() && curr_by_pid.get(&pid).copied().unwrap_or(0) > 0;
        let kind = if current.complete {
            MutationKind::Deleted
        } else {
            MutationKind::Unobserved
        };
        mutations.push(Mutation {
            kind,
            path: prev.entry.path.clone(),
            old_path: None,
            physical_id: prev.has_physical_evidence().then_some(pid),
            evidence: Evidence {
                physical_match: false,
                content_equal: None,
                state_changed: None,
                object_survives: Some(object_survives),
            },
            under_dir_rename: false,
        });
    }

    // ---- post-pass: mark renames that ride under a renamed directory ----
    let dir_renames: Vec<(PathBuf, PathBuf)> = mutations
        .iter()
        .filter(|m| {
            m.kind == MutationKind::RenamedOrMoved
                && m.old_path.is_some()
                && matches!(
                    current
                        .paths
                        .iter()
                        .find(|o| o.entry.path == m.path)
                        .map(|o| o.entry.kind),
                    Some(EntryKind::Dir)
                )
        })
        .map(|m| (m.old_path.clone().unwrap(), m.path.clone()))
        .collect();
    if !dir_renames.is_empty() {
        for m in mutations.iter_mut() {
            if m.kind == MutationKind::RenamedOrMoved
                && !m.under_dir_rename
                && let Some(old) = &m.old_path
                && let Some((od, nd)) = dir_renames.iter().find(|(od, _)| old.starts_with(od))
                && old != od
                && m.path.starts_with(nd)
            {
                m.under_dir_rename = true;
            }
        }
    }

    mutations.sort_by(|a, b| (&a.path, &a.old_path).cmp(&(&b.path, &b.old_path)));
    Reconciliation { mutations }
}

fn content_compare(curr: Option<&ObservedPath>, prev: Option<&ObservedPath>) -> Option<bool> {
    let (c, p) = match (curr, prev) {
        (Some(c), Some(p)) => (c, p),
        _ => return None,
    };
    match (c.valid_hash, p.valid_hash) {
        (Some(a), Some(b)) => Some(a == b),
        _ => None,
    }
}
