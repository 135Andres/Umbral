# EXP-1 — Semantic projection over an arbitrary filesystem

Status: **RESULT RECORDED** (run 2026-09-10; results.json is the raw output).
Question: EXP-1 in RESEARCH-AGENDA.md Phase 1. Hypotheses H-EXP1-A/B per the stage mandate.
Instrument: experiments/exp1/harness.py (stdlib only: sqlite3 + FTS5; no embeddings, no
model, no network). Corpus: experiments/exp1/fixture-corpus-synthetic/ (renamed from
`corpus-messy` on 2026-09-10 after three readers reported the old name/path could be
mistaken for project memory; the fixtures and the results are unchanged — verified by
re-running the instrument and diffing: retrieval, projection and mutations identical,
only machine timings differ). 36 files, 6 top-level dirs,
10.7 KB — deliberately heterogeneous: decisions, ADRs, bugs, vendor comparison, CSV
numbers, code, HTML mockup, empty file, binary, duplicate specs, stale archive).

---

| | |
|---|---|
| **Purpose** | test whether a semantic projection can be derived over an arbitrary, unorganized filesystem without the user reorganizing anything |
| **Question** | can structure be recovered well enough to retrieve and reconcile, using only content and paths? |
| **Hypotheses** | H-EXP1-A (retrieval useful without any user taxonomy) · H-EXP1-B (identity survives mutation) |
| **Method** | 36-file heterogeneous synthetic corpus; three modes B0 (lexical) / B1 (+links) / B2 (+typed edges, supersession demotion); 8 pre-registered retrieval queries; 10 mutation classes |
| **Evidence** | `results.json` (raw output), `run1.log`–`run5.log` (re-runs; identical results after the corpus rename) |
| **Result** | PARTIAL PASS on both hypotheses — retrieval useful without a taxonomy; identity reconciled across every mutation class; but no abstention, and vocabulary mismatch only weakly mitigated |
| **Limitations** | a Hermes-authored 36-file synthetic corpus; no real user material; sqlite3/FTS5 used as the cheapest stdlib instrument and NOT a technology choice |
| **Conclusion** | the projection premise is not falsified at this scale, and is not validated beyond it |
| **Status** | RESULT RECORDED (2026-09-10) |

Classification per mandate §18: every statement below is labeled.

========================================================================
SETUP
========================================================================
Modes: B0 lexical baseline (BM25 over path+basename+body, OR terms) /
B1 B0 + link graph, 1 hop / B2 B1 + typed edges (supersedes/blocked-by/
related/see) + demotion of superseded docs, 2 hops with decay.
Retrieval tests: 8 queries (Q1-Q8) per mandate §14, graded by accept/partial/
forbidden lists fixed in advance — no plausibility scoring.
Mutations: M1-M10 per mandate §15 (rename, move, dir move, duplicate, delete,
restore, external edit, atomic save, vcs branch switch, vcs return).
Scale: corpus duplicated 1x/10x/50x on this machine.
TWO EARLIER INSTRUMENT RUNS WERE DISCARDED (run1/run2): the first had a broken
link extractor (0 links found) and a basename-collision bug in rename detection
that produced 10 false "identity lost" events; the second fixed extraction but
could not detect in-place edits, restores, or the branch state of a checkout.
Those runs are instrument failures (mandate §17), recorded in git history of this
file's first version and NOT counted as evidence about the hypotheses.

========================================================================
MEASUREMENTS (verbatim from results.json)
========================================================================
Projection (deterministic, content-only): 11 docs with resolvable links,
4 typed edges, 1 superseded doc (decisions-2024.md, superseded by ADR-007),
3 questions with state (2 open, 1 resolved), 6 dates, 2 authors.

Retrieval verdicts (top-3):
| id | class                | B0                     | B1 | B2 |
| Q1 | exact-name           | retrieved-correctly    | same | same |
| Q2 | vocabulary-mismatch  | not-found              | retrieved-below-top3 | same |
| Q3 | conceptual           | retrieved-correctly    | same | same |
| Q4 | relationship         | retrieved-correctly    | same | same |
| Q5 | multi-hop            | retrieved-correctly    | same | same |
| Q6 | irrelevant           | spurious-results       | same | same |
| Q7 | missing-information  | retrieved-correctly    | same | same |
| Q8 | provenance           | retrieved-correctly    | retrieved-correctly | retrieved-below-top3 |

Hop distance of accepted docs under B2: Q2 target at 1 hop; Q5 security review
at 1 hop; all other targets at 0 hops (lexical). No accepted target required
2 hops.

Mutations (hash+path registry; basename+token-similarity fallback >= 0.5):
- M1 rename: rename-or-move detected; 0 stale links.
- M2 file move: detected; 0 stale links.
- M3 directory move (4 files): all 4 detected; 0 stale links.
- M4 duplicate: collision detected (identical content, 2 live paths).
- M5 delete: reported as lost (expected loss).
- M6 restore: restored (hash match re-links the same entity).
- M7 external edit: content-changed-in-place (exact path, basename match).
- M8 atomic save (tmp+rename over original): detected as content-changed-
  in-place — the inode was destroyed and it did not matter, because identity
  was tracked by path+hash, not inode.
- M9 vcs branch switch (directory renamed by a checkout): detected as 1 edit +
  3 renames; 0 stale links.
- M10 vcs return: same, reversed. 0 stale links in all 10 mutations.
- 0 unexpected identity losses across all mutations.

Scale (this machine only — NOT a general benchmark):
copies  files  scan_s   cold_index_s  query_ms  naive_reindex_1edit_s
1       36     0.006    0.018         0.24      0.006
10      360    0.045    0.083         0.14      0.025
50      1800   0.13     0.30          0.34      0.124

========================================================================
OBSERVATIONS
========================================================================
O1. The link graph was thin: 11 docs with links, 4 typed edges in a corpus
    deliberately written with cross-references. Most relationships in real
    prose are not links.
O2. The typed "supersedes" edge worked and correctly demoted
    decisions-2024.md under Q8 — at the cost of demoting it BELOW top-3,
    which made Q8's primary answer harder to reach (B2 regression).
O3. Vocabulary mismatch (Q2) was the only query where the graph helped at
    all: latency-notes.md was reached at 1 hop from metrics.json.
O4. The abstention test (Q6) produced low-confidence but nonzero scores
    (top 2.36 vs. 4.15-6.59 for real queries) in all modes; the instrument
    has no threshold and therefore always returns something.
O5. Identity by path+hash+basename-similarity survived all 10 mutations with
    zero unexpected losses and zero stale links in this corpus.
O6. Inode destruction (atomic save, checkout) was invisible to the tracker
    because nothing was keyed on inode.
O7. Restoring a deleted file re-associated with the SAME entity (hash match)
    rather than creating a new one.

========================================================================
INTERPRETATIONS (Hermes — explicitly not measurements)
========================================================================
I1. H-EXP1-A is supported, weakly and narrowly: on a 36-file corpus a
    content-only projection plus plain BM25 answered 6/8 queries correctly
    with zero user-supplied taxonomy. The two failures are informative:
    vocabulary mismatch needs either the graph (weak), embeddings, or a
    model — and abstention needs a score threshold that does not yet exist.
I2. H-EXP1-B is supported on this corpus: hash+path tracking reconciled
    every mutation class in mandate §15 without loss. This says the PROBLEM
    is tractable at this scale with cheap mechanisms — it does NOT say the
    problem is solved (see limitations).
I3. The graph added little at this scale: exactly one query (Q2) improved,
    and the typed demotion caused one regression (Q8). Both effects came
    from the same 4 typed edges. OBSERVATION O1 suggests the limiting factor
    is not graph machinery but how few relationships real files declare.
I4. Multi-hop: no useful query in this corpus needed 2 hops (mandate §16
    answered by measurement, not assumption): the graph question for FSP is
    deferred until a corpus shows a real 2-hop need.
I5. Naive full re-index after a single edit costs 0.124 s at 1,800 files —
    cheap here, but the cost is linear in corpus size and this says nothing
    about 10^5+ files.

========================================================================
LIMITATIONS (what this experiment does NOT demonstrate)
========================================================================
L1. Corpus scale: 36 files, 10.7 KB. All retrieval and identity results are
    small-corpus results. The scale table is 1,800 files maximum.
L2. Single corpus, single machine, single run. No variance measured.
L3. The corpus was written by Hermes. It is realistic in structure but not
    organically grown; a real user corpus may have thinner or denser links.
L4. The identity mechanism (path+hash+similarity) is one cheap candidate,
    not the space: no in-file IDs, no sidecars, no content addressing were
    tried. H3's tiers beyond level 2-3 are untested.
L5. "Useful" was graded against fixed accept lists written by the same
    party that built the instrument — circularity risk, mitigated only by
    pre-registration of the lists before the run.
L6. No duplicate-identity resolution policy was tested (M4 detects, does
    not resolve) — the Org-Roam collision question (S1 §7) is untouched.
L7. No concurrency: all mutations were sequential and quiescent.

========================================================================
FAILURE DIAGNOSTICS (mandate §17 vocabulary)
========================================================================
- H-EXP1-A: PARTIAL PASS. Projection exists and retrieval is useful without
  taxonomy, but abstention fails (O4) and vocabulary mismatch is only
  weakly mitigated (O3).
- H-EXP1-B: PARTIAL PASS at this scale. All mutation classes reconciled
  (O5-O7), but L1/L4/L6/L7 bound the claim severely.
- Instrument itself: two MECHANISM FAILUREs recorded and discarded before
  results were taken (runs 1-2). The published numbers come from run 3.

========================================================================
EXPOSED QUESTIONS (targeted research candidates, not a program)
========================================================================
XQ-1  What score threshold separates real matches from noise? (O4: the gap
      was 2.36 vs 4.15 — a threshold may be learnable, but that is a
      hypothesis, not a finding.)
XQ-2  How dense are links/relationships in ORGANIC corpora? (O1: if real
      corpora have as few links as this one, graph machinery is premature
      and the relationship question shifts to inference — which is exactly
      the class of feature MC §15 says must never auto-become truth.)
XQ-3  Does basename+similarity survive renames WITH heavy edits? (Untested:
      M7/M8 edited in place; no rename+edit mutation was run.)
XQ-4  Duplicate identity resolution policy (L6).
XQ-5  Do embeddings beat the 1-hop graph on Q2-class queries, and at what
      index/maintenance cost? This is the corpus's only graph win, and it
      is the exact trade the corpus (S2 §6) predicts.

========================================================================
PRODUCT / ARCHITECTURE IMPLICATIONS (evidence-bounded only)
========================================================================
- Supports H5 (deterministic-first retrieval) at small scale: the lexical
  baseline alone answered 6/8. It does NOT settle H5 — L1.
- Weakens the urgency of graph machinery at small scale (I3, I4, O1): no
  evidence AGAINST graphs, just no evidence FOR them yet.
- Supports the tractability half of H1/H3 at small scale (O5-O7); does not
  validate any identity ARCHITECTURE (L4).
- Nothing here selects or rejects any technology. The instrument used
  sqlite/FTS5 because it was the cheapest stdlib path; that is a tooling
  fact about the experiment, not an FSP decision.

========================================================================
NEXT EXPERIMENT (smallest, justified by this evidence)
========================================================================
EXP-2 (proposed, not authorized): rename+edit under the identity tracker —
mutate a file's name AND >=50% of its content in one step, measure whether
the tracker preserves, splits, or loses identity, across all tier-3
similarity thresholds. Directly attacks XQ-3 and the weakest part of H-EXP1-B.
Secondary (only if the user wants retrieval depth): EXP-3, XQ-5's embedding
vs 1-hop-graph comparison on the same corpus and queries.
