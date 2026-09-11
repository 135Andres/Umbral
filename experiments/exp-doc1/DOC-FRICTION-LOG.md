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
DOC-FRICTION-006 — a fresh reader could not tell that a document was synthetic
================================================================================
Question: "What has been tested?" (Q8 of the navigation test)
Path taken: README map -> PROJECT-DIRECTION -> then the agent left the documented map and
found experiments/exp1/ by listing directories.
Expected: a documented, mapped experiment record with a status line, plus a clear marker
that the experiment's fixture corpus is not project data.
Actual: at the time of the run, experiments/ was absent from the README map, PROJECT-
DIRECTION still said "Nothing empirical yet", and the fixture corpus contained files named
decisions-2024.md, ADR-007-realtime-updates.md and open-questions.md with nothing saying
they were fixtures for a fictional project.
Failure: CURRENCY + AUTHORITY failure (the most serious class found in this stage): the
canonical statement was stale and the true evidence was unmapped and untracked.
Potential cause: the experiment report and the canonical update were written in the same
stage; the agent's run caught the repository mid-change.
Disposition: FIXED (map, status, commit, fixture warning in README, and a status line on
EXP-1.md). Recorded as evidence for EXP-DOC-1's drift count: this is the second drift
incident of the stage.

================================================================================
DOC-FRICTION-007 — a preserved tension list asserted a superseded position
================================================================================
Question: "Does FSP want graph views?"
Path taken: OPEN-QUESTIONS.md -> the tension footer.
Expected: a tension list whose entries state their own current status.
Actual: the footer still read "T4 view layer: ... versus no dashboards, no graph views",
with no marker that the user had resolved it. A reader stopping there concludes the
opposite of UD-009.
Failure: AUTHORITY failure (a superseded research position presented as live).
Potential cause: when the user's decision landed, the resolution was recorded in the
UPDATE block at the top of the file and in ARCHITECTURE-HYPOTHESES, but not in the footer
that a reader reaches last.
Disposition: FIXED (footer entries now carry OPEN / RESOLVED BY USER and state that a
resolved tension is not a live disagreement). Severity: medium-high, because it is the
exact confusion the architecture exists to prevent — caught only because the test reader
was adversarial.

================================================================================
DOC-FRICTION-008 — a superseded position survived inside a CURRENT file (R6)
================================================================================
Question: "Is provenance user-facing?" (EXP-DOC-1 authority test, run 2)
Path taken: REQUIREMENTS.md alone.
Expected: a current statement, or one marked superseded.
Actual: R6 said "no user-facing requirement for it is stated anywhere", which was true of
the research corpus and false after the charter was recovered (MC §40). A reader of that
file alone answers the question wrongly.
Failure: CURRENCY failure inside a canonical (CURRENT) file — the worst variant of the
class, because the file's own status line asserts currency.
Potential cause: the MC-recovery pass upgraded OPEN-QUESTIONS, RESEARCH-AGENDA and
RESEARCH-INDEX, but did not sweep REQUIREMENTS for statements that MC invalidated.
Disposition: FIXED (R6 carries a dated correction). Escalated into a rule: when a source
is recovered that invalidates statements elsewhere, sweep EVERY canonical file for
statements the source contradicts — not only the files that were already under edit.

================================================================================
DOC-FRICTION-009 — the same defect in the permission model (R12)
================================================================================
Question: "What permission model does FSP have?" (EXP-DOC-1 authority test, run 2)
Path taken: REQUIREMENTS.md R12.
Expected: user intent recorded as such.
Actual: R12 described the model as RESEARCH OPINION, "contested" between S7 and S11, with
no reference to the user's own MC §24 statement — so a user-decided intent read as an
unsettled research question.
Failure: AUTHORITY failure of the same class as DOC-FRICTION-008, found in the same file
by the same reader. Two instances in one file is a pattern, not an accident.
Disposition: FIXED (R12 upgraded to USER INTENT, mechanism left as research).
Rule recorded: after any authority upgrade, sweep the affected canonical files for other
statements about the same subject, because these defects cluster by file.

================================================================================
DOC-FRICTION-010 — history and current truth share a directory
================================================================================
Question: "Is provenance user-facing?" / "Does FSP want graph views?" (both runs)
Path taken: research/ directory listing.
Expected: historical records distinguishable from current artifacts by more than a status
line.
Actual: frozen records (INGEST-NOTES, AUDIT-<date>) sit beside living research artifacts
in the same directory; a reader who opens one and trusts it reaches pre-recovery answers.
Failure: STRUCTURAL navigation/authority failure — the first observed failure that IS
structural rather than currency-based.
Potential cause: history was never given its own home because the repository is young and
each frozen record was created ad hoc.
Disposition: NOT FIXED. A fix (moving frozen records to research/history/ or similar) is a
structural change, and this experiment's own rule (mandate §4) requires a demonstrated
problem and a proportionate response; one observation by two readers is not yet enough to
move files. Recorded as the primary candidate for the next structural change, with its
trigger: a third independent reader making the same error, or the frozen set growing.

================================================================================
SUMMARY (as of 2026-09-10, after EXP-1 and both EXP-DOC-1 runs)
================================================================================
Incidents: 10 recorded: 7 fixed, 1 partially fixed, 2 open (one HIGH — DOC-FRICTION-004,
unarchived sources, awaiting user authorization; one structural — DOC-FRICTION-010).
Classes: currency 5, authority-clarity 4, provenance 1.
No incident was a failure of the topic-per-file organization. Every failure was currency,
status, or provenance discipline — the layer the documentation audit identified as missing.
That finding now has three independent confirmations: the audit, run 1's reader, and run
2's stricter reader, who found two of the currency defects that the first reader and the
author had both missed.
The single most useful measurement in this stage: an identical brief given to two readers
produced "largely succeeds" and "three concrete single-file traps". One reader was not
enough evidence, and the repository would have recorded a false PASS without the second.
The structure itself did not fail in any incident: every failure was in the routing rule
(incomplete by one row), in the currency of a compass document, in a deferred archiving
decision, or in convention discipline. That is evidence FOR H-DOC-1's premise (the
structure is adequate) and evidence that the missing authority/currency layer — the thing
the documentation audit identified — is where the remaining cost lives.
