# STAGE RECORD — 2026-09-11b: Synchronization & Transition to V0 Implementation

Status: HISTORICAL RECORD (frozen). Companion to the 2026-09-10/-10b/-10c/09-11 records.

================================================================================
WHAT THIS STAGE PRODUCED (pointers)
================================================================================
V0 implementation plan (fsp-check) ........ V0-IMPLEMENTATION-PLAN.md
UD-012 (V0 authorized) / UD-013 (V0 stack, scoped) ... DECISIONS.md
H25 (V0 store != Project Reality guard) ... ARCHITECTURE-HYPOTHESES.md
Q25 (SQLite-only vs +JSONL log) ........... OPEN-QUESTIONS.md
Implementation-phase document flow + information-location rules ... DOCUMENTATION-ARCHITECTURE.md §8b
Corpus gap item 4 (V0 research unarchived) . research/RESEARCH-INDEX.md
PROJECT-DIRECTION.md updated (transition phase; evidence lines preserved)

================================================================================
AUDIT FINDINGS (mandate step 4)
================================================================================
1. PROVENANCE GAP (defect class DQ-6, third occurrence): the mandate's "acumulated
   context" cites research (technology selection, adversarial stack audit, fsp-check
   specification, multi-device sync) that exists ONLY in Gemini Deep Research / Hermes
   sessions, not in this corpus. Registered as RESEARCH-INDEX GAPS item 4. UD-013 is
   valid on USER authority alone; the evidence base is missing until archived. No text
   was invented to fill the gap.
2. NO CONTRADICTION between the transition mandate and prior records: "no implementation
   authorized" (MC §48) was a true statement until the user's 2026-09-11 instruction
   superseded it; UD-012 records the change of state rather than rewriting history.
   PROJECT-DIRECTION.md now carries both (transition primary; evidence lines open).
3. IMPLICIT-DECISION CHECK: two items in the mandate could be misread as decisions they
   are not: (a) "out of V0 scope" is NOT rejection of those technologies for FSP (the
   mandate says so; recorded verbatim in the plan and UD-013); (b) the corrected hash
   formulation is normative FOR V0 INVARIANTS, not a new project-wide invariant (A-n
   untouched). H25 added as an explicit scope guard so the V0 store cannot silently
   become the Project Reality representation.
4. HYPOTHESES TREATED AS DECISIONS — none found in the corpus. The stack itself would
   have become one if registered anywhere but DECISIONS.md; it was not.
5. BROKEN REFERENCES — none introduced; mechanical verifier run after edits (IDs and
   README map).
6. SUPERSEDED, NOT DELETED: prior "CURRENT NEXT STEP" text in PROJECT-DIRECTION.md kept
   in place below the new transition header.

================================================================================
DECISIONS THIS STAGE REQUIRES FROM THE USER
================================================================================
None new beyond the mandate itself. Awaiting confirmation (PROPOSED, plan §8-§9):
benchmark targets and V0 success criteria. Still carried: D5, D6/D11, D7 (0.7/0.8,
archive S1-S11 — now also the V0 research reports, GAPS item 4), D8, D9, D10.

================================================================================
NEXT ACTION (one)
================================================================================
Begin V0 implementation of fsp-check per V0-IMPLEMENTATION-PLAN.md — the plan's §5
module order (scan -> identity -> hash -> store -> reconcile -> harness) with the harness
and INV tests built alongside, not after. The user's confirmation of §8-§9 targets can
arrive during implementation; nothing in §1-§7 depends on it.

================================================================================
AMENDMENTS 2026-09-11 (same day)
================================================================================
The user confirmed V0-IMPLEMENTATION-PLAN.md §9 SC-1..SC-5 as V0 completion criteria and
§8 benchmarks as experiments producing EVIDENCE only, with the explicit statement that no
numeric performance threshold (throughput, latency, store size, memory) becomes a product
requirement absent a later explicit decision. Status became READY FOR V0 IMPLEMENTATION.
Guarded intact by the same instruction: UD-012 (V0 authorized), UD-013 (stack provisional,
V0-scoped), H25 (store is not Project Reality), Q25 (OPEN, must not be closed), and the
out-of-V0 list (out of scope, not rejected). The plan and README/DIRECTION were updated
accordingly; no code written.

--- AMENDMENT 2: pre-flight (user mandate, same day) ---
Environment probe run (experiments/v0-preflight/PREFLIGHT-V0-FEDORA.md): Fedora 44,
btrfs workspace, tmpfs /tmp, gcc present, libsqlite3 present, symlink/hardlink/atomic
rename/case sensitivity/nsec mtimes all confirmed. Findings: (1) Rust toolchain NOT
installed — the single setup step before implementation; (2) portability boundary added
to the plan (§12) with INV-8 (ambiguity over invented identity, mandate-directed);
(3) no additional dependencies needed; (4) no blockers otherwise. No decision closed
(Q25 untouched); environment versions recorded for reproducibility only.

--- AMENDMENT 3: increment 1 SCAN implemented (user mandate, same day) ---
fsp-check crate created at fsp-check/ (walkdir + tempfile dev-dep only; rusqlite/blake3/
uuid/proptest deliberately absent until their increments). Scanner: deterministic sorted
output, symlink-aware, read-only, errors represented not dropped; lib seam exposes
observation records so the future reconciler never touches walkdir/syscalls. Determinism
contract A/B/C implemented (state / observation-time / order). Tests 8/8 including the
INV-5 seed and a no-modification proof via raw metadata snapshots. cargo check/test/
clippy/fmt all clean. Two defects found and fixed during the increment, both in the test
suite, not the scanner: (1) Path::starts_with("dirlink/") matches the dirlink entry
itself (trailing slash ignored by component comparison); (2) missing PermissionsExt
import. Toolchain note: user authorized rustup user-level install during this increment
(stable 1.98.1); no crate code installs anything.

--- AMENDMENT 4: increment 2 IDENTITY implemented (user mandate, same day) ---
src/identity.rs: PhysicalId{dev,ino} evidence; compare_same_path / compare_cross_path /
index_by_physical_id; IdentityMatch { SamePhysicalObjectUnchanged, SamePhysicalObject-
Modified, RenamedOrMoved, SameObjectViaHardLink (documented; exercised via hard-link
group test), Deleted (represented at pair level as absence), Ambiguous{IdentityChanged-
AtSamePath|MissingEvidence}, NewObject }. Fields DISCARDED after evaluation: kind (not
identity evidence; guards type-level errors elsewhere), path (context, never evidence).
Fedora/btrfs evidence: rename preserves dev+ino (file + directory tree); hard links
share dev+ino; symlink identity distinct from target; inode reuse NOT observed after
delete+recreate with churn — recorded as observation, NOT assumed impossible; the model
handles both branches. INV-8 exercised by dedicated tests: missing evidence → Ambiguous;
path alone never positive; delete can never produce a positive match. 11 new tests
(6 observed-fs, 5 pure-model), all green with scan's 8. No hashing, no SQLite, no new
dependencies (proptest not yet needed — properties are deterministic here; adopting it
is deferred to the harness increment where generation is the point). Q25 untouched.

--- AMENDMENT 5: increment 3 HASH implemented (user mandate, same day) ---
src/hash_obs.rs + blake3 dependency (this increment's justification) + proptest (dev-dep;
now earns its place: byte-sequence stability + the never-fake-valid-hash property).
Model: ContentObservation {physical_id, hash: Option, hashed_len, stability, deltas,
error}; valid_hash() only when Stable. Guard: stat-before/read/stat-after over
size+ns-mtime+dev/ino; policy = 1 retry then Unstable (explicit constant, recorded as
instrument policy not architecture). Symlinks: NotARegularFile, target never hashed
into the link (scanner contract kept). Directories/specials: not hashable in V0 (no
Merkle). Atomic save observed as PhysicalId change on the same path (rename swaps the
inode): old hash is history, not current truth. Delete+revisit: hash adds content
evidence to the Ambiguous same-path case but does NOT force resolution — same
hash+same size+mtime+new inode remains ambiguous-by-design (documented). 15 new tests
(13 deterministic, incl. controlled concurrency variant explicitly labelled
concurrency-sensitive rather than faked; 2 proptest properties). 34/34 green, clippy 0,
fmt ok. Q25 untouched; no SQLite, no reconcile, no watcher, no concurrency.
