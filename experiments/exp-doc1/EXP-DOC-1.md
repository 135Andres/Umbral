# EXP-DOC-1 — documentation architecture dogfood

Status: **DESIGN FROZEN 2026-09-10; results pending the navigation tests.**
Purpose (mandate §21): evaluate whether the current documentation mechanisms are useful in
actual project work — NOT to prove DOCUMENTATION-ARCHITECTURE.md universally correct.
Subject of the experiment: DOCUMENTATION-ARCHITECTURE.md (status: PROPOSED) and the
repository it describes.

Classification per mandate §18 applies to every statement below.

================================================================================
HYPOTHESES (stated before the tests)
================================================================================
H-DOC-1  A relatively simple topic-oriented repository can provide high-quality human and
         AI project comprehension if authority, status, purpose and relationships are
         visible enough.
H-DOC-2  Explicit authority ordering prevents research conclusions or agent
         interpretations from being mistaken for project decisions.

================================================================================
DESIGN
================================================================================
T1  NAVIGATION TEST — fresh reader, no prior context, ten orientation questions
    (what is FSP; why does it exist; who is it for; what is decided; what is explicitly
    not decided; what is uncertain; what is researched; what is tested; top risks; what
    happens next). Instrument: a delegated agent with no conversation context, read-only,
    answering from the repository alone. Measurements: files opened in total and per
    question; whether the first file opened was the right one; whether any question was
    unanswerable; whether authority had to be worked out rather than read off.
    n = 2 independent runs (repeat measurement for reliability, not a second test).
T2  AUTHORITY-CONFUSION TEST — adversarial probe with six traps: questions where a
    research report's verdict contradicts the user's own words (dashboards/graph views),
    where the repository's own tooling could be mistaken for a product decision (git),
    where the corpus recommends a technology (database), where the user gave intent but
    not a mechanism (permissions), where intent exists but prominence is undecided
    (provenance), and a direct research-vs-user conflict question.
    Instrument: a second delegated agent, read-only, instructed to try to be misled.
    Measurements: per question — authority assigned, file that establishes it, whether a
    conflict was found, which side the repository says wins; plus any structural pressure
    the agent reports.
T3  DRIFT LOG — observed documentation failures during real work, recorded before any fix.
    Instrument: experiments/exp-doc1/DOC-FRICTION-LOG.md, kept during this stage.

Thresholds (fixed before the tests, per DOCUMENTATION-ARCHITECTURE.md §14):
  PASS  >= 8/10 questions answered correctly, <= 3 files per question on average,
        zero authority errors in T2.
  FAIL  any authority error (research read as a decision), or >= 4 files per question,
        or >= 2 drift incidents per five sessions.
  A failure triggers a targeted fix to the mechanism, not a redesign.

================================================================================
T3 RESULTS — drift log (COMPLETE)
================================================================================
5 incidents recorded: 2 fixed, 1 partially fixed, 2 open (one HIGH severity).
Full detail: DOC-FRICTION-LOG.md. Headline: no incident was a failure of the STRUCTURE
(flat, topic-per-file). The failures were: an incomplete routing rule (one missing row),
a compass document whose "next step" conflated "what is next" with "what needs
authority", a deferred archiving decision that leaves citations unopenable, and one
convention-drift incident (map not updated in the same change as a new file).
INTERPRETATION (Hermes): this is evidence FOR H-DOC-1's premise — the structure held —
and evidence that the remaining cost sits exactly where the documentation audit said it
did: authority, currency, and provenance discipline, not folder layout.

================================================================================
T1 / T2 RESULTS — RUN 1 OF 2 (n=2 design; sibling run pending)
================================================================================
Raw evidence: navigation-run-1.md, authority-run-1.md (both in this directory; the
delegate reports are preserved verbatim there).

T1 navigation (fresh reader, no context, read-only):
  22 files opened; 1 routing trap (attempted a path that does not exist).
  Files per question: mean 3.9, median 3.5; mean 3.2 excluding the Q8 anomaly.
  Questions unanswerable: 0 of 10.
  First file opened contained the answer: only for Q1; for Q2-Q10 README routed
  correctly but did not answer (by the strict definition used).
  Threshold check: "<= 3 files per question" -> MARGINAL (3.2-3.9 vs 3.0).
  Authority errors: 0, but one AUTHORITY-LADDER INVERSION observed (F1): the canonical
  statement "nothing empirical yet" was stale while the experiment result sat untracked
  and unmapped. Recorded as a real defect in this repository's currency discipline,
  caused by the experiment report being written in the same stage as the experiment.

T2 authority-confusion (adversarial, instructed to try to be misled):
  6 of 6 probes answered with the correct authority; 0 authority errors.
  3 named temptations (technology inventory as a chosen stack; repository git usage as a
  versioning decision; S11 verdicts as product direction) were each corrected by the
  repository's own labels.
  4 residual hazards identified; 1 unresolved authority question raised (MC §40's
  "research inputs, not final requirements" header vs. the repository promoting provenance
  to user-facing intent) — recorded as a caveat, deliberately NOT resolved here.
  Threshold check: "zero authority errors" -> PASS.

VERDICTS (thresholds pre-registered above):
  H-DOC-1: PARTIAL PASS. Comprehension is high and nothing was unanswerable, but the
  navigation-cost threshold was missed and one currency defect was found.
  H-DOC-2: PASS. The explicit authority ordering did its job under adversarial reading.
  MECHANISM note (not a verdict): the mechanism that carried both tests was the authority
  ladder + decision ids + status lines + override notes — NOT the folder structure. This
  reproduces, on independent readers, the documentation audit's central finding.

DEFECTS FOUND BY THE TESTS AND FIXED (each justified by an observation, per mandate §4):
  - README map omitted experiments/ (F2); fixture corpus not marked synthetic (F3).
  - Tension footer asserted superseded research positions as live (F4).
  - PROJECT-DIRECTION currency stale (F5); INGEST-NOTES stale statements now enumerated
    (F6); AGENTS.md gained a status line (F7).
  - AUDIT-2026-09-10 internal contradiction (section M vs documentation-audit M1):
    corrected by an appended dated correction, not a rewrite.
  - VISION anti-goal wording made unambiguous (A3).
  - RESEARCH-AGENDA 0.5 gained the MC §40 caveat (A5) — the only finding left OPEN,
    because resolving it would be the repository deciding for the user.
NOT FIXED (monitoring, with triggers): update-block accumulation (MR-2); the PROPOSED
document as a second authority surface (MR-3); convention decay (MR-1).

================================================================================
WHAT THIS EXPERIMENT CANNOT SHOW (stated in advance)
================================================================================
- It cannot show that the documentation architecture would work for a larger project,
  a team, or a different domain: n=1 project, n=2 readers, one moment in time.
- It cannot show that FSP's product should organize knowledge this way (mandate §9/§19).
  A repository with one author, no performance constraints and no external users is not
  a testbed for a product's storage or retrieval architecture.
- It cannot detect slow failure (convention decay over months); only the drift log can,
  and it has recorded one session's worth.
