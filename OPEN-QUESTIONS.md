# OPEN QUESTIONS

Status: CURRENT (living) — open questions (T4). Contains dated update blocks (see MR-2).

Category G (architecture-blocking) and H-adjacent. These are NOT answerable by opinion.
Each names what would settle it. The research agenda (RESEARCH-AGENDA.md) turns a subset
of these into work.

================================================================================
UPDATE 2026-09-10 — MASTER CONTEXT RECOVERY (audit outcome)
================================================================================
The Project Master Context (MC) was recovered from session history and archived at
research/sources/PROJECT-MASTER-CONTEXT.md. It answers several questions that were
previously classified open. Per-item status changes (original entries below, unchanged):

  Q3  TARGET USER — LARGELY RESOLVED at intent level. MC §1: useful to ordinary users
      while powerful for serious technical projects and AI-assisted development. MC §37:
      audience ranges individual -> small team -> technical team -> large user base.
      Residual: which user to serve FIRST (sequencing), not who.
  Q4  MULTI-FILE UNDO — RESOLVED AS USER REQUIREMENT. MC §26: the user wants
      history/snapshots/diffs/rollback/recovery without Git knowledge ("Git without
      Git"), with Git integration preserved. The MECHANISM remains research (MC's own
      open questions 12-14); S11's REJECT verdict is overridden by user authority.
  Q8  SCALE — RESOLVED AS POLICY, NOT NUMBER. MC §37: do not prematurely optimize for
      massive scale; do not make choices that unnecessarily prevent scaling later. The
      10^4-10^6 envelope is therefore not a design requirement but a non-blocking
      direction. H8 must be re-read accordingly.
  Q9  LOCAL-FIRST — RESOLVED. MC §3, §29: works fully locally; internet required only
      for services/integrations that need it; eventually hosted web infrastructure is
      in the intended scope (§29, §36).
  Q11 SURFACES — RESOLVED AT SCOPE LEVEL. MC §29/§30/§35: desktop, web, CLI, API,
      local server, home server, VPS, eventually hosted — all intended. SEQUENCE still
      open (what ships first).
  Q13 PROVENANCE — STRENGTHENED. MC §40: it should be possible to understand where
      information came from, who/what changed it, which AI/session produced a result,
      and what context was used. Reads as user-facing. Residual: how prominent in the
      UX.
  Q14 NON-TEXT ASSETS — PARTIALLY RESOLVED. MC §27: many file types in scope;
      distinguish editable/readable/indexable/previewable/opaque; do not assume all
      types can be interpreted semantically. The metadata-location mechanism remains
      open.
  T3  PERMISSION MODEL — RESOLVED BY USER. MC §24: the user explicitly wants a layered
      model (NORMAL -> CONFIRMATION REQUIRED -> SEMI-BYPASS -> FULL BYPASS), with
      semi-bypass authorizing edit/move/delete, and per-project/per-AI/per-integration/
      per-session configurability. This matches S7 and overrides S11's two-mode
      simplification. The exact semantics of each layer remain research (MC §43-Q10/11).
  T4  VIEW LAYER — RESOLVED BY USER. MC §7/§8: dashboards, kanban, graph, timeline and
      customizable command-center home are explicitly wanted. S11's REJECT verdicts are
      overridden; its evidence (graph hairballs, dashboard maintenance cost) is retained
      as design constraints. Automatic view generation must stay user-controllable
      (MC §7).
  T1  CONTEXT PACKAGES — STRENGTHENED. MC §12: the user prefers the hybrid model
      (system suggests/generates, user decides); Context Package is a user-intended
      product concept. The protocol-vs-bespoke-format question remains open.
  T2, T5 — unchanged (still open).

New user-stated items that were previously missing entirely (all USER INTENT, from MC):
dashboard/home command center (§8), universal search/command center with natural-language
queries (§9), tasks as first-class objects creatable from anywhere (§19), optional task
dependencies (§20), derived-but-overridable project state (§21), workflows as capability
never methodology (§22), the automation catalog (§23), background AI work (§25), multi-AI
coexistence with separate provenance (§14), "intentionally untouched" (§17), onboarding
principle "setup is not ingestion" (§34), interaction modes list (§35), file-type
capability tiers (§27), and the no-hidden-prompt principle (§11 -> invariant A11).

================================================================================
ADDED 2026-09-10 (coexistence stage) — three questions the new direction raises
================================================================================
These are architecture-blocking and NOT answerable by opinion. Full context:
COEXISTENCE-STRATEGIES.md; evidence: research/COEXISTENCE-RESEARCH.md.

Q16 — IS COEXISTENCE HARM MATERIAL, OR A DEVELOPER-ONLY CONCERN?
   No source found either way. If multi-AI coexistence failures cause no material harm to
   real users, most of the candidate strategies are unnecessary and the cheaper answer
   (version control plus conventions) suffices. This question decides whether the stage
   continues. Settles by: E-CO-1.
Q17 — CAN COORDINATION PRIMITIVES LIVE IN PLAIN FILES, OR DO THEY REQUIRE A LIVE PROCESS?
   Advisory claims in plain files are weak; atomic claims normally need a process. The
   plain-file constraint (A1/A2) and real coordination may be in tension. Settles by:
   E-CO-2 (with the engine switched off) and E-CO-3.
Q18 — DOES FSP MEDIATE COEXISTENCE, OR DEFER TO EXISTING MECHANISMS (GIT, ISOLATION)?
   The strongest competitor to every candidate strategy is "use worktrees/branches and let
   the human merge" (strategy S5). Settles by: E-CO-2 and E-CO-3, comparing mediated and
   isolated conditions on the same task.
NOTE ON SCOPE: these are not the same as the user-decision fork raised in
COEXISTENCE-STRATEGIES.md §3 (O14) — whether FSP serves developers-with-agents first or
ordinary users first. That is a product decision, listed in the stage record.

Q19 — DO AI PARTICIPANTS NEED AN EXPLICIT SAFETY FLOOR, OR DO PLATFORM PERMISSION
      SYSTEMS ALREADY SUPPLY IT?
   The safety floor (ENVIRONMENT-INTELLIGENCE.md H17) may be redundant with tool-permission
   prompts that vendors already implement. If platforms cover it, H17's floor is a
   nice-to-have, not a requirement. Settles by: E-CO-6a (with a platform-permission arm).
Q20 — IS "THE USER HAS NOT DECIDED" DISTINGUISHABLE FROM "THE USER HAS NO POSITION"?
   H21 (stance without authority) depends on this distinction being representable and
   perceived. No evidence found either way. Settles by: E-CO-9 (deferred), or by product
   research with real users.
Q21 — DOES THE STANDING DIMENSION ("WHO HAD THE AUTHORITY TO DECIDE") HAVE OPERATIONAL VALUE?
   Git records authorship, provenance records origin; neither records authority. No evidence
   was found that any participant or user acts on this distinction today. If it has no
   operational value, a large part of the standing dimension is decorative. Settles by:
   E-CO-6b-style probing of whether models/users change behaviour when standing is exposed.
   OPEN and explicitly unresolved; do not treat the standing dimension as established.
Q22 — DOES THE RECORD MODEL SURVIVE AS PLAIN FILES READABLE WITHOUT FSP?
   The minimum-model research (research/PROJECT-REALITY-MINIMUM-MODEL.md §9, UNKNOWN-1)
   found no primary source verifying that self-contained plain-file records stay fully
   interpretable after FSP's disappearance; nanopublications are the closest prior art.
   Settles by: E-MIN-1 variant or a dedicated portability test.
Q23 — DO MULTI-AI CONCURRENT WRITES NEED PER-RECORD VERSION VECTORS, OR DO SINGLE-WRITER
      CAUSAL MARKERS SUFFICE?
   Pass finding UNKNOWN-2: record topology already preserves concurrency as coexisting
   records; the question affects derived merging only. Settles by: E-CO-3 (concurrent
   writer collision test), not before it.
Q24 — DOES FSP NEED A UNIVERSAL STATUS VOCABULARY, PER-SCOPE VOCABULARIES, OR NONE?
   Surveyed status sets are domain-closed (USLM legal set, ADR set); FSP's own governance
   of a recorded vocabulary is untested and has a known drift risk (DOC-FRICTION-018
   precedent). Settles by: product research, only if the record model is ever carried
   toward design.
Q25 — IS SQLITE ALONE SUFFICIENT FOR V0 PERSISTENCE, OR IS AN EXTERNAL APPEND-ONLY LOG
      (E.G. JSONL) ALSO NEEDED?
   Raised by the user in the 2026-09-11 transition mandate as a technical hypothesis to
   validate DURING V0, not before it. Interacts with H25 (V0 store is an observation
   instrument, not the Project Reality representation) and with FAL-3/H24's capture-at-
   event-time rule: a log matters exactly when crash-consistency of observation history
   cannot be guaranteed by the store alone. Settles by: V0 crash/recovery experiments
   (V0-IMPLEMENTATION-PLAN.md §6 INV-7, §7-A).

================================================================================
(original question list, as written 2026-09-10 before the recovery)
================================================================================

-------------------------------------------------------------------------------
Q1 — What is the actual source of truth, and what is genuinely non-derivable?
The corpus never decides what belongs to the user's files versus the application's state
versus the derived index. Session histories, provenance traces, UI-asserted metadata and
plugin state have no assigned home. Both failure modes are documented: state stored only
internally becomes hostage to an opaque store (S1 §9), and state kept coherent across a
filesystem plus a store reproduces the two-writers problem (S9 Evidence 1).
Settles when: the state model is explicitly partitioned and each partition has a
justified home. Blocks: essentially everything downstream.

-------------------------------------------------------------------------------
Q2 — How are concurrent writers reconciled: external tools and internal agents?
Both S1 §13 and S9 §Failure Modes describe it; neither resolves it. Note S11 §9-1 is
also listed there, and its lesson is that locking is not the answer if the lock is not
held by all writers.
Settles when: a demonstrated reconciliation strategy exists (or a demonstrated proof that
one is unnecessary because no second writer exists).

-------------------------------------------------------------------------------
Q3 — Who is the user?
Individual developer / general knowledge worker / small team / everyone. The corpus
assumes different users in different reports: S2 and S3 assume developers with
repository-scale corpora; S8 assumes a knowledge worker who will not learn query syntax;
S11 §2 addresses a general user who will not accept the filesystem mental model at all.
Settles when: the user is named, and the product is allowed to exclude others.

-------------------------------------------------------------------------------
Q4 — Is multi-file AI undo a product requirement or a feature of the versioning layer?
R8 and R9 both depend on this. S6 recommends building it; S11 recommends reusing an
existing engine and rejecting a bespoke one.
Settles when: the user states whether "undo the whole AI operation" is load-bearing.

-------------------------------------------------------------------------------
Q5 — Is a per-file store acceptable for durable user data, or must everything be a file?
A10 (rebuildability) is satisfied by a store that is durable and backed up, but the
strongest reading of A1/A2 (everything must survive without the application) is not.
Settles when: the fallback for a corrupt store is chosen.

-------------------------------------------------------------------------------
Q6 — Does retrieval need multi-hop graph reasoning, or single-hop neighbourhoods?
S5 §Weak Evidence states this is unverified and that the answer determines whether graph
structures are load-bearing at all. If retrieval is single-hop, several candidate
mechanisms (and several performance concerns) disappear.
Settles by: measurement over a realistic corpus, or evidence from comparable products.

-------------------------------------------------------------------------------
Q7 — Can authority be granted in advance, or must it be requested per action?
P6 versus A6. C10 says per-action prompting fails in practice; the corpus offers no
demonstrated model that grants durable scoped authority to probabilistic automation and
holds under adversarial input. S7's tiers and S11's two modes are both proposals.
Settles when: one model is chosen and its failure modes are stated.

-------------------------------------------------------------------------------
Q8 — What scale must the product actually support?
10^4, 10^5 or 10^6 files. The corpus's design envelope is benchmark-derived (S2 §10).
Choosing 10^6 pre-commits the architecture (H8) and imports its costs; choosing 10^4
removes most of the hard problems and most of the justification for a heavy engine.
Settles when: the user states the intended corpus size and the intended hardware.

-------------------------------------------------------------------------------
Q9 — Is "local-first" an absolute constraint or a default?
Both readings appear. S9 §7 says zero bytes leave by default. S11 assumes a fully
offline-capable and reactive tool. If hosting, multi-device sync or collaboration
eventually matter, the answer is materially different.
Settles when: the user states whether remote/cloud modes are ever intended (Q11).

-------------------------------------------------------------------------------
Q10 — Is there a semantic layer over arbitrary files with no mandatory classification?
This is the central assumption of the product. No source in the corpus demonstrates a
working instance at scale; the corpus contains an argument that machine-derived structure
without user classification produces a synchronisation gap that a filesystem cannot
close (S11 §2 HYPOTHESIS).
Settles by: a prototype, or by external evidence from a system that has done it.

-------------------------------------------------------------------------------
Q11 — Which surfaces and deployment modes are in scope?
Desktop / CLI / server / web / mobile and whether any of them are real. S9 recommends a
sequence; nothing has been agreed. The web surface is the hardest to reconcile with A1
and A8 (S9 §Recommendations 10, S11 §8).
Settles when: the user ranks surfaces.

-------------------------------------------------------------------------------
Q12 — Does the FSP client need to be a single artifact or a suite of processes?
The corpus argues both ways (S10 §16 microkernel; S11 §9 failures of multi-process
daemons; S9 §8 single process). This is an engineering and packaging question with real
consequences for install and failure modes, and it is not a technology choice.
Settles when: the deployment model (Q11) and the plugin model (R11) are settled.

-------------------------------------------------------------------------------
Q13 — Is provenance a user-facing product feature or an internal mechanism?
R6 claims it as a differentiator; no requirement for a user-facing provenance feature is
stated anywhere in the corpus. If provenance is internal, its cost/benefit changes
completely.
Settles when: the user states whether provenance is something the product surfaces, or
something it merely retains.

-------------------------------------------------------------------------------
Q14 — What happens to non-text assets?
The metadata strategy of H4 does not apply to binaries, media and formats that reject
embedded metadata. Sidecars introduce orphan risk (S1 §Approach C, S5). The corpus leaves
this unresolved while treating media as in scope (J7).
Settles when: a non-text metadata mechanism is chosen or a scope exclusion is stated.

-------------------------------------------------------------------------------
Q15 — How does the system behave at the moment it is wrong?
No candidate architecture in the corpus specifies behaviour on index corruption, stale
derived state, conflicting concurrent edits, or an AI operation that has already written
to disk. Constraint C3 says these will happen.
Settles when: failure behaviour is specified as a first-class part of any architecture,
not as a residual category.

-------------------------------------------------------------------------------
TENSIONS (canonical list: ARCHITECTURE-HYPOTHESES.md)
T1 context packaging: product concept versus protocol payload. OPEN.
T2 "build only the differentiator": convergent principle, divergent targets. OPEN.
T3 permission model: multi-tier versus two-mode. RESOLVED BY USER (MC §24; UD-008) —
   the layered model is the intent; the research's two-mode simplification is
   superseded and retained only as design evidence.
T4 view layer: dashboards/kanban/graph/timeline versus "no dashboards, no graph views".
   RESOLVED BY USER (MC §7/§8; UD-009) — the views are wanted; the research's
   rejection is superseded and retained only as design constraint (C9, C11).
T5 filesystem authority versus non-derivable state — the corpus recommends the
   architecture it also documents as having failed. OPEN, highest risk (R1).
T6 scale envelope: 10^4-10^6 as design constraint versus "personal corpora need far
   less". RESOLVED AS POLICY BY USER (MC §37); the empirical question stays open (Q8).

A tension marked RESOLVED records that the USER's own words settled it. It does NOT
mean the research was wrong: the underlying evidence is preserved as constraint
material. Do not cite a resolved tension as a live disagreement.
