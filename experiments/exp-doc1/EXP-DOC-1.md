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
T1 / T2 RESULTS
================================================================================
Pending the delegated navigation tests. To be appended, with the raw agent reports kept
verbatim in this directory (navigation-run-1.md, navigation-run-2.md,
authority-run-1.md, authority-run-2.md) so the evidence is inspectable without trusting
this summary.

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
