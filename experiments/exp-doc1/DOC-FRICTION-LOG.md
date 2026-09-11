# DOC-FRICTION LOG

Purpose: the documentation architecture is under dogfood evaluation. This file records
ONLY meaningful friction observed while actually working — per the stage mandate, a
failure is recorded first and NOT immediately fixed. A fix happens only when a specific
observed problem justifies it, and the fix is recorded here.

Format per entry: Question / Path taken / Expected / Actual / Failure / Potential cause /
Disposition.

================================================================================
DOC-FRICTION-001 — an experiment had no home for its own specification
================================================================================
Question: "What exactly is EXP-1, as specified?"
Path taken: PROJECT-DIRECTION.md ("NEXT STEP") -> RESEARCH-AGENDA.md Phase 1 ->
research/AUDIT-2026-09-10.md §L -> experiments/ (did not exist).
Expected: one place holding the experiment's hypotheses, corpus, measurements and
thresholds, reachable from the README map.
Actual: EXP-1 existed as a recommendation in three documents, each partial: the audit
sketched it, the agenda listed Phase 1 goals, PROJECT-DIRECTION named it as next step.
No specification existed anywhere. The specification had to be authored from three
sources during this stage.
Failure: NAVIGATION + PROVENANCE failure (an artifact class with no home).
Potential cause: DOCUMENTATION-ARCHITECTURE.md §10's routing rule has no row for "an
experiment that has been proposed but not yet specified or run"; §11's naming section
described experiments/ but not the shape of an experiment's own directory.
Disposition: FIXED (proportionate, one-line class of change). §10 gained a routing row;
§11 now names the experiment directory shape. Recorded as the first evidence that the
routing rule was incomplete rather than the structure being wrong.
Severity: medium — cost was authoring time, not lost information.

================================================================================
DOC-FRICTION-002 — "next step" ambiguity between what is next and what needs authority
================================================================================
Question: "May EXP-1 run now, or is it blocked?"
Path taken: PROJECT-DIRECTION.md §CURRENT NEXT STEP -> OPEN-QUESTIONS.md 0.7/0.8 ->
RESEARCH-AGENDA.md Phase 0.
Expected: a clear statement of which next actions require the user's authority and which
do not.
Actual: PROJECT-DIRECTION listed "decide 0.7 and 0.8, then run EXP-1" as the next step,
which reads as though the experiment is blocked pending decisions. It is not: the user's
instruction authorized the experiment directly, and 0.7/0.8 concern what comes AFTER the
evidence (surface sequence, MVP boundary).
Failure: AUTHORITY-CLARITY failure, mild. The authority ladder resolved it correctly
(explicit current instruction wins), so no wrong action was taken — but the document cost
a re-read.
Potential cause: PROJECT-DIRECTION conflates "what is next" with "what requires a
decision"; the two are ordered differently.
Disposition: PARTIALLY FIXED. PROJECT-DIRECTION now separates the two lines explicitly
(EXP-1 ran without 0.7/0.8; those decisions gate the NEXT stage). No structural change.
Severity: low.

================================================================================
DOC-FRICTION-003 — current status of a question required reading two places
================================================================================
Question: "Is the scale question (Q8) still open?"
Path taken: OPEN-QUESTIONS.md -> the dated UPDATE block at the top -> then the original
Q8 entry further down -> then ARCHITECTURE-HYPOTHESES.md T6.
Expected: the current status of each question in one place.
Actual: the file carries a dated update block above the original text, so a reader must
read both and reconcile. Predicted in DOCUMENTATION-ARCHITECTURE.md as MR-2 (update-block
accumulation); this is the first observed instance.
Failure: minor NAVIGATION friction; no wrong conclusion reached.
Potential cause: the MC-recovery update was applied as a dated block to preserve
provenance, which was the right call then and is now a cost.
Disposition: NOT FIXED (monitoring). Trigger for consolidation remains §12 M2 (more than
two update blocks, or ~400 lines). One instance is not enough evidence to restructure.
Severity: low.

================================================================================
DOC-FRICTION-004 — provenance terminates at an unarchived source
================================================================================
Question: "Why do we believe lexical+AST retrieval is sufficient (the claim underlying
EXP-1's baseline)?" — asked while writing EXP-1's evidence section.
Path taken: ARCHITECTURE-HYPOTHESES.md H5 -> source references S2/S11 -> research/
RESEARCH-INDEX.md -> gap 2: the S1-S11 texts are not archived in the repository.
Expected: a citation that can be opened and read.
Actual: the citation resolves to the conversation that produced the repository, not to a
file. In this session the texts were available (they are in context); in any future
session they will not be.
Failure: PROVENANCE failure — the most serious class this architecture exists to prevent.
Potential cause: archiving was deferred twice pending user authorization (it is ~120 KB of
verbatim transcription).
Disposition: NOT FIXED — requires user authorization. Escalated as the single most
valuable documentation action available. Severity: HIGH.

================================================================================
DOC-FRICTION-005 — the map drifted within one change
================================================================================
Question: none (self-observed defect).
Actual: EXP-1.md was created under experiments/exp1/ and the README map was NOT updated
in the same change, violating DOCUMENTATION-ARCHITECTURE.md §10's own rule ("the new file
must be added to the README map in the same change").
Failure: DRIFT incident — exactly the indicator §10 lists.
Potential cause: the rule is a convention with no enforcement, and the creating agent (me)
was mid-experiment. MR-1 (convention decay) manifesting on the first opportunity.
Disposition: FIXED (map updated). Recorded as evidence for EXP-DOC-1's drift count:
1 drift incident of 1 opportunity.
Severity: low in cost, high in signal.

================================================================================
SUMMARY (as of 2026-09-10, after EXP-1)
================================================================================
Incidents: 5 recorded, of which 2 fixed, 1 partially fixed, 2 open (one high severity).
The structure itself did not fail in any incident: every failure was in the routing rule
(incomplete by one row), in the currency of a compass document, in a deferred archiving
decision, or in convention discipline. That is evidence FOR H-DOC-1's premise (the
structure is adequate) and evidence that the missing authority/currency layer — the thing
the documentation audit identified — is where the remaining cost lives.
