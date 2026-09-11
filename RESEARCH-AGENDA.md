# RESEARCH AGENDA

Status: CURRENT (living) — research plan (T2/T4 hybrid).

Category H. What must be investigated before an architecture can be selected, in a
sequence that does not assume the answer. Nothing here is scheduled and nothing is
assigned. Priority order is by blocking power, not by interest.

-------------------------------------------------------------------------------
MAPPING TO THE USER'S RESEARCH TRACKS (MC §44, §47)
-------------------------------------------------------------------------------
The phases below are the research SEQUENCE. The user already defined the research TRACKS
in the charter; they map as follows, and the charter's naming wins when a report is
produced (RESEARCH-<TOPIC>.md, MC §47):

  Track A — Knowledge / Documentation Architecture (MC §44-A)
      -> Phase 1 (central assumption, Q10), Phase 2 (state model, Q1/Q2/Q15)
  Track B — AI Context Architecture (MC §44-B)
      -> Phase 3 (retrieval, cost, context budget; Q6)
  Track C — AI Agent Operating Layer (MC §44-C)
      -> Phase 4 (authority and automation, Q7) + Phase 5 (versioning interaction)
  Track D — Versioning / "Git without Git" (MC §44-D)
      -> Phase 5 (Q4); must challenge whether anything needs to be built at all (MC §26)
  Track E — UX / Product Architecture (MC §44-E)
      -> not yet in the phase list; add as Phase 7 when the documentation/view intent
         (MC §7-§9, §35, §38) is researched. Named here so it is not lost.

Track order is NOT prescribed by the user. The phases below are ordered by blocking power
(what prevents everything else), not by track letter.

-------------------------------------------------------------------------------
PHASE 0 — ESTABLISH THE PREMISES (user decisions, not research)
================================================================================
UPDATE 2026-09-10: the Project Master Context (MC) was recovered from session history
(research/sources/PROJECT-MASTER-CONTEXT.md) and answers most of Phase 0. Revised:

  0.1 Scale/hardware — ANSWERED as policy (MC §37): no premature optimization; no
      choices that unnecessarily prevent scaling. Residual is empirical (Phase 3), not a
      user decision. REMOVED from the user queue.
  0.2 Target user — ANSWERED at intent level (MC §1, §37): ordinary users AND serious
      technical projects. Residual: sequencing only. REMOVED from the user queue.
  0.3 Surfaces — ANSWERED at scope level (MC §29/§30/§35): desktop, web, CLI, API,
      local/remote server, eventually hosted, all intended. Residual: ORDER. Narrowed,
      still needs one user answer.
  0.4 Local-first — ANSWERED (MC §3, §29): fully local; internet only where a service
      needs it; hosted infrastructure eventually. REMOVED from the user queue.
  0.5 Provenance — ANSWERED as intent (MC §40): user-visible provenance wanted.
      Residual: prominence in UX. REMOVED from the user queue.
      CAVEAT (found by the EXP-DOC-1 authority test): MC §40 is the user's own section
      but its header says "These are research inputs, not final requirements." So this
      is USER INTENT read from the user's words, NOT a recorded requirement. If the
      distinction ever matters, ask the user.
  0.6 Source archiving — PARTIALLY DONE: MC archived verbatim at
      research/sources/PROJECT-MASTER-CONTEXT.md. S1-S11 full texts exist in this
      session's conversation history and can be archived verbatim on request; they are
      NOT yet in the repository. Still open, one command away.

STILL GENUINELY OPEN (cannot be recovered from any source):
  0.7 Which surface sequence to target first (desktop-first vs CLI-first vs other).
  0.8 MVP boundary: MC §48 requires requirements -> research -> architecture -> choice
      -> MVP before implementation, but no MVP scope has been proposed or accepted.

================================================================================
(original Phase 0 text, superseded by the update above — retained for the record)
================================================================================

These are not studies. They are questions only the user can answer, and every study below
is wasted effort until they are answered.

  0.1 Target corpus scale and hardware class (Q8). Determines whether the hard problems
      are in scope at all.
  0.2 Target user (Q3). Determines how much of the interface and permission model exists.
  0.3 Surfaces and deployment modes in scope (Q11).
  0.4 Whether local-first is an absolute or a default (Q9).
  0.5 Whether provenance is a user-facing feature (Q13).
  0.6 Whether the supplied research corpus (S1-S11) is to be archived in this repository.

-------------------------------------------------------------------------------
PHASE 1 — VALIDATE OR REFUTE THE CENTRAL ASSUMPTION (Q10)
-------------------------------------------------------------------------------
Single most important open technical question: can a semantic layer be projected over an
arbitrary, unclassified file tree without the user performing classification, and remain
coherent under external mutation?

  1.1 [PARTIALLY RUN 2026-09-10 as EXP-1 — see experiments/exp1/EXP-1.md] Indexed a
      36-file arbitrary corpus with content-only semantics; measured 8 retrieval classes
      against fixed accept lists and 10 mutation classes. Result: PARTIAL PASS on both
      hypotheses. NOT YET DONE: the real user directory (EXP-1 used a Hermes-authored
      corpus — limitation L3), and exposure of retrieval to a model.
  1.2 Test the coherence boundary: mutate files externally (rename, move, duplicate,
      delete, checkout) while the system is running, and record what breaks and why.
  1.3 Measure the real question from Q6: does useful retrieval require multi-hop
      relationships, or is a single-hop neighbourhood sufficient? Answer with numbers.
  1.4 Record negative results. A refutation of 1.1 is a successful outcome and changes
      the product, not the research.

-------------------------------------------------------------------------------
PHASE 2 — SETTLE THE STATE MODEL (Q1, Q2, Q15)
-------------------------------------------------------------------------------
  2.1 Partition every kind of state the product touches (user content, user-asserted
      semantics, machine inferences, session history, provenance, configuration,
      derived artefacts) and assign each a home with a stated justification.
  2.2 For each partition, answer: if this is lost, what is lost, and how is it recovered?
  2.3 Specify the reconciliation protocol between external writers and internal writers,
      including the case where the user moves a directory tree mid-operation.
  2.4 Specify behaviour when the system is wrong (Q15): stale index, conflicting edit,
      partially applied operation.

-------------------------------------------------------------------------------
PHASE 3 — RETRIEVAL AND CONTEXT (Q6, and the cost question in C5)
-------------------------------------------------------------------------------
  3.1 Benchmark on the user's real material: cold index build, incremental update,
      lexical query latency, memory, storage overhead. The corpus numbers are
      benchmark-suite numbers (S2 §10); reproduce or replace them.
  3.2 Determine empirically whether selective embedding justifies its cost for the
      intended corpora (S2 §17 assumes no; S2 §5.1 gives the counter-case).
  3.3 Determine the real context budget: measure what a model actually needs to be useful
      on this corpus, rather than assuming a packaging scheme.

-------------------------------------------------------------------------------
PHASE 4 — AUTHORITY AND AUTOMATION (Q7, R12, H7)
-------------------------------------------------------------------------------
  4.1 State a single authority model and its threat model, then test it against the
      failure scenarios already enumerated in the corpus (S7 §17 scenarios; S11 §9).
  4.2 Determine whether automation should be foreground-only in the first version, and
      what would have to be demonstrated before background action is justified (S11
      §17-10 defers it; this must be decided, not assumed).

-------------------------------------------------------------------------------
PHASE 5 — VERSIONING AND HISTORY (Q4, R8)
-------------------------------------------------------------------------------
  5.1 Decide whether multi-file AI undo is a requirement.
  5.2 If yes, evaluate ambient-capture-over-existing-primitives against simpler
      alternatives, on the criteria below.

-------------------------------------------------------------------------------
PHASE 7 — AI-NATIVE COEXISTENCE (added 2026-09-10; the current central direction, UD-011)
-------------------------------------------------------------------------------
Not one of the charter's five tracks (MC §44); it cuts across C (agent layer), B (context)
and D (versioning). Reference problem statement and candidate strategies:
COEXISTENCE-STRATEGIES.md. Evidence: research/COEXISTENCE-RESEARCH.md.

  7.1 Establish whether the problem is material (E-CO-1). Do this FIRST: if staleness and
      concurrent-writer harm are not real for the intended user, the rest is optional.
  7.2 Test whether plain-file conventions can carry coordination with the engine off
      (E-CO-2) — this decides strategy S1 vs S4 and tests the no-single-point-of-failure
      requirement.
  7.3 Measure concurrent-writer harm and whether advisory claims reduce it (E-CO-3).
  7.4 Test cross-provider handoff (E-CO-4) against the weak evidence already recorded (S3 §8).
  7.5 Test honest attribution, including UNKNOWN (E-CO-5).
  7.6 Only then compare strategies S1-S7 against K1-K9, with coexistence as the reference
      problem. External model briefs (research/briefs/) may inform 7.6 but never decide it.
  7.7 E-CO-6a — SAFETY-FLOOR COMPREHENSION (the environment-intelligence cycle's highest-
      information experiment; see ENVIRONMENT-INTELLIGENCE.md §7-§8). Runnable with the
      EXP-DOC-1 fresh-reader instrument, no prototype, no user authority. Candidate to run
      in parallel with 7.1: the two dimensions are independent (see objection OBJ-13).
  7.8 E-CO-7 — PROGRESSIVE-DISCLOSURE EFFICIENCY. Needs the E-CO-6a surfaces as arms.
  7.9 DEFERRED: E-CO-8 (needs a prototype to introspect) and E-CO-9 (needs a stance
      mechanism to exist; C11 makes paper-rejection cheaper than testing). E-CO-1 remains
      the kill-test for the currency dimension; E-CO-6a is the kill-test for the
      self-description dimension. They are independent: either can fail without killing
      the other.

-------------------------------------------------------------------------------
PHASE 6 — COMPARISON CRITERIA (how any architecture will be judged)
-------------------------------------------------------------------------------
No architecture may be selected before the criteria are stated. Candidate criteria,
derived from the invariants, not from any candidate:

  K1  Does it satisfy A1 and A2 (user files authoritative; survivable without the app)?
  K2  Does it satisfy A3 (no mandated taxonomy)?
  K3  Does it satisfy A4 (external reorganisation is non-destructive)?
  K4  Does it keep A5 (inference never silently authoritative)?
  K5  Does it hold under C3 (two concurrent writers)?
  K6  What is its behaviour when it is wrong (Q15)?
  K7  What does it cost on the target hardware at the target scale (Q8)?
  K8  Can it be described to a human without the human needing to adopt a metaphor the
      product imposes (the mental-model test)?
  K9  What can be removed from it without breaking K1-K4 (minimality test)?

  Every candidate in ARCHITECTURE-HYPOTHESES.md must be scored against K1-K9 before any
  selection. Selection criteria are not an architecture and are safe to write down now.

-------------------------------------------------------------------------------
RESEARCH NOT YET DONE — NAMED GAPS IN THE SUPPLIED CORPUS
-------------------------------------------------------------------------------
  G1  No empirical study of the central assumption (Q10) appears anywhere in S1-S11.
  G2  No research on the intended user (Q3); every report assumes a different one.
  G3  No research on governance, licence or contribution model.
  G4  No research on non-text asset metadata (Q14) beyond listing the problem.
  G5  No research on cost of operation: model spend per unit of work is mentioned
      (S7, S11) but never estimated.
  G6  No research on accessibility, localisation or i18n.
  G7  No research comparing candidates on equal criteria (Phase 6); each report scores
      its own proposal against its own assumptions.
