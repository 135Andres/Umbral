# EXP-DOC-1 — authority-confusion test run 1 (verbatim delegate report)

Instrument: a second delegated agent, no conversation context, read-only, instructed to
try to be misled. Preserved verbatim (condensed to findings) because the evidence must be
inspectable without trusting this repository's summary.

Date: 2026-09-10. Duration: 119.7 s. Tool calls: 12. Files opened: 20.

## MEASUREMENTS

Six probe questions, each with a single authoritative answer reachable in 2-5 files:
Q1 dashboards/graphs — YES (UD-009, MC §7/§8). Conflict with research found; user wins.
Q2 Git as versioning architecture — NO (MC §26, §52; nothing selected). No conflict.
Q3 database chosen — NO (MC §28, §52). No conflict.
Q4 permission model — layered intent (UD-008, MC §24); mechanism open. Conflict; user wins.
Q5 provenance user-facing — YES as intent (MC §40), not a UD. Conflict; charter intent wins.
Q6 research vs user on views — direct conflict; user wins (UD-009).
Authority errors (research read as a decision without correction): 0.
Times the agent reported being tempted and corrected by the repository's own labels: 3
(technology inventory read as a stack; repository git usage read as a versioning decision;
S11 verdicts read as product direction).

## FINDINGS (the agent's own words, condensed)

A1. The repository "largely succeeds at its stated purpose": every probe had one
    authoritative answer, and in every research-vs-user conflict the repository states in
    words which side wins and why.

A2. Residual weakness (a): superseded original text retained BELOW dated update blocks in
    OPEN-QUESTIONS.md and RESEARCH-AGENDA.md — "reading order determines the answer."
    Partially addressed (tension footer marked resolved; the update-block structure itself
    is retained as history by design and flagged as MR-2).

A3. Residual weakness (b): tension footers and VISION.md's anti-goal phrasing preserved
    the research position in language that can read as project direction. Fixed for the
    footer and the anti-goal; ARCHITECTURE-HYPOTHESES.md's tension entries now carry
    "RESOLVED BY USER" markers.

A4. Residual weakness (c): the two live drift risks the repository itself predicted — its
    own git usage, and the un-mapped/untracked experiments/ directory. Both fixed
    (README map; git commit) and both remain named risks (MR-4, X1).

A5. Residual weakness (d): MC §40's own header ("research inputs, not final requirements")
    cuts against the repository's promotion of provenance to user-facing intent. The
    agent correctly refused to resolve this itself and reported it as a live tension.
    ACTION TAKEN: recorded as a caveat at RESEARCH-AGENDA 0.5, with an explicit note to ask
    the user if the distinction ever matters. NOT resolved — it is a genuine authority
    question, and resolving it would be the repository deciding for the user.

A6. The agent's verdict on the whole: "research recommendations are distinguishable from
    user decisions, but only if the reader honors status lines, the update blocks, and the
    authority ladder; a reader who samples documents rather than reading them in the
    declared order can still be misled on questions 2, 3, 5 and 6."

## INTERPRETATION (Hermes — not measurement)

- H-DOC-2: SUPPORTED, and more strongly than the navigation run: the agent was explicitly
  trying to be misled and could not produce an authority error, though it identified four
  residual hazards and one unresolved authority question (A5).
- The mechanism that did the work was not the folder layout. It was: decision records with
  UD-ids, the authority ladder at entry, status lines, and explicit override notes. This is
  consistent with the documentation audit's central finding (the missing layer was
  authority, not structure).
- A6's condition ("only if the reader honors ... the declared order") is a real limit: the
  architecture is convention-based, and this run is the second independent confirmation
  that convention discipline, not structure, is the failure surface.
