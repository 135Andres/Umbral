# DOCUMENTATION ARCHITECTURE — FSP

Status: **PROPOSED** (2026-09-10), being applied as an experiment (see §14). This document
governs the STRUCTURE of the repository, not its content, and it is not authority over
any project fact. The user may reject or amend any part; nothing here is a product
decision.

Evidence base: research/DOC-ARCHITECTURE-RESEARCH.md (external sources, with what they
support and what they do not). This file is the design that follows from that evidence.

SCOPE BOUNDARY (§19 of the mandate): the FSP repository is a LABORATORY for documentation
ideas, not a proof of FSP's product architecture. A choice made here — flat folders, IDs,
git — implies nothing about how FSP itself should store or organize a user's knowledge.
Two risks follow and are accepted explicitly:
  X1. Drift pressure: "we organize our repo this way, so FSP should too."
  X2. Reverse drift pressure: "our repo is fine without X, so FSP doesn't need X."
Both are invalid. Repository structure is an experiment with its own constraints
(one project, one user, no performance limits, no external users).

================================================================================
1. DESIGN PRINCIPLES
================================================================================
Each principle: the rule, why it holds, its source, and what it forbids.

DP-1  Organize by the reader's question, not by the artifact's origin.
      A reader arrives with a question ("what is decided?", "what is uncertain?"), not
      with a taxonomy. Group information by the question it answers.
      Source: Diátaxis (Procida) — four kinds of documentation defined by user need, not
      by product structure; ISO/IEC/IEEE 42010 — organization by stakeholder concern.
      Forbids: directories named after internal processes (e.g. /phase2, /gemini).

DP-2  One decision per record; decisions are historical artifacts, not living text.
      A decision record states context, decision, consequences; it is not rewritten when
      circumstances change — it is superseded by a new record that references it.
      Source: Nygard, "Documenting Architecture Decisions" (2011); adr.github.io; MADR;
      Fowler's ADR bliki entry.
      Forbids: editing a decision's meaning in place; deleting a superseded decision.

DP-3  Rationale is first-class and travels with the claim.
      "Why we believe this" is as important as "what we believe". Every non-obvious
      statement carries its source and, where relevant, the alternative it beat.
      Source: ISO/IEC/IEEE 42010 (rationale as part of an architecture description);
      the FSP charter itself (MC §40 provenance lesson).
      Forbids: bare assertions; conclusions without their evidence trail.

DP-4  Every kind of information has exactly one home.
      There is a defined place for each kind of artifact, and a rule for what to do with
      a new piece of information.
      Source: arc42 ("gives every kind of architecture information a clear place");
      DITA topic-based authoring (one topic, one purpose, reusable by reference).
      Forbids: orphan files; "I'll put it somewhere for now".

DP-5  Status and authority are visible at the point of entry, not buried.
      A reader must be able to tell whether a statement is user-decided, canonical,
      research, interpretation, or exploration without opening every document.
      Source: information foraging theory (Pirolli & Card 1999) — navigation depends on
      "information scent"; the FSP charter's authority concern (MC §40).
      Forbids: unlabeled content; a status line that only says "active".

DP-6  Single source of truth; everything else is a pointer or a derived view.
      A fact lives in one file. Indexes, summaries and read orders are derived and
      disposable. Copying a fact is allowed only when it is a verbatim source archive.
      Source: DITA reuse; docs-as-code practice; FSP's own invariant A10 (derived state
      is rebuildable) — applied to the repository, not inferred from it.
      Forbids: two files describing the same current state; "convenience copies".

DP-7  Separate current truth from history explicitly.
      Current-state documents describe what is true now. Decision records, research
      reports, audits and experiment results are historical artifacts and never edited to
      match the present. Superseded items say what superseded them.
      Source: ADR supersession protocol; lab-notebook practice (a record is an
      immutable entry, not a wiki page).
      Forbids: silent rewrites of historical records; changelogs inside current-state
      files.

DP-8  Research evidence and interpretation are different documents.
      A source says X; Hermes interprets Y; FSP decides Z. These are three artifacts with
      three authorities, and the chain from one to the next must be traceable by ID.
      Source: research provenance practice (electronic lab notebooks; data-provenance
      documentation) and the FSP charter's explicit Gemini/Hermes cross-verification
      protocol (MC §46).
      Forbids: a research report that reads as a specification; an interpretation
      presented as a finding.

DP-9  Documentation must survive the tool.
      Plain Markdown, ordinary folders, no hidden state required to understand the
      content, no dependency on Hermes or any specific AI.
      Source: FSP invariant A2 (Sunset Test) applied to the repository; MC §29.
      Forbids: tool-specific formats as the only readable form; content that exists only
      in an agent's memory.

DP-10 Minimum structure that works.
      Every structural element (folder, document type, metadata field, index) must justify
      its maintenance cost. Add structure only when an observed failure requires it.
      Source: this mandate (§17); the FSP charter's frustration priority #1 (excessive
      maintenance complexity, MC §38).
      Forbids: templates, schemas, indexes and generated files created in advance of need.

================================================================================
2. DOCUMENT ONTOLOGY
================================================================================
Nine types. Types were kept only where the lifecycle or authority differs; where they do
not differ, the documents are the same type in the same file.

Legend: A = authority, L = lifecycle, W = who may create, AI = may an AI treat it as
authoritative.

DT1 CHARTER (source of user intent)
   Purpose: the user's own founding statements. A: USER. L: immutable (verbatim archive).
   W: user only. AI: yes, as USER INTENT — never as an architecture decision.
   Home: research/sources/. Current instance: PROJECT-MASTER-CONTEXT.md (MC §n).

DT2 CANONICAL KNOWLEDGE (current truth about the product)
   Purpose: the smallest set of documents that state what FSP is, must hold, needs, and
   cannot violate. A: project (derived from charter + decisions; never from research).
   L: living — edited in place, no changelog inside. W: Hermes proposes, user ratifies
   anything that changes meaning. AI: yes, as current — but check the status line.
   Homes: PROJECT-DIRECTION, VISION, INVARIANTS, PRINCIPLES, REQUIREMENTS, CONSTRAINTS.

DT3 DECISION RECORD
   Purpose: a commitment made by the user, with its basis and consequences. A: USER.
   L: immutable once recorded; superseded by a later record, never rewritten.
   W: user (Hermes may draft a candidate record; it is not a decision until accepted).
   AI: yes — this is the highest non-charter authority in the repository.
   Home: DECISIONS.md (split into one file per decision when supersession starts or the
   log passes ~20 entries — threshold stated so the split is not a judgement call).

DT4 HYPOTHESIS / OPEN QUESTION
   Purpose: candidate mechanisms not validated; questions whose answers change the
   architecture. A: project (research-derived, unratified). L: living.
   W: Hermes. AI: yes, but must never present one as chosen or settled.
   Homes: ARCHITECTURE-HYPOTHESES, OPEN-QUESTIONS.

DT5 RESEARCH ARTIFACT (external evidence, unratified)
   Purpose: what external sources or other AIs claim, and how it was verified.
   A: source (attributed, never adopted). L: immutable once ingested; corrected only by
   an explicit amendment note. W: Hermes. AI: may cite; must not treat as FSP position.
   Homes: research/reports/ (S1-S11 when archived), research/*-RESEARCH.md,
   research/RESEARCH-INDEX.md, research/history/INGEST-NOTES.md.

DT6 EXPERIMENT (specification and result)
   Purpose: a question answered by measurement. A: project; result is EVIDENCE, not a
   decision. L: spec frozen before running; result appended; both immutable afterwards.
   W: Hermes drafts, user approves the run. AI: may cite the result as evidence.
   Home: experiments/EXP-<n>-<slug>.md — directory created when the first experiment
   exists, not before.

DT7 AUDIT / PROCESS RECORD
   Purpose: a dated review of the project's own state (what was wrong, what changed).
   A: Hermes, as a record — findings inside it carry their own authorities.
   L: immutable, dated, historical. W: Hermes. AI: yes, as history.
   Home: research/AUDIT-<date>.md.

DT8 INDEX / DERIVED VIEW
   Purpose: navigation aid over other documents. A: none (derived). L: disposable,
   rebuildable. W: anyone. AI: yes, for orientation only — never as the fact itself.
   Homes: README (entry router), research/RESEARCH-INDEX.md.

DT9 AGENT ROUTER
   Purpose: the conventional entry point for AI agents that expect one. A: none
   (pointer). L: kept minimal. Home: AGENTS.md.

NOT adopted as types (deliberately): meeting notes (none exist; MC has no such
requirement), changelogs (history is the repository's own), glossaries (no vocabulary
conflict yet), specifications and design proposals (premature — MC §48 forbids
implementation-phase artifacts), user/developer documentation (the product does not exist
yet; when it does, it is a different repository, not this memory).

================================================================================
3. AUTHORITY MODEL
================================================================================
The ladder (highest first). Conflicts resolve downward; a lower level never overrides a
higher one:

  1. Explicit current user instruction
  2. Decision record (DT3, UD-nnn)
  3. Charter (DT1, MC §n) — user intent, not architecture
  4. Canonical knowledge (DT2) — derived from 2-3, current-state
  5. Experiment result (DT6) — evidence
  6. Research artifact (DT5) — external claim, attributed
  7. Interpretation / hypothesis (DT4) — Hermes
  8. Session history, agent memory, model inference

The two confusions this must prevent, and the mechanism for each:
  C-A  "Hermes thinks this" mistaken for "the project decided this".
       Mechanism: decision records are the ONLY place a commitment lives; they carry
       UD-IDs and a user-authority line. Everything else carries a status line.
  C-B  "A source says X" mistaken for "FSP has adopted X".
       Mechanism: research artifacts carry a source ID and the words "unratified"; the
       technology inventory in ARCHITECTURE-HYPOTHESES.md is marked "not selected"; no
       research artifact may use decision language.

Enforcement is documentary, not technical: a status line at the top of each document, an
ID prefix on each claim (A-n, C-n, R-n, J-n, P-n, H-n, Q-n, T-n, UD-n, S-n, MC §n), and
the routing rule in §10. No metadata schema, no frontmatter, no validator — deliberately,
until an observed failure demands one.

================================================================================
4. LIFECYCLE
================================================================================
Two lifecycles, because one size does not fit:

LIVING documents (DT2 canonical, DT4 hypotheses/questions, DT8 indexes)
  DRAFT -> CURRENT -> (rewritten in place | SUPERSEDED-BY-POINTER | ARCHIVED)
  A rewrite in place is allowed only while the document still describes current truth.
  If the change is a reversal of a recorded commitment, it is a DECISION, not an edit:
  record UD-n, then update the living document to reference it.
  Who: Hermes may edit; the user must ratify any change of meaning.

FROZEN documents (DT1 charter, DT3 decisions, DT5 research, DT6 experiments, DT7 audits)
  CREATED -> (ACTIVE | SUPERSEDED) -> ARCHIVED
  Frozen documents are never edited except to add a supersession line at the top:
      Status: SUPERSEDED BY UD-<nnn> (date) — see that record.  [illustrative format only]
  Who: the user may supersede a decision; Hermes may mark research or audits superseded
  by a later artifact. No one may rewrite a frozen record's content.

AI role: an AI may PROPOSE a lifecycle transition and may never perform one on a decision
record. That asymmetry is the point.

================================================================================
5. NAVIGATION MODEL
================================================================================
Entry points, in the order a stranger encounters them:
  1. README.md — what this repository is, the epistemic contract, the authority ladder,
     the map, the read order, and the routing rule for new information.
  2. PROJECT-DIRECTION.md — the one-screen answer to "where does this project stand?".
  3. AGENTS.md — the same entry for an AI agent that looks for that convention.

Four navigation paths, each a sequence of files (not a folder tree):
  ORIENT    README -> PROJECT-DIRECTION -> VISION -> DECISIONS
  DECIDE    DECISIONS -> OPEN-QUESTIONS -> RESEARCH-AGENDA -> ARCHITECTURE-HYPOTHESES
  EVIDENCE  RESEARCH-INDEX -> research reports -> experiments -> DECISIONS
  WORK      a task's question -> the ID namespaces it touches -> the specific files

Information scent: every file name states its subject; every file's first three lines
state its type, status and authority; every cross-reference uses an ID rather than prose.
A reader should never have to open a file to learn whether it is current.

================================================================================
6. RELATIONSHIP MODEL
================================================================================
Relationships are expressed as IDs in plain text, not as a graph structure:
  MC §n -> A-n -> Q-n -> H-n -> EXP-n -> evidence -> UD-n -> R-n
Read as: a charter statement grounds an invariant, which raises a question, which has
candidate hypotheses, one of which gets an experiment, whose result supports or kills a
decision, which may create a requirement.

Rules:
  - IDs are stable and never reused.
  - A reference is written where the relationship is asserted, in both directions where
    the relationship is load-bearing (e.g. an experiment names its question; the question
    names the experiment).
  - No index file is maintained by hand for this; the references are the index.
Rationale: a link graph is the cheapest thing that works and survives any tooling change.
A graph engine is NOT required and NOT planned; if it ever becomes useful, the IDs are
already there to build it from.

================================================================================
7. HISTORICAL MODEL
================================================================================
Three mechanisms, no more:
  H-1 Supersession lines. A superseded decision or artifact keeps its text and gains a
      one-line pointer to what replaced it. Current-state files do not carry history.
  H-2 Dated, immutable records. Audits, ingests and experiment results are dated files;
      they are never updated, only superseded by later dated files.
  H-3 Version control. The repository is a git repository, so every change has a diff, an
      author and a time, without any of that appearing inside the documents.
      Applied 2026-09-10 (see §12, M1). Reversible: removing .git loses history only.
      EXPLICIT NON-INFERENCE: using git for this repository is not a decision about FSP's
      versioning architecture (MC §26 requires that to be researched independently).
      This is the single largest drift risk introduced by this document; it is recorded
      here so it cannot be quietly forgotten.

Anti-pollution rule: when an old artifact would confuse a reader or an AI about current
truth, the artifact is not deleted — it is superseded with a pointer, and current-state
documents stop referring to it.

================================================================================
8. RESEARCH / EVIDENCE / EXPERIMENT CHAIN
================================================================================
SOURCE -> CLAIM -> VERIFICATION -> INTERPRETATION -> FSP RELEVANCE -> HYPOTHESIS ->
EXPERIMENT -> RESULT -> DECISION

Where each step lives:
  SOURCE          research/sources/ (verbatim, immutable) or an external citation
  CLAIM           inside the research artifact, with the source's own wording where
                  possible and an S-n reference
  VERIFICATION    same artifact: agreement, disagreement, missing information; a claim
                  from another AI is never adopted on the strength of its confidence
  INTERPRETATION  same artifact, explicitly labeled as Hermes
  FSP RELEVANCE   same artifact, or a line in ARCHITECTURE-HYPOTHESES if it creates one
  HYPOTHESIS      ARCHITECTURE-HYPOTHESES (H-n)
  EXPERIMENT      experiments/EXP-n (spec frozen before the run)
  RESULT          same file, appended after the run; negative results kept
  DECISION        DECISIONS.md (UD-n), user authority only

Rule that keeps the chain honest: no step may be skipped in the record. In particular, an
experiment result never becomes a decision by being written down — it becomes a decision
when the user records one.

================================================================================
9. CONTEXT-ECONOMY MODEL
================================================================================
The repository must let an agent answer a task with the minimum sufficient context. The
mechanism is documentary, not generated:

  1. README (small, always loaded) gives: what this is, the authority ladder, the map.
  2. PROJECT-DIRECTION gives current state in one screen.
  3. The ID namespaces let an agent request exactly what a task needs: e.g. "constraints
     and open questions about identity" -> CONSTRAINTS.md + OPEN-QUESTIONS.md (Q10/Q14)
     + ARCHITECTURE-HYPOTHESES.md (H3), not the corpus.
  4. Each file's header states its type and status, so an agent can decide whether to read
     it at all from the first three lines.
  5. Historical and research artifacts are explicitly labeled, so they can be excluded
     from a task context unless the task is about evidence.

What is deliberately NOT built now: generated context maps, manifests, frontmatter
schemas, embedding-based selection. They are plausible (research/DOC-ARCHITECTURE-RESEARCH.md
notes that the "smallest set of high-signal tokens" framing is standard practice) but each
adds a maintained artifact, and the current corpus is 15 files. Revisit when a measured
task requires more than a handful of reads — see EXP-DOC-1.

================================================================================
10. DUPLICATION POLICY AND ROUTING RULE
================================================================================
New information arrives constantly. The rule, applied in order:

  1. Is it a user commitment?                      -> DECISION RECORD (DT3, UD-n)
  2. Does it come from outside FSP (source, report, other AI)?
                                                   -> RESEARCH ARTIFACT (DT5), with S-n
  3. Is it the result of a measurement?            -> EXPERIMENT (T6, EXP-n)
  3b. Is it a proposed experiment not yet specified? -> EXPERIMENT (T6, EXP-n) in
                                                     experiments/, spec written BEFORE
                                                     the run (a proposed experiment is a
                                                     document, not a plan in prose)
  4. Does it change what is currently true about the product?
                                                   -> CANONICAL KNOWLEDGE (DT2), in the
                                                      existing file that owns that subject
  5. Is it a candidate mechanism or an unanswered question?
                                                   -> HYPOTHESIS / OPEN QUESTION (DT4)
  5a2. Is it a PRODUCT-level hypothesis (about what FSP is/does, not how it is built)?
                                                   -> its own CANONICAL (T2) hypothesis
                                                      document if numerous; INDEX entry in
                                                      ARCHITECTURE-HYPOTHESES.md; never in
                                                      DECISIONS.md
  5b. Is it a research DIRECTION the user authorized, whose output is candidate
      strategies rather than findings?             -> CANONICAL (T2) strategy document +
                                                      research/ evidence file + the
                                                      direction recorded as a decision
                                                      record if the user authorized it
  6. Is it process (how we work on FSP)?           -> the Hermes skill, not the repo
  7. Is it none of the above?                      -> do not write it down.

If the answer is "canonical knowledge" and no file owns the subject, that is the only
justified reason to create a new document — and the new file must be added to the README
map in the same change.

Duplication checks performed at write time:
  - Does another file already state this? Then reference it by ID instead of restating.
  - Is this a summary of something? Then it is an INDEX (DT8) and must say it is derived.
  - Is this the same fact in two places? One of them is wrong by definition; fix both in
    the same change.

Drift indicators to watch (recorded per §18): the same fact found twice; a file whose
header status disagrees with its content; an ID referenced but undefined; a document not
in the README map; a dated artifact edited after its date.

================================================================================
11. NAMING AND LOCATION CONVENTIONS
================================================================================
Adopted after the conceptual model, not before it. Deliberately minimal:

Top level, canonical and current-state (UPPERCASE-KEBAB, no numbering, no prefixes):
  README.md, AGENTS.md, PROJECT-DIRECTION.md, VISION.md, DECISIONS.md, INVARIANTS.md,
  PRINCIPLES.md, REQUIREMENTS.md, CONSTRAINTS.md, ARCHITECTURE-HYPOTHESES.md,
  OPEN-QUESTIONS.md, RESEARCH-AGENDA.md, DOCUMENTATION-ARCHITECTURE.md

research/history/  FROZEN records only (dated, never edited except for a supersession
              pointer). Anything a reader could mistake for current state does not live
              here, and anything here is not current state. The directory is the signal;
              the status line is the backstop.
research/   external evidence and process records
  RESEARCH-INDEX.md        index of sources (derived + per-source authority notes)
  <TOPIC>-RESEARCH.md      a research report produced for FSP — the naming convention
                           the user already specified in MC §47 (RESEARCH-<TOPIC>.md);
                           used verbatim there, prefixed by directory here
  INGEST-NOTES.md          historical: the initial ingestion record
  AUDIT-<date>.md          dated audits
  DOC-ARCHITECTURE-RESEARCH.md  evidence base for this document
  sources/                 verbatim, immutable archives (MC, and S1-S11 when archived)

experiments/  created when the first experiment exists. One directory per experiment:
              experiments/<exp-id>/EXP-<id>.md   (spec written BEFORE the run; result and
                                                  diagnostics appended AFTER it)
              experiments/<exp-id>/             (instrument, corpus, raw results as needed)
              Naming follows the repository's own choice (exp1/, exp-doc1/), not a new
              scheme: the experiment id is already unique.

No numbering prefixes (01-, 02-): they encode an order that will change, and they add a
rename cost to every insertion. Order is expressed in the README read order, which is
cheap to change. No dates in canonical file names (a dated canonical file is a
contradiction: dates belong to frozen artifacts).

================================================================================
12. MIGRATION PLAN
================================================================================
M0 — ALREADY TRUE (before this audit): 13 flat canonical documents + research/. No
     change needed to the folder structure; it was already minimal and it survives the
     audit. This is the single most important finding: the structure was not the problem.

M1 — APPLIED 2026-09-10 (this change set):
     - Created DOCUMENTATION-ARCHITECTURE.md (this file) and its research evidence file.
     - Created AGENTS.md (thin agent entry point).
     - README: fixed stale statements ("no DECISIONS.md", "no decision exists yet");
       added the authority ladder; added the routing-rule pointer; updated the map.
     - INGEST-NOTES: marked as a historical record; stale counts corrected; the duplicated
       tension list reduced to a pointer (canonical list lives in ARCHITECTURE-HYPOTHESES
       / OPEN-QUESTIONS).
     - PRINCIPLES: documentation principles moved here (they were mixed into the product
       principles file, which is a different audience and authority).
     - AUDIT-2026-09-10: gained the documentation-architecture audit section.
     - RESEARCH-AGENDA: mapped to the research tracks the user already defined (MC §44/§47).
     - git initialised for the repository (H-3), one commit.

M1b — APPLIED 2026-09-10 (second change set; triggered by observed failures, not taste):
     Two structural changes, both with their trigger recorded in the friction log:
     (1) `research/history/` created; INGEST-NOTES, AUDIT-2026-09-10 and the EXP-1 stage
         record moved into it. Trigger: two independent EXP-DOC-1 readers reported that
         frozen records beside living artifacts let them reach pre-recovery answers
         (DOC-FRICTION-010/012).
     (2) The EXP-1 fixture corpus renamed `corpus-messy` -> `exp1/fixture-corpus-synthetic`
         and given an adjacent FIXTURE-CORPUS.md. Trigger: three readers reported that a
         directory listing could mistake the fixtures for project memory
         (DOC-FRICTION-011). Fixtures were NOT edited: that would change the experiment's
         inputs and invalidate its results. Verified reproducible after the move.
     Also: the git non-inference moved to the front door (README) after two readers named
     the repository's own git usage as the strongest wrong signal in the whole repository.

M2 — WHEN TRIGGERED (no action now):
     - experiments/ + EXP-1 record: when the first experiment runs.
     - Split DECISIONS.md into per-decision files: at ~20 decisions or the first
       supersession, whichever comes first.
     - research/reports/: when S1-S11 are archived verbatim.
     - Split OPEN-QUESTIONS.md if it exceeds ~400 lines or its update blocks exceed two.

M3 — DEFERRED, NEEDS THE USER (do not do without instruction):
     - Any metadata/frontmatter schema.
     - Any generated index, manifest, or context-map.
     - Any restructuring of the corpus (S0-S11) beyond verbatim archiving.
     - Any renaming of existing canonical documents.

Migration preserved all provenance: no content was deleted; superseded text inside
OPEN-QUESTIONS and RESEARCH-AGENDA was kept under explicit "(superseded)" markers, and
INGEST-NOTES retained its full text with a status header.

================================================================================
13. TRADEOFFS
================================================================================
Better with this architecture:
  - Authority is legible at the point of entry (the ladder + status lines).
  - New information has one routing decision instead of a judgement call.
  - History exists (supersession + git) without contaminating current-state files.
  - The research chain is traceable end to end, which is the project's own thesis.
  - The structure stays small enough for one person to hold in mind.
Worse / newly introduced:
  - Three new documents (this file, its evidence file, AGENTS.md) — maintenance cost is
    real; they earn it only if they prevent sprawl, which is not yet demonstrated.
  - Two lifecycles (living/frozen) require the writer to choose correctly; a mistake here
    is exactly the confusion the architecture exists to prevent.
  - The status line and ID discipline are conventions, not enforcement. They will decay
    without review.
  - A PROPOSED structural document creates a second authority surface while it awaits
    acceptance; mitigated by marking it PROPOSED and non-authoritative, but not removed.
  - git in the repository introduces the X1 drift risk named in §7.

================================================================================
14. THIS IS AN EXPERIMENT
================================================================================
The documentation architecture is itself FSP experiment EXP-DOC-1, and it is expected to
be wrong somewhere. Per the mandate (§18), it evolves from observed failure, not from
theory.

Specification:
  Question: does the structure above let a reader and an AI reach the right information,
            with correct authority, without loading the repository?
  Hypothesis: a stranger (human or AI) can answer the ten orientation questions below
            from README + PROJECT-DIRECTION alone, and any other question by opening at
            most three files; and no reader will mistake a research claim for a decision.
  Test: (a) navigation test — give the ten questions to a fresh agent with no context and
            record how many files it opens and whether it reports the correct authority
            for each answer; (b) drift log — for the next five substantive sessions, record
            one line per incident of: same fact in two places / stale statement / ID
            referenced but undefined / document missing from the map.
  Success: >= 8 of 10 questions answered correctly; <= 3 files opened on average; zero
            authority errors; < 2 drift incidents across five sessions.
  Failure: any authority error (a research claim read as a decision), or >= 4 files per
            question, or >= 2 drift incidents — each of which triggers a targeted fix to
            the structure rather than a redesign.
  The ten questions: what is FSP; why does it exist; who is it for; what has been decided;
            what is explicitly NOT decided; what is uncertain; what is being researched;
            what has been tested; what are the top risks; what happens next.
  Recorded: this file; results appended to experiments/EXP-DOC-1 when run.

Failure modes this experiment is specifically watching for (from the mandate §18):
human confusion at entry; AI confusion about authority; duplicated facts; failed
navigation; ambiguity about what is current; context too large; history mistaken for
current truth; structure that costs more to maintain than it returns.

================================================================================
15. MAINTENANCE RISKS
================================================================================
MR-1  Convention decay: status lines and IDs are not enforced. Mitigation: the drift log
      in EXP-DOC-1; a status line is checked whenever a file is edited.
MR-2  Update-block accumulation: OPEN-QUESTIONS and RESEARCH-AGENDA now carry dated update
      blocks above their original text. This is a known, temporary compromise: it
      preserved provenance during the MC recovery. Mitigation: consolidate on the next
      substantive edit; the blocks are history, not current state.
MR-3  Second authority surface: this PROPOSED document. Mitigation: it never states a
      project fact; it only states structure, and it is superseded by user acceptance or
      rejection.
MR-4  Repository/product conflation (§19): the git decision and the flat structure are the
      two most tempting precedents. Mitigation: stated explicitly in §7 and §0.
MR-5  Growth by accretion: the failure mode this document exists to prevent. Mitigation:
      the routing rule, applied before any write.

================================================================================
16. OPEN QUESTIONS (documentation architecture only)
================================================================================
DQ-1  Does AGENTS.md earn its place, or is it a third surface duplicating README?
      Resolve after EXP-DOC-1's navigation test (does a fresh agent find the repo through
      AGENTS.md or through README?).
DQ-2  Is one DECISIONS.md right at this scale, or should decisions be one file each from
      the start? Current answer: threshold at ~20 or first supersession. Unvalidated.
DQ-3  Should frozen research artifacts live under research/reports/ (S1-S11) or be
      superseded by a single curated evidence document? Unresolved; depends on whether the
      corpus is archived verbatim.
DQ-4  Does the repository need a glossary? Not yet — no term is used with two meanings
      except "context package" (T1), which is tracked as a tension instead.
DQ-5  When FSP's own product documentation begins, does it live here or in a separate
      repository? Deferred; likely separate, and this repository should not grow into it.

================================================================================
17. WHAT SHOULD NOT BE CHANGED YET
================================================================================
- No folder renumbering, no prefix schemes, no renaming of existing canonical files.
- No frontmatter or metadata schema; no YAML headers.
- No generated indexes, manifests, or context maps.
- No graph structure or database; IDs and inline links only.
- No templates or required document skeletons.
- No splitting of DECISIONS.md, OPEN-QUESTIONS.md or ARCHITECTURE-HYPOTHESES.md yet.
- No restructuring of the corpus beyond verbatim archiving.
- No product-facing documentation (README for users, install docs, tutorials) — the
  product does not exist, and MC §48 forbids implementation-phase artifacts.
- No conflation of this architecture with FSP's product architecture (§19).
