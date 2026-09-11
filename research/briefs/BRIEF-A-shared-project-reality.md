# BRIEF-A — Shared AI Project Reality

For an external reasoning model (Astra, Fable, Sol/ChatGPT, or another). You are NOT being
asked to summarize or design a product. You are being asked to think hard about one narrow
problem. Your output is external research, not a project decision.

--------------------------------------------------------------------------------
1. PROBLEM
--------------------------------------------------------------------------------
Several different AI systems, made by different vendors, work on the same project folder
on one person's computer. Each builds its own private understanding of the project from
whatever it happened to read. None of them can tell whether that understanding is still
true, and none can see what the others understood.

Today the only way to be sure is to read everything again.

QUESTION: What abstraction would let heterogeneous AIs reason about the same evolving
project without sharing the same internal context?

--------------------------------------------------------------------------------
2. CONSTRAINTS (only those relevant here)
--------------------------------------------------------------------------------
- The person's own files are the source of truth and must stay readable and editable by
  ordinary tools with the software switched off.
- No mandatory folder structure or taxonomy.
- Local-first: nothing leaves the machine without explicit authorization.
- No assumption that AI vendors cooperate, adopt a protocol, or even know this exists.
- It must work for AI systems that do not exist yet.
- A human holds authority that no AI may hold.

--------------------------------------------------------------------------------
3. KNOWN EVIDENCE (established, with sources)
--------------------------------------------------------------------------------
- Agent memory layers exist and can be shared between agents, but inside one vendor's
  runtime (Letta memory blocks; Mem0; Graphiti/Zep bi-temporal graph).
- The staleness problem is named and studied: distributed agent teams "can read the latest
  shared facts and still act on an obsolete plan"; validation must be dependency-scoped
  (arXiv:2609.03340); current benchmarks do not measure belief revision (arXiv:2605.06527).
- Interoperability protocols exist and are converging: MCP (tools/resources; its 2026-07-28
  revision deprecated Roots, Sampling and Logging), A2A (task/artifact/agent-card, Linux
  Foundation), ACP (editor↔agent) and IBM's ACP (now folded into A2A).
- The de-facto current answer for "how does an AI understand this repo" is a per-vendor
  instruction file: AGENTS.md, CLAUDE.md, GEMINI.md, .cursor/rules. Advisory, provider-
  specific, carries no state.
- Multi-agent LLM systems often fail to outperform single agents, and a published taxonomy
  of their failure modes exists (MAST, arXiv:2503.13657).

--------------------------------------------------------------------------------
4. UNKNOWNS (do not assume an answer)
--------------------------------------------------------------------------------
- Whether staleness causes material harm to real users, or is a design-time concern.
- Whether validity can be expressed in plain files at all, or requires a live process.
- Whether models can be induced, reliably and cheaply, to declare what they read.
- Whether a shared validity signal changes behaviour or is ignored like a lock file.

--------------------------------------------------------------------------------
5. WHAT WE WANT FROM YOU
--------------------------------------------------------------------------------
Reason about, and answer as sharply as you can:
1. Is "shared validity" (knowing whether your understanding is current) the right
   abstraction, or is the real missing abstraction something else? Name alternatives.
2. What is the smallest artifact that could carry validity between heterogeneous AIs —
   and could it be a plain file that ordinary tools ignore harmlessly?
3. What breaks first as participants scale from two to ten?
4. What would you refuse to build here, and why?
5. Propose ONE mechanism not described above, and state the experiment that would
   falsify it.

--------------------------------------------------------------------------------
6. OUTPUT FORMAT
--------------------------------------------------------------------------------
- A plain-language summary first (5 sentences maximum, no jargon).
- Then: proposed abstraction(s); smallest artifact; scale failure modes; what you would
  refuse; one novel mechanism + falsification test.
- Label every claim you make as ESTABLISHED / REASONED / SPECULATIVE.
- State explicitly where you disagree with the framing above.

--------------------------------------------------------------------------------
7. PLAIN-LANGUAGE NOTE FOR THE USER (why this brief exists)
--------------------------------------------------------------------------------
The question in one sentence: *when several different AI assistants work on your project,
how can each of them tell whether what it knows is still true, without reading everything
again?* This brief asks a model to attack that question and propose something concrete.
It is one of four narrow briefs; none of them asks a model to design FSP.
