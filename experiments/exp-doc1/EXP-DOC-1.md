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
T1 / T2 RESULTS — FOUR READERS COMPLETE (2 pairs, dispatched independently)
================================================================================
Raw evidence: navigation-run-{1,2,3}.md and authority-run-{1,2,3}.md in this directory
(delegate reports preserved verbatim; run 3 = the third reader of each kind, dispatched in
a second independent pair). NOTE ON NUMBERING: runs 2 and 3 were both dispatched before the
run-1 fixes landed, so each reports defects that were already corrected when it reported;
every finding is marked inline in its own record.
Validity note: run 2 was dispatched before the run-1 fixes landed, so a few findings it
reports were already corrected when it reported; each is marked in its record. Run 2's
navigation reader self-disclosed a confound (it loaded the project skill first, giving it
a file map) and reported its file count as a lower bound — so the navigation half is
weaker evidence than the authority half.

CROSS-RUN MEASUREMENTS
  Navigation: 22 files (run 1) / 16 files (run 2, confounded); mean per question 3.9 and
  3.5; median 3.5 both. Unanswerable: 0/10 both. Budget (<=3/question): MARGINAL both.
  Authority: 0 authority errors in both runs; 3 temptations named in run 1, 10 misled-risk
  items in run 2. Run 2's stricter standard found 3 concrete single-file traps that run 1
  did not.

RUN 1 (see the run-1 records for the full detail)

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

RUN 2 (see navigation-run-2.md, authority-run-2.md)
  Navigation: 0 unanswerable; Q9 ("top risks") had no single home (FIXED: PROJECT-DIRECTION
  gained a risk block); independently re-confirmed the run-1 currency defect.
  Authority: 0 errors in its own answers, but it listed 10 misled-risk items and identified
  THREE places where a single-file reader reaches a wrong answer:
    (a) REQUIREMENTS.md R6 asserted no user-facing provenance requirement exists
        (contradicted by MC §40) — FIXED with a dated correction;
    (b) REQUIREMENTS.md R12 framed the permission model as contested research with no
        reference to UD-008 — FIXED (upgraded to USER INTENT, mechanism left open);
    (c) frozen historical files (INGEST-NOTES, AUDIT) sit beside living artifacts in
        research/, distinguished only by a status line — NOT FIXED (structural; recorded).
  Also fixed from run 2: ARCHITECTURE-HYPOTHESES H10 now marks S11's counter as superseded;
  harness.py header and results.json now carry an explicit "this is not a technology
  choice" disclaimer at the artifact that carries the misleading signal.
  Its verdict: "superseded research positions were left standing as live statements in
  otherwise-current files — the same confusion one level down."

RUN 3 (see navigation-run-3.md, authority-run-3.md)
  Navigation: 18 files, 0 unanswerable, mean 3.6/question (MARGINAL again). New structural
  finding: 22 fixture files could be mistaken for project memory (third report of the class
  -> trigger met -> fixed). Second report that Q6/Q9/Q10 have no single home.
  Authority: 0 errors, 10 misled-risk items, verdict PARTIAL SUCCESS. New and valid: the
  decision log has NO record for provenance, so a reader of DECISIONS.md alone cannot find
  the answer (fixed by documenting the gap, not by inventing a decision). Its most useful
  sentence: "the repository's front door is disciplined and its back rooms are not."

CROSS-RUN CONVERGENCE (4 readers)
  All four reached the correct authority for every probe and made 0 authority errors.
  All four found the navigation budget marginal (mean 3.5-3.9 files per question vs 3.0).
  All four converged on the same mechanism-level conclusion: the architecture works when
  read in order and fails when a single file is read alone, and every such failure was a
  SUPERSEDED STATEMENT LEFT STANDING IN AN OTHERWISE-CURRENT FILE.
  Verdicts ranged from "largely succeeds" to "mostly holds, but not for the reason it
  claims" — i.e. the stricter the reader, the more defects found, which is the argument for
  the n=2-pairs design.

VERDICTS (thresholds pre-registered above; all four runs considered)
  H-DOC-1: PARTIAL PASS. Comprehension high, nothing unanswerable in 20 questions across
  two readers; the navigation-cost threshold was missed in both runs (marginal), and the
  navigation evidence is weakened by run 2's self-disclosed confound.
  H-DOC-2: PARTIAL PASS, not the PASS run 1 suggested. Zero authority errors were made by
  ANY of the four readers, but the stricter standards found single-file traps that run 1
  missed: two in REQUIREMENTS (fixed), one structural in research/ (fixed), one in the
  decision log's silence about provenance (fixed by documenting the gap). The mechanism
  protected careful readers; it did not make being misled impossible.
  THRESHOLD DEFECT (recorded as a design lesson, not an experiment failure): the
  pre-registered "zero authority errors" criterion measured the READER, not the repository.
  All four readers met it while three of them still reported places where an isolated file
  gives a wrong answer. A better criterion, adopted for future runs: "no single file, read
  alone, yields a superseded answer to a probe question".
  MECHANISM note (not a verdict): across both runs and both batches, every failure was
  CURRENCY or PROVENANCE, never folder structure. Third independent confirmation of the
  documentation audit's central finding — the missing layer was authority/currency, not
  layout.
  EXPERIMENT note: the n=2 design earned its keep. Run 1 alone would have produced an
  over-confident PASS.

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
document as a second authority surface (MR-3); convention decay (MR-1); frozen files
beside living files in research/ (new, structural — the fix would be moving history into
its own directory, which is a structural change this experiment is not authorized to make
on one observation); the technology inventory's shortlist shape (M9 — judged
disproportionate to change, since the heading prevented the trap for both readers); the
git-adjacent non-inference surface (M7 — stated at §0/§7/MR-4, not yet where the signal
is).

WHAT THE EXPERIMENT ACTUALLY SHOWED (one line)
A simple topic-oriented repository does give high comprehension and does not present
research as decisions — but it decays into misleading text wherever a superseded position
is left standing in a file that is otherwise current. The mechanism's failure surface is
currency discipline, and it is invisible to a single reader.

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
