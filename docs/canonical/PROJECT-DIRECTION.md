# PROJECT DIRECTION

Status: CURRENT (living) — canonical knowledge (T2). Strategic compass, not an architecture spec.

Strategic compass, not an architecture spec. Created 2026-09-10 after the
project-intelligence audit recovered the Project Master Context (MC) from session
history. Update this file when decisions land — it should stay under one screen per
section.

WHAT WE ARE BUILDING
An open-source, local-first, AI-native workspace whose human mental model is "my files
and folders" (MC §4), with projects, tasks, decisions, questions, knowledge, AI sessions
and context as semantic overlays the user never has to maintain (MC §5, §50). The AI
side gets structured, efficient context; the human side gets "your filesystem manager,
but better" (MC §3).

WHO IT IS FOR
Ordinary users first-class, powerful enough for serious technical projects and
AI-assisted development (MC §1); audience intended to range from individual to small
team to technical team to large user base (MC §37). Sonora is the first intended
consumer (MC §2).

WHY IT MATTERS
Today, humans organize files but AI has to ingest them wholesale; knowledge tools
demand taxonomies or swallow files into proprietary databases. Umbral's bet: keep the
filesystem sovereign and make the semantic layer a derived, rebuildable, user-governed
projection (MC §12, §28, §39).

WHAT WE HAVE DECIDED
See DECISIONS.md (UD-001..UD-013). Headlines: open source, no ads; no mandatory
taxonomy; no hidden prompt injection; Sonora not an authority; local-first with file
survivability; research before implementation; Gemini-first research with Hermes
cross-verification; layered permission intent; views and proactive intelligence wanted.
Known gap in that log: provenance has no record — the charter states it (MC §40) but its
own section header demotes it, and this repository deliberately did not resolve that for
the user. See DECISIONS.md "GAPS IN THIS LOG".

CANDIDATE HYPOTHESIS SET (2026-09-10b, NOT a direction change)
The coexistence work produced a candidate broader concept — SHARED ENVIRONMENT REALITY
(reality / standing / currency), with Shared Validity as its currency dimension — and a
separate self-description hypothesis for the instrument itself. Eight hypotheses (H15-H23),
two proposed mechanisms (M9 calibration precondition, M10 read/write recording split) and
the objections C1-C13 (OBJ-1..13 in that file) are recorded in ENVIRONMENT-INTELLIGENCE.md. All unratified. The two
kill-tests are independent: E-CO-1 (currency) and E-CO-6a (safety floor).

CURRENT CENTRAL DIRECTION (UD-011, 2026-09-10)
Coexistence with a world of AI systems: what must Umbral provide so a human and several
heterogeneous AIs can safely, coherently and continuously share one filesystem without the
human reorganizing around any one AI's assumptions. This does NOT replace the semantic-
projection thesis (MC §4-§6), which remains fundamental. Seven candidate strategies and
eight invented mechanisms are recorded in COEXISTENCE-STRATEGIES.md — all unratified, none
selected. The first question is whether the problem is material at all (E-CO-1).

WHAT WE ARE CURRENTLY TESTING
EXP-1 ran 2026-09-10 (experiments/exp1/): a content-only semantic projection over a
36-file arbitrary corpus, measured on 8 retrieval classes and 10 filesystem mutations.
Result: PARTIAL PASS on both hypotheses — retrieval useful without any user taxonomy,
identity reconciled across every mutation class, but no abstention and vocabulary
mismatch only weakly mitigated. The highest-risk open problem is unchanged: the
two-writers problem, filesystem authority + derived state + AI writers + external
mutation (OPEN-QUESTIONS Q1/Q2/Q15; tension T5).

WHAT WE ARE NOT BUILDING
A note app, an Obsidian clone, a project manager, an AI chat app, a filesystem browser,
a Git replacement, an AI agent (MC §1). Not a database-with-file-export. Not a taxonomy.
Not autonomous background writers of user files. Not a prompt-injecting host (A11).
No product. Since 2026-09-11 the minimal technical prototype fsp-check IS authorized
(UD-012) — an experimental observation/reconciliation base, explicitly not Umbral and not a
product surface; scope and limits in V0-IMPLEMENTATION-PLAN.md.

TOP RISKS (one line each; ranking is Hermes interpretation, full reasoning in
research/history/AUDIT-2026-09-10.md §H)
R1 The two-writers problem: filesystem authority + derived state + AI writers + external
   mutation. Highest impact, highest uncertainty. Unresolved (T5, Q1/Q2/Q15).
R2 Premature scale commitment — largely defused as policy by MC §37; empirical part open.
R3 Identity under churn (A4 vs. the least intrusive mechanisms).
R4 The central premise is unvalidated at scale: no evidence yet that semantic projection
   over arbitrary files works beyond EXP-1's 36-file corpus.
R5 Failure behaviour when the system is wrong is unspecified in every candidate.

WHAT COULD CHANGE OUR MIND
- A demonstrated failure of the overlay premise: if semantics cannot be projected over
  arbitrary files coherently under external mutation (Q10/T5), the product thesis needs
  rework, not patching.
- Evidence that users will not accept the filesystem mental model (S11's open doubt).
- A candidate architecture scoring K1-K9 so decisively that scope assumptions change.
- Any new explicit user decision — user authority always outranks research.

CURRENT NEXT STEP (updated 2026-09-12b: v0.1 EVIDENCE COMPLETE EXCEPT THE READER PROTOCOL)

Development proceeds by incremental versions (v0.1, v0.2, ...) rather than jumping to V1.
v0.1 exists as the development crate `umbral/` — a local, read-only observation instrument
whose reader-facing surface closes V0's SC-5 gap. Its evidence is registered in
docs/versions/v0.1.md. The reader protocol has since been executed by the project owner and
**its acceptance criterion A1 was not satisfied**: three of eight questions could not be
answered from the output alone, and two findings from that run — mislabelled output lines, and
limits expressed only reactively — await an owner decision. v0.1 is therefore **not declared
complete**. V1 remains reserved for a version that has passed a separate AUDIT READINESS
REVIEW and an external audit; no version is designated as the audit candidate in advance.

`fsp-check/` is unchanged and remains frozen at PARTIAL. The two are separate: V0 is
evidence for v0.x, never a dependency of it.

PRIOR NEXT STEP (2026-09-11c: V0 CLOSED — status PARTIAL) — preserved for continuity
V0 is closed and frozen (closeout: experiments/v0-harness/V0-CLOSEOUT.md). V1 is NOT
started and is not authorized. The prototype's core was falsified through an independent
oracle without producing an implementation failure inside the tested scope; the PARTIAL
status comes only from SC-5, the missing reader-facing surface, which was outside V0's
mandate. What V0 did not demonstrate, and what it leaves open, is listed in the closeout
record's V1 handoff section — Q25 (SQLite-only vs SQLite + JSONL) remains OPEN.

Current work is publication preparation, not product work.
Unchanged evidence lines (open, not cancelled, runnable independently):
E-CO-1 (coexistence kill-test), E-CO-6a (safety floor), E-MIN-1 (field ablation with
readers). Prior next-step text preserved below for continuity.

Two distinct lines, deliberately separated:

  Evidence line (no user authority needed): E-CO-1 — the staleness harm test, which
  decides whether the coexistence direction continues (see COEXISTENCE-STRATEGIES §4).
  Deferred: EXP-2 (rename+edit identity), the weakest measured point of EXP-1 (XQ-3).
  Real-corpus line: re-run EXP-1's protocol against a directory the user authorizes,
  because every EXP-1 result is bounded by a Hermes-authored 36-file corpus (L1/L3).

  Authority line (needs the user): 0.7 surface sequence, 0.8 MVP boundary, and
  authorization to archive the S1-S11 report texts (the repository's highest-severity
  provenance defect). These gate the NEXT stage, not the evidence stage — EXP-1 ran
  without them, correctly. Full statement: research/history/STAGE-2026-09-10-exp1-dogfood.md.
