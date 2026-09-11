# BRIEF-B — Multi-Agent Coexistence Around One Filesystem

For an external reasoning model. Narrow problem, deep reasoning. Your output is external
research, not a project decision, and not a product design.

--------------------------------------------------------------------------------
1. PROBLEM
--------------------------------------------------------------------------------
On one person's computer, a single project folder is written to by: the person, several AI
assistants from different vendors, an IDE, a formatter, scripts, a file-sync client, and a
version-control tool. Nothing coordinates them. The filesystem's answer to two writers is
"last write wins", silently.

QUESTION: How can multiple independent AI systems safely coexist around one filesystem,
where the filesystem stays the user's and nothing is allowed to become the central
orchestrator of all AI?

--------------------------------------------------------------------------------
2. CONSTRAINTS
--------------------------------------------------------------------------------
- The filesystem is shared with tools that know nothing about this system.
- No central orchestrator may be required; participants are chosen by the human.
- No vendor cooperation may be assumed.
- Degradation, not dependency: with the coordination mechanism switched off, the project
  must remain workable.
- Local-first; plain files must remain the durable layer.
- The human is the only participant with authority.

--------------------------------------------------------------------------------
3. KNOWN EVIDENCE
--------------------------------------------------------------------------------
- The closest published prior art implements atomic file ownership: agents call
  `room_claim(path)`, which rejects the claim if another agent holds it, plus an
  append-only broadcast log (AgentRoom, arXiv:2608.23740). It requires a CRDT-backed
  workspace and agents that use its tools.
- Concurrency-control research for multi-agent systems argues serializability is the
  desired property and that classical techniques do not transfer cleanly to LLM agents
  (CoAgent, arXiv:2606.15376).
- Pre-LLM coordination models already exist and are well specified: blackboards,
  Contract Net, tuple spaces (Linda). Their participants are deterministic; LLM agents are
  probabilistic.
- Version control already solves multi-writer safety for text: content-addressed history,
  branching, merge, and isolated parallel checkouts via worktrees.
- File-watching and reconciliation over a live filesystem have hard kernel limits and
  atomic-save behaviour that destroys inodes (established in earlier project work).

--------------------------------------------------------------------------------
4. UNKNOWNS
--------------------------------------------------------------------------------
- Whether "advisory visibility" (claims nobody is obliged to honour) reduces collisions at
  all, or is theatre.
- Whether a claim/lease can be expressed in plain files without a live process, given that
  atomicity normally requires one.
- Whether the human can tolerate a new class of state ("who is working on what").
- Whether real users experience concurrent-writer harm outside of developer workflows.

--------------------------------------------------------------------------------
5. WHAT WE WANT FROM YOU
--------------------------------------------------------------------------------
1. Attack the premise: is "coexistence" a real problem, or a developer-only problem that
   version control already solves? Give your honest assessment and what evidence would
   settle it.
2. If real: what is the minimum set of primitives that makes concurrent participation safe
   without a central orchestrator? Justify each primitive's existence.
3. Advisory vs enforced: can advisory coordination ever work with probabilistic
   participants, or is enforcement mandatory? What does enforcement cost?
4. Where exactly does the plain-file constraint break the mechanism, and what is the least
   harmful concession?
5. Name the failure mode you consider most likely to kill such a system in practice.

--------------------------------------------------------------------------------
6. OUTPUT FORMAT
--------------------------------------------------------------------------------
- Plain-language summary first (5 sentences maximum).
- Then: premise assessment; minimum primitive set; advisory-vs-enforced position; the
  plain-file breaking point; most likely killer.
- Label claims ESTABLISHED / REASONED / SPECULATIVE.
- Say explicitly what you would NOT build.

--------------------------------------------------------------------------------
7. PLAIN-LANGUAGE NOTE FOR THE USER
--------------------------------------------------------------------------------
In one sentence: *when several AI assistants and your own tools edit the same folder at the
same time, how do they avoid trampling each other without putting one program in charge of
everything?* This brief asks a model to try to break that idea before we build anything.
