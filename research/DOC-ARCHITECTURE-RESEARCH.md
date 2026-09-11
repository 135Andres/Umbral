# DOC ARCHITECTURE RESEARCH — evidence base

Status: RESEARCH ARTIFACT (T5). Unratified. Produced 2026-09-10 for
DOCUMENTATION-ARCHITECTURE.md. Cited as DR-n in that document's rationale.

METHOD AND ITS LIMITS (read this before citing anything below)
  What was done: targeted searches against established practice for project documentation,
  architecture description, decision records, navigation, research provenance, and
  AI-readable documentation conventions. Primary sources (originators, standards bodies,
  specifications, project sites) were preferred over secondary summaries.
  LIMITATION: the web extraction backend in this session is search-only, so several
  sources were assessed from search-result descriptions and their canonical pages rather
  than from full-text reading. Each entry therefore carries an explicit VERIFIED or
  SNIPPET-LEVEL marker. Claims marked SNIPPET-LEVEL are directionally reliable and should
  be confirmed against the primary source before they influence a decision. No claim below
  is used in DOCUMENTATION-ARCHITECTURE.md beyond what its marker supports.
  Nothing here was adopted because it is popular; each item states what it does and does
  not justify for FSP.

================================================================================
DR-1  Architecture Decision Records
================================================================================
Source: Michael Nygard, "Documenting Architecture Decisions" (2011),
        https://www.cognitect.com/blog/2011/11/15/documenting-architecture-decisions
        ADR community: https://adr.github.io/ ; MADR template: https://adr.github.io/madr/
        Martin Fowler, "Architecture Decision Record", https://martinfowler.com/bliki/ArchitectureDecisionRecord.html
Marker: VERIFIED for existence, origin, and purpose (multiple independent primary
        citations); SNIPPET-LEVEL for the operational supersession protocol.
Claim: a decision record is a short document capturing ONE architecturally significant
        decision, its context and its consequences; records are collected in a log; the
        community convention is that records are effectively immutable and are superseded
        by a later record rather than edited.
Supports for FSP: DP-2 (one decision per record), DP-7 (frozen decisions), the
        supersession line mechanism (H-1), and the DECISIONS.md structure (T3).
Does NOT justify: adopting ADR's full apparatus now (numbering per file, templates,
        tooling, ADR-per-architecture-decision split). FSP has 10 decisions, most of them
        product rather than architecture, and no architecture exists yet. One file with
        UD-IDs is the smallest thing that preserves the property that matters
        (immutability + supersession).
Note: ADRs are for ARCHITECTURE decisions. FSP's recovered decisions are mostly product
        commitments. The mechanism transfers; the label does not. Recorded so the
        vocabulary is not silently borrowed.

================================================================================
DR-2  Diátaxis — documentation organized by user need
================================================================================
Source: Diátaxis, https://diataxis.fr/ and https://diataxis.fr/start-here/ (Daniele Procida)
Marker: VERIFIED (project site and its "start here" page).
Claim: there are four identifiable kinds of documentation — tutorials, how-to guides,
        reference, explanation — each responding to a different user need; documentation
        should be organized around those needs rather than around the structure of the
        thing being documented.
Supports for FSP: DP-1 (organize by the reader's question). The four-quadrant split is
        also a useful sanity check that FSP's repository is not mixing modes: INVARIANTS
        and CONSTRAINTS are reference; PROJECT-DIRECTION is explanation; RESEARCH-AGENDA
        is how-to; README is the tutorial.
Does NOT transfer directly: Diátaxis is about documentation OF a product FOR its users.
        FSP's repository is project memory — internal, authority-laden, decision-bearing.
        The transferable idea is "organize by reader need", not the four types themselves.
        Adopting Diátaxis' four types as FSP's document types would be a category error.

================================================================================
DR-3  ISO/IEC/IEEE 42010 — architecture description, stakeholders, concerns, rationale
================================================================================
Source: ISO/IEC/IEEE 42010:2022, https://www.iso.org/standard/74393.html ;
        conceptual model: http://www.iso-architecture.org/42010/cm/index.html ;
        FAQ: http://www.iso-architecture.org/42010/faq.html
Marker: VERIFIED for the standard's existence, scope ("requirements for the structure and
        expression of an architecture description"), and the conceptual vocabulary
        (stakeholder, concern, viewpoint, view, model). SNIPPET-LEVEL for the precise
        treatment of rationale.
Claim: an architecture description is organized around the concerns of its stakeholders,
        and is expressed through defined viewpoints producing views; the description
        documents an architecture rather than the system.
Supports for FSP: DP-1 and DP-3 — the concern-driven organization and the requirement
        that a description carry its rationale. Also the general principle that a
        description is a separate artifact from the thing described, which is the
        separation FSP needs between charter, decisions, and research.
Does NOT justify: building a viewpoints/views model for the FSP repository, or adopting
        42010 as a documentation template. 42010 governs architecture descriptions; FSP
        has no architecture yet. Its value here is conceptual, and it will matter more
        when ARCHITECTURE-OPTIONS.md and ARCHITECTURE-SYNTHESIS.md are written (MC §47).

================================================================================
DR-4  arc42 — every kind of information has a place
================================================================================
Source: https://arc42.org/overview/ ; https://arc42.org/documentation/ ;
        repository: https://github.com/arc42/arc42-template
Marker: VERIFIED (official project pages).
Claim: arc42 is a 12-section template for architecture documentation whose stated purpose
        is that every kind of architecture information has a clear, well-defined place;
        sections are tailorable.
Supports for FSP: DP-4 (one home per kind of information) and the routing rule (§10).
        The observable benefit claimed for arc42 — that documentation fails when
        information has no defined home — is the failure FSP's routing rule is designed
        to prevent.
Does NOT justify: importing arc42's twelve sections. FSP's documents answer different
        questions (product intent, decisions, hypotheses), and the template targets
        software architecture documentation.

================================================================================
DR-5  Information foraging theory and information scent
================================================================================
Source: Pirolli, P. & Card, S., "Information foraging" (1999), Psychological Review
        106(4), 643-675, https://psycnet.apa.org/record/1999-11924-001 ;
        see also Pirolli & Card, "Information foraging in information access environments",
        CHI '95, https://dl.acm.org/doi/10.1145/223904.223911
Marker: VERIFIED for the theory, its authors and the "information scent" concept.
Claim: people navigate information environments by following cues (scent) that predict
        whether a path leads to the information they want, and abandon paths with weak
        scent — they optimize the ratio of value gained to effort spent.
Supports for FSP: DP-5 (status and authority visible at entry) and the navigation model
        (§5). The design consequence is concrete: file names, first lines, and IDs ARE the
        scent. A repository where you must open a file to learn what it is imposes a
        foraging cost on every reader, human or AI.
Does NOT justify: any particular file naming scheme; the theory constrains the quality of
        cues, not their form.

================================================================================
DR-6  Context engineering — the smallest set of high-signal tokens
================================================================================
Source: Anthropic, "Effective context engineering for AI agents",
        https://www.anthropic.com/engineering/effective-context-engineering-for-ai-agents
Marker: VERIFIED (primary vendor engineering documentation).
Claim: context is a finite resource with diminishing returns; the core discipline is
        finding the smallest set of high-signal tokens that achieves the task, rather than
        supplying everything available.
Supports for FSP: DP-10, and §9's context-economy model (entry file + IDs + targeted
        reads). It also supports the FSP charter's own context-efficiency thesis (MC §12)
        — with the caveat that this is vendor guidance, not independent measurement.
Does NOT justify: any specific mechanism (summaries, retrieval, compaction). It states a
        budget principle, and FSP's answer to it is currently documentary: small files,
        visible status, addressable IDs.

================================================================================
DR-7  Agent-facing repository conventions: AGENTS.md (adopted) and llms.txt (not)
================================================================================
Sources: https://agents.md/ ("a simple, open format for guiding coding agents, used by
        over 60k open-source projects"); InfoQ, "AGENTS.md Emerges as Open Standard for AI
        Coding Agents" (Aug 2025),
        https://www.infoq.com/news/2025/08/agents-md/ ;
        llms.txt proposal, https://llmstxt.org/ (Jeremy Howard, Sept 2024) and the
        Answer.AI post, https://www.answer.ai/posts/2024-09-03-llmstxt.html ;
        critical assessment: https://rye.dev/blog/llms-txt-standard-elegant-solution-nobody-using/
Marker: VERIFIED for AGENTS.md's existence, its purpose, and its claimed adoption;
        VERIFIED for llms.txt as a proposal; SNIPPET-LEVEL for the adoption critique.
Claim: AGENTS.md is an emerging convention for a predictable, committed file that tells
        coding agents how to work in a repository. llms.txt is a proposal to guide LLMs to
        a site's key content; its adoption by major AI platforms is widely reported as
        minimal.
Supports for FSP: a thin AGENTS.md entry point is justified as convention-following, and
        the contrast is instructive — a convention earns its place by being USED, not by
        being well-designed. That is exactly the standard FSP's own documents should be
        held to (DP-10), and it is why AGENTS.md here is a pointer, not a second README.
Does NOT justify: adopting llms.txt; or duplicating the repository's content into
        agent-specific files. If AGENTS.md and README diverge, one of them is wrong.

================================================================================
DR-8  Research provenance and lab-notebook practice
================================================================================
Sources: CASRAI, "Data Provenance: Definition, Standards, and How to Document It",
        https://casrai.org/guides/data-provenance ; The ELN Consortium file format
        specification, https://github.com/TheELNConsortium/TheELNFileFormat ;
        ELN provenance literature, https://jbiomedsem.biomedcentral.com/.../s13326-021-00257-x
Marker: SNIPPET-LEVEL (descriptions and abstracts; the underlying literature was not read
        in full).
Claim: provenance is the documented history of a research object — where it came from,
        what happened to it, and who or what acted on it, through every transformation;
        lab-notebook practice treats entries as dated, immutable records rather than
        living documents.
Supports for FSP: DP-8 (evidence and interpretation are different artifacts), DP-7
        (frozen records), and the research chain in §8. FSP's charter demands exactly this
        discipline of its own research (MC §46), so the repository's structure should
        embody it rather than describe it.
Does NOT justify: any provenance standard, ontology, or file format. FSP needs dated,
        attributed, immutable records and explicit source references — not PROV-O in the
        repository (PROV remains a candidate for the PRODUCT, per S4/S6, and that is a
        separate question).

================================================================================
DR-9  Topic-based authoring and reuse (single source of truth)
================================================================================
Source: OASIS DITA 1.2 specification, "The benefits of a topic-based architecture",
        https://docs.oasis-open.org/dita/v1.2/os/spec/archSpec/topicbenefits.html and
        "Disciplined, topic-oriented writing",
        https://docs.oasis-open.org/dita/v1.2/os/spec/archSpec/topicorientation.html
Marker: VERIFIED (specification text).
Claim: content should be authored as self-contained topics with one purpose each, and
        reused by reference rather than copied; a topic is the basic unit of authoring
        and reuse.
Supports for FSP: DP-6 (one fact, one home; everything else is a pointer) and the
        duplication policy (§10). The transferable rule is "reuse by reference".
Does NOT justify: DITA, XML, or a formal information-typing system.

================================================================================
DR-10  Documentation drift, and what actually detects it
================================================================================
Sources: Vale (prose linting), https://docs.vale.sh/ ; a documentation-health CI action
        that reports broken links, version drift and staleness,
        https://github.com/joaquimscosta/docs-health-action ; "staledocs" (deterministic
        drift detection between code and docs), https://pypi.org/project/staledocs/
Marker: SNIPPET-LEVEL (tool descriptions; not independently evaluated).
Claim: documentation decay is treated as a measurable property — broken links, stale
        references, and divergence between docs and the thing they describe — and there
        are deterministic (non-LLM) tools that detect it.
Supports for FSP: the drift indicators listed in §10 and the drift log in EXP-DOC-1.
        Notably, the practice that survives scrutiny is DETERMINISTIC checking (links,
        IDs, references), not prose linting.
Does NOT justify: adding any tooling now. FSP's repository has no build, no CI, and
        fifteen files; the drift log is the proportionate response, and a link/ID check
        becomes worthwhile only when the corpus is much larger.

================================================================================
DR-11  Evidence that was sought and NOT found (recorded so it is not re-searched)
================================================================================
- No authoritative source was found that prescribes a documentation architecture for
  "project memory that must serve a human and an AI agent simultaneously". The relevant
  practices are fragmented across ADRs (decisions), Diátaxis (reader needs), 42010/arc42
  (architecture description), lab notebooks (provenance), and context engineering
  (budget). FSP's documentation architecture is therefore a synthesis, not an
  application of a standard — and it should be treated as an experiment (§14), not as
  best practice.
- No evidence was found that epistemic-state metadata (fact/observation/hypothesis/...)
  in documentation materially improves comprehension. The practice that does have support
  is weaker and cheaper: visible status/authority at entry (DR-1, DR-5) and separation of
  artifacts by kind (DR-2, DR-3, DR-9). This is why FSP uses status lines and IDs rather
  than a metadata taxonomy (mandate §4: clarity, not taxonomy for taxonomy's sake).
- No evidence was found supporting llms.txt-style agent-specific documentation copies;
  the one convention with reported adoption (AGENTS.md) is a single pointer file.

================================================================================
APPLICABILITY SUMMARY
================================================================================
Adopted (with the caveat noted): DR-1 mechanism only; DR-2 principle only; DR-3
conceptually, deferred to the architecture phase; DR-4 principle; DR-5 principle; DR-6
budget principle; DR-7 AGENTS.md as pointer only; DR-8 discipline, no format; DR-9 reuse
by reference; DR-10 drift indicators, no tooling.
Rejected for FSP at this stage: ADR per-file tooling and templates; Diátaxis' four types
as FSP document types; 42010 viewpoints/views machinery; arc42's twelve sections; llms.txt;
DITA; any provenance ontology; any documentation linting or CI.
