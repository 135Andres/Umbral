# BRIEF-D — Authority, Disagreement and Provenance Among AIs

For an external reasoning model. Narrow problem, deep reasoning. Output is external
research, not a project decision.

--------------------------------------------------------------------------------
1. PROBLEM
--------------------------------------------------------------------------------
Several AIs read the same project and disagree. One says X; another says not-X; the person
has not decided. Or: two AIs agree on X and the person says not-X. Or: an AI changes a file
while another was reasoning about the old version.

Today all of these collapse into the same bytes on disk, with no record of who said what,
on what basis, or that a disagreement ever existed.

QUESTION: How should conflicting AI interpretations, proposals and actions coexist without
allowing any AI to silently become authoritative?

--------------------------------------------------------------------------------
2. CONSTRAINTS
--------------------------------------------------------------------------------
- The human is the only authority; AI output enters as a proposal, never as accepted truth.
- No hidden state: whatever records this must survive the software's disappearance.
- Models will not reliably self-report accurately.
- The system must not become the arbiter (that would move authority from the user to it).
- Disagreement must remain visible rather than being resolved by whoever wrote last.

--------------------------------------------------------------------------------
3. KNOWN EVIDENCE
--------------------------------------------------------------------------------
- Mature provenance vocabularies exist: W3C PROV-DM/PROV-O (entities, activities, agents)
  and in-toto attestations for verifiable claims about how an artifact was produced.
- A valid attestation proves ORIGIN, not TRUTH (established in earlier project research on
  C2PA).
- Agent identity standards exist for machine-to-machine authentication (SPIFFE-based agent
  identity; delegated authorization via token exchange). They answer "which agent is this",
  not "what does its claim mean".
- Long-standing HCI work on mixed-initiative interaction (Horvitz, CHI 1999) frames the
  real criterion: automation must be interruptible, and the cost of its mistakes must be
  low — not merely "permissioned".
- Research on agent memory validity shows stale beliefs are a first-class failure
  (arXiv:2609.03340, arXiv:2605.06527).

--------------------------------------------------------------------------------
4. UNKNOWNS
--------------------------------------------------------------------------------
- Whether users want to see disagreement, or find it noise.
- Whether an AI's stated basis (evidence, context used) can be trusted enough to record.
- Whether "contested" as a durable state helps decisions or blocks them.
- Whether attribution can be honest: when a change appears, can the system say UNKNOWN
  rather than guess, and would that be acceptable?

--------------------------------------------------------------------------------
5. WHAT WE WANT FROM YOU
--------------------------------------------------------------------------------
1. Design the minimum record that lets a project hold three simultaneous states: a claim by
   AI A, a contradicting claim by AI B, and no decision by the user. Keep it small.
2. Where should such records live so they survive the software, stay human-readable, and do
   not become a second source of truth?
3. What stops this from degenerating into a pile of AI opinions nobody reads? Be specific.
4. How should the system represent "the user has not decided" without implying "no
   position"? And "the user decided" without implying "the AI was right"?
5. What is the strongest argument that this whole layer is unnecessary and the user should
   simply read their own files?

--------------------------------------------------------------------------------
6. OUTPUT FORMAT
--------------------------------------------------------------------------------
- Plain-language summary first (5 sentences maximum).
- Then: minimum record; where it lives; anti-sprawl mechanism; the two state
  representations; the strongest argument against the layer.
- Label claims ESTABLISHED / REASONED / SPECULATIVE.

--------------------------------------------------------------------------------
7. PLAIN-LANGUAGE NOTE FOR THE USER
--------------------------------------------------------------------------------
In one sentence: *when two AI assistants tell you different things about your own project,
how do you keep both answers visible, keep your own decision separate from theirs, and know
who said what?* This brief asks a model to design the smallest thing that makes that
possible — and to argue why it might be unnecessary.
