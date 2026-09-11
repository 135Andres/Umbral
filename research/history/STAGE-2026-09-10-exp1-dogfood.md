# STAGE RECORD — 2026-09-10: EXP-1 + documentation dogfood

Status: HISTORICAL RECORD (frozen). What this file holds and why it exists: the two
experiment records (experiments/exp1/EXP-1.md, experiments/exp-doc1/EXP-DOC-1.md) carry
their own results, and the drift log carries its own incidents. This file carries ONLY
what has no other home: the stage's drift check, the decisions that require the user, and
the next action. Nothing here is duplicated from those files.

================================================================================
DRIFT CHECK (mandate §31 P) — was any evidence accidentally converted into authority?
================================================================================
Checked, and answered item by item:

ARCHITECTURE — no. No document in this repository now names a chosen architecture, a
database, an index technology, a vector store, a graph technology, an AI protocol, a
provider, a semantic representation, an identity mechanism, a permission mechanism or a
UI architecture. Verified mechanically after the stage (ID/selection sweep) and by two
independent adversarial readers who were specifically probing for it. The one live
structural pull — this repository is itself a git repo — is declared non-inferential in
DOCUMENTATION-ARCHITECTURE.md §0/§7 and MR-4, and remains flagged as a residual risk
because the declaration sits away from the signal.

PRODUCT DECISION — no new one. Nothing the user has not said was recorded as decided.
The four things the tests pushed toward (provenance user-facing; permission layering;
views; Git-without-Git) are all traceable to the user's own charter, and each is recorded
with its MC section.

TECHNOLOGY CHOICE — one near-miss, caught and neutralised. EXP-1's instrument uses
sqlite3 + FTS5 because it is the cheapest thing in the standard library. Two readers
independently reported that runnable code plus a large results.json reads as a stack
decision. Fixed where the signal is: harness.py's header and results.json now carry an
explicit "this is not a technology choice" disclaimer, in addition to the existing
statements in EXP-1.md and the "NOT SELECTED" inventory.

METHODOLOGY — one case, deliberately NOT closed. MC §40's own header says "these are
research inputs, not final requirements" while the repository promotes provenance to
user-visible intent. The repository does not resolve this; the caveat is recorded at
RESEARCH-AGENDA 0.5 with an instruction to ask the user if the distinction ever matters.
Resolving it internally would have been the repository deciding for the user.

USER COMMITMENT — none created, none altered. No UD was added, changed, or superseded this
stage.

DOCUMENTATION STRUCTURE — two candidates, BOTH APPLIED after their triggers were met.
  (a) Frozen records beside living artifacts (reported by readers 2 and 3): applied —
      research/history/ created and the three frozen records moved into it.
  (b) Fixture corpus indistinguishable from project memory (reported by readers 1, 2, 3):
      applied — the corpus was renamed exp1/fixture-corpus-synthetic and given an adjacent
      FIXTURE-CORPUS.md. The fixtures were not edited, and the instrument was re-verified
      reproducible after the move.
  Both were withheld on first observation and applied only when a second/third independent
  reader made the same error. That is the discipline the mandate required, and it is the
  main process lesson of this stage.

DRIFT CHECK — ADDENDUM (after readers 3 and 4). Two further near-misses were found and
neutralised at the artifact that carried the signal: the repository's own git usage
(the strongest wrong signal in the repository, now declared non-inferential in README's
front matter as well as in DOCUMENTATION-ARCHITECTURE §7) and the H9 hypothesis title
("with Git as a projection", now marked CANDIDATE ONLY). One genuine gap was found and
documented rather than closed: the decision log has no record for provenance, because
writing one would silently resolve the MC §40 caveat that this repository leaves to the
user. DECISIONS.md now names that gap explicitly.

================================================================================
DECISIONS REQUIRED FROM THE USER (only these)
================================================================================
D1. SURFACE SEQUENCE (OPEN-QUESTIONS 0.7). Which surface is built first: desktop,
    CLI, or a headless core/daemon with thin clients? Everything else about surfaces is
    already answered (MC §29/§30/§35). This gates the stage AFTER evidence, not the
    evidence itself.
D2. MVP BOUNDARY (OPEN-QUESTIONS 0.8). MC §48 requires requirements -> research ->
    architecture -> choice -> MVP before implementation. No MVP scope has been proposed
    or accepted. Needed before any implementation discussion.
D3. ARCHIVE AUTHORIZATION. The S1-S11 report texts exist in the conversation that
    produced this repository and nowhere else. Archiving them verbatim (~120 KB, 11 files)
    would close the highest-severity provenance defect in the repository
    (DOC-FRICTION-004), which four independent readers each surfaced in some form. It needs
    one word from the user, because it is bulk transcription of material the user supplied.
    Note for the decision: the four readers also showed that citations pointing at an
    unarchived source are the single most repeated provenance weakness in the repository.
D4. (Optional, single confirmation) UD-007 — "Gemini-first research with Hermes as
    independent cross-verifier" was inferred from the user's behavior (delivering the
    corpus and requiring verification), not stated. Confirm once and it becomes a stated
    decision.

NOT asked, because the answer exists in the repository: target user, local-first scope,
provenance prominence direction, permission layering, view layer, scale policy, licence
direction. All recovered; asking again would be the known failure mode.

================================================================================
NEXT ACTION (one)
================================================================================
EXP-2 — the rename+edit identity test (spec proposed in experiments/exp1/EXP-1.md,
section NEXT EXPERIMENT). It is the smallest experiment that attacks the weakest measured
point of EXP-1 (XQ-3: no rename-with-heavy-edit mutation was run) and the top product risk
that identity mechanisms are supposed to answer (R3/A4). It needs no user authority: it is
an instrument measurement on the existing fixture corpus.

The alternative next action — re-running EXP-1's protocol against a REAL user directory —
is strictly more valuable evidence (EXP-1's corpus was authored by Hermes: limitations
L1/L3), and is blocked only by the user naming a directory to use. If the user names one,
that run takes precedence over EXP-2.

================================================================================
POINTERS (do not duplicate)
================================================================================
Experiment results and diagnostics .. experiments/exp1/EXP-1.md
Documentation dogfood results ....... experiments/exp-doc1/EXP-DOC-1.md
Documentation friction incidents .... experiments/exp-doc1/DOC-FRICTION-LOG.md
Verbatim reader reports ............. experiments/exp-doc1/*-run-*.md
Raw instrument output ............... experiments/exp1/results.json
