# EXP-DOC-1 — navigation test run 3 (delegate report, condensed)

Instrument: a third independent delegated agent, no conversation context, read-only.
Dispatched 2026-09-10 22:13:37 — BEFORE the run-1 defect fixes landed, so some findings it
reports were already corrected when it reported; marked inline.

Duration: 178.3 s. Tool calls: 8. Files opened: 18 (+ a directory listing).

## MEASUREMENTS

Files opened: 18 (run 1: 22; run 2: 16, confounded).
Files per question: Q1 3, Q2 4, Q3 4, Q4 2, Q5 3, Q6 4, Q7 3, Q8 4, Q9 4, Q10 5.
Mean 3.6, median 3.5. Unanswerable: 0/10.
Budget (<=3 per question): MARGINAL, consistent with runs 1 and 2.

## FINDINGS

N3-1. NEW AND STRUCTURAL: "the README MAP does not list experiments/ at all, so EXP-1 and
      the friction log had to be found by directory listing." True when it ran (fixed at
      the map's next edit); this is the third reader to hit the same class.

N3-2. NEW AND STRUCTURAL — the strongest new finding of the whole experiment: "the
      repository contains 44 .md files, 22 of them under experiments/corpus-messy/ which
      are experiment fixture data that superficially resemble project documents
      (README.md, open-questions.md, decisions-2024.md, research/*). A reader who lists
      the tree before reading the top-level README can mistake that corpus for project
      memory." The reader did not open any of them — but it explicitly flagged the risk,
      and it is the third independent report of this class (batch 1 F3, batch 2 M8, here).
      ACTION: trigger met, structural fix applied — see DOC-FRICTION-011.

N3-3. Q6 (uncertainty), Q9 (risks) and Q10 (next steps) each cost 4-5 files because they
      are distributed across OPEN-QUESTIONS, ARCHITECTURE-HYPOTHESES, CONSTRAINTS,
      RESEARCH-AGENDA, PROJECT-DIRECTION and the audit. This is the second independent
      report that these three questions have no single home (batch 1 said the same of Q9).
      ACTION: PROJECT-DIRECTION already gained a risk block for Q9; Q6/Q10 remain
      distributed by design (they are genuinely cross-cutting). Recorded, not restructured.

N3-4. Authority ambiguity was reported as "mostly avoidable", with three named places:
      PROJECT-DIRECTION's stale "nothing empirical yet" (already fixed); the dated update
      blocks (MR-2, known); DOCUMENTATION-ARCHITECTURE as a self-declared second authority
      surface (MR-3, known). The reader noted the documents "self-label the ambiguity, so
      the confusion was flagged rather than silent".

N3-5. Positive: the ID namespaces were used as designed — "worked well for cross-checking
      a claim in a second file without re-reading whole documents." The README map's
      per-file one-liners were "the main tool for deciding which to open."
