# COEXISTENCE RESEARCH — evidence base

Status: **RESEARCH ARTIFACT (T5), unratified.** Evidence base for COEXISTENCE-STRATEGIES.md.
Sources are numbered CO-n and are external unless marked S-n (the S1-S11 corpus already
ingested, cited by its existing identifiers).

METHOD AND LIMITS (read before citing)
  What was done: targeted searches on protocols, coordination models, agent memory,
  concurrency control, provenance and identity, preferring specifications, project sites,
  and papers over commentary. Cross-checked against what the S1-S11 corpus already
  established so this file does not re-derive it.
  LIMITATION: the extraction backend in this session is search-only, so several sources
  were assessed from specifications' index pages and abstracts rather than full text.
  Every entry carries VERIFIED (spec or primary page read) or SNIPPET-LEVEL (description
  or abstract only). No claim below is used in COEXISTENCE-STRATEGIES.md beyond what its
  marker supports.

================================================================================
PART 1 — PROTOCOLS FOR AI INTEROPERABILITY
================================================================================
CO-1  Model Context Protocol (MCP) — current specification
      Source: modelcontextprotocol.io, specification 2026-07-28 and its changelog; SEP-2577.
      Marker: VERIFIED (spec pages).
      Claims: the 2026-07-28 revision introduces a stateless protocol core, Multi
      Round-Trip Requests, header-based routing and cacheable list operations. Roots,
      Sampling and Logging are DEPRECATED (SEP-2577), still functional for at least twelve
      months, with new implementations told not to adopt them.
      Relevance: MCP standardises how a host exposes tools/resources/prompts to a model.
      It does NOT define project state, file versions, change attribution, or who may act
      on what. Two MCP servers exposing the same folder share no notion of "the same
      project at the same moment".
      Also relevant: the deprecation of Roots removes the protocol's only (advisory)
      filesystem-boundary primitive — consistent with S7's earlier finding that the prompt
      is not a boundary.

CO-2  MCP Apps and the extension model
      Source: blog.modelcontextprotocol.io posts 2026-01-26 (MCP Apps) and 2026-03-11
      (Understanding MCP Extensions).
      Marker: SNIPPET-LEVEL.
      Claims: extensions layer new capabilities on the core protocol; MCP Apps is the first
      official extension, letting tools return interactive UI rendered in the client.
      Relevance: the protocol is growing toward presentation, not toward shared project
      state. Nothing in the extension model addresses multi-writer reality.

CO-3  Agent2Agent (A2A)
      Source: a2a-protocol.org specification (v1.0.x) and the project's GitHub specification.
      Marker: VERIFIED (specification).
      Claims: open standard for agent-to-agent communication, now under the Linux
      Foundation. Data model centres on Task, Message, AgentCard, Part, Artifact,
      Extension. Supports synchronous request/response, streaming, and asynchronous push
      notifications for long-running tasks. Explicitly designed so agents are "potentially
      opaque" to each other — interoperability without shared internals.
      Relevance: A2A solves agent-to-agent messaging and task delegation. It has a Task
      lifecycle and Artifacts, but no filesystem reality, no shared project memory, and no
      notion of a change to a file. Also: it assumes agents that speak A2A; the AI a user
      actually uses today (a chat window) does not.

CO-4  Agent Client Protocol (ACP, Zed) and IBM's Agent Communication Protocol (ACP)
      Sources: agentclientprotocol.com (Zed); research.ibm.com/projects/agent-communication-protocol
      and i-am-bee/acp.
      Marker: VERIFIED (project pages).
      Claims: Zed's ACP standardises editor↔agent communication (an editor hosts the thread,
      the agent owns its runtime). IBM's ACP standardised agent interoperability over REST;
      the repository now states ACP is part of A2A under the Linux Foundation.
      Relevance: the protocol landscape is CONVERGING (two ACPs, one absorbed into A2A).
      For Umbral this is a warning against owning a protocol and an argument for owning
      conventions and local mechanisms instead.

CO-5  Instruction-file conventions (AGENTS.md, CLAUDE.md, GEMINI.md, .cursor/rules, Skills)
      Source: agents.md; the repository's own prior research (S10, DR-7).
      Marker: VERIFIED for existence and purpose (AGENTS.md project site); SNIPPET-LEVEL
      for the comparative landscape.
      Claims: AGENTS.md is a widely adopted convention for telling coding agents how to work
      in a repository; per-vendor equivalents exist (CLAUDE.md, GEMINI.md, .cursor/rules);
      Skills packages add progressive disclosure of procedures.
      Relevance: the CURRENT de-facto answer to "how do heterogeneous AIs understand one
      project" is a per-vendor instruction file. It is advisory, unenforced, provider-
      specific, and carries no state: it tells an agent how to behave, not what is true,
      what changed, or who else is acting.

================================================================================
PART 2 — SHARED STATE, MEMORY, AND COORDINATION
================================================================================
CO-6  Agent memory layers
      Sources: Letta (docs.letta.com — memory blocks, shared memory blocks); Mem0
      (arXiv:2504.19413; github.com/mem0ai/mem0); Zep/Graphiti (arXiv:2501.13956;
      github.com/getzep/graphiti).
      Marker: VERIFIED for Letta's shared-memory concept (documentation); SNIPPET-LEVEL for
      the papers' claims.
      Claims: memory blocks persist in an agent's context and can be SHARED between agents
      so that one agent's update is visible to others; Mem0 provides a managed memory layer
      between application and user; Graphiti/Zep builds a bi-temporal knowledge graph so
      facts carry validity intervals.
      Relevance: these solve memory FOR AGENTS (the agent's own context), not reality FOR A
      PROJECT. A shared memory block is still inside one vendor's runtime; it does not
      survive the agent's absence, and it is not the filesystem. Graphiti's bi-temporal
      model is the closest existing idea to "this fact was true between T1 and T2" — worth
      studying, but it is a database, not a plain-file convention.

CO-7  Staleness of agent context and memory (the closest prior art to this stage's question)
      Sources: "Fresh Memory, Stale Plans: Dependency-Scoped Validation for Distributed
      LLM-Agent Memory" (arXiv:2609.03340); "STALE: Can LLM Agents Know When Their Memories
      Are No Longer Valid?" (arXiv:2605.06527).
      Marker: SNIPPET-LEVEL (abstracts).
      Claims: distributed agent teams can read the latest shared facts and still act on an
      obsolete plan; validation must be dependency-scoped rather than global; current
      benchmarks measure static retrieval and overlook belief revision.
      Relevance: HIGH. This is direct evidence that the problem this stage is investigating
      is real, named, and unsolved — and that it is being studied at the level of agent
      MEMORY, not at the level of a shared filesystem that ordinary tools also write to.

CO-8  Coordination models predating LLMs (still the best conceptual vocabulary)
      Sources: blackboard architecture (the Umbral corpus already references it in S3/S6);
      Contract Net Protocol (Smith, 1980); Linda / tuple spaces (Gelernter).
      Marker: SNIPPET-LEVEL for the primary papers; VERIFIED for the concepts' existence
      and definitions.
      Claims: a blackboard is a shared data structure over which independent specialists
      coordinate by reading and writing state; contract net assigns tasks by announcement
      and bidding; tuple spaces provide associative shared memory decoupled from the
      processes using it.
      Relevance: Umbral's coexistence problem is a NEW INSTANCE of these old patterns, with
      three differences that matter: (1) participants are probabilistic, not deterministic;
      (2) the shared medium is the user's own filesystem, not an application's memory;
      (3) one participant (the human) holds authority the others cannot have.

CO-9  Concurrency control for multi-agent systems (research, 2026)
      Sources: "CoAgent: Concurrency Control for Multi-Agent Systems" (arXiv:2606.15376);
      "AgentRoom: Concurrent Multi-Agent Coding in a CRDT-Backed Shared Workspace"
      (arXiv:2608.23740).
      Marker: SNIPPET-LEVEL.
      Claims: CoAgent argues serializability is the right property for a multi-agent system
      and that classical techniques do not transfer directly to LLM agents; AgentRoom
      implements a shared workspace where agents call room_claim(path) to atomically take
      ownership of a file, rejecting the claim if another agent holds it, with an
      append-only broadcast log for coordination.
      Relevance: HIGHEST. AgentRoom is the closest published prior art to a "coexistence
      substrate" — and it is (a) a research prototype, (b) built on a CRDT workspace that
      is not the user's filesystem, and (c) requiring agents to use its MCP tools. That is
      exactly the design space Umbral must differentiate inside or decide it cannot beat.

CO-10  Why multi-agent systems fail
      Sources: "Why Do Multi-Agent LLM Systems Fail?" (arXiv:2503.13657) and the MAST
      taxonomy (sky.cs.berkeley.edu/project/mast/).
      Marker: SNIPPET-LEVEL.
      Claims: a taxonomy of failure modes for multi-agent LLM systems; performance gains
      over single agents are often minimal.
      Relevance: an argument AGAINST building multi-agent orchestration as the product's
      value, and FOR providing the conditions under which independently-chosen AIs do not
      corrupt shared state. It supports the substrate framing over the orchestrator framing.

CO-11  Durable execution and agent state persistence
      Sources: Temporal documentation (workflows, durable execution, "Durable AI");
      LangGraph checkpointers (docs.langchain.com).
      Marker: VERIFIED (product documentation).
      Claims: workflow engines persist execution state so a process can resume after
      failure; LangGraph checkpoints graph state at each super-step, enabling persistence,
      human-in-the-loop, and fault tolerance; checkpoints are identified by monotonically
      increasing ids.
      Relevance: these solve RESUMPTION inside one framework. They are provider- and
      framework-specific, which is precisely the cross-provider continuity gap (S3 §8
      already found the evidence weak).

CO-12  Semantic file systems (the 1991 attempt)
      Sources: Gifford et al., "Semantic File Systems" (1991); the USENIX retrospective
      introduction; Wikipedia.
      Marker: VERIFIED for the paper's existence and thesis; SNIPPET-LEVEL for the
      retrospective's explanation of why the approach did not become general.
      Claims: semantic file systems provide associative access to files by extracting
      attributes with file-type transducers and exposing virtual directories.
      Relevance: the same idea, thirty-five years earlier, and it did not displace the
      hierarchical filesystem. The Umbral corpus already treats this as a design constraint
      (S1/S8); it belongs in this file because it is the precedent for "put semantics in the
      filesystem layer".

================================================================================
PART 3 — AUTHORITY, PROVENANCE, IDENTITY
================================================================================
CO-13  W3C PROV (PROV-DM / PROV-O)
      Source: w3.org/TR/prov-dm/ and w3.org/TR/prov-o/.
      Marker: VERIFIED (W3C Recommendations).
      Claims: provenance is information about entities, activities and people involved in
      producing a thing; PROV-DM is the conceptual model, PROV-O its OWL encoding.
      Relevance: the standard vocabulary for "who/what/why" already exists and is stable.
      The Umbral corpus already cites it (S4, S6). What does NOT exist is a convention for
      attaching it to ordinary files in a way that survives tools which know nothing about
      it.

CO-14  in-toto attestations
      Sources: in-toto.io; github.com/in-toto/attestation.
      Marker: VERIFIED (project pages).
      Claims: a specification for generating verifiable claims about how an artifact was
      produced, verifying that each step was carried out as planned and by authorized
      parties.
      Relevance: a mature model for "attested chain of custody" that Umbral could borrow
      conceptually (claims, steps, authorization) without importing the machinery. Note the
      same caveat as C2PA in the Umbral corpus: a valid attestation proves origin, not truth.

CO-15  Agent identity (SPIFFE/SPIRE, delegated authorization)
      Sources: Google Cloud "Agent Identity" documentation (SPIFFE-based);
      draft-lundholm-kaif (Kindred Agent Identity Framework, IETF datatracker — combines
      RFC 8693 token exchange with SPIFFE workload identity).
      Marker: VERIFIED for the Google product documentation; SNIPPET-LEVEL for the IETF
      draft (an individual draft, not a standard).
      Claims: agents can be given strongly attested cryptographic identities; delegated
      agent-to-service authorization can be built on token exchange plus workload identity.
      Relevance: identifies WHO an agent is, machine-to-machine. It does not answer what an
      agent's *interpretation* means, nor who owns a claim about the project. Useful for
      the "which agent did this" half of provenance; silent on the epistemic half.

CO-16  Version control as coordination (the existing answer to multi-writer files)
      Sources: the Umbral corpus already covers Git/Jujutsu/Fossil at length (S6, S11);
      Kleppmann et al., "A Highly-Available Move Operation for Replicated Trees"
      (IEEE TPDS 2021; trvedata/move-op) for the CRDT tree case.
      Marker: VERIFIED for the papers and projects; the corpus entries are as previously
      recorded.
      Claims: Git already provides content-addressed history, branching, merge, and — via
      worktrees — isolated checkouts for parallel workers; the move-op paper provides a
      verified algorithm for conflict-free concurrent directory moves.
      Relevance: this is the strongest competing answer to the whole stage (see the
      adversarial critique). It solves multi-writer SAFETY, not multi-interpreter MEANING,
      and it requires every participant to be a Git participant.

================================================================================
PART 4 — HUMAN INTERFACE TO AUTOMATION
================================================================================
CO-17  Mixed-initiative interaction
      Source: Horvitz, "Principles of Mixed-Initiative User Interfaces" (CHI 1999).
      Marker: VERIFIED (paper page and PDF).
      Claims: interfaces should combine direct manipulation with automated services,
      deciding when to act, when to ask, and when to stay out of the way, weighing the cost
      of failure against the cost of interruption.
      Relevance: the classical statement of the human-control problem this stage is about.
      It supplies the criterion Umbral's authority model must satisfy: automation must be
      interruptible and its failures must be cheap, not merely "permissioned".

CO-18  Adjacent products claiming "agent-native workspace"
      Sources: even.dev ("the agent-native workspace"); agentmux.ai ("agent operating
      environment", running Claude Code, Codex and Gemini agents as first-class panes each
      with its own identity and memory); everydev.ai listing for Vecbase ("one workspace,
      one AI team... shared memory and files").
      Marker: SNIPPET-LEVEL (product sites; no independent evaluation).
      Claims: products already position themselves as workspaces where multiple
      heterogeneous agents work together over shared files and memory.
      Relevance: HIGH for differentiation. The "many agents in one workspace" space is
      already occupied at the orchestration/terminal level. What none of them claims is the
      user's own filesystem remaining the substrate with plain-file, provider-neutral
      conventions and a rebuildable projection. This is where Umbral must differentiate or
      decline to compete.

================================================================================
WHAT THIS EVIDENCE DOES NOT SETTLE
================================================================================
- Whether the coexistence problem causes MATERIAL harm to real users, or whether it is a
  design-time concern that rarely bites (no study found either way). See experiment E-CO-1.
- Whether an ordinary user with two or three AI subscriptions experiences any of this, or
  whether it is felt only by developers running concurrent agents.
- Whether Umbral's plain-file constraint is compatible with the coordination primitives that
  actually work (claims, leases, validity) or whether those require a live process.
- Whether the convergence of A2A/MCP will absorb the problem before Umbral could ship
  anything (CO-3, CO-4, CO-5).
