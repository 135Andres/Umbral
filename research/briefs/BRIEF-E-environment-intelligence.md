# BRIEF-E — Environment Intelligence

For an external reasoning model (Astra, Fable, Sol/ChatGPT, or another). You are NOT being
asked to summarize or design a product. You are being asked to think hard about one narrow
question and to attack the framing if it is wrong.

Your output is EXTERNAL RESEARCH. It is not a project decision, it carries no authority over
this project, and it will be critiqued before anything from it enters project memory.

--------------------------------------------------------------------------------
1. PROBLEM
--------------------------------------------------------------------------------
A person works in an ordinary filesystem. Several AI systems, IDEs, scripts, sync tools and
the person all read and write it. Each AI reconstructs the project from scratch by reading
files. Every one of them can be wrong in ways that reading cannot reveal: it cannot tell
whether a decision it read about was later superseded, whether the user actually decided
something or merely discussed it, whether another participant is editing a file right now, or
who wrote a given sentence.

QUESTION: What should an environment know about ITSELF that a language model should not have
to reconstruct — and what should it deliberately NOT know?

--------------------------------------------------------------------------------
2. CONSTRAINTS
--------------------------------------------------------------------------------
- The knowledge must survive the software that produced it: readable as ordinary files.
- No participant may own it, including the tool that maintains it.
- No vendor cooperation may be required to use it.
- It must degrade: with the tool switched off, the files must remain intelligible.
- The human is the only authority; the environment may report standing, never grant it.
- It must not impose a taxonomy on the user's files.

--------------------------------------------------------------------------------
3. KNOWN EVIDENCE (established; do not re-derive)
--------------------------------------------------------------------------------
- Capability discovery is a solved, standardised pattern for cooperating participants:
  MCP tools, A2A agent cards, Kubernetes API discovery, D-Bus introspection, JMX, SNMP MIB,
  OpenAPI. All require a live process, a cooperating peer, or both.
- The only shipped example of runtime truth exposed as ordinary files, usable by anything
  with no cooperation, is /proc and sysfs — and it exposes state, not meaning or authority.
- Truth maintenance systems (Doyle 1979; de Kleer 1986) solved justification-tracking and
  belief retraction decades ago; argumentation frameworks (Dung 1995) formalise competing
  claims. Neither has a filesystem embodiment.
- Kubernetes controllers reconcile actual state toward a DECLARED desired state. A user's
  filesystem has no declared desired state.
- Tool-interface quality measurably determines agent success, and degrades as the number of
  candidate tools grows.
- MCP does not version the state a tool returns; stale agent belief is a named, measured
  failure ("fresh memory, stale plans").
- Provenance vocabularies (W3C PROV, in-toto) prove origin, not truth, and assume the actor
  identifies itself.

--------------------------------------------------------------------------------
4. UNKNOWNS (we genuinely do not know these)
--------------------------------------------------------------------------------
- Whether participants actually act on stale understanding in ways that cause material harm,
  or whether they re-read files often enough that staleness is self-correcting.
- Whether "who had the authority to decide this" is operationally useful or philosophically
  decorative.
- Whether an environment can hold an assessment without becoming an implicit authority.
- Whether any of this matters to ordinary users, or only to developers running concurrent
  agents. Every piece of harm evidence we found concerns developers.

--------------------------------------------------------------------------------
5. WHAT WE WANT FROM YOU
--------------------------------------------------------------------------------
1. Draw the boundary: list the facts about an environment that a model genuinely CANNOT
   recover by reading the files, and ruthlessly exclude everything it can. We suspect this
   list is much shorter than the proposals assume; tell us if we are wrong in either
   direction.
2. For each surviving fact, say where it should live so that it survives the software and
   does not become a second source of truth.
3. What should the environment deliberately NOT know or record? Name the knowledge whose
   collection would be harmful (privacy, manipulation, or over-reach) even if technically
   possible.
4. Attack the framing: is "the environment knows about itself" a real distinction, or is it
   observability plus provenance wearing a new name? Give the strongest version of the
   reductionist case, then say what, if anything, survives it.

--------------------------------------------------------------------------------
6. OUTPUT FORMAT
--------------------------------------------------------------------------------
- Plain-language summary first (5 sentences maximum).
- Then: the cannot-recover list (with exclusions); where each fact should live; the
  do-not-record list; the reductionist case and what survives it.
- Label every claim ESTABLISHED / REASONED / SPECULATIVE.

--------------------------------------------------------------------------------
7. PLAIN-LANGUAGE NOTE FOR THE USER
--------------------------------------------------------------------------------
In one sentence: *what does a project environment know about itself that an AI cannot figure
out by simply reading the files?* This brief asks a model to draw that line precisely — and
to argue that the line may be much thinner than we think.
