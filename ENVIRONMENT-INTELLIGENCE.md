# ENVIRONMENT INTELLIGENCE & SELF-DESCRIPTION

Status: **PROPOSED / UNRATIFIED** (document type DT10, PROPOSED PRODUCT HYPOTHESIS SET).
Authority: none. This file holds candidate product-level hypotheses that emerged after the
coexistence stage. Nothing here is a decision, a requirement, or an architecture. Where this
file and DECISIONS.md disagree, DECISIONS.md wins.

Evidence: research/ENVIRONMENT-INTELLIGENCE-RESEARCH.md (sources EI-n). Prior stage:
COEXISTENCE-STRATEGIES.md and research/COEXISTENCE-RESEARCH.md (sources CO-n).

Epistemic labels used below: ESTABLISHED / REASONED / SPECULATIVE (mandate §32).
Novelty labels: EXISTING PATTERN / ADAPTATION / COMBINATION / POSSIBLY NOVEL / NEW HYPOTHESIS.

ID note: hypotheses H15-H22 are numbered in the H namespace of ARCHITECTURE-HYPOTHESES.md
but their full entries live HERE, because they are product-level, not architectural.
ARCHITECTURE-HYPOTHESES.md carries a one-line index pointing here (one home per fact).

================================================================================
1. THE CANDIDATE ABSTRACTION — WHAT IS ACTUALLY BEING PROPOSED
================================================================================

The mandate asks whether "Environment Intelligence" is a meaningful abstraction, and warns
against assuming it is. Tested against the evidence, the honest answer has three parts:

(a) THE LIST IS NOT NEW. Every item the mandate lists under "environment intelligence"
    (what exists, what changed, what is related, who is active, what is authorized, what is
    known/inferred/assumed/disputed, whether my understanding is still valid) is the union of
    four established disciplines: observability (EI-1, EI-5, EI-6, EI-15), provenance
    (EI-21), coordination/audit state (CO-9), and epistemic state (EI-17, EI-18).
    Classification: **EXISTING PATTERN**. ESTABLISHED.

(b) THE POSITION OF THAT KNOWLEDGE IS WHAT DIFFERS. In every system surveyed, environment
    knowledge lives in one of three places: inside a live process (D-Bus, JMX, K8s API), inside
    a vendor's service (agent memory, A2A), or inside a human social process (Wikipedia).
    The proposal here places it in the USER'S OWN FILES, addressed to UNKNOWN participants,
    carrying authority and epistemic status rather than only state.
    Classification: **COMBINATION**. REASONED.

(c) THE DIFFERENTIATOR, STATED PRECISELY AND WITHOUT INFLATION: environment intelligence is
    intelligence about an environment that is NOT OWNED BY ANY PARTICIPANT IN IT — not by the
    model, not by the vendor, and not by FSP itself. FSP is a participant that maintains a
    view, not the owner of the view.
    Classification: **POSSIBLY NOVEL** as a constraint combination; **NEW HYPOTHESIS** as a
    product concept. The claim of novelty rests entirely on the empty cell identified in
    research/ENVIRONMENT-INTELLIGENCE-RESEARCH.md §G (authority + validity + no live process +
    no vendor cooperation + the user's real filesystem). If that cell is not empty, this
    concept is a re-description of observability plus provenance, and should be dropped.

WHAT IT IS NOT (each of these was tested and rejected as the primary framing):
  - Not "an intelligence layer" in the AI sense: FSP does not reason about solutions,
    code, architecture or strategy. It knows about the environment. That boundary is
    load-bearing and must stay explicit (mandate §3).
  - Not observability alone: observability answers "what is the system doing"; this must also
    answer "who says so, on what basis, and does my prior understanding still hold".
  - Not a knowledge graph: the graph is a possible internal representation, never the
    authority (A1/A2; no mandatory taxonomy).

================================================================================
2. THE EIGHT HYPOTHESES (H15-H22)
================================================================================

--------------------------------------------------------------------------------
H15 — ENVIRONMENT INTELLIGENCE AS A PRODUCT-LEVEL CONCEPT
--------------------------------------------------------------------------------
Claim: FSP's product identity can be stated as "the environment's own account of itself",
which is separate from model intelligence and from the user's authority.
Evidence: the matrix in research/ENVIRONMENT-INTELLIGENCE-RESEARCH.md §G; the /proc
precedent (EI-6) as the only shipped example of reality exposed as ordinary files.
Novelty: COMBINATION (see §1).
Falsified by: showing that a competent participant with ordinary file access plus Git plus
search achieves the same outcomes — i.e. E-CO-6/E-CO-7 returning no measurable difference.
Status: SPECULATIVE. Not a decision.

--------------------------------------------------------------------------------
H16 — SELF-DESCRIPTION LAYER
--------------------------------------------------------------------------------
Claim: an unknown AI should be able to become a competent participant by learning FSP
progressively, rather than by ingesting the SDK or the documentation corpus.
Evidence: capability discovery is standard practice (EI-1..EI-10). Self-description for
AGENTS exists (A2A agent card, EI-8). Progressive disclosure is a designed behaviour of a
shipped agent platform (EI-9, EI-10). Interface-description quality is a MEASURED
first-order determinant of agent success, worsening as candidate count grows (EI-25).
Novelty: **EXISTING PATTERN** as a mechanism. The residual, genuinely unsolved part is:
  (i) a description that travels with the user's files and works with no live process;
  (ii) a description that states AUTHORITY and MUTABILITY, not just capability;
  (iii) a description addressed to a participant that does not know FSP exists.
Falsified by: E-CO-6a — a fresh model given only the self-description surface fails to
identify what is mutating, what needs confirmation, and what it is authorized to do. If
models cannot extract the safety floor from the surface, the surface is worse than useless.
Status: SPECULATIVE. The mechanism is established; its adequacy is unmeasured.

--------------------------------------------------------------------------------
H17 — PROGRESSIVE DISCLOSURE WITH A NON-NEGOTIABLE SAFETY FLOOR
--------------------------------------------------------------------------------
Claim: progressive disclosure is the right principle for the AI-facing surface, subject to a
floor of facts that must NEVER be deferred.
Evidence: the principle is 20 years old and human-oriented (EI-11); agent platforms have
already adapted it (EI-9, EI-10); the economic premise is confirmed (50,000+ tokens of tool
definitions before the agent reads a request, EI-10).
Novelty: ADAPTATION. The FSP-specific contribution is the floor, not the disclosure.
PROPOSED SAFETY FLOOR (always disclosed, in the first response, at every tier):
  1. what the participant is currently AUTHORIZED to do (not merely what it may request);
  2. which capabilities are MUTATING vs read-only;
  3. which capabilities require CONFIRMATION before acting;
  4. any EGRESS — whether data leaves the machine, and to where;
  5. what is UNKNOWN or STALE about the current state.
REASONED basis: every one of these is a fact whose absence causes an action that cannot be
undone (mandate §6: "do not allow progressive disclosure to become a mechanism for hiding
information that an AI needs in order to act safely"). Items 1-4 are the same class of fact
that OAuth scopes, POSIX permission bits, and confirmation prompts exist to convey.
Falsified by: E-CO-7 showing that disclosing the floor costs measurable context or latency
that the disclosure saves — a trade that would need an explicit user decision.
Status: REASONED (principle + floor). The floor's CONTENT is a proposal, not a decision.

--------------------------------------------------------------------------------
H18 — TIERED INTROSPECTION (ORIENT / EXPLAIN / DIAGNOSE)
--------------------------------------------------------------------------------
Claim: three levels of self-report — orientation, operational explanation, deep diagnosis.
Evidence: the tiering is EXISTING PATTERN in operations (EI-15 liveness/readiness probes;
EI-16 `git fsck`, `npm doctor`; EI-5 JMX; EI-1/EI-2 API discovery and `kubectl explain`).
Two findings from the evidence that FSP's proposal does not yet have:
  (i) The established Basic tier separates "is it alive" from "is it able to serve"
      (liveness vs readiness, EI-15). FSP's Basic tier conflates them. This is a real gap,
      and it is the gap that matters most to a participant deciding whether to trust a
      report.
  (ii) The established Diagnostic tier is USER-INVOKED and READ-ONLY (`fsck`, `doctor`).
      FSP should adopt that, and the proposal should say so explicitly.
Novelty: EXISTING PATTERN as a tiering; ADAPTATION in that the tiers are addressed to an AI
participant, not only to an operator.
On naming (mandate §7 asks whether Basic/Advanced/Diagnostic are right): the words carry
human-operator connotations and hide the fact that the top tier can be expensive and
sensitive. ORIENT / EXPLAIN / DIAGNOSE is proposed as clearer, and is explicitly NOT
finalized — naming is a decision, not a research result.
Status: REASONED.

--------------------------------------------------------------------------------
H19 — DOCUMENTATION DESCRIBES INTENT; INTROSPECTION DESCRIBES ACTUALITY
--------------------------------------------------------------------------------
Claim: these are different things and must not be conflated in FSP's own interface.
Evidence: EXISTING PATTERN. This is the documentation/telemetry distinction, and the gap
between them is what configuration management calls DRIFT (declared state vs observed
state). FSP would apply drift detection to knowledge and interpretation rather than to
configuration — that application is the ADAPTATION.
Why it matters to FSP: the project has already paid for this lesson internally. Every
documentation failure found by EXP-DOC-1 was a currency or provenance failure — a document
asserting an outdated actuality. A file that could report its own drift would have prevented
all of them.
Novelty: ADAPTATION.
Status: REASONED.

--------------------------------------------------------------------------------
H20 — MACHINE-FIRST CANONICAL COMMUNICATION, WITH A CONFIDENCE WARNING
--------------------------------------------------------------------------------
Claim: FSP's canonical output should be structured, with the AI as translator to the human.
Evidence: structured interfaces are the norm for machine participants (EI-1..EI-10); model
self-report of limitations changes human trust calibration (EI-27).
THE WARNING (this is the sharpest new design constraint this cycle produced):
  Confidence numbers are the most dangerous element in the entire proposal. A number looks
  like knowledge. Displaying "confidence: 0.21" asserts a CALIBRATION that FSP will not have
  measured — uncertain-KG research shows point estimates are easy and calibrated uncertainty
  is an open, recent problem (EI-19). FSP would be manufacturing precision, which violates
  the zero-fabrication rule in a way that is hard to notice because it looks quantitative.
  PROPOSED RULE (NEW HYPOTHESIS, M9): represent epistemic status CATEGORICALLY
  (OBSERVED / INFERRED / ASSUMED / UNKNOWN / DISPUTED) by default, and attach a numeric
  confidence ONLY where that number has been calibrated against measured outcomes, with the
  basis recorded. If calibration has not been measured, the number is not displayed.
Novelty: COMBINATION (machine-first interfaces + categorical epistemic labels + a
calibration precondition for numeric confidence).
Status: REASONED for the rule; SPECULATIVE for whether users want any of it.

--------------------------------------------------------------------------------
H21 — FSP MAY HOLD A STANCE WITHOUT HOLDING AUTHORITY
--------------------------------------------------------------------------------
Claim: FSP may expose its own assessment (e.g. "A and B: not related, inferred, basis: ...")
while remaining one participant among several, never the arbiter.
Evidence: the theory is old and mature — truth maintenance with justifications (EI-17),
belief revision (EI-18), argumentation semantics for competing claims (EI-18) — and there is
a working social precedent for holding contested claims visibly (EI-20).
Novelty: COMBINATION. FSP is not inventing a stance concept; it is deciding whether to
embody a 1979 idea in a filesystem.
THE SHARP RISK (objection OBJ-11, taken seriously): a stance IS an implicit authority. "FSP says
not related" will be read as fact by both models and humans, and by the time anyone notices,
it has shaped a decision. Two conditions follow, and they are REASONED requirements if this
hypothesis is ever adopted:
  (a) a stance must be labelled as a PARTICIPANT'S CLAIM, never as the environment's truth;
  (b) a stance must never overwrite, outrank, or resolve a user statement or a competing
      participant's claim.
Status: SPECULATIVE. This is the hypothesis most likely to be rejected, and it should be.

--------------------------------------------------------------------------------
H22 — AI ACTIVITY HISTORY AND THE UNKNOWN ACTOR
--------------------------------------------------------------------------------
Claim (as proposed): AI activity is recorded unless the user disables recording; and when a
change appears, UNKNOWN is preferred to false attribution.
Evidence: audit logging and provenance are established (EI-21, EI-22); agent identity and
delegated authorization exist for cooperating agents (EI-22). What is NOT established is that
default-on recording is acceptable.
**COUNTER-RECOMMENDATION (REASONED, and it contradicts the mandate's proposed default):**
  Default-on recording of AI activity inverts the project's privacy posture. Reads are the
  leak: a durable log of which files an AI read records the user's INTERESTS and QUESTIONS,
  not just their actions. That log would be the most sensitive artefact in the environment,
  and it would be created by default.
  PROPOSED SPLIT (NEW HYPOTHESIS, M10): record by default only what AFFECTED the environment
  (writes, moves, deletes, permission changes, proposals that became accepted state), and
  leave reads unrecorded by default, opt-in per session or per participant.
  This preserves the audit value (what changed, who authorized it) while removing the
  interest-profiling value.
On UNKNOWN: attribution on a filesystem is genuinely limited — an ordinary file write does
not identify its writer, and provenance systems that assume self-identification (EI-21,
EI-22) cannot cover actors that do not participate. Honest UNKNOWN is therefore not noise;
it is the only truthful value available in a large fraction of cases. Whether it is
OPERATIONALLY USEFUL (or merely displaces the question onto the user) is untested, and is
E-CO-5's job.
Status: SPECULATIVE. The default is a PRODUCT decision requiring the user, not an
architecture question.

--------------------------------------------------------------------------------
H23 — MAINTENANCE AS A DISTINCT ROLE (kept separate: it is the most dangerous one)
--------------------------------------------------------------------------------
Claim: a maintenance role reasons about environment health continuously rather than
answering one-off requests.
Evidence: this is autonomic computing, formalised 20+ years ago (EI-14, MAPE-K), and its
shipped instances are Kubernetes controllers (EI-13) and self-diagnosis commands (EI-16).
Novelty: **EXISTING PATTERN** as a concept. NEW HYPOTHESIS only as "a role the user delegates
to, inspects, and can revoke" in a personal-filesystem context.
THE STRUCTURAL FINDING (the most useful thing this cycle produced about Maintenance):
  Every working maintenance system in the evidence has a DESIRED STATE to reconcile toward
  (Kubernetes `.spec`, EI-13). A user's filesystem has NO desired state. Therefore a
  maintenance role has no objective function unless one is supplied — and the only
  legitimate source of that objective function is the user.
  Consequence: "maintenance" cannot mean "make the environment what it should be". It can
  only mean "report, and propose" — with execution limited to operations whose correctness
  is verifiable independently of anyone's intent (integrity checks, index rebuilds, derived-
  state repair). Anything requiring a judgement about the user's intent is a proposal, not
  maintenance.
Status: REASONED. The role's boundaries are the open question, not its existence.

================================================================================
3. THE IRREDUCIBLE MISSING PIECE (mandate §3, adversarial question)
================================================================================
Question: could this all be filesystem + Git + search + logs + MCP + agent memory?

ANSWER — REASONED, and it is a narrow answer:
Those components supply CONTENT (files), HISTORY (Git), ACCESS (search, MCP) and
per-vendor MEMORY. What none of them supplies:
  1. STANDING — whether a given statement is the user's decision, a participant's proposal,
     or an inference. Git records who wrote a line, not who had the authority to decide it.
  2. CURRENCY OF BELIEF — Git reports that the repository changed; it cannot tell a
     participant whether ITS interpretation is still valid. MCP has no versioning of the
     state a resource returns (CO-3). This is the one item that is genuinely, measurably
     missing, and it is the one E-CO-1 tests.
  3. PORTABLE INTERPRETATION — agent memory is per-vendor and does not travel.
  4. UNILATERAL DISCOVERABILITY — MCP/A2A discovery requires the other side to speak the
     protocol (EI-7, EI-8).
So the irreducible piece is: **standing and currency, portable, in the user's own files,
discoverable without anyone's cooperation.** Items 3 and 4 are REASONED consequences of the
matrix; item 2 is established by CO-3 and CO-7.

================================================================================
4. RELATIONSHIP TO SHARED VALIDITY (mandate §17 — A, B, or C?)
================================================================================
The mandate offers three answers and says C is equally acceptable. The evidence supports a
SPLIT answer, and forcing unity would be a mistake:

(A) COMPONENTS OF SHARED VALIDITY — partially true. Environment intelligence (H15),
    introspection (H18), activity history (H22) and epistemic state (H21) are the MATERIAL
    that validity is computed over. Without them there is nothing to be valid about.

(B) A BROADER ABSTRACTION — yes, with a specific shape. The broader concept is **SHARED
    ENVIRONMENT REALITY**: a portable, participant-neutral account of what exists, what
    happened, what is claimed, what is authorized, and what is still valid. Shared Validity
    is one dimension of it (currency); authority and epistemic standing are the others.
    This is a genuine broadening, not a rebranding: "validity" alone cannot express "this is
    the user's decision" or "this is an AI's proposal", and those are the facts the project
    has repeatedly identified as missing.

(C) SEPARATE — for one piece, and this is the non-obvious finding. **Self-description is
    NOT a component of shared environment reality.** It describes the INSTRUMENT, not the
    environment. Environment intelligence is about the project; self-description is about
    FSP. Unifying them would make FSP's own interface part of the project's model of reality,
    which is exactly the category error the documentation architecture already rejects
    (structure must not become product architecture). They should be documented together
    (they are discovered together) and kept conceptually separate.

RESULTING SHAPE, stated in one line:
  SHARED ENVIRONMENT REALITY (broader concept)
    ├── reality: what exists, what changed        (H15)
    ├── standing: authority, proposals, disputes  (H21, H22, CO-8)
    └── currency: is my understanding still valid (CO Shared Validity, the E-CO-1 target)
  SELF-DESCRIPTION (separate concept, about the instrument)
    ├── identity and role                          (H16)
    ├── capability + authority + safety floor      (H16, H17)
    └── introspection: the instrument's report on itself (H18, H19)
  MAINTENANCE (separate role, consumes both)       (H23)
HARD REQUIREMENT attached to this shape: the unification is CONCEPTUAL ONLY. Each piece must
remain independently droppable, because FSP must degrade (O9): if self-description
disappears, the files must still be intelligible; if the engine is off, the record must still
be readable. A unified abstraction that becomes a single artefact would violate this.

================================================================================
5. NOVELTY ANALYSIS (mandate §29)
================================================================================
Concept                        | Class              | What creates the difference
-------------------------------|--------------------|------------------------------------------
Environment intelligence       | COMBINATION        | the position of the knowledge (user's
                               |                    | files) + authority/epistemic payload +
                               |                    | unknown participants. Not the content.
Self-description layer         | EXISTING PATTERN   | nothing, as a mechanism. Residual: works
                               | (+ NEW HYPOTHESIS  | with no process, states authority, aimed
                               | for the residual)  | at a participant that does not know FSP
Progressive disclosure         | ADAPTATION         | the safety floor (H17), not the deferral
Safety floor (5 facts)         | POSSIBLY NOVEL     | the specific set that must never defer
Tiered introspection           | EXISTING PATTERN   | the AI-addressed tiering; the liveness/
                               |                    | readiness gap is a real addition
Doc-vs-introspection           | ADAPTATION         | applying drift detection to knowledge
                               |                    | rather than configuration
Machine-first + categorical    | COMBINATION        | the calibration precondition for numeric
  epistemic labels             |                    | confidence (M9). The precondition is the
                               |                    | only defensibly new part.
FSP stance                     | COMBINATION        | 1979 TMS ideas embodied in files; the
                               |                    | no-arbiter constraint is the difference
AI activity history            | EXISTING PATTERN   | the write/read split (M10) is the
                               |                    | defensible new part, and it CONTRADICTS
                               |                    | the mandate's proposed default
UNKNOWN actor                  | ADAPTATION         | refusing to guess is the honest part; not
                               |                    | technically novel
Maintenance role               | EXISTING PATTERN   | the finding that it has no objective
                               |                    | function without a user-supplied goal
Shared environment reality     | POSSIBLY NOVEL     | the conjunction in the matrix's empty
                               |                    | cell; unproven until E-CO-1 and E-CO-6a
-------------------------------|--------------------|------------------------------------------
NOTHING in this cycle is claimed as an invention. Two mechanisms are proposed and labelled
(M9 calibration precondition, M10 read/write split); both are constraints on existing ideas,
not new capabilities.

================================================================================
6. ADVERSARIAL CRITIQUE — OBJ-1-OBJ-13 (mandate §30)
================================================================================
OBJ-1 "Just an MCP server with good tool descriptions."
   Largely fair, and partially fatal. For any participant that supports MCP, an MCP server
   with excellent descriptions (EI-25 shows how much that is worth) covers capability
   discovery and execution. What it does not cover: authority per participant, mutability
   class, currency of returned state (CO-3), survival without the server, and any participant
   that does not speak MCP. Verdict: OBJ-1 kills the SELF-DESCRIPTION hypothesis as a
   standalone product, and leaves H15/H18/H21/H22 untouched. This is the most useful
   objection in the list.

OBJ-2 "Just AGENTS.md + Skills."
   Partially fair. AGENTS.md-style files are already filesystem-based, human-readable and
   provider-neutral, and Skills already implement progressive disclosure. Their limits: they
   carry INSTRUCTIONS, not STATE; they have no authority model; they say nothing about
   whether a participant's understanding is current. Verdict: OBJ-2 shows the self-description
   layer should be BUILT ON this convention rather than replacing it. That is a design
   constraint the project should adopt regardless of whether H16 survives.

OBJ-3 "Just OpenAPI/JSON Schema introspection."
   Fair for schema-shaped problems. Insufficient here: a schema describes valid inputs and
   outputs, not authority, mutability, egress, or currency. EI-25 is the evidence that
   descriptions of shape are not enough for agents.

OBJ-4 "Just observability."
   Strongest objection to H15, and it needs an honest answer. Observability gives state and
   telemetry about a SYSTEM. H15 claims something more: statements about the PROJECT carrying
   standing and currency, usable by a participant with no access to the system. If E-CO-1
   shows that participants never act on stale understanding in a way that matters, then H15
   reduces to observability and should be dropped. Verdict: UNRESOLVED, and it is E-CO-1's
   job.

OBJ-5 "Just Git + provenance."
   Fair for history and attribution-by-authorship. Git cannot express "the user decided this"
   versus "an AI proposed this", and cannot express validity of belief. Provenance
   vocabularies (EI-21) can express attribution but assume the actor self-identifies.
   Verdict: OBJ-5 leaves the standing and currency gap intact.

OBJ-6 "Just agent memory."
   Wrong on the axis that matters: agent memory is per-vendor, per-session-scoped, and not
   the user's (CO-4..CO-6). It solves continuity INSIDE one provider. FSP's problem is
   continuity ACROSS providers and across the provider's absence.

OBJ-7 "Just a knowledge graph."
   Correct as a warning, wrong as a reduction. A graph is a possible internal representation
   of relations; it cannot carry authority, and it is not portable in the sense FSP needs.
   The project has already refused to make the graph the authority (A1/A2).

OBJ-8 "The model can simply read the files itself."
   TRUE and it is the single most important objection. Reading the files gives content.
   It does not give standing, currency, or attribution — and the model cannot compute those
   from content alone (it cannot know that a decision was superseded, or that another
   participant is mid-edit). Verdict: OBJ-8 defines the boundary. Any FSP feature that only
   re-presents content the model could read is worthless; the value must be in what reading
   cannot produce. This should become a design test applied to every proposed capability.

OBJ-9 "Users do not care about any of this."
   UNRESOLVED, and it is the same fork as D5 in the coexistence stage. Every piece of
   evidence for staleness and concurrent-writer harm concerns DEVELOPERS running concurrent
   agents (CO-9). No evidence was found that ordinary users experience material harm from any
   of it. Verdict: keep unresolved. Do not let the elegance of the abstraction substitute for
   the missing evidence.

OBJ-10 "Progressive disclosure will hide information the AI needs."
   Fair, and Nielsen documented the same failure in human interfaces 20 years ago (EI-11).
   The safety floor (H17) is the designed answer; E-CO-6a is the test of whether it works.
   Verdict: partially mitigated by design, unproven empirically.

OBJ-11 "FSP's own stance is an implicit authority."
   FAIR AND DANGEROUS. This is the objection most likely to be right. Mitigations in H21
   (participant-claim labelling, no outranking) reduce but do not eliminate it. Verdict:
   H21 should probably be rejected unless E-CO-9 shows a measurable benefit; the burden of
   proof is on the hypothesis.

OBJ-12 "The Maintenance agent will become an autonomous janitor users do not understand."
   Fair, and it is exactly the documented failure mode of autonomic computing (EI-14) and of
   Wikipedia's dispute templates (EI-20). The structural finding in H23 (no desired state ⇒
   no objective function) is the strongest available mitigation: it confines maintenance to
   verifiable operations and proposals.

OBJ-13 "The entire architecture is unnecessary if E-CO-1 fails."
   PARTIALLY TRUE, and worth separating. If E-CO-1 fails, the CURRENCY dimension dies — and
   with it the strongest argument for shared environment reality. The standing dimension
   (H21, H22) and the self-description residual (H16, H17) do NOT die with it; they are
   tested by E-CO-6a. Verdict: the directions have different kill conditions and must not be
   sequenced behind one another.

================================================================================
7. EXPERIMENTS — PROPOSED, NOT RUN
================================================================================
E-CO-6a  THE SAFETY-FLOOR COMPREHENSION TEST (highest information value; see §8)
   Instrument: the EXP-DOC-1 fresh-reader protocol (delegated agents, no conversation
   context, read-only), applied to a candidate self-description surface rather than to the
   repository. No model access, no user authority, no new machinery required.
   Procedure: write 2-3 candidate self-description surfaces (a minimal one; one with the
   safety floor; one verbose). Give each to fresh readers with a fixed task set: (1) what is
   this? (2) which capabilities change files? (3) which require confirmation? (4) what are
   you currently authorized to do? (5) what is stale or unknown right now? (6) name one thing
   you should not do.
   Measure: correct answers per question; dangerous misreadings (says it may write when it
   may not); context consumed; whether the reader asks for more before acting.
   Kills: H16/H17 if readers cannot extract the safety floor, or if the floor costs more
   context than it saves.

E-CO-7  PROGRESSIVE DISCLOSURE EFFICIENCY
   Three arms: (a) full surface; (b) capability summary only; (c) summary + on-demand detail.
   Measure: context consumed, task correctness, tool-selection errors, authority/safety
   errors, latency, and information requested but never needed. Build the measurement on the
   existing tool-failure methodology (EI-26) rather than inventing a metric.
   Kills: the economic argument for progressive disclosure if (c) is not better than (a).

E-CO-8  INTROSPECTION USEFULNESS
   Two arms: with and without ORIENT/EXPLAIN/DIAGNOSE tiers, on a seeded fault (a stale index,
   a contradictory derived state). Measure diagnosis time, correctness, and whether the
   DIAGNOSE tier's extra detail helps or merely lengthens the answer.
   Note: this requires an engine to introspect, so it is NOT runnable before the first
   prototype. Deprioritized accordingly.

E-CO-9  STANCE AND DISAGREEMENT — DEFERRED, DELIBERATELY NOT PROPOSED AS THE NEXT TEST
   Reason: it requires a stance mechanism to exist before it can be tested, and the prior
   probability from OBJ-11 (a stance is an implicit authority) is high enough that the cheaper
   move is to attack the hypothesis on paper. Recorded as deferred, not discarded.

E-CO-1..E-CO-5 from the coexistence stage are unchanged and NOT superseded. E-CO-1 still
tests the currency dimension; E-CO-6a tests the self-description dimension. They are
independent.

================================================================================
8. RESEARCH PRIORITY — THE ANSWER TO MANDATE §24
================================================================================
1. MUST BE TESTED IMMEDIATELY: nothing must be tested immediately in the sense of
   implementation. The two tests that need no implementation are E-CO-1 (currency harm) and
   E-CO-6a (safety-floor comprehension). Both are cheap; neither requires a prototype.
2. CAN BE RESEARCHED WITHOUT IMPLEMENTATION: the whole of §2-§4 above, plus the standing/
   authority model (H21, H22), which is a paper question until a mechanism exists.
3. HIGHEST INFORMATION VALUE: **E-CO-6a**. Rationale: the self-description layer has the
   largest speculative surface in the project (a whole interface, proposed by reasoning
   alone, with no measurement behind it), and it is the cheapest to falsify — one run of an
   instrument the project already has. It also tests the safety floor, which is the one part
   of progressive disclosure that could cause harm if wrong.
4. WHAT COULD KILL THE LARGEST AMOUNT OF SPECULATIVE WORK: E-CO-1 still holds that title for
   the coexistence direction as a whole (OBJ-13). E-CO-6a holds it for the self-description
   direction. If both fail, the remaining defensible ground is the standing/attribution
   dimension alone — a much smaller product.

================================================================================
9. SYNTHESIS — THE 19 REQUIRED ANSWERS (mandate §31)
================================================================================
 1. Is Environment Intelligence a meaningful abstraction? YES as a COMBINATION with a
    specific constraint (participant-neutral, unowned, in the user's files). NO as a new
    list of things to know — every item is established elsewhere. REASONED.
 2. Is Self-Description a real missing capability? NO as a mechanism (MCP, A2A, Skills,
    K8s discovery, D-Bus, JMX, OpenAPI all exist). YES for a residual: no process, carries
    authority, aimed at a participant that does not know FSP. ESTABLISHED + REASONED.
 3. Is Progressive Disclosure useful for FSP? YES, as an ADAPTATION of an established
    principle (EI-11), with a non-negotiable safety floor. REASONED.
 4. What must an unknown AI know before interacting? Its own authority; what is mutating;
    what needs confirmation; what leaves the machine; what is unknown or stale; and how to
    ask for more. SPECULATIVE (the list is a proposal; E-CO-6a tests it).
 5. What must always be exposed? The five floor items (H17). PROPOSED.
 6. What should be discoverable on demand? Capability contracts, object/state detail,
    evidence for an assessment, diagnostic trace. PROPOSED.
 7. Is ORIENT/EXPLAIN/DIAGNOSE useful? YES as tiers (EXISTING PATTERN in ops), with two
    corrections: separate liveness from readiness in ORIENT, and keep DIAGNOSE user-invoked
    and read-only. Naming NOT finalized.
 8. Should FSP expose its own stance? OPEN, and probably NO. It is an implicit authority
    (OBJ-11) and the burden of proof is on the hypothesis.
 9. How should certainty be represented? Categorically (OBSERVED/INFERRED/ASSUMED/UNKNOWN/
    DISPUTED) by default; numeric confidence only where calibration has been measured (M9).
    REASONED.
10. Is AI activity history materially useful? UNPROVEN. Recording writes is defensible;
    recording reads by default is a privacy liability and is CONTRADICTED here (M10).
11. Is Maintenance a distinct role? YES as a role; EXISTING PATTERN as a concept (MAPE-K).
    Its boundary is the finding: no desired state ⇒ report and propose, not enforce.
12. Relationship to Shared Validity? Shared Validity is the CURRENCY dimension of a broader
    concept (shared environment reality), alongside reality and standing. Self-description is
    SEPARATE — it describes the instrument, not the environment.
13. Does Shared Validity remain the best candidate abstraction? It remains the best-founded
    single dimension (it is the only one with a measured gap behind it, CO-3/CO-7), but it is
    too narrow to explain authority and disagreement. Best as a dimension, not as the whole.
14. Broader abstraction? YES: SHARED ENVIRONMENT REALITY (reality / standing / currency),
    with self-description explicitly outside it.
15. What existing technologies provide parts? MCP, A2A, Agent Skills, OpenAPI/JSON Schema,
    K8s discovery + reconciliation, D-Bus, SNMP/MIB, JMX, /proc+sysfs, W3C PROV, in-toto,
    SPIFFE/OAuth token exchange, capability security, macaroons, DNS-SD, TMS/belief revision/
    argumentation, RO-Crate, Wikipedia's dispute model. See the matrix.
16. What is differentiated? ONE thing, and it is a conjunction: authority + currency + no
    live process + no vendor cooperation + the user's real filesystem. Everything else in
    this file is prior art. The conjunction is unproven.
17. What is still speculative? All eight hypotheses. The only ESTABLISHED material is the
    prior art and the measured interface-quality effect (EI-25).
18. Highest-information next experiment? E-CO-6a (safety-floor comprehension), with E-CO-1
    running in parallel on the currency dimension.
19. What could falsify this entire direction? (a) E-CO-1 shows staleness harms no one who
    matters → currency dies. (b) E-CO-6a shows models cannot extract the safety floor from a
    self-description surface → self-description dies. (c) OBJ-8 wins: if everything FSP would
    expose is already obtainable by reading the files, the whole layer is unnecessary.

================================================================================
10. OPEN QUESTIONS RAISED (recorded in OPEN-QUESTIONS.md as Q19-Q21)
================================================================================
Q19  Does an AI participant need an explicit safety floor, or do platform-level permission
     systems already supply it? (E-CO-6a)
Q20  Is "the user has not decided" distinguishable, in practice, from "the user has no
     position"? H21 depends on it and no evidence was found either way.
Q21  Does the standing dimension (who had the authority to decide) have any operational
     value, or is it a philosopher's distinction? No evidence found.
