# BRIEF-F — Minimum Self-Description for an Unknown AI

For an external reasoning model. Narrow problem, deep reasoning. Your output is external
research, not a project decision.

--------------------------------------------------------------------------------
1. PROBLEM
--------------------------------------------------------------------------------
An AI that has never heard of this tool, this project, or this user encounters the
environment. It may support no standard protocol, it may be closed-source, it may be a
runtime nobody has seen yet. Before it acts, it must know four things it cannot safely guess:

  what this environment is;
  what it can be asked to do;
  what it is CURRENTLY AUTHORIZED to do;
  and what it must not do.

QUESTION: What is the minimum self-description that lets an unknown AI become a competent
participant — and specifically, how is authority conveyed to a participant that has no
pre-existing trust relationship with anything in the environment?

--------------------------------------------------------------------------------
2. CONSTRAINTS
--------------------------------------------------------------------------------
- Must work as ordinary files, with no live process, no SDK, no protocol support.
- Must not require the AI's vendor to cooperate or to have implemented anything.
- Must be cheap: the description competes with the project's own content for context.
- Must be honest about what is unknown.
- Must not be able to grant authority; it may only describe it.
- Must remain useful if the AI ignores it entirely (the environment must not depend on
  compliance).

--------------------------------------------------------------------------------
3. KNOWN EVIDENCE (established; do not re-derive)
--------------------------------------------------------------------------------
- Self-description for agents already exists in standardised form: A2A agent cards at a
  well-known path describe identity, reachability, capabilities and how access is secured;
  MCP servers expose tool lists with JSON Schemas; Kubernetes publishes its own API
  specification; D-Bus objects can be introspected at runtime; JMX registers MBeans; SNMP
  defines MIBs.
- Agent Skills implement progressive disclosure in a filesystem-based architecture: content
  is loaded in stages, not upfront.
- Agent platforms now ship on-demand tool discovery and programmatic tool calling precisely
  because definitions alone can consume tens of thousands of tokens before the agent reads
  the request.
- Tool-interface quality is a measured first-order determinant of agent success, and the
  failure worsens as candidate tools multiply; a 2026 study reports large gains from
  rewriting tool descriptions for agents rather than humans.
- Tool selection failures are diagnosable (models that never call a needed tool look
  identical, in aggregate, to models that call it and ignore the result).
- Progressive disclosure is a 2006 human-interface principle whose documented failure mode is
  hiding too much.

--------------------------------------------------------------------------------
4. UNKNOWNS
--------------------------------------------------------------------------------
- Whether an authority statement in a FILE changes agent behaviour at all, or is ignored.
- Whether models can reliably distinguish "you may request this" from "you are authorized
  for this" — we suspect they cannot, and that this is the crux.
- Whether a safety floor can be stated without being so alarming that it suppresses useful
  action, or so terse that it is skimmed.
- Whether the minimum surface should be one file, several files, or a generated file that
  cannot be hand-edited.

--------------------------------------------------------------------------------
5. WHAT WE WANT FROM YOU
--------------------------------------------------------------------------------
1. Specify the minimum self-description: exactly what must be in it, and what must be left
   out (with reasons for the exclusions). Assume a hard budget of a few hundred words.
2. Solve the authority problem concretely: given that the environment cannot trust the AI and
   the AI cannot verify the environment, how should "you are currently authorized for X" be
   represented so that it is neither ignorable nor self-granting?
3. Design against the two failure modes: an AI that acts without reading the description,
   and an AI that reads it and over-claims authority.
4. Propose the measurement: a task set and scoring that would show whether a fresh model
   given only this surface can (a) identify what mutates, (b) identify what needs
   confirmation, (c) state its own authority correctly, (d) name something it must not do.
5. Argue the case that this whole layer is unnecessary because platform-level permission
   systems and tool schemas already cover it.

--------------------------------------------------------------------------------
6. OUTPUT FORMAT
--------------------------------------------------------------------------------
- Plain-language summary first (5 sentences maximum).
- Then: the minimum surface (as a concrete sketch); the authority representation; the two
  failure modes and their mitigations; the measurement design; the case against.
- Label every claim ESTABLISHED / REASONED / SPECULATIVE.

--------------------------------------------------------------------------------
7. PLAIN-LANGUAGE NOTE FOR THE USER
--------------------------------------------------------------------------------
In one sentence: *what is the smallest thing you can write down so that any AI — including
one that has never heard of this tool — knows what it is allowed to do before it touches your
files?* This brief asks a model to specify that minimum and how to test whether it works.
