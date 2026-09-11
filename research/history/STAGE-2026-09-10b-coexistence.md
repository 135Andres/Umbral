# STAGE RECORD — 2026-09-10b: AI-native coexistence research

Status: HISTORICAL RECORD (frozen). Companion to
research/history/STAGE-2026-09-10-exp1-dogfood.md. Carries only what has no other home:
the drift check, the user decisions this stage created or requires, and the next action.

================================================================================
WHAT THIS STAGE PRODUCED (pointers; no duplication)
================================================================================
Problem definition, existing research, solved-vs-unsolved, missing abstractions,
seven candidate strategies, eight invented mechanisms, adversarial critique and five
experiments ......................... COEXISTENCE-STRATEGIES.md
External evidence, source by source ... research/COEXISTENCE-RESEARCH.md
Four narrow briefs for external models research/briefs/BRIEF-{A,B,C,D}-*.md
Research direction decision .......... DECISIONS.md UD-011
New architecture-blocking questions .. OPEN-QUESTIONS.md Q16-Q18
Research phase ....................... RESEARCH-AGENDA.md Phase 7

================================================================================
DRIFT CHECK (mandate §11 and §15)
================================================================================
ARCHITECTURE SELECTED — no. Seven strategies and eight mechanisms exist as unratified
hypotheses. No file names a chosen architecture, protocol, database, coordination
primitive or identity mechanism. The three-layer discipline was applied to every entry
(product thesis / architectural hypothesis / implementation mechanism) and no entry was
promoted upward.

INVENTIONS PRESENTED AS EVIDENCE — no. Every invented mechanism (M1-M8) is labelled
NEW HYPOTHESIS / PROPOSED MECHANISM at its own heading, with assumptions, failure modes,
abuse modes, the experiment required and the falsification condition. M8 is explicitly
"the candidate missing abstraction" and is a hypothesis, not a finding.

EXTERNAL MODELS GIVEN AUTHORITY — no. The four briefs are scoped research delegations.
Each states that its output is external research, not a project decision, and the stage
record in RESEARCH-AGENDA 7.6 says briefs may inform comparison but never decide it. No
brief asks a model to design FSP, and none received the repository.

A RESEARCH OPINION BECOMING THE PRODUCT — one near-miss, neutralised. The strongest
candidate strategy on the evidence (S4 coordination substrate) is also the most expensive
and the one most likely to make FSP a single point of failure. It is recorded with that
objection attached (O9), and the adversarial section states that the stage's central
question is UNANSWERED until E-CO-1 runs (O7). The repository does not treat coexistence as
proven-necessary.

A CORPUS FINDING RE-STATED AS NEW — checked. The coexistence problem overlaps S3 (agent
operating layer, session portability), S6 (versioning/provenance) and S10 (extensibility).
This stage's genuinely new contributions are: the validity abstraction as the candidate
missing piece; the external evidence that the problem is named and unsolved outside this
project (CO-7, CO-9); and the adversarial finding that the problem may be developer-only
(O7, UD-012 below).

================================================================================
DECISIONS THIS STAGE CREATED
================================================================================
UD-011 — AI-native coexistence is the current central research direction, explicitly NOT a
product decision, with the semantic-projection thesis preserved (DECISIONS.md).

================================================================================
DECISIONS THIS STAGE REQUIRES FROM THE USER
================================================================================
D5 (NEW, and the most consequential question this stage raised): WHO SUFFERS THE
   COEXISTENCE PROBLEM? The evidence says multi-writer and staleness harm is documented for
   DEVELOPERS RUNNING CONCURRENT AGENTS (CO-9, CO-10), while the charter puts ordinary users
   first (MC §1). If coexistence is a developer problem, strategy S5 (isolation via
   worktrees/branches, integrated as "Git without Git") may be sufficient, and the elegant
   substrate (S4+S6) may be unnecessary. If it is a general problem, the substrate is the
   differentiator. This is a product fork, not a research question — it cannot be settled by
   E-CO-1 alone, because the experiment would need to know whose workflow it simulates.
D6 (NEW): MAY THE FOUR BRIEFS BE SENT to external models (Astra, Fable, Sol/ChatGPT), and
   if so, to which, and is any part of the briefs to be withheld? They contain no repository
   content and no personal data, but sending them is the user's call.
D7 (CARRIED, unresolved): 0.7 surface sequence; 0.8 MVP boundary; authorization to archive
   S1-S11 verbatim (the repository's highest-severity provenance defect, now also the
   evidence base for Phase 7 — the coexistence research cites S3/S6/S10 by section, and those
   texts are still not in the repository).
D8 (CARRIED, optional): confirm UD-007 (Gemini-first method, inferred from behaviour).

================================================================================
NEXT ACTION (one)
================================================================================
E-CO-1 — the staleness harm test (specified in COEXISTENCE-STRATEGIES.md §4). It is the
cheapest experiment that can cancel the whole direction, and every other coexistence
experiment is wasted if it fails. It needs no user authority and no model access: it can run
on the existing fixture corpus with the instrument EXP-1 already established.

Sequencing note: D5 (whose workflow is simulated) does not block E-CO-1, because the
experiment can be run in the developer-conditions arm first and repeated later under
ordinary-user conditions if D5 resolves that way.
