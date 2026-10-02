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
    /// Whether the reference (previous) observation was complete. A verdict relative to an
    /// incomplete reference may rest on an entry that existed but was not seen — `created`
    /// in particular (D-V01-9). Carried on every verdict; the report states it where it is
    /// material.
    pub reference_complete: bool,
    /// `Some(true)` = the object still exists elsewhere in the current set (a hard-link
    /// entry vanished but the content did not). Only meaningful for `Deleted`.
    pub object_survives: Option<bool>,
    pub reason: Option<AmbiguityReason>,
    /// Which observations the verdict rests on, and which of their fields the rules consulted
    /// (`UD-033`, `UD-034`). Recorded by the phase that reached the verdict; it changes nothing
    /// about the verdict itself.
    pub basis: Basis,
}

/// The two sides of a comparison (`UD-031`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Side {
    Reference,
    Compared,
}

/// What a verdict rests on, on one side.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SideBasis {
    /// The observation the verdict relates on this side, and the fields the rules consulted on
    /// it — possibly none.
    Related {
        path: PathBuf,
        fields: Vec<&'static str>,
    },
    /// The subject path has no observation in this run, and the verdict rests on that.
    Absent,
    /// The verdict relates no observation of its own on this side: on a conflicting-candidates
    /// verdict, the other side is represented by the counterparts.
    NotRelated,
}

/// The basis of one verdict: one entry per side, and the other entries it rests on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Basis {
    pub reference: SideBasis,
    pub compared: SideBasis,
    /// Other entries the verdict rests on, each on its side, in observation order.
    pub counterparts: Vec<(Side, PathBuf)>,
    /// The fields consulted on every counterpart.
    pub counterpart_fields: Vec<&'static str>,
}

const ID_FIELDS: &[&str] = &["dev", "ino"];
const ID_HASH_FIELDS: &[&str] = &["dev", "ino", "hash"];

fn related(path: &Path, fields: &[&'static str]) -> SideBasis {
    SideBasis::Related {
        path: path.to_path_buf(),
        fields: fields.to_vec(),
    }
}

impl Basis {
    fn pair(prev: &Path, cur: &Path, fields: &[&'static str]) -> Self {
        Basis {
            reference: related(prev, fields),
            compared: related(cur, fields),
            counterparts: Vec::new(),
            counterpart_fields: Vec::new(),
        }
    }
}

/// The fields `same_observable` consults: identity (already matched), kind, and — when both
/// sides are regular files — size and mtime.
fn observable_fields(prev: &ObservedPath, cur: &ObservedPath) -> Vec<&'static str> {
    let mut fields = vec!["dev", "ino", "kind"];
    if prev.kind == cur.kind && prev.kind == EntryKind::File {
        fields.extend(["size", "mtime"]);
    }
    fields
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

                    let mut fields = observable_fields(prev, cur);
                    let (kind, content_changed) = if same_observable(prev, cur) {
                        (MutationKind::Unchanged, None)
                    } else if hashes_equal(prev, cur) {
                        // Metadata moved, bytes did not.
                        fields.push("hash");
                        (MutationKind::Unchanged, Some(false))
                    } else {
                        fields.push("hash");
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
                            reference_complete: previous.complete,
                            object_survives: None,
                            reason: None,
                            basis: Basis::pair(&prev.path, &cur.path, &fields),
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
                    reference_complete: previous.complete,
                    object_survives: None,
                    reason: None,
                    basis: Basis::pair(
                        &previous.paths[pi].path,
                        &current.paths[ci].path,
                        ID_FIELDS,
                    ),
                },
            });
        } else {
            // More than one candidate on at least one side. Never pick one — and never drop
            // one either: every previous path in the group is reported too (D-V01-8). A
            // previous path that is still present in the current set gets its own same-path
            // verdict instead, so it is not reported twice. Each verdict names every other
            // member of the group, on both sides (F-TD-7).
            let members: Vec<(Side, &Path)> = prevs
                .iter()
                .map(|&i| (Side::Reference, previous.paths[i].path.as_path()))
                .chain(
                    curs.iter()
                        .map(|&i| (Side::Compared, current.paths[i].path.as_path())),
                )
                .collect();
            let others = |side: Side, path: &Path| -> Vec<(Side, PathBuf)> {
                members
                    .iter()
                    .filter(|(s, p)| !(*s == side && *p == path))
                    .map(|(s, p)| (*s, p.to_path_buf()))
                    .collect()
            };
            for &pi in prevs.iter() {
                prev_matched[pi] = true;
                let prev = &previous.paths[pi];
                if cur_by_path.contains_key(prev.path.as_path()) {
                    continue;
                }
                mutations.push(Mutation {
                    kind: MutationKind::Ambiguous,
                    path: prev.path.clone(),
                    old_path: None,
                    evidence: Evidence {
                        physical_identity: None,
                        old_physical_identity: Some(*pid),
                        content_changed: None,
                        complete_scan: current.complete,
                        reference_complete: previous.complete,
                        object_survives: None,
                        reason: Some(AmbiguityReason::ConflictingCandidates),
                        basis: Basis {
                            reference: related(&prev.path, ID_FIELDS),
                            compared: SideBasis::NotRelated,
                            counterparts: others(Side::Reference, &prev.path),
                            counterpart_fields: ID_FIELDS.to_vec(),
                        },
                    },
                });
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
                        reference_complete: previous.complete,
                        object_survives: None,
                        reason: Some(AmbiguityReason::ConflictingCandidates),
                        basis: Basis {
                            reference: SideBasis::NotRelated,
                            compared: related(&current.paths[ci].path, ID_FIELDS),
                            counterparts: others(Side::Compared, &current.paths[ci].path),
                            counterpart_fields: ID_FIELDS.to_vec(),
                        },
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
                reference_complete: previous.complete,
                object_survives: None,
                reason,
                basis: Basis::pair(&prev.path, &cur.path, ID_HASH_FIELDS),
            },
        });
    }

    // Absence is stated only where it is true (A2-T2b-3): a path present on a side but not
    // consulted by its verdict — the renamed-over case, Q26 — is named with no fields.
    let prev_paths: std::collections::BTreeSet<&Path> =
        previous.paths.iter().map(|p| p.path.as_path()).collect();
    let side_of = |present: bool, path: &Path| {
        if present {
            related(path, &[])
        } else {
            SideBasis::Absent
        }
    };

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
                    reference_complete: previous.complete,
                    object_survives: None,
                    reason: None,
                    basis: Basis {
                        reference: side_of(prev_paths.contains(cur.path.as_path()), &cur.path),
                        // Its identity was consulted for a partner in phase 2, and none existed.
                        compared: related(
                            &cur.path,
                            if cur.physical_id().is_some() {
                                ID_FIELDS
                            } else {
                                &[]
                            },
                        ),
                        counterparts: Vec::new(),
                        counterpart_fields: Vec::new(),
                    },
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
        // The identity is consulted only to look for survivors, and every survivor is named.
        let survivors: Vec<(Side, PathBuf)> = match (object_survives, prev.physical_id()) {
            (Some(true), Some(id)) => current
                .paths
                .iter()
                .filter(|c| c.physical_id() == Some(id))
                .map(|c| (Side::Compared, c.path.clone()))
                .collect(),
            _ => Vec::new(),
        };
        let basis = Basis {
            reference: related(
                &prev.path,
                if object_survives.is_some() {
                    ID_FIELDS
                } else {
                    &[]
                },
            ),
            compared: side_of(cur_by_path.contains_key(prev.path.as_path()), &prev.path),
            counterpart_fields: if survivors.is_empty() {
                Vec::new()
            } else {
                ID_FIELDS.to_vec()
            },
            counterparts: survivors,
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
                reference_complete: previous.complete,
                object_survives,
                reason,
                basis,
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
