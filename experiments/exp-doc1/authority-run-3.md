# EXP-DOC-1 — authority-confusion test run 3 (delegate report, condensed)

Instrument: a third independent adversarial reader, no conversation context, read-only,
instructed to try to be misled. Dispatched 2026-09-10 22:13:37, before the run-1 fixes;
findings already corrected are marked inline.

Duration: 179.3 s. Tool calls: 7. Files opened: 17.

## MEASUREMENTS

6 of 6 probes answered with the correct authority; 0 authority errors.
Conflicts found: probes 1, 4, 5, 6; user's side reported winning in each.
10 misled-risk items listed (same count as run 2, different items).
Verdict: PARTIAL SUCCESS — the repository "mostly holds, but not for the reason it claims,
and not uniformly."

## NEW FINDINGS (not present in runs 1-2)

A3-1. **The provenance gap is real, and this reader diagnosed it precisely**: "DECISIONS.md
      is the authoritative answer to probes 1 and 4, but it does not carry a UD for
      versioning (probe 2) or provenance (probe 5) ... probe 5 has no home in DECISIONS.md
      at all and must be assembled from RESEARCH-AGENDA 0.5 + OPEN-QUESTIONS Q13 + MC §40.
      The absence of a UD for provenance is a genuine gap, not a distinction the reader
      failed to make."
      ACTION: DECISIONS.md now carries a "GAPS IN THIS LOG" section naming the provenance
      gap and its unresolved caveat, and explaining why versioning correctly has none. The
      gap is documented, NOT closed: writing UD-011 would silently resolve the MC §40
      caveat that this repository deliberately left to the user.

A3-2. **The front door / back rooms distinction** — the most useful single sentence any
      reader produced: "the repository's front door (README, AGENTS.md, the authority
      ladder) is disciplined and its back rooms (frozen files, file footers, hypothesis
      titles, the .git directory, the experiment harness) are not."
      ACTION: the .git non-inference moved to README's front matter; H9's title now carries
      a "CANDIDATE ONLY" line; frozen files moved to research/history/; the harness and
      results.json carry their disclaimer at the artifact.

A3-3. Structural pull ranked as the strongest wrong signal in the repository: the live git
      repo, against a probe that literally asks whether Git is the versioning architecture,
      with the correction "buried in a PROPOSED document about documentation, not in README
      or DECISIONS." ACTION: fixed at README.

A3-4. Confirms the fixture-corpus hazard independently (see navigation-run-3 N3-2):
      "experiments/exp1/ contains a real, runnable Python instrument that imports sqlite3
      and FTS5, plus raw results.json. A reader who inspects the repository's actual
      contents can reasonably conclude a database and index technology was chosen."

A3-5. Notes that the S11 override is stated in four places but contradicted in two
      (OPEN-QUESTIONS footer, frozen INGEST-NOTES §4) — both already addressed.

## VERDICT (the reader's own words, condensed)

"The claim 'research recommendations, Hermes interpretations and historical records cannot
be mistaken for the user's decisions' is TRUE at the level of the authority ladder and the
decision records, and FALSE as an absolute — a reader who reads any single file in
isolation can be misled on probes 3, 5 and 6, and the repository's structural signals
actively invite the probe-2 error. The failure mode the architecture was built to prevent
(research given veto power over user intent) has been corrected in the record but not yet
purged from every file that still carries the pre-correction text."

## INTERPRETATION (Hermes — not measurement)

- Four readers, four verdicts: "largely succeeds" (run 1), "largely but not fully" (run 2),
  "mostly holds, not for the reason it claims" (run 3), and run 2's stricter variant. The
  convergence is on the same conclusion: the mechanism works when read in order and fails
  when a file is read alone. Every instance of that failure was a superseded statement left
  standing in an otherwise-current file.
- The falsifiable standard in DOCUMENTATION-ARCHITECTURE §14 ("zero authority errors") was
  met by all four readers, but the standard the readers themselves applied — "can a reader
  be misled at all?" — was not. The pre-registered threshold was too weak, and this is
  recorded as a design lesson rather than a failed experiment: the threshold measured the
  reader, not the repository.
- Structural conclusion supported by four independent readings: the topic-per-file
  organization is adequate; what decays is status, currency and provenance discipline, and
  the decay is invisible to the person who wrote the text.
