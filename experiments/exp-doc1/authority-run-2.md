# EXP-DOC-1 — authority-confusion test run 2 (verbatim delegate report, condensed)

Instrument: second independent adversarial reader, no conversation context, read-only,
instructed to try to be misled. Run 2 was dispatched BEFORE the run-1 fixes, so a few
findings it reports were already corrected by the time it reported; marked inline.

Date: 2026-09-10. Duration: 177.5 s. Tool calls: 13. Files opened: 23.

## MEASUREMENTS

6 of 6 probes answered with the correct authority; 0 authority errors.
Conflicts found: probes 1, 4, 5, 6 (research vs. user); in every case the reader reported
the user's side winning. Probes 2 and 3: no conflict, correctly reported as UNDECIDED.
Temptations named: 10 distinct "misled risks" listed (vs. 3 in run 1) — this reader was
substantially more aggressive, and the extra findings are the value of run 2.

## FINDINGS — misled risks, with disposition

M1. REQUIREMENTS.md R6 still asserted "no user-facing requirement for [provenance] is
    stated anywhere" — contradicted by MC §40 and RESEARCH-AGENDA 0.5. A reader of that
    file alone answers probe 5 wrongly. LIVE at the time of report. FIXED: R6 now carries
    a dated correction and states provenance as USER INTENT (MC §40).

M2. REQUIREMENTS.md R12 presented the permission model as RESEARCH OPINION "contested"
    (S7 multi-tier vs. S11 two-mode) with no reference to UD-008. A user-decided intent
    could be read as an unsettled research question. LIVE. FIXED: R12 upgraded to USER
    INTENT (MC §24; UD-008), mechanism left as research.

M3. ARCHITECTURE-HYPOTHESES.md H10 presented S11's two-mode recommendation as the live
    counterweight with no inline marker of the override. LIVE. FIXED: H10 now carries a
    "SUPERSEDED AS A COUNTER (2026-09-10)" note while keeping C10's design bound.

M4. OPEN-QUESTIONS.md footer said "Tensions are recorded, not resolved" while the top
    update block marked T3/T4 resolved — the file gave two opposite answers depending on
    where the reader stopped. Already FIXED before this report arrived (run-1 fix). The
    reader's phrasing — "the resolved answer and the unresolved answer live in one file,
    separated by ~120 lines" — is the clearest statement of the defect.

M5. history/INGEST-NOTES.md §4 restates the pre-recovery T4 framing as an unresolved
    contradiction. Partially addressed (the file now enumerates its own false statements
    and carries a frozen-historical status line). The reader's deeper point stands and is
    NOT fixed: frozen historical files sit beside living artifacts in research/,
    distinguished only by a status line.

M6. VISION.md's anti-goal bullet could be read as rejecting graph views. Already FIXED.

M7. THE STRONGEST STRUCTURAL PULL: the repository is a live git repo with commits, and one
    probe asks precisely whether Git is FSP's versioning architecture. The correction is a
    self-aware paragraph buried in a ~500-line PROPOSED document, and H9's title ("with
    Git as a projection") reinforces the wrong read. PARTIALLY FIXED: the non-inference is
    stated at §0/§7 and MR-4; the reader's recommendation (put it where the signal is)
    remains open for the .git-adjacent surface. Recorded as a live structural risk.

M8. experiments/exp1/ contains real runnable code using sqlite3+FTS5 and a large
    results.json; nothing in those artifacts said "not a technology choice". A reader
    treating running code as a stack decision answers probe 3 wrongly. FIXED in the way the
    reader asked: harness.py's header now carries an explicit "READ THIS BEFORE TREATING
    ANYTHING BELOW AS A CHOICE" paragraph, and results.json now carries a `_disclaimer`
    field. The disclaimer lives with the misleading signal, not only in prose elsewhere.

M9. The ~45-technology inventory is a "shortlist-shaped object" even under its "NOT
    SELECTED" heading, and it appears twice (ARCHITECTURE-HYPOTHESES + INGEST-NOTES §3.1).
    NOT FIXED — judged disproportionate: the heading did prevent the trap for both run-2
    readers, and deleting the inventory would destroy the record that these technologies
    were considered. Monitoring.

M10. The S11 override is recorded in prose in VISION.md, not as its own decision record;
    only UD-009/UD-010 carry overrides into the decision log. Partially addressed: UD-009
    IS the override for the view layer. The reader's general point — that an override
    should be a decision record, not prose — is recorded as a candidate convention change,
    NOT applied (it would mean re-litigating a decision the user already made).

## VERDICT (the reader's own words, condensed)

"The repository largely but not fully succeeds... the claim under attack — that user words
outrank research — holds at the level of the decision log and the charter. It does NOT
hold uniformly across the repository. Three concrete failures let a careful reader reach a
wrong answer... The failure mode is not that research was presented as a decision; it is
that SUPERSEDED RESEARCH POSITIONS WERE LEFT STANDING AS LIVE STATEMENTS IN OTHERWISE-
CURRENT FILES, which is the same confusion one level down."

## INTERPRETATION (Hermes — not measurement)

- H-DOC-2 (PASS/FAIL): FAIL on the strict threshold for this run — the reader identified
  three places where a single-file reader reaches a wrong answer. It reported 0 authority
  ERRORS in its own answers, so the mechanism protected the reader who read carefully; but
  the standard is not "a careful reader survives", it is "a reader cannot be misled".
  Two of the three failures were mine and are now fixed; the third (frozen files beside
  living files) is structural and unresolved.
- This run is the strongest evidence in the whole stage for the value of the experiment:
  run 1 said "largely succeeds"; run 2, dispatched with an identical brief, found three
  concrete single-file traps. A single reader would have produced an over-confident PASS.
- Recurring theme across both runs and both batches: every failure was currency or
  provenance, never folder structure. Third independent confirmation.
