# EXP-DOC-1 — navigation test run 1 (verbatim delegate report)

Instrument: a delegated agent with no conversation context, read-only, answering from the
repository alone. Preserved verbatim because the experiment's evidence must be inspectable
without trusting this repository's summary of it.

Date: 2026-09-10. Duration: 88.5 s. Tool calls: 9. Files opened successfully: 22 (+1 path
attempted and not found).

## MEASUREMENTS

Per-question first-file correctness: Q1 true; Q2-Q10 false (README routes but does not
answer; by the agent's strict definition of "first file contained the answer").

Files per question (from the reported paths): Q1 3, Q2 2, Q3 3, Q4 2, Q5 4, Q6 5, Q7 3,
Q8 8, Q9 5, Q10 4. Mean 3.9, median 3.5. Excluding Q8 (the anomaly): mean 3.2.

Unanswerable questions: none.
Routing failure: the agent attempted <repo>/RESEARCH-INDEX.md, which does not
exist (it lives at research/RESEARCH-INDEX.md). Recorded as a routing trap.

## FINDINGS (the agent's own words, condensed; full JSON in the batch report)

F1. THE HIGHEST-VALUE FINDING — an authority-ladder inversion on Q8 ("what has been
    tested?"): the canonical file (PROJECT-DIRECTION.md) said "Nothing empirical yet"
    while the actual experiment result sat in an untracked, unmapped experiments/exp1/.
    The agent had to decide the fact from raw artifacts because the higher-authority
    source was stale. This is a real defect in MY work, caused by writing the agent's
    report in this same stage: PROJECT-DIRECTION and README were updated afterwards, so
    run 1's finding is a snapshot of the repository mid-change, not of its final state.

F2. The README map omitted experiments/ entirely at the time of the run (fixed).

F3. The agent named the fixture corpus as a hazard: files named decisions-2024.md,
    ADR-007-realtime-updates.md, open-questions.md sit inside the FSP repository with no
    "fixture" marker, and could be read as FSP records. Fixed (README map + a warning).

F4. OPEN-QUESTIONS.md's tension footer still asserted the SUPERSEDED research positions
    ("T4 ... no dashboards, no graph views") as live tensions. A reader stopping at the
    footer would conclude the opposite of the user's decision. Fixed (footer now marks
    T3/T4/T6 resolved by user, T1/T2/T5 open).

F5. PROJECT-DIRECTION.md's status line said CURRENT while its "currently testing" section
    was stale. Fixed.

F6. INGEST-NOTES.md self-declared precedence worked, but its stale content required
    reading both documents. Improved (explicit list of now-false statements).

F7. The agent flagged that README.md has no Status line, and that DOCUMENTATION-
    ARCHITECTURE.md is a "second authority surface" while reading as a specification
    (this is MR-3, self-predicted; not fixed, by design).

F8. The agent explicitly reported its own interpretive discipline: it treated README as
    orientation, not evidence, and used the status lines and the authority ladder to
    resolve each question. It found no false decision, only stale or ambiguously ordered
    text.

## INTERPRETATION (Hermes — not measurement)

- H-DOC-1 (simple topic-oriented repository gives high comprehension if authority/status
  are visible): SUPPORTED on this run, with a caveat that the run caught the repository
  mid-change and the failures it found were currency failures, not structure failures.
- H-DOC-2 (explicit authority ordering prevents research being mistaken for decisions):
  SUPPORTED on this run. The agent named the exact temptations (technology inventory,
  repository git usage, S11 verdicts) and reported that the repository's own labels
  corrected it each time — which is the mechanism working as designed.
- The threshold question ("<= 3 files per question"): mean 3.2 excluding the anomaly,
  3.9 including it. MARGINAL PASS. This suggests the pre-registered threshold was too
  strict for a reader that must also verify authority, or that the repository needs a
  task-oriented entry point (see DQ-3).
