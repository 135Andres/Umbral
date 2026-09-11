# ARCHITECTURE HYPOTHESES

Status: CURRENT (living) — hypotheses and open questions (T4). Nothing here is chosen.

Category F / G. NOTHING HERE IS SELECTED. No architecture has been chosen. Each entry is
a candidate mechanism with its source, its counter-evidence, and what would have to be
true for it to work. Reading order does not imply preference.

-------------------------------------------------------------------------------
HOW TO READ THIS FILE
-------------------------------------------------------------------------------
- "Claim" = what the corpus asserts.
- "Counter" = the strongest opposite evidence in the corpus.
- "Would depend on" = the empirical or product question that decides it.
- A hypothesis with no counter-evidence still has no validation.

-------------------------------------------------------------------------------
H1 — Filesystem as source of truth, internal index as disposable projection
Claim: keep user files authoritative; maintain a derived, rebuildable internal index;
never write semantics the user did not author into their files.
Source: S5 §Approach 2 (recommended over database-authoritative alternatives),
S9 §Approach 2, S8 §16, S11 §16.
Counter: S9 Evidence 1 and S11 §9 document that this exact model failed in practice for
Logseq when internal state and body text had to stay coherent; the failure was the
reason for a full rewrite to a database-first model.
Would depend on: Q1/Q2 — what is genuinely derivable and what cannot be, and whether
non-derivable state can live outside user files without creating the two-writers problem.

-------------------------------------------------------------------------------
H2 — Two-tier split between durable user data and derived state
Claim: separate the storage primitives by rebuildability — source of truth, event log,
state database, derived state, cache, index, materialised view — and let each live in the
substrate appropriate to it.
Source: S5 §Separation of Storage Primitives, S6 §Architectural Implications, S4 §16.
Counter: none in the corpus; but the taxonomy is a research construct, and no candidate
implementation of the boundary is proposed beyond a directory name.
Would depend on: Q2.

-------------------------------------------------------------------------------
H3 — Tiered identity: declared identifier, then content hash, then heuristic
Claim: identity = declared identifier in the file, falling back to content hash, then to
similarity heuristics, then to new-entity classification. Duplicate identifiers must be
tolerable, not fatal.
Source: S1 §7 pipeline, S4 §5, S5 §Failure Mode 2.
Counter: injecting identifiers into user files is exactly the "syntax pollution" S1 §9
identifies as a lock-in mechanism in Logseq Classic.
Would depend on: whether usable identity is achievable without writing anything into
user files (Q10) — the requirement (A4) and the mechanism (H3) are not obviously
compatible.

-------------------------------------------------------------------------------
H4 — Semantic metadata in-file, derived metadata in-index
Claim: user-authored semantics go in the file (frontmatter/attributes); inbound links,
counts, embeddings, taxonomy scores and AI inferences stay derived and disposable.
Source: S1 §Approach B + §10.4, S4 §Entity Record, S5 §Evaluation.
Counter: S4 §8 records that frontmatter is not usable across non-text formats (source
code, CSV, media) — the pattern cannot be universal; a sidecar alternative reintroduces
orphan/collision risk (S1 §Approach C, S5).
Would depend on: Q2, and on a decision about non-text assets (Q14).

-------------------------------------------------------------------------------
H5 — Deterministic structural retrieval first, embeddings optional and selective
Claim: lexical + syntax/AST-derived retrieval covers the majority of real retrieval; dense
vectors are an auxiliary, applied selectively to prose, never universally.
Source: S2 §6/§17, S4 §16 (structural before semantic), S11 §7/§8, S10 §Struct.
Counter: S2 §5.1 records the structural failure of lexical search — vocabulary mismatch,
i.e. an agent that does not know the words cannot find the file. That failure is the case
for embeddings, and selective embedding does not cover arbitrary corpora where the user
does not annotate.
Would depend on: H8 (actual scale) and Q5 (multi-hop retrieval).

-------------------------------------------------------------------------------
H6 — Context assembled as a bounded, curated, versionable artifact
Claim: the interface to a model is an assembled artifact with boundaries, budget and
provenance, not an ad-hoc question over whatever fits.
Source: S2 §16, S3 §Context Packages (hybrid lifecycle preferred).
Counter: S11 §6/§17 retires bespoke packaging in favour of an open protocol plus
declarative instruction files; S11 also warns that anything requiring maintenance decays
(C9).
Would depend on: Q6 — whether the artifact is a protocol payload, a persisted document,
or both.
NOTE: the corpus uses "Context Package" both as a product concept and as a schema. These
are different claims and must not be merged. See TENSION T1.

-------------------------------------------------------------------------------
H7 — Background/persistent agents with mediated authority
Claim: agents can run continuously and across sessions, acting only inside granted scope,
with containment and reversible operations.
Source: S3 §Multi-agent, S6 §Background Agents, S7 §17.
Counter: S11 §17-7/§17-10 rejects proactive intelligence and defers background agents
outright; S10 §17 defers visual workflow builders and native binary loading. The
strongest anti-recommendation in the corpus is against exactly this hypothesis.
Would depend on: Q7 — whether the authority model can be demonstrated before autonomy is
built. Sequence matters more than the mechanism here.

-------------------------------------------------------------------------------
H8 — Scale envelope hypothesis
Claim: the design must hold from 10^4 to 10^6 files, which implies certain classes of
mechanism (precomputed inverted index; disk-backed quantised vector store; parallel
traversal) and rules out others (naive live vector indexing; universal embedding).
Source: S2 §10, S3 §Scalability, S5 §Scalability.
Counter: the numbers are benchmark-derived (Linux kernel, Chromium), not
use-case-derived; S11 §8 argues the practical tooling answer (lexical + AST + fuzzy) is
sufficient for personal corpora with an order of magnitude less RAM cost.
Would depend on: Q8 — the answer determines how much of the candidate architecture
survives at all. This is the single highest-leverage unknown in the corpus.

-------------------------------------------------------------------------------
H9 — Versioning: ambient local capture, with Git as a projection
     (CANDIDATE ONLY — no versioning model is selected; the repository being a git repo
      is documentation infrastructure, not this decision)
Claim: continuous local checkpointing independent of the version control system, with
clean commits projected outward on demand, both implemented with existing primitives.
Source: S6 §Exec/§Approach D/§Recommendations 1, S11 §17-5 (integrate rather than build).
Counter: S6 itself asks whether this is reinventing Git and answers both ways. The
corpus's own recommendation is to reuse primitives, not to write a versioning engine.
Would depend on: Q4.

-------------------------------------------------------------------------------
H10 — Multi-tier permission model
Claim: several operational sensitivity levels (per-action, mode-based, scoped
auto-approval, unattended) with deterministic enforcement outside the model.
Source: S7 §17.
Counter: S11 §17-12 rejects fine-grained matrices as fatigue-inducing and recommends two
modes (read/plan vs. act). Constraint C10 supports the rejection.
SUPERSEDED AS A COUNTER (2026-09-10): the user's own words require the layered model
(MC §24, recorded as UD-008; tension T3 resolved by user). C10's evidence still bounds the
DESIGN — the layers must not become per-action prompting — but it no longer argues against
the layered intent. What remains open is the mechanism (Q7).
Would depend on: Q7.

-------------------------------------------------------------------------------
H11 — Dual-boundary security: deterministic enforcement plus kernel containment
Claim: automated action is only safe when the semantic layer evaluates policy out of band
and the execution layer is confined by the operating system.
Source: S7 §17, S11 §11, S10 §11.
Counter: no counter-evidence; the corpus is unusually consistent here. Residual doubt:
the enforcement primitives are platform-specific and at least one platform's traditional
mechanism is deprecated (S7 §8).
Would depend on: Q12.

-------------------------------------------------------------------------------
H12 — Protocol-mediated capability surface
Claim: expose capabilities as tools through an open protocol (the corpus names MCP),
rather than through the application's private interface.
Source: S3 §13, S10 §3/§16, S11 §17-2.
Counter: none stated. But note this is simultaneously a technology reference, a design
choice and an ecosystem bet; if it is selected it should be selected explicitly, with its
counterfactual stated.
Would depend on: protocol-specific research, not yet done.

-------------------------------------------------------------------------------
H13 — Headless core with replaceable interfaces
Claim: one core engine, no UI coupling, with desktop / CLI / server / web surfaces
against it; extension logic sandboxed and capability-scoped.
Source: S9 §Recommendations, S10 §16/§17.
Counter: S9 §Recommendations 10 and S11 §8 note that the distribution models have
asymmetric constraints (web cannot reach the filesystem), so "one core, all surfaces" is
a real cost even if it is a real benefit.
Would depend on: Q11.

-------------------------------------------------------------------------------
TENSIONS (T)
-------------------------------------------------------------------------------
T1  "Context Package" as product concept (S2 §16, S3, S4) versus S11's verdict that
    context packaging should be delegated to an open protocol instead of a bespoke
    format (S11 §6/§17-2). Both survive; the schema in S3 §Context Packages must NOT be
    read as a specification until this is settled.
T2  Convergent "build only what is different" principle, divergent answers about what
    that is: S1/S3 name an identity/semantic overlay engine; S2 names an assembler and
    change feed; S4 resists; S6 builds an engine; S9 keeps filesystem-first with a
    daemon; S10 10 names a microkernel mediator; S11 names an AST+protocol projection
    engine. Convergent values, competing priorities.
T3  S7's multi-tier permission model versus S11's two-mode simplification.
    RESOLVED BY USER (2026-09-10): MC §24 explicitly wants the layered model
    (NORMAL / CONFIRMATION / SEMI-BYPASS / FULL BYPASS) with semi-bypass covering
    edit/move/delete. S11's simplification is overridden; the semantics of each layer
    remain research (MC §43-Q10/Q11).
T4  S8's view layer (tables, boards, timelines, local graph) versus S11's rejection of
    dashboards and graph views.
    RESOLVED BY USER (2026-09-10): MC §7/§8 explicitly want dashboards, kanban, graph and
    timeline views plus a customizable command center. S11's evidence (graph hairballs,
    dashboard maintenance cost) is retained as design constraint, not as product verdict.
T5  Filesystem authority (H1) versus non-derivable state (S9 §4). The most dangerous
    tension in the corpus: the recommended architecture is the one documented as failing.
T6  Scale envelope: S2/S5 treat 10^4-10^6 files as the design constraint, while S11 §8
    argues personal corpora need an order of magnitude less and that the heavy mechanisms
    are unjustified. RESOLVED AS POLICY BY USER (2026-09-10): MC §37 — do not prematurely
    optimize for massive scale, and do not make choices that unnecessarily prevent
    scaling later. The empirical question (what the product must actually sustain on the
    target hardware) remains open: Q8 / RESEARCH-AGENDA Phase 3.

-------------------------------------------------------------------------------
NOT SELECTED — AND MUST NOT BE SELECTED YET
-------------------------------------------------------------------------------
The corpus names many technologies. Every one of them is a reference or candidate
(I / J), never a decision: SQLite, FTS5, sqlite-vec, Tantivy, LanceDB, DuckDB, LMDB,
RocksDB, HNSW/IVF-PQ, Tree-sitter, Comrak, MCP, W3C PROV, JSON Canvas, UUIDv7/ULID,
BLAKE3, FastCDC, libgit2/gitoxide, Jujutsu, Pijul, Fossil, Dolt, yrs/Loro/Automerge,
Cedar, OPA, Biscuit, Landlock, Bubblewrap, seccomp, Seatbelt, AppContainer, WASI,
Wasmtime/Extism, QuickJS, Tauri, Axum/Tokio, LiteLLM, Ollama/llama.cpp, OpenRouter,
cmdk, Sigma.js/Cosmograph.
Selecting any of these inside this file — or anywhere in this repository before the
research agenda is worked — would convert a candidate into an authority it has not earned.

-------------------------------------------------------------------------------
CANDIDATE ARCHITECTURES SEEN IN THE CORPUS (unnamed by the project, unresolved)
-------------------------------------------------------------------------------
The corpus proposes at least three families, mutually incompatible in emphasis. None is
preferred here. Each is a research input, and each can be traced to its source.

  Family 1 — Overlay index over the user's files.
    Semantics in file metadata, disposable relational index, views as queries.
    Source: S1 §5 Approach E, S1 §21, S5 §Approach 2.
  Family 2 — Retrieval and inference core with a protocol surface.
    Deterministic structural/lexical retrieval, selective semantics, protocol-mediated.
    Source: S2 §17, S3 §17, S11 §17, S10 §17.
  Family 3 — Database-first with a filesystem projection.
    Database as system of record, files as an export format, kept in sync.
    Source: S9 §Approach 1 (presented as a failure mode), S5 §Approach 1.
    Status: the corpus rejects it; it is recorded because the rejection is a comparison,
    and because all three must be compared against the same criteria.

CONSTRAINTS NOTE: everything in this file is provisional and, more importantly, so is
any future selection that precedes a scope decision (Q8) and an authority decision (Q1).
