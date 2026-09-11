# BRIEF-C — Cross-AI Handoff (work moving between providers)

For an external reasoning model. Narrow problem, deep reasoning. Output is external
research, not a project decision.

--------------------------------------------------------------------------------
1. PROBLEM
--------------------------------------------------------------------------------
A person starts a piece of work with one AI (say, architectural thinking), continues it
with a second (say, code editing), and reviews it with a third. Today the only portable
thing between them is the raw conversation — which is provider-shaped, full of dead ends,
and often unusable by the next model.

QUESTION: How can work move from one AI provider to another without losing project
continuity — and what exactly has to travel?

--------------------------------------------------------------------------------
2. CONSTRAINTS
--------------------------------------------------------------------------------
- No provider cooperation; no shared protocol adoption.
- The artifact must be readable by a human and usable by any model.
- The transcript is not a reliable carrier (provider-specific formats, private reasoning
  traces, dead ends).
- Context is expensive; the handoff must be small.
- Nothing leaves the machine except what the user authorises.

--------------------------------------------------------------------------------
3. KNOWN EVIDENCE
--------------------------------------------------------------------------------
- The execution tier (system prompts, tool schemas, cache breakpoints, reasoning traces)
  is provider-specific and does not transfer; a semantic tier (goal, decisions, touched
  files, blockers, next step) plausibly does. This was already analysed in earlier project
  research, which concluded the cross-provider evidence is WEAK.
- Framework handoffs exist but are intra-framework: [OI] Agents SDK handoffs move control
  between agents inside that SDK; LangGraph checkpointers persist state inside that
  framework; Temporal makes execution durable inside that engine.
- Protocol-level task handoff exists across vendors in principle (A2A Task/Artifact/
  AgentCard), but assumes both sides speak A2A.
- Distributed-agent research finds that teams can hold fresh facts and still act on an
  obsolete plan, so a handoff artifact needs validity, not just content (arXiv:2609.03340).

--------------------------------------------------------------------------------
4. UNKNOWNS
--------------------------------------------------------------------------------
- Whether a structured receipt actually improves a receiving model's performance versus
  giving it the goal alone, or versus giving it the full transcript.
- How much of a receipt a receiving model ignores.
- Whether models write honest receipts (including what they failed to do).
- Whether the user can maintain handoff artifacts, or whether they decay like every other
  document that requires discipline.

--------------------------------------------------------------------------------
5. WHAT WE WANT FROM YOU
--------------------------------------------------------------------------------
1. What is the minimum set of fields a handoff artifact needs to be useful to a DIFFERENT
   model family — and what should deliberately be excluded?
2. What is the biggest risk that a handoff artifact misleads the receiving model, and how
   would you design against it?
3. Is a human-readable artifact the right carrier, or is a machine format better? Argue
   both sides and take a position.
4. How should a receiving model know the handoff is STALE (the project changed since it was
   written)?
5. Design the cheapest experiment that would show whether handoffs work at all.

--------------------------------------------------------------------------------
6. OUTPUT FORMAT
--------------------------------------------------------------------------------
- Plain-language summary first (5 sentences maximum).
- Then: minimum fields; misleading risk + mitigation; human vs machine carrier (with a
  position); staleness handling; the experiment design (hypothesis, arms, measurement,
  success and failure criteria).
- Label claims ESTABLISHED / REASONED / SPECULATIVE.

--------------------------------------------------------------------------------
7. PLAIN-LANGUAGE NOTE FOR THE USER
--------------------------------------------------------------------------------
In one sentence: *if you start something with one AI and finish it with another, what
exactly has to be written down so the second one can continue properly?* This brief asks a
model to specify that minimum and design the test.
