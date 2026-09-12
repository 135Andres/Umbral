# V0 CLOSEOUT — FORMAL SCOPE FREEZE AND V1 HANDOFF

Status: CLOSING RECORD (2026-09-11, user mandate "V0 — cierre formal, congelación de
alcance y handoff a V1"). After this record, **V0 is frozen** except where new evidence
contradicts an already-made claim. No functional code was modified by this increment.

--------------------------------------------------------------------------------
## 0. FINAL STATUS

**V0 STATUS: PARTIAL**

The functional core scan → identity → hash → reconcile → store was subjected to
falsification through an independent oracle and produced **no implementation failure
within the tested scope**. The PARTIAL status comes exclusively from SC-5, which requires
a reconstruction surface for a new reader and lies outside this V0's functional mandate.

This is neither "prototype incomplete" nor "production ready".

--------------------------------------------------------------------------------
## 1. FINAL EVIDENCE AUDIT (SC-1..SC-5)

| Criterion | State | Evidence | Limitation |
|---|---|---|---|
| SC-1 harness covers §7-A scenarios reproducibly | PASS | 512 proptest cases over real trees; explicit precondition gate; deterministic seeds; proptest shrinking exercised on 4 findings | Generator space is bounded (3 paths + d0); not exhaustive |
| SC-2 INV-1..INV-8 hold | PASS | INV-1 both halves (hash tests, stable-only persistence); INV-2 rebuild through sequences incl. rename/delete/recreate; INV-3 structured Evidence on every mutation; INV-4/5/6 via scanner+projection tests; INV-7 real process kills (4 store points + 2 sequence points); INV-8 ambiguity properties | INV-7 scope: process-kill only (no power loss, no storage corruption) |
| SC-3 killed-mid-write store recovers consistently | PASS | store crash tests + crash-during-observe→reconcile→persist sequence; post-kill reopen consistent; rebuild exact | Same scope limit: process-kill, single filesystem (tmpfs/btrfs), single writer |
| SC-4 zero-change re-observation → no mutations | PASS (interpreted) | reconcile(A,A) → only Unchanged entries (dedicated test + proptest self-reconcile); measured 2040 Unchanged / 0 any-other-kind on the benchmark corpus | Interpretation recorded: Unchanged entries are positive statements of sameness, not changes; a literal empty-list reading was rejected by documented interpretation, not by silent redefinition |
| SC-5 fresh reader reconstructs what fsp-check observed and when | PARTIAL — storage/evidence exists, reader-facing reconstruction surface absent | history table (seq, observed_at_ns) + active projection + `inspect`/`rebuild` CLI suffice for an engineer with the schema | No reader-facing surface for a non-author reader; NOT a demonstrated storage failure; does NOT imply a data-model change; does NOT authorise UI/API/presentation decisions |

No objective contradiction between criterion and evidence was found; no result changed
from increment 6.

--------------------------------------------------------------------------------
## 2. FORMAL SCOPE STATEMENT (what V0 demonstrates — non-anthropomorphic wording)

Observation. V0 records, for every entry under a scan root: the path (as raw bytes, no
lossy UTF-8 conversion), the entry kind (file / directory / symlink / other), and the
physical metadata obtainable at observation time (dev, ino, size, mtime at nanosecond
resolution where the platform provides it). Absence of obtainable metadata is recorded as
absence, not as an invented value.

Physical identification. Two observations of the same path can be classified as the same
filesystem object when dev+ino match on both sides and are available; same-path pairs with
different dev+ino are classified as changed identity, with content evidence (when valid
hashes exist on both sides) separating "recreated" (different content) from "not
determined" (identical content — duplicate content is never treated as identity). Physical
identity is evidence for reconciliation; nothing in V0 claims semantic identity of project
objects.

Content verification. For regular files, a guarded streaming BLAKE3 observation records a
hash together with the bytes-length hashed and a stability verdict. A hash is persisted
only when the before/after guard observed no relevant change; observations invalidated by
detected change are recorded as unstable with no hash. Every persisted hash represents
exactly the bytes read during one valid observation and makes no claim about bytes after
that observation.

Reconciliation. Comparing two observation sets yields a deterministic, idempotent set of
classified transitions — Unchanged, Modified, Created, Deleted (complete scan only),
Unobserved (incomplete scan), RenamedOrMoved (physical-identity proven; path swaps yield
two renames), Recreated, Ambiguous (with recorded reason). Every classification carries
structured evidence. Insufficient or conflicting evidence produces Ambiguous; it is never
resolved by heuristics on path, hash or timestamps.

Persistence. Observations are appended to an append-only SQLite history (paths as BLOBs);
the active projection (latest observation per path) is a pure replay of that history, kept
in one embedded store with WAL and synchronous=FULL. One record spans history+projection
in a single transaction.

Reconstruction. After loss or corruption of the projection, replaying the history
reproduces the projection exactly (demonstrated across generated sequences, after every
crash point tested, and after deliberate projection loss).

Crash class tested. Termination of the process at instrumented points during and between
persistence steps, in separate processes with fresh databases. NOT tested: power loss,
storage/filesystem corruption, multi-writer concurrency.

NOT demonstrated by V0: any reader-facing reconstruction surface for a new reader (SC-5);
continuity across device sync; behaviour under power failure; correctness under
concurrent writers; event-driven change detection (scan-based only); any semantic,
project-reality, authority, provenance-policy or multi-agent layer; performance
suitability for any production target (benchmarks are evidence, not requirements).

--------------------------------------------------------------------------------
## 3. V0 SCOPE FREEZE

V0 IN SCOPE (frozen):
  scan · filesystem metadata observation · physical identity evidence · content hashing
  with stability guards · reconciliation · ambiguity preservation · append-only
  observation history · rebuildable projection · process-crash consistency ·
  falsification harness.

V0 OUT OF SCOPE (frozen; not rejected for any future version):
  SC-5 reader surface · FTS5 · watcher/daemon · multi-writer concurrency · power-loss
  durability · network · MCP · UI · multi-device · semantic/project-reality layer ·
  background agents · policy/permissions · the JSONL decision · any mechanism not needed
  to close the current test.

This freeze is the experimental boundary of V0. It creates no new architectural
decisions; the stack remains UD-013 (provisional, V0-scoped, non-irreversible).

--------------------------------------------------------------------------------
## 4. Q25 STATE (OPEN)

What is now known (evidence, not decision):
  - SQLite-only has demonstrated per-observation atomicity (one transaction spans
    history+projection; no half-observation presented as valid after any tested crash);
  - history is append-only and the projection is demonstrably rebuildable from it;
  - process-crash consistency is demonstrated at multiple kill points;
  - WAL growth is observable and was measured (4.1 MB WAL vs 324 KB DB after 2040
    observations, without checkpointing);
  - no sufficient evidence yet exists to decide SQLite-only vs SQLite + external JSONL;
  - the WAL-growth observation must NOT be read as evidence that JSONL is needed — it is
    uncheckpointed WAL, an expected property of the current configuration.

Q25 remains OPEN.

--------------------------------------------------------------------------------
## 5. SC-5 GAP RECORD

`SC-5: PARTIAL — storage/evidence exists, reader-facing reconstruction surface absent.`

Explicitly: this is not a demonstrated storage failure; it does not imply a data-model
change; it does not authorise a UI; it does not authorise a new API; it does not
authorise deciding how presentation should work. It is an observability/presentation gap
handed to V1 (§8).

--------------------------------------------------------------------------------
## 6. HARNESS METHODOLOGY EVALUATION

Four corrections preserved as evidence (details in FALSIFICATION-REPORT.md):
  1. invalid-operation precondition gate (generator emitted ops without preconditions);
  2. atomic-replace temp built outside the target directory (same-fs rename semantics);
  3. second occurrence of the `Path::starts_with("dir/")` trailing-slash trap;
  4. the oracle did not model hard-link aliasing — content written through one path is
     visible at every path sharing the inode; the implementation was right, the oracle
     wrong.

Methodological lesson (recorded, not abstracted):
  When the oracle and the implementation disagree, the implementation must NOT be adapted
  to the oracle automatically; first determine which of the two is wrong against reality.

--------------------------------------------------------------------------------
## 7. FINAL RED-TEAM

Claim tested: "V0 está funcionalmente demostrado dentro de su alcance declarado."

Checked against: the INV-5 wording ("zero mutations") vs the Unchanged-entries behaviour
— resolved: the interpretation was already recorded under SC-4 in increment 6 and in this
record (§1); Unchanged is a statement of sameness, so INV-5 is satisfied under the
recorded interpretation. No other contradiction found between the frozen claims and the
evidence: every "PASS" cites a named test or experiment; every limitation is stated in
the claim itself.

`FINAL RED-TEAM: NO NEW BLOCKER FOUND`

--------------------------------------------------------------------------------
## 8. V1 HANDOFF (open inputs only — nothing resolved here)

Must resolve before expanding the filesystem core:
  - Q25 (SQLite-only vs SQLite + JSONL) — decide on demonstrated properties, not on
    absence of problems; decide whether power-loss durability is a requirement at all;
  - multi-writer concurrency (currently one logical writer by design);
  - power-loss durability, if it is confirmed as a requirement;
  - open identity/rename edge cases: inode-reuse ambiguity (same path, same content,
    new inode is Ambiguous by design — is that the desired terminal state?), multiple
    vanished carriers for one physical id (ConflictingCandidates), and directory-rename
    child reporting (currently individual entries flagged `under_dir_rename`).

Observability / reader surface (SC-5 gap):
  - how a new reader reconstructs what evidence exists;
  - how "when it was recorded" is represented and surfaced (observed_at_ns exists in
    history; nothing exposes it to a non-author reader);
  - how observed / derived / ambiguous are distinguished in that surface.

Potentially later (no commitment):
  FTS5 · watcher · daemon · UI · MCP · multi-device · semantic layer · Project Reality.

--------------------------------------------------------------------------------
Provenance: all claims in this record cite named tests, benchmark records, or the
falsification report in this directory. Commits: 6887708 (scan) → 05179db (identity) →
d7ca7da (hash) → aa3e80f (store) → f38963d (reconcile) → 1f6d13f (harness) → this record.
