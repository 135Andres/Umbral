# INGEST NOTES

Status: HISTORICAL RECORD (T7), frozen 2026-09-10. This is the ingestion record of the
first bootstrap session: what was extracted from the research corpus, what was deliberately
not persisted, and why. It is kept because the reasoning behind a discarded item is
evidence, not because it describes current state. Do not update it; supersede it with a
new dated record if needed. Where this file and a canonical file disagree, the canonical
file wins — this one records what was believed at ingestion time.

Known drift in this file, left in place as history: it was written before the Master
Context was recovered, so §1 counts A1-A10 (now A1-A11), it describes 11 files (now more),
and its §4 tension list (T1-T7) predates the canonical list. T1-T5 are canonical in
ARCHITECTURE-HYPOTHESES.md; T6 was added there during the documentation audit; T7 is folded
into T1 (both concern "context package" being used as product concept and as schema).

Record of what was extracted from the corpus, what was deliberately not persisted, and
why. This file exists so that a future session does not re-derive these classification
decisions, does not resurrect material that was intentionally dropped, and does not
mistake a discarded example for a requirement.

--------------------------------------------------------------------------------
1. CLASSIFICATION SUMMARY (classes A-M, per S0 §5)
--------------------------------------------------------------------------------
A  PRODUCT INVARIANT ............ 10 items -> INVARIANTS.md (A1-A10)
B  PRODUCT VISION / NORTH STAR .. 3 competing formulations -> VISION.md
C  USER REQUIREMENT ............. 13 core/formative + 8 candidate -> REQUIREMENTS.md
D  DESIGN PRINCIPLE ............. 9 product principles + 6 documentation principles
                                  -> PRINCIPLES.md
E  CONSTRAINT .................. 12 constraints -> CONSTRAINTS.md
F  ARCHITECTURAL HYPOTHESIS ..... 13 hypotheses -> ARCHITECTURE-HYPOTHESES.md
G  OPEN ARCHITECTURAL QUESTION .. 15 questions + 5 tensions -> OPEN-QUESTIONS.md
H  RESEARCH QUESTION ............ 7 phases + 9 comparison criteria + 7 named gaps
                                  -> RESEARCH-AGENDA.md
I  REFERENCE / EXAMPLE .......... many; retained as a single inventory in
                                  ARCHITECTURE-HYPOTHESES.md ("Not selected")
J  POSSIBILITY / FUTURE IDEA .... 8 candidates -> REQUIREMENTS.md (J1-J8)
K  IMPLEMENTATION DETAIL ........ NOT persisted; see §3
L  TEMPORARY EXPLORATION ........ NOT persisted; see §3
M  REDUNDANT / DO NOT PERSIST ... identified, not carried; see §3

A further 14th class was written and then removed during the audit: an architect-side
hypothesis synthesised from the corpus rather than derived from it. It was deleted
because every entry in ARCHITECTURE-HYPOTHESES.md must carry a source; a hypothesis with
no source is this repository inventing architecture, which is exactly what it must not do.
Architect-side reasoning belongs in a marked research note, not mixed with corpus extraction.

--------------------------------------------------------------------------------
2. DOCUMENTATION PLAN — WHAT WAS PERSISTED AND WHY
--------------------------------------------------------------------------------
| Information                    | Cat | Persistence | File                     |
|--------------------------------|-----|-------------|--------------------------|
| Purpose, thesis, mental model  | B   | PERMANENT   | VISION.md                |
| Competing north star statements| B   | PERMANENT   | VISION.md                |
| Anti-goals                     | B   | PERMANENT   | VISION.md                |
| Filesystem authority           | A   | PERMANENT   | INVARIANTS.md            |
| Sunset test / survivability    | A   | PERMANENT   | INVARIANTS.md            |
| No mandated taxonomy           | A   | PERMANENT   | INVARIANTS.md            |
| External reorganisation safety | A   | PERMANENT   | INVARIANTS.md            |
| Inference != authority         | A   | PERMANENT   | INVARIANTS.md            |
| Bounded delegated authority    | A   | PERMANENT   | INVARIANTS.md            |
| Rebuildable derived state      | A   | PERMANENT   | INVARIANTS.md            |
| Local-first, provider-neutral  | A   | PERMANENT   | INVARIANTS.md            |
| Decision heuristics            | D   | PERMANENT   | PRINCIPLES.md            |
| Requirements (core/formative)  | C   | PERMANENT   | REQUIREMENTS.md          |
| Candidate features (unweighted)| J   | PERMANENT   | REQUIREMENTS.md          |
| Environment limits (watchers,  | E   | PERMANENT   | CONSTRAINTS.md           |
| identity, instruction load)    |     |             |                          |
| Candidate mechanisms           | F   | PERMANENT   | ARCHITECTURE-HYPOTHESES  |
| Source tensions                | G   | PERMANENT   | ARCHITECTURE-HYPOTHESES  |
| Unresolved blocking questions  | G   | PERMANENT   | OPEN-QUESTIONS.md        |
| Research sequence + criteria   | H   | PERMANENT   | RESEARCH-AGENDA.md       |
| Source inventory + authority   | -   | PERMANENT   | research/RESEARCH-INDEX  |
| Classification decisions (this)| -   | PERMANENT   | research/INGEST-NOTES.md |

--------------------------------------------------------------------------------
3. DELIBERATELY NOT PERSISTED
--------------------------------------------------------------------------------
These were classified K / L / M or J-without-weight and left out on purpose. They are
recorded here only so the decision is visible. None of them should reappear as a
requirement, a specification or an architecture without new evidence.

3.1 Technology names (category I) — NOT persisted as selections.
    SQLite, FTS5, sqlite-vec, Tantivy, LanceDB, DuckDB, LMDB, RocksDB, HNSW, IVF-PQ,
    Tree-sitter, Comrak, MCP, W3C PROV, JSON Canvas, UUIDv7/ULID, BLAKE3, FastCDC,
    libgit2, gitoxide, Jujutsu, Pijul, Fossil, Dolt, yrs, Loro, Automerge, Cedar, OPA,
    Biscuit, Landlock, Bubblewrap, seccomp, Seatbelt, AppContainer, WASI, Wasmtime,
    Extism, QuickJS, Tauri, Axum, Tokio, LiteLLM, Ollama, llama.cpp, OpenRouter, cmdk,
    Sigma.js, Cosmograph.
    Reason: appearing in a research report is not selecting it. They are kept as one
    inventory entry, explicitly marked "not selected", so the list is not lost without
    becoming an implied decision.

3.2 Implementation detail (category K) — NOT persisted as specification.
    - Concurrency PRAGMAs, WAL settings, busy timeouts.
    - Debounce windows (300-500 ms), chunk sizes, overlap percentages.
    - Merkle-tree change detection, FastCDC parameters, hashing choices per tier.
    - Pipeline phase lists (8-phase context lifecycle, 5-stage ingest).
    - Physical layout suggestions (cache directories, object-store sharding).
    - SQL table sketches for provenance.
    Reason: these are plausible engineering choices for an architecture that does not
    exist yet. Persisting them as documentation creates false authority. They belong in
    a future implementation or research report, reproduced from source when needed.

3.3 Temporary exploration (category L) — NOT persisted.
    - The eight-phase context lifecycle walked end to end.
    - The five-stage pipeline diagrams (filesystem -> structural -> semantic -> engine ->
      package -> model).
    - The nine-concept distinction table (undo / checkpoint / snapshot / version /
      history / backup / branch / merge / provenance).
    - The per-scale feasibility narratives (10^4 / 10^5 / 10^6).
    - The "five-minute experience" and "six-month power user" walkthroughs.
    - The comparison matrices of twelve knowledge-management products.
    Reason: they are reasoning aids, not project knowledge. The conclusions they support
    are captured elsewhere; the derivations are not worth maintaining. The product
    comparison matrices are the one partial exception — they are the raw material for a
    future competitive analysis and should be re-derived from sources when needed, not
    archived here in a form that will silently age.

3.4 Examples used to explain a concept — NOT converted into requirements.
    - The "technical specification that is simultaneously a deliverable, evidence and a
      blocker" (used to illustrate polyhierarchy).
    - The project/task/decision/evidence entity examples.
    - The poisoned-repository, halved-cleanup-command, compromised-MCP-server and
      symlink-escape scenarios (security illustrations).
    - The "Coffee with Anna" style onboarding narrative (generic).
    Reason: they illustrate a principle and are not requirements. The security scenarios
    are retained only as test cases referenced by Q7, because they are the corpus's own
    failure-mode catalogue rather than an illustration.

3.5 Redundant (category M) — identified, not carried.
    - The same anti-pattern list appears in S1, S3, S8, S9, S10 and S11 with different
      wording. Collapsed into the invariants and the hypotheses' counter-evidence.
    - "Two-master synchronisation" appears in at least five reports. Recorded once (C3).
    - Atomic-save / inode behaviour is explained in S1, S5 and S9. Recorded once (C2).
    - The argument "filesystem-first versus database-first" is restated in every report.
      Recorded once as candidate families, not as a running debate.
    Reason: repetition consumes maintenance and creates the illusion of independent
    confirmation. Rule applied: repetition is not validation.

3.6 Unevidenced or contested claims that were kept OUT of the constraints and
    requirements files:
    - Specific UX latency budgets ("first paint under 16 ms", "fuzzy search under 15 ms").
    - Specific memory footprints per scale tier.
    - Specific benchmark figures presented as design limits.
    - Specific permission-fatigue percentages and instruction-count thresholds.
    Reason: kept where they illuminate direction (C6, C10, C11); excluded where they
    would function as targets. Any number that will be a product promise must be
    re-measured on the target hardware (RESEARCH-AGENDA 3.1).

--------------------------------------------------------------------------------
4. CONTRADICTIONS AND TENSIONS FOUND (preserved, not resolved)
--------------------------------------------------------------------------------
T1  Context packaging: product concept with a schema (S2, S3, S4) vs. delegation to an
    open protocol (S11).
T2  "Build only the differentiator" is stated in five reports with five different
    answers about what the differentiator is.
T3  Permission model: multi-tier sensitivity levels (S7) vs. two operational modes (S11).
T4  View layer: tables, boards, timelines and local graph (S8) vs. rejection of
    dashboards and graph views (S11).
T5  Filesystem authority (S1, S5, S9) vs. non-derivable state (S5, S9) — the corpus
    recommends the architecture it also documents as having failed (Logseq).
T6  Scale envelope: 10^4-10^6 files treated as the design constraint (S2, S5) vs.
    arguments that personal corpora need an order of magnitude less (S11).
T7  Context Packages: S2 describes them as the mechanism; S11 §17 rejects bespoke
    formats. Also, the same term is treated as product concept and as schema — the two
    claims must not be merged.

--------------------------------------------------------------------------------
5. MISSING CONCEPTS (not present anywhere in the corpus)
--------------------------------------------------------------------------------
- A definition of success, a quality metric or an acceptance threshold.
- Governance, licence, stewardship, contribution model — despite "open source" being a
  premise.
- Business or sustainability model, and therefore cost constraints.
- The intended user, stated once and consistently.
- Any accessibility, i18n or platform-parity requirement.
- Cost of operation (model spend per unit of work) as a design input.
- Testing, verification or release strategy.
- A stated minimal product: every report proposes its own MVP, and no two agree.
- An explicit list of what the product will NOT do, from the user's side (the anti-goals
  in VISION.md are research opinions, not user statements).

--------------------------------------------------------------------------------
6. AUDIT OF THIS DOCUMENTATION SET
--------------------------------------------------------------------------------
Performed against the four-phase mandate (Phase 4 audit questions).

1  Duplication? Removed by construction: each fact lives in one file. Known residual: the
   invariants summarise constraints they depend on (A10 relies on C3), and the
   architecture file restates some invariants as hypothesis dependencies. Both are
   references, not copies. SUPERSEDED: the tension list used to be duplicated here and in
   OPEN-QUESTIONS.md; the canonical list is now T1-T6 in ARCHITECTURE-HYPOTHESES.md /
   OPEN-QUESTIONS.md, and §4 below is retained only as the ingestion-time record.
2  Hypothesis presented as decision? No. No file asserts a selected architecture, and
   ARCHITECTURE-HYPOTHESES.md states the prohibition explicitly.
3  Research presented as requirement? No. Requirements carry status labels; the
   recommendation-derived ones are marked RESEARCH OPINION. The one risk is R1-R7, which
   read as plain requirements while being corpus premises — flagged at the head of the
   file and repeated in the report.
4  Example converted into requirement? No. See §3.4.
5  Important idea lost? Two accepted losses: (a) the competitive comparison matrices,
   intentionally left to be re-derived (§3.3); (b) H14, an architect-side synthesis, was
   written and then removed because it had no source. All source-derived concepts are
   landed. Highest-value items verified present: filesystem authority, no mandated
   taxonomy, rebuildable index, identity under churn, provenance and epistemic state,
   authority of automation, context economy, provider neutrality, external-mutation
   coherence.
6  Any file that should be merged? Candidates: PRINCIPLES.md and INVARIANTS.md could
   merge (both are "properties we hold"). Kept separate because invariants are about the
   product and principles are about how to decide — merging them would blur a distinction
   that the corpus itself blurs, and this project's whole problem is blurred distinctions.
   Re-evaluate after the first decision.
7  Any file that should be deleted? No file currently fails the "would losing it damage
   future work" test. DECISIONS.md was not created — see the final report.
8  Comprehensible to a human? Twelve small files, one purpose each, read order declared in
   README.md. The heaviest file is ARCHITECTURE-HYPOTHESES.md (~14 KB).
9  Efficient for an AI? Flat, plain Markdown, no nesting beyond one directory, explicit
   labels (A1, C3, H5, Q8, T2), so any claim can be referenced in one token without
   loading the whole set.
10 Is the filesystem still the primary human model? Yes for the project's own
   documentation: plain files, plain folders, no database, no proprietary container.
11 Is user authority preserved? [UPDATED 2026-09-10 after the MC recovery] Yes. The
   charter (research/sources/PROJECT-MASTER-CONTEXT.md) is now the source of the user's
   intent, decisions are recorded as UD-001..UD-010, and research verdicts that conflicted
   with the charter were explicitly subordinated to it. The remaining gap is sequencing,
   not authority: which user to serve first (0.7) and the MVP boundary (0.8).
12 Is uncertainty preserved? Yes — statuses, tensions, "undefined at vision level",
   named gaps, and explicit non-tests.
13 Is provenance preserved? [UPDATED 2026-09-10] Yes for the charter: MC is archived
   verbatim. Partially for the research corpus: each claim cites S1-S11 and a section, but
   those texts are not yet archived (RESEARCH-INDEX gap 2), so those citations still point
   at a transcript. Fixing that is one command, awaiting user authorization.
14 Is metadata being treated as authority? No: research status, citation and
   classification are all recorded as provenance, and none of them is used to promote a
   claim above its status. Repetition across reports explicitly does not raise status.
