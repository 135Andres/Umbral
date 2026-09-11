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
demand taxonomies or swallow files into proprietary databases. FSP's bet: keep the
filesystem sovereign and make the semantic layer a derived, rebuildable, user-governed
projection (MC §12, §28, §39).

WHAT WE HAVE DECIDED
See DECISIONS.md (UD-001..UD-010). Headlines: open source, no ads; no mandatory
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
the objections C1-C13 are recorded in ENVIRONMENT-INTELLIGENCE.md. All unratified. The two
kill-tests are independent: E-CO-1 (currency) and E-CO-6a (safety floor).

CURRENT CENTRAL DIRECTION (UD-011, 2026-09-10)
Coexistence with a world of AI systems: what must FSP provide so a human and several
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
Nothing at all yet — no implementation is authorized (MC §48).

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

CURRENT NEXT STEP
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
