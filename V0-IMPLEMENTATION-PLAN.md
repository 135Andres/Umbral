# V0 IMPLEMENTATION PLAN — fsp-check

Status: **PROVISIONALLY ADOPTED (V0 scope), authorized by the user 2026-09-11**
(synchronization-and-transition mandate). Document type: implementation plan — NOT an
architecture specification, NOT ratification of any V0-external choice. Every V0-scoped
adoption here is reversible by a new user decision; nothing in this file settles MC §52
(still-explicitly-undecided: final architecture, DB, protocol, versioning, UI, semantic
model).

STATUS VOCABULARY USED HERE (existing project labels, no new states):
DECIDIDO = has a UD record · PROVISIONALMENTE ADOPTADO = user-authorized for V0 only,
explicitly non-final · PROPUESTO = Hermes proposal awaiting user confirmation ·
HIPÓTESIS = to be validated by V0 evidence · ABIERTO = no decision, kept open ·
FUERA DE ALCANCE = out of V0 scope (not rejected for FSP) · SUPERSEDed/HISTÓRICO =
marked in place, never deleted.

================================================================================
1. OBJECTIVE
================================================================================
Build a small technical prototype, provisionally named **fsp-check**, demonstrating that
FSP can:
  - observe a real filesystem;
  - catalog its observable state;
  - detect mutations;
  - reconcile observed state;
  - persist sufficient information;
  - reconstruct its representation;
  - maintain verifiable invariants;
  - recover from reasonable failures.

fsp-check is an EXPERIMENTAL BASE, not FSP. It implements no product surface.

PROJECT STATE (2026-09-11, closeout): **V0 CLOSED AND FROZEN — STATUS: PARTIAL.**
All six increments implemented and falsified through an independent oracle; final
criteria evaluation §15; formal scope statement, freeze lists, Q25 state, SC-5 gap
record and V1 handoff in experiments/v0-harness/V0-CLOSEOUT.md. V0 is frozen except
where new evidence contradicts an already-made claim. As always: FSP is not designed,
no final architecture or full stack is decided, no hypothesis is demonstrated, Project
Reality is not resolved.

================================================================================
2. SCOPE
================================================================================
IN SCOPE (V0): single-directory-tree observation; deterministic scan; mutation detection
and classification; observation history + active projection in one embedded store;
reconciliation; invariant checks; crash/failure recovery tests; property-based harness
generating real filesystem trees and operation sequences.

FUERA DE ALCANCE DE V0 (explicit user list; NOT rejected for FSP, only out of V0 scope):
Tokio · Rayon · notify · Iroh · MCP · HTTP · Tauri · React · cryptographic identity ·
P2P · plugins · mobile · Tree-sitter inside the core.
Also out of V0 (mandate §"LO QUE V0 NO DEBE INTENTAR RESOLVER"): multi-device sync, P2P
collaboration, multi-AI authority, semantic graph, complete Project Reality,
Self-Description, any UI, persistent agents, cloud, distributed architecture.

================================================================================
3. STACK (PROVISIONALMENTE ADOPTADO — V0 only, non-irreversible)
================================================================================
Rust · SQLite (via rusqlite) · walkdir · BLAKE3 · UUIDv7 · tempfile · proptest.
Authority: USER (2026-09-11 mandate; supporting research — technology-selection report,
adversarial stack audit — exists in Gemini/Hermes research sessions but is NOT yet
archived in this corpus; see research/RESEARCH-INDEX.md GAPS item 4).
These are registered in DECISIONS.md UD-013 as V0-scoped adoptions, explicitly NOT
irreversible architectural decisions. Adopting or changing any V0 dependency updates
DECISIONS.md per DOCUMENTATION-ARCHITECTURE.md §IMPLEMENTATION-PHASE DOCUMENT FLOW.

================================================================================
4. CONCEPTUAL MODEL (HIPÓTESIS being tested by V0, not decided)
================================================================================
    Filesystem → deterministic scanner → filesystem snapshot → reconciler
    → classification → SQLite { observations/history + active projection }

Algorithm shape:
    traversal → stat() → compare with known state → relevant change?
      NO  → nothing
      YES → content/hash → classification → persistence

PRINCIPLE (user-stated): **ruta → identidad física → contenido** is an efficiency and
observation strategy, NOT a claim of semantic identity. The reconciler does NOT attempt
full conceptual identity of project objects in V0.

CORRECTED INVARIANT FORMULATION (user correction of an over-strong Gemini formulation;
the original `hash(p) == BLAKE3(read(p))` wrongly implied the file keeps those bytes after
observation). Correct meaning:

  **A persisted hash represents exactly the bytes FSP read during one valid observation.**

FSP observes a LIVE filesystem; it does not control or freeze the reality it observes.
This distinction is normative for every invariant below.

PERSISTENCE HYPOTHESIS (kept open, validated during V0 — Q25): SQLite as observations
history + active projection is the V0 working proposal; whether an external JSONL log is
also needed is NOT assumed. The V0 store is NOT assumed to be the future Project Reality
representation (H25); Records/Relations/provenance/standing/currency are NOT introduced
prematurely.

================================================================================
5. PLANNED MODULES (PROPUESTO — structure for the implementer, not a spec freeze)
================================================================================
  scan      deterministic traversal (walkdir): stable order, stable error handling
  identity  physical identity handling (dev/ino where available), path mapping
  hash      BLAKE3 content hashing, chunked reads, re-read guard for changing files
  store     rusqlite: observations/history + active projection; schema is a V0-internal
            detail, NOT an FSP schema decision
  reconcile compare observed vs known state; emit classified mutations
  classify  create / write / append / overwrite / atomic replace / rename / dir rename /
            delete / recreate (then chmod / symlink / hard link / platform-specific)
  verify    invariant checks over the store
  harness   proptest + tempfile: generates real trees and operation sequences; fault
            injection points for crash/persistence failures

================================================================================
6. INVARIANTS (VERIFIABLE; all subordinate to the corrected hash formulation)
================================================================================
  INV-1  Every persisted hash equals BLAKE3 of exactly the bytes read in the observation
         that recorded it; no invariant claims anything about bytes after that observation.
  INV-2  The active projection is a pure function of the observation history (rebuildable;
         store corruption of the projection alone must be recoverable by replay).
  INV-3  Every classified mutation names its evidence: the observation pair (before/after)
         that justifies the classification; no classification without recorded evidence.
  INV-4  No silent loss: any path present in the store's known state that disappears from
         the observed state is represented (delete or reconcile-pending), never dropped.
  INV-5  No false change: re-observing an unchanged tree produces zero mutations
         (deterministic scanner; mtime/ctime instability must not fabricate changes).
  INV-6  Impossible states are detectable: e.g. hash recorded for a path whose stat shows
         a different size than the hashed read, or a projection referencing a
         non-existent observation — the verifier must flag, not silently repair.
  INV-7  Crash during persistence leaves the store loadable and either fully-before or
         fully-after the interrupted observation (no half-observation).
  INV-8  When the physical evidence does not determine an identity (inode reuse, rename
         ambiguity, hash mismatch), the reconciliation records AMBIGUOUS — it never
         silently invents or silently resolves an identity. (Added by the user's
         pre-flight mandate 2026-09-11.)

================================================================================
7. RED-TEAM PROBLEM LIST (classification is PROPOSED — Hermes classification; the user
   may reclassify. Each item must be exercised by the harness or explicitly recorded as a
   V0 limitation.)
================================================================================
  A — necesario para V0 (harness must exercise it):
      missed events (scan-based detection is the mechanism, so missed-event detection is
      core) · file changing during hash · atomic saves · rename · delete + recreate ·
      unreliable physical identity (dev/ino reuse) · case sensitivity · symlinks ·
      hard links · crash during persistence · SQLite failure/recovery
  B — limitación aceptable de V0 (documented, tested only to characterize behaviour):
      permissions (observe-and-record, no enforcement) · clock behaviour (recorded, not
      trusted for ordering beyond observation sequence) · network filesystems (observe
      behaviour, no guarantee)
  C — investigación posterior:
      inotify/FSEvents-style event streams (notify is out of V0 scope by user decision) ·
      identity resolution beyond physical+content (the reconciler does not attempt
      semantic identity in V0)

================================================================================
8. TESTS AND BENCHMARKS — CONFIRMED BY THE USER (2026-09-11) AS EXPERIMENTS
================================================================================
User confirmation statement (verbatim intent): no arbitrary numeric performance targets
are established. Throughput, latency, store size and memory usage are measured and
recorded as EVIDENCE only; none becomes a product requirement unless a later explicit
user decision says so.
Tests: property-based (proptest) operation sequences over tempfile trees; crash-injection
tests around store writes; recovery tests (kill mid-write, reopen, verify INV-7).
Benchmarks (PROPOSED targets, awaiting user confirmation): scan+hash throughput on a
synthetic 10k-file tree; full re-observe latency with zero changes (INV-5 path); store
size growth over simulated histories. Benchmarks produce EVIDENCE, not decisions: a
benchmark that contradicts a hypothesis updates that hypothesis per the document flow,
it does not silently change the design.

================================================================================
9. SUCCESS CRITERIA — CONFIRMED BY THE USER (2026-09-11) AS V0 COMPLETION CRITERIA
================================================================================
  SC-1  Harness generates and runs all §7-A scenarios reproducibly.
  SC-2  INV-1..INV-7 hold, or every violation is a documented, understood limitation.
  SC-3  A killed-mid-write store always recovers to a consistent state (INV-7 demonstrated).
  SC-4  Zero-change re-observation produces zero mutations and bounded time.
  SC-5  A fresh reader (agent or human) can reconstruct what fsp-check knew and when,
        from the store alone.

================================================================================
10. RISKS
================================================================================
  - Scope creep toward "real FSP" — mitigated by §2 out-of-scope list being user-fixed.
  - Schema drift: the store schema is V0-internal; premature generalization would smuggle
    in the Project Reality representation (violates H25 guard).
  - Physical-identity platform divergence (inode reuse, case-insensitivity) — classified
    A; expected to produce the most interesting failures.
  - Hash-during-mutation races on live files — classified A; INV-1's formulation is what
    makes this survivable (record what was read, claim nothing else).

================================================================================
11. OPEN QUESTIONS (kept ABIERTO; none closed by inference)
================================================================================
  Q25 (OPEN-QUESTIONS.md): SQLite-only vs SQLite + external JSONL log — validated in V0.
      OPEN (user instruction 2026-09-11: do not close).
  Whether scan-based detection suffices or an event stream becomes necessary (§7-C).
  [Resolved by user confirmation 2026-09-11, formerly listed here: benchmark targets and
  success criteria. §8 confirmed as experiments (evidence only, no numeric thresholds);
  §9 SC-1..SC-5 confirmed as V0 completion criteria.]

================================================================================
12. PORTABILITY BOUNDARY (added by the Fedora pre-flight, 2026-09-11 — see
    experiments/v0-preflight/PREFLIGHT-V0-FEDORA.md for the evidence)
================================================================================
Minimal frontier, no premature abstraction: filesystem OBSERVATION code (stat/dev/ino,
walkdir traversal, path/OsStr bytes, rename/symlink/hardlink calls) stays separated from
RECONCILIATION logic (comparison, classification, invariants, store). The reconciler
consumes observation records, never raw syscalls — this keeps future platform ports a
listener problem, without adding indirection beyond that one seam.
Linux-specific assumptions that must not leak into reconciliation:
  - dev+ino reuse after delete (possible; §7-A must test it);
  - case sensitivity of the dev filesystem;
  - high-resolution mtimes trusted for ordering (never; §7-B);
  - lossy UTF-8 path conversion (paths are bytes; use OsStr/PathBuf end to end);
  - atomic-rename-across-filesystems (impossible; cross-device is copy+delete).
Tests that depend on Linux-specific behaviour are LABELLED as such (mandate §6):
deterministic tests vs behaviour-dependent tests (fs-type, kernel).
Identity evidence classification (increment 2): PhysicalId{dev,ino} = Unix-specific
evidence (None on non-Unix by design, absence recorded not invented); rename-preserves-
dev+ino = observed on Linux AND rename(2)-documented, filesystem-family-local, NOT a
universal law; inode-reuse behaviour = filesystem-dependent; the Ambiguous verdicts and
the path-is-not-identity rule = portable model properties (pure, tested without a
filesystem).
Reconcile evidence rules (increment 5): same hash ≠ same identity (duplicate content
is two objects); same path ≠ same object; Deleted requires scan completeness (per-path
errors => Unobserved); Ambiguous is a first-class outcome for conflicting candidates,
missing evidence, and unresolved delete+recreate; matching is global-by-identity, not
per-path, so swaps classify as renames. All rules are pure-model properties tested
without a filesystem.
Hash evidence (increment 3): guard = size+ns-mtime+dev/ino before/after; retry policy
MAX_GUARD_ATTEMPTS=2 is a V0 instrument policy, NOT an FSP architecture decision; a
stat-guard cannot see a rewrite that restores size and exact mtime inside the read
window (documented residual risk; hash-comparison across observations partially
backstops); content equality NEVER implies identity equality (two objects, same bytes =
duplicate content, different identities). INV-1 both halves tested: stable observation
yields exactly-the-read-bytes hash; unstable/errored observation yields NO hash.
Environment of record: Fedora 44, kernel 7.1.13, btrfs workspace, tmpfs /tmp — recorded
for reproducibility only, NOT a project requirement.

================================================================================
14. Q25 EVIDENCE — SQLITE-ONLY VS SQLITE+JSONL (increment 4; Q25 remains OPEN)
================================================================================
What was tested (evidence, not opinion): process-kill crash trials at four points
(before first write, between history appends, immediately after a commit, clean run),
each in a fresh database and a separate process. Results on Fedora (btrfs DB files):
  - atomicity per observation: one transaction covers history+projection; every kill
    left the store reopenable with history and projection consistent (INV-7 held);
  - rebuild: replay of history alone reproduced the projection byte-for-byte after
    every kill and after deliberate projection loss;
  - rollback: a failed transaction left zero trace in either table;
  - history: append-only, every observation kept, unstable hashes never stored as
    valid (INV-1 at rest).
What SQLite-only already provides for V0: atomicity, crash-consistency, rebuildable
derived state, queryable history. What JSONL would add: raw-text inspectability and
independence from SQLite's file format — a PORTABILITY/recoverability argument, not a
correctness one; no V0-required property was found that SQLite cannot provide.
Status: Q25 stays OPEN; SQLite-only is sufficient so far, evidence still thin (one
filesystem, one process shape, no power-loss tests — synchronous=FULL asserts
process-kill durability only, not power-cut durability). Reconcile increment will
exercise the store harder and add evidence before any closure.

================================================================================
15. V0 CRITERIA EVALUATION (increment 6 harness, 2026-09-11)
================================================================================
Evidence: experiments/v0-harness/{BENCHMARKS.md, FALSIFICATION-REPORT.md}.

  SC-1  Harness reproducibly generates and runs the §7-A scenarios ............ PASS
        Generated sequences (512 proptest cases) over real trees; explicit
        precondition gating; deterministic seeds via proptest shrinking
        (minimisation is proptest's own, exercised on four real findings).
  SC-2  INV-1..INV-8 hold .................................................... PASS
        INV-1 both halves tested; INV-2 (rebuild) through sequences; INV-7 via
        real process kills incl. crash during observe→reconcile→persist; INV-8
        exercised by the ambiguity properties. No invariant had to be weakened.
  SC-3  A store killed mid-write always recovers consistently ................ PASS
        (process-kill scope only: no power-loss or storage-corruption claim.)
  SC-4  Zero-change re-observation produces no mutations ...................... PASS (interpreted)
        Literal reading would require reconcile(A,A) to return an EMPTY list;
        the model returns one Unchanged entry per path — an explicit positive
        statement of sameness, not a change. Measured: 2040 Unchanged, zero
        Modified/Created/Deleted/Recreated/Renamed/Ambiguous. Interpretation
        recorded here so the criterion is not silently redefined.
  SC-5  A fresh reader can reconstruct what fsp-check knew and when .......... PARTIAL
        The store holds history (seq, observed_at_ns) + active projection and
        `inspect`/`rebuild` expose it; but V0 has no query surface for "what did
        it know at time T" beyond reading the history table directly, and no
        human-facing summary document. Sufficient for an engineer with the
        schema; NOT yet sufficient for a non-author reader. Honest verdict:
        PARTIAL — the remaining gap is presentation, not storage.

================================================================================
13. DEFINITION OF DONE
================================================================================
V0 is finished when the user-confirmed §9 criteria (SC-1..SC-5) pass, the red-team A-class scenarios are
exercised, failures are documented (not hidden), and a stage record freezes the outcome.
V0 does NOT authorize: product behaviour claims, architecture decisions beyond V0 scope,
or promotion of any provisional adoption without a new explicit user decision.
