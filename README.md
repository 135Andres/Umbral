# FSP — File System Pro

Project memory for FSP: an open-source, local-first, AI-native workspace whose human
mental model is "my files and folders", with project / task / decision / knowledge / AI
capabilities layered as an optional overlay rather than a mandated structure.

STATUS: DISCOVERY -> RESEARCH -> ARCHITECTURE.
No architecture has been selected. No technology has been chosen. Decisions exist only as
recorded user commitments (DECISIONS.md); nothing else in this repository is a decision.

-------------------------------------------------------------------------------
WHAT THIS WORKSPACE IS
-------------------------------------------------------------------------------
A small, durable, external memory of the project: what it intends, what must remain true,
what is required of it, what constrains it, what is hypothesised, what is unknown, what
has been decided, and what research must come next.

It is also a laboratory for the project's own ideas about organization and provenance —
deliberately, and with the boundary that repository structure is NOT evidence about FSP's
product architecture (see DOCUMENTATION-ARCHITECTURE.md §0).

-------------------------------------------------------------------------------
WHAT IS DELIBERATELY NOT HERE
-------------------------------------------------------------------------------
- No architecture selection, no technology choice, no schema, no API surface, no code.
- No SONORA-LESSONS.md as a product document. Sonora is a separate project and never an
  architectural authority for FSP (UD-004); its lessons appear only inside the charter
  (MC §40) and are treated as research inputs.
- No transcript archive, no AI-reasoning dump, no generic software-architecture advice.
- No feature wishlist. Unweighted ideas are classified and left out on purpose
  (see research/INGEST-NOTES.md §3).
- No product-facing documentation (install guides, tutorials). The product does not exist;
  MC §48 forbids implementation-phase artifacts.

-------------------------------------------------------------------------------
AUTHORITY LADDER (highest first; a lower level never overrides a higher one)
-------------------------------------------------------------------------------
  1. Explicit current user instruction
  2. Decision record — DECISIONS.md, UD-nnn
  3. Charter — research/sources/PROJECT-MASTER-CONTEXT.md, cited as MC §n
  4. Canonical knowledge — PROJECT-DIRECTION, VISION, INVARIANTS, PRINCIPLES,
     REQUIREMENTS, CONSTRAINTS (current-state, derived from 2-3)
  5. Experiment result — experiments/EXP-n (evidence, not a decision)
  6. Research artifact — S0-S11, research/*-RESEARCH.md, audits (attributed, unratified)
  7. Hypothesis / interpretation — ARCHITECTURE-HYPOTHESES, OPEN-QUESTIONS
  8. Session history, agent memory, model inference

Two confusions this ladder exists to prevent:
  - "Hermes thinks this" read as "the project decided this" -> commitments live ONLY in
    decision records.
  - "A source says X" read as "FSP adopted X" -> research artifacts are attributed and
    unratified, and the technology inventory in ARCHITECTURE-HYPOTHESES.md is explicitly
    "not selected".

-------------------------------------------------------------------------------
EPISTEMIC CONTRACT (applies to every future session and edit)
-------------------------------------------------------------------------------
1. Classes: A INVARIANT | B VISION | C REQUIREMENT | D PRINCIPLE | E CONSTRAINT |
   F ARCHITECTURAL HYPOTHESIS | G OPEN QUESTION | H RESEARCH QUESTION | I REFERENCE |
   J POSSIBILITY | K IMPLEMENTATION DETAIL | L EXPLORATORY | M REDUNDANT.
2. Status: CONFIRMED | USER INTENT | STRONG HYPOTHESIS | WEAK HYPOTHESIS |
   OPEN QUESTION | RESEARCH NEEDED | EXPLORATORY | REJECTED / DO NOT USE.
3. Repetition is not confirmation. Appearing often in the corpus means it was repeated,
   not that it was validated.
4. A research report is not authority. A research opinion — including an adversarial
   verdict — stays an opinion unless the user's own words say otherwise (see VISION.md on
   where S11 was overridden by MC).
5. Conflicting findings are preserved as tensions, never silently resolved. The canonical
   tension list is T1-T5 in ARCHITECTURE-HYPOTHESES.md / OPEN-QUESTIONS.md.
6. Every claim carries a source reference: MC §n, S0-S11, UD-nnn, A-n, C-n, R-n, J-n,
   P-n, H-n, Q-n, T-n, EXP-n. If the origin cannot be identified, mark it UNKNOWN.
7. Nothing that matters may live only inside a tool. Files are the record, for this
   project too.
8. Documentation minimalism: a file is created only if losing its content would damage
   future product, architecture, research or decision-making. Before creating one, apply
   the routing rule in DOCUMENTATION-ARCHITECTURE.md §10.

-------------------------------------------------------------------------------
SOURCE CORPUS
-------------------------------------------------------------------------------
Primary user source: the PROJECT MASTER CONTEXT (MC) — the user's founding document,
archived verbatim at research/sources/PROJECT-MASTER-CONTEXT.md (recovered 2026-09-10
from session history; previously known only as a theme index).

Research sources: S1-S11, eleven AI-assisted research reports produced by Gemini Deep
Research, ingested and cross-verified by Hermes per MC §46. Their full texts exist in
session history and are cited as S1-S11 throughout; they are not yet archived under
research/sources/ (one request away). See research/RESEARCH-INDEX.md for scope,
authority status and known gaps — including where S11's product verdicts were
superseded by explicit user statements in MC.

-------------------------------------------------------------------------------
READ ORDER
-------------------------------------------------------------------------------
ORIENT    README -> PROJECT-DIRECTION -> VISION -> DECISIONS
DECIDE    DECISIONS -> OPEN-QUESTIONS -> ARCHITECTURE-HYPOTHESES -> RESEARCH-AGENDA
EVIDENCE  research/RESEARCH-INDEX -> research/ -> DECISIONS
WORK      the question -> the ID namespaces it touches -> those files only

-------------------------------------------------------------------------------
MAP
-------------------------------------------------------------------------------
README.md ......................... orientation, authority ladder, epistemic contract, map
AGENTS.md ......................... entry point for AI agents (pointer to README)
PROJECT-DIRECTION.md .............. strategic compass (what/who/why/decided/testing/next)
VISION.md ......................... purpose, mental model, north star, anti-goals
DECISIONS.md ...................... user decisions (UD-001..) — the only commitments
INVARIANTS.md ..................... A1-A11: properties that must hold regardless of design
PRINCIPLES.md ..................... P1-P9: how decisions should be made
REQUIREMENTS.md ................... R1-R13 (unprioritised) + J1-J8 candidates
CONSTRAINTS.md .................... C1-C12: environment facts bounding every candidate
ARCHITECTURE-HYPOTHESES.md ........ H1-H13 candidates (none chosen) + T1-T5 tensions
OPEN-QUESTIONS.md ................. Q1-Q15 that block architecture
RESEARCH-AGENDA.md ................ research phases, comparison criteria K1-K9, named gaps
DOCUMENTATION-ARCHITECTURE.md ..... PROPOSED structure of this repository (not a product doc)
research/RESEARCH-INDEX.md ........ source inventory, authority status, gaps
research/DOC-ARCHITECTURE-RESEARCH.md  evidence base for the documentation architecture
research/INGEST-NOTES.md .......... historical: the initial ingestion record
research/AUDIT-2026-09-10.md ...... historical: first project-intelligence audit
research/STAGE-2026-09-10-exp1-dogfood.md  historical: drift check, decisions required,
                                    next action for the EXP-1 / dogfood stage
research/sources/ ................. verbatim archives (PROJECT-MASTER-CONTEXT.md)
experiments/ ...................... one directory per experiment: spec + result +
                                    instrument + raw results (EXP-1; EXP-DOC-1)
experiments/corpus-messy/ ......... SYNTHETIC fixture data for a fictional project
                                    ("Kestrel"). NOT FSP records: its files are named
                                    like real project documents (decisions-2024.md,
                                    ADR-007-*.md, open-questions.md) because that is
                                    what the experiment needed. Read nothing there as
                                    a statement about FSP.

-------------------------------------------------------------------------------
LANGUAGE NOTE
-------------------------------------------------------------------------------
These files are written in English because ten of the eleven source reports are English
and the technical vocabulary is English (S11 is in Spanish). Translation is a one-command
change if preferred.
