# RESEARCH INDEX

Inventory of the supplied source corpus: 11 AI-assisted research reports plus the project
mandate and bootstrap brief. Every report is a research artifact produced by a language
model, not a user-authored specification, not a validation, and not an authority.

-------------------------------------------------------------------------------
AUTHORITY STATUS OF THE WHOLE CORPUS
-------------------------------------------------------------------------------
  Status: RESEARCH MATERIAL.
  - Agreement between reports is convergent reasoning, not confirmation.
  - Repetition across reports is repetition, not validation.
  - Numbers are borrowed from public benchmarks (Linux kernel, Chromium, published
    embedding throughput), not measured on the intended use case.
  - Recommendations in these reports are unratified. None is a decision.
  - Where two reports disagree, the disagreement is preserved (see the tension list in
    OPEN-QUESTIONS.md and ARCHITECTURE-HYPOTHESES.md).

-------------------------------------------------------------------------------
S0  Project mandate + workspace bootstrap brief + Master Context
    Provides: the product framing (filesystem-first, local-first, AI-native, no mandated
    taxonomy), the classification system in use here, the epistemic rules, and the
    founding product charter.
    RECOVERY (2026-09-10 audit): the full Master Context was recovered from the project's
    original working session (2026-09-10) and archived verbatim. It is PRIVATE — the
    owner's founding charter — and is deliberately not part of this repository, so its
    `MC §n` citations cannot be checked here. The earlier note that it existed only as a
    theme index is obsolete. MC is the primary user-intent source for this repository
    (cited as "MC §n" throughout the project files); the canonical documents are its public
    representation.
    Status: USER INTENT for the product framing and epistemic rules; MC statements are
    user-owned vision whose architecture-level items remain hypotheses by MC's own
    declaration (§16, §42, §52).
    Contributes: the north star (§51), target user (§1, §37), UX priorities (§38),
    anti-goals (§1, §39), invariants A1-A11, the permission-layer intent (§24),
    "Git without Git" (§26), context-package hybrid preference (§12), proactive
    intelligence intent (§18), dashboard/view intent (§7-8), onboarding principle (§34),
    business model (§36), scale policy (§37), and the classification/status systems.

-------------------------------------------------------------------------------
S1  Knowledge and Documentation Architecture: Reconciling Filesystem Primacy with
    Semantic Knowledge Graphs
    Scope: the semantic overlay itself — the filesystem/graph tension, identity under
    filesystem churn, relationship persistence, portability, and five candidate
    approaches to the semantic layer.
    Status: RESEARCH MATERIAL. Recommends an asynchronous relational overlay.
    Contributes: A1, A2, A3, A4, C1, C2, C4, H1, H3, H4; the anti-pattern list;
    the five knowledge architectures (Family 1 of the candidate families).

-------------------------------------------------------------------------------
S2  AI Context Engineering for Large-Scale Project Workspaces
    Scope: retrieval and context assembly at scale — lexical vs. structural vs. dense
    retrieval, context compression, the case for and against a dedicated context engine.
    Status: RESEARCH MATERIAL. Recommends structural/lexical first with selective
    embedding; also the source of the 10^4-10^6 scale envelope.
    Contributes: C5, C6, H5, H8; the context budget problem; the scale tables.

-------------------------------------------------------------------------------
S3  Provider-Neutral AI Operating Layer for Local-First Filesystem Workspaces
    Scope: how automation, providers and sessions are mediated — gateway vs. embedded
    adapters, tool protocols, session portability, multi-agent coordination.
    Status: RESEARCH MATERIAL. Recommends embedded in-process adapters over a gateway;
    recommends bifurcating session state.
    Contributes: A9, H6, H7, H12; the session portability problem (R10).

-------------------------------------------------------------------------------
S4  Semantic Model, Identity, Provenance, and Epistemic State in Filesystem-Oriented
    AI Workspaces
    Scope: how meaning, identity, provenance and uncertainty are represented; the
    three-axis state model (epistemic / lifecycle / authority); negative assertions.
    Status: RESEARCH MATERIAL. The most epistemically careful report in the corpus.
    Contributes: A5, A10, C8, H4; the state model as a candidate; notes that cryptographic
    provenance proves origin, never truth.

-------------------------------------------------------------------------------
S5  Filesystem and Internal Index Architecture
    Scope: what belongs in the filesystem versus an internal engine; rebuildability
    tiers; watcher behaviour; the SQLite-monolith boundary analysis.
    Status: RESEARCH MATERIAL. Recommends filesystem-authoritative with an ephemeral
    index; documents the failure of the same model in practice.
    Contributes: A10, C3, C4, H1, H2; the separation of storage primitives.

-------------------------------------------------------------------------------
S6  Architectural Foundations for Ambient, Transparent Versioning and Provenance
    Scope: versioning and history — Git as ambient tracker, operation logs, content
    addressing, atomic multi-file AI operations, W3C PROV modelling.
    Status: RESEARCH MATERIAL. Recommends a two-tier architecture; explicitly asks and
    answers, in both directions, whether it is reinventing Git.
    Contributes: R8, H9; the multi-file undo problem (Q4); the provenance schema.

-------------------------------------------------------------------------------
S7  AI Authorization, Capability Security, and Autonomy in Local-First Agentic Workspaces
    Scope: authority and containment — capability vs. authorization, policy evaluation,
    operating-system sandboxing, permission scoping.
    Status: RESEARCH MATERIAL. The corpus's strongest security analysis; recommends a
    dual-boundary model and a multi-tier permission scheme.
    Contributes: A6, A7, C10, H10, H11; the failure scenarios; the permission tiers.

-------------------------------------------------------------------------------
S8  UX Architecture: Extreme Power Without Extreme Complexity
    Scope: the interface — progressive disclosure, lenses over the filesystem,
    confidence presentation, undo over modal confirmation, graph scope, onboarding.
    Status: RESEARCH MATERIAL. Mostly convergent with the product premise; contains
    several unevidenced UX assertions (see INGEST-NOTES.md).
    Contributes: P7, P9; the confidence-presentation and reversibility positions;
    the view layer (J6).

-------------------------------------------------------------------------------
S9  Local-First Architecture and Deployment
    Scope: deployment and synchronisation — local-first vs. offline-first, database-
    authoritative vs. filesystem-authoritative, multi-device sync, headless core,
    migration path, and what the MVP should exclude.
    Status: RESEARCH MATERIAL. Recommends filesystem-authoritative plus a headless core;
    contains the strongest evidence against the recommended model (Logseq).
    Contributes: A1, A8, C3, H1, H13; the MVP exclusion list; the deployment sequence.

-------------------------------------------------------------------------------
S10 Extensibility and Ecosystem Architecture
    Scope: plugins, integrations and automation — in-process vs. out-of-process
    extension hosts, sandboxed bytecode, capability brokers, the seven API planes.
    Status: RESEARCH MATERIAL. Recommends a microkernel mediator.
    Contributes: R11, H12, H13; the sandboxing tiers; what to keep internal.

-------------------------------------------------------------------------------
S11 Desconstrucción Crítica y Validación Adversarial (adversarial critique)
    Scope: adversarial deconstruction of the product premise — concurrency failures,
    instruction-load degradation, permission fatigue, graph-view collapse, and a
    per-capability verdict set (SIMPLIFY / INTEGRATE / REJECT / KEEP / DEFER).
    Status: RESEARCH MATERIAL — an adversarial opinion, not a verdict of record.
    Contributes: the strongest tensions (T1-T5), C6, C10, C11, and the alternative
    framing of the differentiator as a projection engine.

-------------------------------------------------------------------------------
S12 Linux kernel documentation — Multigrain Timestamps
    Scope: inode timestamp semantics — what ctime is, that it is not settable from
    userland, that coarse-grained timestamps can hide a change inside one jiffy, and
    that fine-grained (multigrain) timestamps are a per-filesystem opt-in.
    Status: EXTERNAL TECHNICAL DOCUMENTATION. Describes kernel behaviour; says nothing
    about Umbral and validates no Umbral design.
    Home: research/sources/S12-KERNEL-MULTIGRAIN-TIMESTAMPS.md — a REFERENCE RECORD with
    citation and the minimum quotation, deliberately NOT a verbatim archive (the source
    is a living document and is under its own licence).
    Retrieved: 2026-09-12, against local kernel 7.1.13-200.fc44.x86_64.
    Contributes: the external basis for UD-018 (ctime is an optimisation heuristic and
    may never be used to assert content-verified) and the limits section of EXP-CTIME.
    Does NOT contribute: any claim that ctime always changes, or any evidence about
    ext4 on this project's CI runner — that remains UNKNOWN.

-------------------------------------------------------------------------------
2026-09-11 CYCLE — MINIMUM MODEL OF PROJECT REALITY (pointer)
-------------------------------------------------------------------------------
  Evidence RM-1..RM-18 (four independent research passes: knowledge representation,
  legal/archival/scientific records, distributed/temporal systems, red team; plus one
  field-ablation experiment, experiments/min1/) is indexed with classes and limitations in
  research/PROJECT-REALITY-MINIMUM-MODEL.md §3. Verdict: the user-supplied five-primitive
  hypothesis reduced to a one-record-type model (H24); three collapses falsified. Pass
  source claims carry URL-level provenance; only USLM was re-verified first-hand (stated
  in the report as a limitation).

-------------------------------------------------------------------------------
GAPS IN THE CORPUS
-------------------------------------------------------------------------------
  1  [RESOLVED 2026-09-10] The master context document was recovered from the project's
     original working session and archived verbatim. It is private and is not part of this
     repository (see S0).
  2  S1-S11 full texts are NOT archived in this repository. They exist verbatim in the
     conversation history of the session that produced this repository (the ingestion
     session) and can be archived under research/sources/ on request. Until then,
     citations point to that transcript, not to documents. Recommended: archive them.
  4  [OPENED 2026-09-11] The V0 supporting research is NOT archived: the technology-
     selection report, the adversarial stack audit, the fsp-check prototype specification
     and the multi-device sync investigation exist in Gemini Deep Research / Hermes
     research sessions only. The 2026-09-11 stack adoption (UD-013) rests on USER
     authority and is valid without them, but the corpus cannot show the evidence until
     they are archived under research/sources/. Same defect class as item 2 (DQ-6:
     PASS in-session, FAIL across sessions).
  3  No report covers governance, licence or contribution model beyond MC §36 (open
     source, paid hosting possible, no ads); no accessibility/localisation work; no
     cost-of-operation estimates. See RESEARCH-AGENDA gaps G1-G7.
  4  No report tests the central assumption empirically. Every one of them reasons about
     it. (MC §44 anticipated this: specialized reports + red-teaming, not one giant
     report.)
  5  [NEW] S11's adversarial verdicts on dashboards, graph views, proactive intelligence,
     native versioning and permission granularity were superseded by explicit user
     statements in MC (§7-8, §18, §24, §26). S11 remains valuable as evidence and
     constraint material, but its product verdicts must not be cited as direction.
