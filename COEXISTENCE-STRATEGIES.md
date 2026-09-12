# COEXISTENCE STRATEGIES

Status: **PROPOSED / UNRATIFIED.** Candidate strategies for the current central research
direction (UD-011). Nothing here is selected, ranked as a decision, or architecture.
Every strategy is a hypothesis; every invented mechanism is labelled as such.

Layers (per the mandate's §11): each entry states whether it concerns PRODUCT THESIS,
ARCHITECTURAL HYPOTHESIS, or IMPLEMENTATION MECHANISM, and none is promoted upward.

Evidence base: research/COEXISTENCE-RESEARCH.md (CO-n), plus the already-ingested S1-S11.
No strategy below is supported by evidence for its OUTCOME — only for the problems it
addresses. That distinction is the point of the experiments in §4.

================================================================================
0. THE PROBLEM, STATED PLAINLY
================================================================================
A real project folder is written to by: the human, one or more AI assistants, an IDE, a
formatter, a script, a sync client, and a version-control tool. Each AI arrives with its
own private context, built at some earlier moment, from a partial read of that folder.

Three failures follow, and they are not the same failure:

  F1 SHARED REALITY — no participant can tell whether what it understands is still true.
     Every AI re-derives the project from scratch; none can ask "has anything changed
     since I looked?" Today the only answer is "read everything again".
  F2 CONCURRENT ACTION — writers do not know about each other. At the filesystem level the
     last write wins silently; nothing records that two participants were editing the same
     thing, or that one of them was reasoning from a version that no longer exists.
  F3 AUTHORITY — an AI's output is indistinguishable in form from a human's. A model's
     inference, a model's proposal and a user's decision all end up as the same bytes in
     the same file, with no record of which was which, or of the disagreement when two
     models say different things and the user has not decided.

The candidate insight of this stage (labelled HYPOTHESIS, tested in §4):

> The missing abstraction is not shared CONTEXT (retrieval, memory and RAG already give
> that) and not shared HISTORY (Git gives that). It is shared VALIDITY: a cheap, portable
> way for any participant to know whether what it believes about the project is still
> true, who else is acting, and whose word is whose.

================================================================================
1. SEVEN CANDIDATE STRATEGIES
================================================================================
Each: core idea / what Umbral owns / what stays external / strengths / weaknesses / failure
modes / implementation implications / UX / interoperability / privacy / provenance /
concurrency / reversibility / complexity / enables / prevents.
Mechanical fields are compressed; the ones that decide the strategy are in prose.

--------------------------------------------------------------------------------
S1 — THIN CONVENTION (files, a manifest, and no engine)
--------------------------------------------------------------------------------
LAYER: product thesis (minimal) + architectural hypothesis (weak).
Core idea: coexistence is achieved by AGREEMENT, not by software: a small set of plain-file
conventions in the project folder (who may act, where outputs go, how a handoff is
recorded, where claims live), plus a validator anyone can run.
Owns: the conventions, their documentation, and a checker. Nothing else.
External: all state, all enforcement, all storage.
Strengths: zero lock-in; works with any AI that can read files (CO-5); survives the
software's disappearance by construction (A1/A2); trivially local-first; no hidden metadata.
Weaknesses: advisory only; no enforcement, no validity signal, no concurrency safety; each
provider must be told (adoption is per-vendor, CO-5); decays without discipline (C9).
Failure modes: two AIs both write "the plan" and both are right; conventions rot; a
participant ignores them because nothing compels it.
Implementation: near zero. UX: invisible (good and bad). Interop: maximal. Privacy: maximal.
Provenance: whatever participants choose to write. Concurrency: none. Reversibility: total.
Complexity: minimal. Enables: nothing that requires a live mechanism. Prevents: everything
that requires coordination in real time.

--------------------------------------------------------------------------------
S2 — CONTEXT BROKER (Umbral assembles what each AI should see)
--------------------------------------------------------------------------------
LAYER: architectural hypothesis. (This is the S2/S3 thesis, restated.)
Core idea: Umbral's job is to give each participant the smallest sufficient context, and to
control what leaves the machine.
Owns: retrieval, ranking, budget, egress control.
External: state ownership, authority, concurrency.
Strengths: directly addresses F1's cost (re-reading everything) and the context-economy
thesis; established literature and tooling (S2); privacy control point (A7).
Weaknesses: does not address F2 or F3 at all; a broker without a substrate is a search
engine; every AI must be told to use it; the "context engine" costs documented in S2 §10
apply in full.
Failure modes: context assembled from a stale index (EXP-1's Q8-class problem); nobody
calls it; it becomes the heavy daemon S2 §17 warned against.
Implementation: substantial (indexing, ranking, budget). UX: invisible. Interop: via MCP.
Privacy: strong. Provenance: only of what it assembled, not of what was done with it.
Concurrency: none. Reversibility: high. Complexity: high. Enables: cheaper AI work.
Prevents: nothing, but solves only a third of the problem.

--------------------------------------------------------------------------------
S3 — AUTHORITY AND PROVENANCE LEDGER (Umbral owns the epistemic layer)
--------------------------------------------------------------------------------
LAYER: architectural hypothesis. (The S4/S6 thesis.)
Core idea: Umbral records decisions, proposals, provenance and epistemic state; AI output
enters as a proposal; only a human act graduates it to accepted truth.
Owns: the epistemic record — who claimed what, on what evidence, with what authority.
External: the files themselves and every writer that is not Umbral.
Strengths: directly addresses F3, the failure with the highest trust cost; the corpus
already has the vocabulary (epistemic/lifecycle/authority axes, S4); aligns with A5.
Weaknesses: needs participation (or must infer, which is unreliable); risks hidden state
(A1/A2) unless the ledger is plain files; the user must actually curate proposals, and C9
says they will not maintain what they are not obliged to maintain.
Failure modes: proposal backlog nobody reviews; ledger and filesystem diverge; an
unattributed change gets silently attributed to the user.
Implementation: moderate. UX: must be near-invisible or it becomes maintenance. Interop:
none required. Privacy: the ledger itself is sensitive (C12). Provenance: its whole point.
Concurrency: none (records, not prevents). Reversibility: high. Complexity: moderate.
Enables: defensible answers to "why is this here" and "who decided this". Prevents:
nothing by itself.

--------------------------------------------------------------------------------
S4 — COORDINATION SUBSTRATE (Umbral mediates; it does not dispatch)
--------------------------------------------------------------------------------
LAYER: architectural hypothesis (the most coexistence-specific one).
Core idea: Umbral provides the primitives that make concurrent participation safe: claims on
paths, a reality version, validity signals, and attributed change records. AIs remain
peers that the USER chooses; Umbral never orchestrates them.
Owns: coordination primitives and their integrity.
External: which AIs exist, what they do, how they are invoked.
Strengths: addresses F1+F2+F3 in one place; matches the closest prior art's finding that
atomic ownership claims are what make concurrent agents safe (CO-9); does not require Umbral
to win the orchestration market (CO-10 warns against orchestration).
Weaknesses: Umbral becomes a dependency for the very thing it promises (if the mediator is
down, do AIs stop?); participants must call it (adoption); the primitives are only useful
if more than one participant exists — the same "nobody needs it alone" problem as
versioning; plain-file constraint may conflict with atomic claims (see M3 and Q17).
Failure modes: claims leak (agent crashes holding a lease); reality version goes stale
between check and write (TOCTOU); the mediator becomes a single point of failure and of
authority.
Implementation: substantial; the hard part is the plain-file/atomicity tension.
UX: needs a human view of "who is doing what" or it is invisible machinery.
Interop: an MCP server + a CLI are the adoption paths. Privacy: local by default.
Provenance: by construction. Concurrency: its whole point. Reversibility: high if claims
are advisory. Complexity: high. Enables: safe parallel work by independently chosen AIs.
Prevents: nothing, but it commits Umbral to being a live process (see the counterargument).

--------------------------------------------------------------------------------
S5 — FILESYSTEM-MEDIATED ISOLATION (use what version control already does)
--------------------------------------------------------------------------------
LAYER: implementation mechanism, elevated by its advocates to architectural hypothesis.
Core idea: do not mediate at all — isolate. Each participant works in its own checkout
(Git worktrees, branches, or a copy), and the human integrates. Coexistence becomes a merge
problem, which is solved.
Owns: the integration experience — "Git without Git" surfaced as review-and-accept.
External: everything else, including the isolation mechanism itself.
Strengths: battle-tested, reversible, no hidden state, works today with any agent that can
run git; the human keeps authority naturally (they accept or reject).
Weaknesses: only works for text, in git repos, for participants that are git-aware;
merges are textual, not semantic — two AIs can merge cleanly and still be incoherent;
fails exactly for the non-developer user the charter puts first (MC §1); "Git without Git"
is undercut if the mechanism IS Git.
Failure modes: merge conflicts dumped on a user who does not know what a conflict is;
divergent branches nobody reconciles; the isolation hides work from the other participants
so F1 gets worse, not better.
Implementation: low (orchestrate existing tools). UX: the hard part, and it is the same
hard part as Umbral's versioning question (Q4). Interop: high for dev tools, low otherwise.
Privacy: high. Provenance: git's, which is linear attribution only. Concurrency: solved by
isolation, at the cost of visibility. Reversibility: excellent. Complexity: low.
Enables: safe parallel agent work for developers today. Prevents: any solution for
non-text assets and for users who will not reason about branches.

--------------------------------------------------------------------------------
S6 — REALITY PROJECTION WITH VALIDITY SIGNALS
--------------------------------------------------------------------------------
LAYER: architectural hypothesis (the candidate differentiator).
Core idea: Umbral maintains a cheap, rebuildable projection of the folder that answers one
question well: "is what you believe still true, and what changed since you looked?" — via
a reality version plus per-path change records, exposed to any participant.
Owns: the projection and the validity signal. Not the AIs, not the files, not authority.
External: everything else.
Strengths: attacks the gap no existing system fills (CO-7: the problem is named and
unsolved; CO-6: memory layers solve it for one vendor's agent, not for a project);
compatible with A10 (rebuildable) and A1 (files are truth); the cheapest useful primitive
is small (a hash of a manifest) — it does not require the heavy context engine.
Weaknesses: a validity signal nobody asks for is worth nothing (adoption again); requires
a live-ish process to be useful in real time; the projection is another thing to keep
correct, i.e. another source of the very staleness it reports.
Failure modes: false confidence (the projection says "unchanged" while a semantic change
happened inside an unchanged hash — e.g. an external tool rewriting a file identically);
false alarm fatigue (everything looks changed after a checkout); the signal becomes the
product instead of a primitive.
Implementation: small for the core, larger for adoption. UX: "your assistant's view is 3
changes behind" is a genuinely new UI element. Interop: an MCP resource + CLI output.
Privacy: local. Provenance: partial (change records, unattributed detection).
Concurrency: detects, does not prevent (pairs with S4). Reversibility: high (disposable).
Complexity: moderate. Enables: safe delegation to a stale-prone participant; honest
handoffs; knowing when to re-read. Prevents: nothing.

--------------------------------------------------------------------------------
S7 — INTEROPERABILITY HUB (Umbral translates protocols; owns no state)
--------------------------------------------------------------------------------
LAYER: implementation mechanism.
Core idea: Umbral is a gateway: an MCP server, an A2A client, an ACP bridge — whatever the
ecosystem speaks, Umbral speaks it and routes.
Owns: translation.
External: everything meaningful.
Strengths: rides ecosystem convergence (CO-3, CO-4) instead of competing with it; low
conceptual risk; immediately useful to power users.
Weaknesses: no differentiation — every gateway does this; it inherits every protocol's
churn (MCP deprecated three primitives in one revision, CO-1); it puts Umbral on the
critical path of calls it does not understand.
Failure modes: protocol churn forces continuous rework; the hub becomes the single point
of failure the counterargument warns about; security surface grows.
Implementation: moderate and never finished. UX: invisible. Interop: its purpose.
Privacy: it sees everything — the worst position to hold for privacy (C12).
Provenance: none of its own. Concurrency: none. Reversibility: high. Complexity: high and
unbounded. Enables: any AI, today. Prevents: nothing.

--------------------------------------------------------------------------------
COMBINATIONS (stated because the strategies are not exclusive)
--------------------------------------------------------------------------------
S1 + S6 (conventions that carry a validity signal) is the cheapest coherent whole.
S4 + S6 (a substrate whose core primitive is validity, with claims as the second) is the
strongest candidate on the evidence — and the most expensive to adopt.
S3 + S5 (ledger + isolation) is the developer-friendly variant that avoids new machinery.
S2 is not a strategy for THIS problem; it is a component of any of them.
S7 is a delivery channel, not a strategy.

================================================================================
2. INVENTED MECHANISMS — ALL LABELLED NEW HYPOTHESIS / PROPOSED MECHANISM
================================================================================
None of these exists in the sources reviewed. Each is invented here to make a strategy
concrete. They are hypotheses to be attacked, not designs.

--------------------------------------------------------------------------------
M1 — REALITY VERSION (workspace epoch)                                  [NEW HYPOTHESIS]
Problem: F1. No participant can say "the project changed since I looked" cheaply.
Mechanism: a deterministic function of the folder's *content-relevant* state — paths,
sizes and content hashes of non-ignored files, combined into one value (a Merkle-style
root) — published as a short, human-readable string, updated whenever the projection
recomputes. Any participant can store the version it read and compare later.
Assumptions: cheap to recompute incrementally; ignore-rules are stable and shared; the
hash is stable across tools (no mtime, no inode — EXP-1 established why).
Expected benefit: O(1) staleness detection instead of re-reading; a shared coordinate for
"when" that does not depend on clocks or on any provider.
Failure modes: semantic change invisible to a content hash; hash churn from tools that
rewrite files identically; two participants disagreeing on ignore rules produce different
versions of "the same" project.
Abuse modes: a participant claims a false version to justify acting on stale data; version
leakage reveals file existence/metadata to a remote model (it must stay local).
Experiment required: E-CO-1. Falsified if: agents with a version check make no fewer
stale-context errors than agents without one (i.e. the problem is not material).

--------------------------------------------------------------------------------
M2 — CONTEXT VALIDITY ENVELOPE (what an AI believed, and when)          [NEW HYPOTHESIS]
Problem: F1 at the participant level. Even with M1, nothing records what a given AI
actually read.
Mechanism: a small, plain-text declaration produced at the start of a session: reality
version, paths read, the goal, assumptions, and an expiry. It lives with the work product
(handoff receipt, proposal, or plan) so any other participant can see whether the plan was
built on a reality that has since moved.
Assumptions: participants can be induced to write it (a convention, an MCP tool, or a
prompt); it is cheap enough not to be skipped.
Expected benefit: turns "this plan is wrong" into "this plan was right when it was made,
and here is what changed" — the difference between a mistake and a misunderstanding.
Failure modes: skipped under time pressure; written but never checked; becomes another
document that decays (C9).
Abuse modes: a plausible-looking envelope that hides which files were actually read; use
of the envelope to claim authority ("it was valid, so my change was authorised").
Experiment required: E-CO-4 (handoff fidelity). Falsified if: envelopes add no measurable
correctness to a cross-model handoff.

--------------------------------------------------------------------------------
M3 — CLAIM / LEASE ON PATHS (advisory, plain-file, expiring)            [NEW HYPOTHESIS]
Problem: F2. Concurrent writers overwrite each other silently.
Mechanism: a participant announces "I am working on these paths until T" in a plain,
append-only record in the project (or via an MCP tool that maintains it). Others see it.
Expiry is mandatory so a crashed participant cannot block the project forever. Claims are
ADVISORY: nothing is prevented, everything is visible.
Assumptions: visibility is enough to prevent most collisions (the blackboard premise,
CO-8); a crashed agent's lease expires rather than deadlocks.
Expected benefit: silent overwrite becomes visible collision; the human can see who is
working on what.
Failure modes: nobody claims; stale claims mislead; participants ignore claims and write
anyway (nothing enforces); claim granularity is wrong (file-level too coarse for a big
file, too fine for a task).
Abuse modes: claiming paths to exclude competitors; claiming to appear busy; a malicious
participant watching claims to learn what is being worked on.
Experiment required: E-CO-3. Falsified if: claims do not reduce collision or rework rates.

--------------------------------------------------------------------------------
M4 — INTERPRETATION RECORDS (disagreement as a first-class artifact)    [NEW HYPOTHESIS]
Problem: F3, specifically the case the mandate names: A says X, B says not-X, the user has
not decided.
Mechanism: interpretations are stored as attributed, separate artifacts (never merged into
the shared document): "model M, at reality version V, concluded X, on evidence E". Two
contradictory records coexist; the shared file stays as the user left it; the contradiction
is visible as a contradiction rather than silently resolved by whoever wrote last.
Assumptions: the user would rather see disagreement than false consensus; models can be
induced to write to a designated place instead of editing shared prose.
Expected benefit: the project stops accumulating false agreement; "contested" becomes a
state the project can hold.
Failure modes: record sprawl (every AI writes an opinion about everything); the records
become the real project while the files stagnate; nobody reads them.
Abuse modes: flooding with plausible records to shift perceived consensus; a record that
misattributes its own authorship; use of "contested" to block a decision indefinitely.
Experiment required: E-CO-5. Falsified if: interpretation records are never read, or
increase user confusion.

--------------------------------------------------------------------------------
M5 — UNATTRIBUTED CHANGE AS A FIRST-CLASS STATE                        [NEW HYPOTHESIS]
Problem: F3's precondition. When a change appears, tools assume "the user did it" (or
"sync did it"). Neither is knowledge.
Mechanism: reconciliation classifies every observed change by actor: human (claimed),
agent (claimed, with session id), tool (inferred from patterns), or UNKNOWN — and UNKNOWN
is recorded as UNKNOWN, never guessed. Changes whose actor is unknown are surfaced, not
absorbed.
Assumptions: participants can be induced to declare their writes (M3/M2); a declared-write
journal is cheaper than inference; the user prefers honesty about uncertainty.
Expected benefit: an honest audit trail; the ability to answer "who changed this?" with
"I don't know" instead of a wrong name.
Failure modes: almost everything is UNKNOWN, so the signal is noise; declared writes are
missed when a tool writes without declaring; the journal becomes a second source of truth
that disagrees with the filesystem.
Abuse modes: attributing a malicious write to a human or to "sync"; declaring a write that
never happened.
Experiment required: E-CO-5. Falsified if: unattributed detection produces more false
attributions than honest UNKNOWNs, or if users do not care.

--------------------------------------------------------------------------------
M6 — PRE-WRITE REALITY CHECK (optimistic concurrency for AI actions)   [NEW HYPOTHESIS]
Problem: F2's silent variant — an AI writes based on a version it read minutes ago.
Mechanism: before a write, compare the version the participant read against the current
one; on mismatch, the write is refused by default and the participant must re-read (or
explicitly force, which is recorded as a deliberate override).
Assumptions: a check is cheap; refusal is safer than silent overwrite; the participant can
recover by re-reading.
Expected benefit: the same guarantee version control gives a developer, extended to any
participant — without requiring the participant to be a git user.
Failure modes: check-then-write race (the file changes between check and write) — the
mechanism is not atomic unless the writer holds the path (M3); false refusals after
harmless changes (formatting, sync) cause participants to force-refuse by habit, which is
C10's permission fatigue in a new costume.
Abuse modes: forcing writes to look authorised; using the check to discover what else
changed (metadata leakage).
Experiment required: E-CO-3. Falsified if: the check is bypassed by default or produces
more rework than it prevents.

--------------------------------------------------------------------------------
M7 — HANDOFF RECEIPT (provider-neutral task state)                     [NEW HYPOTHESIS]
Problem: cross-provider continuity (S3 §8 found the evidence weak; the mandate's §4.E).
Mechanism: a small, plain artifact written at the end of a session: goal, what was done,
what was decided, what is blocked, open questions, files touched, reality version, and the
next step — deliberately NOT the transcript, and deliberately readable by a human and by
any other model.
Assumptions: the semantic tier transfers even when the execution tier does not (S3's
bifurcation); a human-readable receipt is also machine-usable.
Expected benefit: work can move between providers without replaying a conversation; the
human can see the state of work without reading a chat log.
Failure modes: receipts diverge from reality; they become stale plans (CO-7's exact
finding); models produce confident receipts that omit what they actually did.
Abuse modes: a receipt that claims work not done; omission of a failed step.
Experiment required: E-CO-4. Falsified if: a receiving model with a receipt performs no
better than one given only the goal.

--------------------------------------------------------------------------------
M8 — COEXISTENCE CONTRACT (the candidate missing abstraction)          [NEW HYPOTHESIS]
Problem: F1+F2+F3 together, plus adoption. Everything above needs a place to be declared
and a way for a participant to discover it.
Mechanism: one small, plain, provider-neutral file at the project root that states: which
participants exist and what each may do; where outputs go; where claims, receipts and
interpretations live; what the ignore rules are; and where the reality version is written.
It is the interface between the filesystem and any model's context — the thing a new
participant reads first, in the role AGENTS.md plays today (CO-5) but carrying state and
scope rather than only instructions.
Assumptions: a convention with state is enough to be useful; participants can be induced
to read it (an MCP server and a CLI make it easier); the user can maintain it (C9 says
this is the risk).
Expected benefit: one place to answer "how do I participate safely here?" — and a
provider-neutral artifact that outlives every vendor's instruction-file convention.
Failure modes: it becomes the maintenance burden the charter's #1 frustration names; it
drifts from reality; a competing convention (AGENTS.md) simply wins by adoption.
Abuse modes: a contract that grants more authority than the user intended; a malicious
participant editing the contract to widen its own scope (the confused-deputy problem, S7,
in a new place).
Experiment required: E-CO-2. Falsified if: two heterogeneous participants cannot
coordinate using only the contract and plain files.

================================================================================
3. ADVERSARIAL CRITIQUE (applied to the strongest candidates)
================================================================================
The questions the mandate requires, answered without flattery. Each objection states which
strategy it damages and whether it is fatal.

O1. Could ordinary filesystem tooling already solve this?
    For the INSTRUCTION half, yes: AGENTS.md-style conventions work today (CO-5). For
    VALIDITY and CONCURRENCY, no: no ordinary tool tells a participant whether its view is
    current, and the filesystem's own answer to concurrent writes is last-write-wins.
    Damages S1 (partially), not fatal to S4/S6.
O2. Could existing agent frameworks solve it?
    They solve it INSIDE themselves (LangGraph checkpointers, CO-11; Letta shared memory,
    CO-6) — for agents the user runs in that framework. The problem is defined by
    heterogeneity, so intra-framework solutions do not compose. Not fatal.
O3. Could Git solve enough of it?
    Git solves multi-writer SAFETY, reversibility and attribution-by-commit, and worktrees
    solve isolation (CO-16, S5). It does not solve validity ("your plan predates three
    commits"), interpretation conflict, or non-text/non-developer cases. This is the
    strongest competitor and the reason S5 is a real strategy rather than a straw man.
O4. Could MCP solve enough of it?
    MCP standardises access to tools and resources; it has no project state, no versions of
    reality, and it just deprecated its only boundary primitive (CO-1). Umbral could BE an MCP
    server, but MCP is the channel, not the answer. Not fatal — and it is the adoption path.
O5. Could a shared database solve it?
    Yes, technically — and that is the architecture the Umbral corpus documents as failing
    (Logseq; S9 Evidence 1) and which breaks A1/A2. A database solves coordination by
    removing the filesystem's authority. Fatal for anyone who wants the filesystem to stay
    the truth.
O6. Could a simple folder + manifest solve it?
    Closer than it sounds. This is S1 plus M1/M2/M8 with no process at all: the manifest IS
    the reality version, written by whoever recomputes it. The weakness is that without a
    process nothing recomputes it, so it silently rots — the failure mode that kills
    convention-only systems. Not fatal, but it caps S1 at "useful for disciplined users".
O7. Does this introduce more complexity than value?
    This is the strongest objection to the whole stage. Evidence: multi-agent systems often
    underperform single agents (CO-10); the coordination vocabulary is thirty years old
    (CO-8); and no evidence was found that real users suffer material harm from coexistence
    failures (CO-18's products exist, but none reports the problem). UNTIL E-CO-1 RUNS,
    THIS OBJECTION IS UNANSWERED. It is the reason the first experiment is a harm test, not
    a feature test.
O8. Does it create a new authority problem?
    Yes, if the mediator arbitrates. A substrate that decides which claim wins, or which
    version is canonical, has become the authority the charter reserves for the user (A5,
    A6). Mitigation is design-level, not evidential: the substrate must be a
    reporter (visible, advisory), never an arbiter. Damages S4 if implemented carelessly.
O9. Does this make Umbral a dangerous single point of failure?
    Any live mediator does (S4, S7). Mitigation: the conventions must remain usable — and
    the project must remain workable — with Umbral switched off; degradation, not dependency.
    This is a hard constraint on S4, and it is testable (E-CO-2 includes "turn the engine
    off; does the project still work?").
O10. Does it force users into Umbral?
    Only if state lives exclusively in Umbral. M1/M2/M7 are plain files by construction, so no.
    But a claim/lease that exists only in a running process is a soft lock-in; that is an
    argument for the plain-file form of M3 even though it is weaker mechanically.
O11. Does it violate local-first?
    No, if the primitives are local. The danger is that validity and claims are only useful
    when participants are REMOTE models; the metadata must therefore be assembled locally
    and only the minimum sent (A7). Requires care, not a new principle.
O12. Does it create lock-in or hidden metadata?
    Plain files: no. A process-owned journal: yes. This is the same tension as Q1/Q2 (the
    state-partition problem, OPEN-QUESTIONS Q1/Q2) in a new place, and it is unresolved —
    see Q17 and tension T5.
O13. Does it require every AI provider to cooperate?
    For full value, yes — and that will not happen. The realistic path is unilateral:
    conventions any model can read, plus tools (MCP/CLI) that make compliance easier. If the
    design needs provider cooperation, it is dead on arrival. This is the second-strongest
    objection after O7.
O14. Does it work with future unknown AIs?
    Only the convention-based parts do. A protocol-specific mechanism inherits protocol
    churn (CO-1 deprecated three primitives in one revision). This favours plain files over
    protocols for the durable layer.

COUNTER-INSIGHT worth recording: the strongest case against this stage is not that the
problem is fake — CO-7 shows it is studied, CO-9 shows it breaks systems — but that the
problem is currently felt by DEVELOPERS RUNNING CONCURRENT AGENTS, while Umbral's charter puts
ordinary users first (MC §1). If coexistence is a developer problem, then S5 (Git-mediated)
may be sufficient and the elegant substrate may be unnecessary. That is a product-level
fork the user must decide, not a research question. See UD-012.

================================================================================
4. EXPERIMENTS (smallest decisive ones; all on the existing fixture corpus)
================================================================================
E-CO-1  STALENESS HARM TEST — is the problem material?
  Hypothesis: a participant acting on context that has since changed produces measurably
  worse outcomes than one that checks validity first.
  Method: two conditions (with/without M1+M6), same task, scripted external changes between
  read and write; measure wrong-action rate and rework.
  Decisive for: O7, and for whether the stage continues at all. Cheapest possible kill test.
E-CO-2  CONVENTION-ONLY COEXISTENCE TEST — can plain files carry it?
  Hypothesis: two heterogeneous participants (two different models, invoked separately, no
  shared runtime) can coordinate over one folder using only the contract, claims, receipts
  and interpretations (M2/M3/M7/M8), with the engine OFF.
  Measure: task completion, collisions, whether each participant discovered the conventions
  unaided, and whether the project stays usable with the mediator disabled (O9).
  Decisive for: S1 vs S4, and for the adoption question (O13).
E-CO-3  CONCURRENT WRITER COLLISION TEST — how bad is it, and do claims help?
  Extend EXP-1's instrument: two writers editing the same paths, with and without advisory
  claims and the pre-write check; measure silent-overwrite rate, lost-update rate, false
  refusals.
  Decisive for: M3/M6 and for F2's severity.
E-CO-4  HANDOFF FIDELITY TEST — does a receipt beat a transcript, and does a transcript
        beat nothing?
  Three arms: goal only / receipt only / full transcript. Model A performs half a task;
  model B finishes it. Measure completion correctness and context cost.
  Decisive for: M7, M2, and the cross-provider continuity claim (S3 §8's weak evidence).
E-CO-5  ATTRIBUTION TEST — can the system honestly say "I don't know who changed this"?
  Script a realistic sequence of changes by human, agent, IDE, formatter and git; measure
  attribution accuracy and the rate of UNKNOWN (M5) versus guessing.
  Decisive for: M5 and for the provenance half of the product thesis.

All five are runnable with the tooling EXP-1 already established (stdlib, no network, no
model dependency for E-CO-1/3/5; E-CO-2/4 need two model clients the user chooses).
