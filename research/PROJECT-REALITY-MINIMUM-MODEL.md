# PROJECT REALITY — MINIMUM MODEL RESEARCH

Status: **RESEARCH ARTIFACT (T5), unratified.** Research cycle 2026-09-11 (mandate:
"Finding the Minimum Representation of Project Reality"). No decision, no invariant change,
no storage selection. The candidate model below is a HYPOTHESIS, attacked and reduced, not
selected.

================================================================================
1. RESEARCH QUESTION
================================================================================
What is the smallest representation of project reality that must survive so that a future
human or AI can correctly interpret and safely evolve a project when the current files and
version history are insufficient?

Prior hypothesis (supplied by the user in the mandate; NOT a prior project finding — it
appears nowhere in the corpus before this cycle):

  M = {Entity, Assertion, Relation, Provenance/Attribution, Time}

with seven proposed collapses (decision authority -> attribution qualifier; supersession ->
relation; disproven hypothesis -> negative assertion; domain invariant -> constraint;
conflicting records -> epistemic status + topology; freshness -> derived state; handoff
context -> transient state).

The mandate's job was to destroy M if possible. Verdict: **M is DAMAGED, not destroyed, and
it is not minimal as five peer primitives.** Its content survives as one record type plus
one identity concept plus two discipline rules — and three of its seven collapses fail as
stated.

================================================================================
2. METHODOLOGY
================================================================================
Four independent research passes (delegated, parallel, adversarial by design):
  PASS-1 knowledge representation (RDF/RDF-star, property graphs, nanopublications, TMS,
        ATMS, Dung argumentation, ProbLog) — 14 findings, all with URLs.
  PASS-2 legal/regulatory and scientific records (USLM, Akoma Ntoso/FRBR, PREMIS, ISAD(G),
        ALCOA+ lab notebooks, preregistration, MADR/ADR) — 8 findings, 15 sources.
  PASS-3 distributed systems and time (event sourcing, Kafka compaction, TSQL2/SQL:2011,
        Zep/Graphiti temporal KG, PROV-DM, Lamport, Dynamo version vectors, HLC, CRDTs,
        ledger correction, GDPR rectification) — 13 findings, 17 sources.
  PASS-4 red team (smart-AI-is-enough, executable specification, internal collapse attacks,
        hygiene "files+Git+AGENTS.md is enough").
Own work by Hermes:
  - 13 project scenarios across scales (personal script to regulated system; includes
    non-technical and unknown-actor cases) — research/scratch-minimum-model-scenarios.md.
  - EXPERIMENT MIN-1: field-ablation over a toy record model of the scenarios. Each field
    is removed; a required future query either stays answerable or breaks.
    experiments/min1/ablation.py, run1.log. Throwaway fixture; not product code, not a
    schema decision.
  - One primary-source spot verification by Hermes (USLM User Guide: @startPeriod,
    @endPeriod, @status confirmed verbatim, including "may not correspond to an effective
    date or repeal date"). All other source claims are pass-reported with URLs and carry
    the pass's own OBSERVED/EVIDENCE/INFERENCE classification; they were not re-verified
    first-hand and are marked EVIDENCE-PASS in §3. This is stated openly as a provenance
    limitation.

Convergence is the report's strongest structural signal: four passes with different
mandates, run blind to each other, minimized M in compatible directions (§4).

================================================================================
3. EVIDENCE (numbered RM-n; class in parentheses)
================================================================================
Load-bearing claims only; the full per-source detail lives in the pass outputs (session
history) and is summarized here.

RM-1  (EVIDENCE-PASS, OBSERVED by pass) RDF 1.1's only structural primitive is the triple;
      entities are subject/object positions, relations are predicate positions.
      w3.org/TR/rdf11-concepts
RM-2  (EVIDENCE-PASS, OBSERVED) RDF-star exists precisely so provenance/confidence/temporal
      validity can be asserted ABOUT statements in the same syntax: metadata on assertions,
      not a peer primitive. w3.org/2021/12/rdf-star.html
RM-3  (EVIDENCE-PASS, OBSERVED) Nanopublications = Assertion + Provenance + PubInfo as
      three named graphs per claim + hash: the closest prior art to M's shape, and it
      realizes provenance and time as assertions-about-assertions. nanopub.net
RM-4  (EVIDENCE-PASS, OBSERVED) W3C RDF-star/PG mapping losslessly unfolds property-graph
      edges into statements: Relation is derivable from Assertion. Same source as RM-2.
RM-5  (EVIDENCE-PASS, OBSERVED) ATMS (de Kleer 1986) computes belief status as derived
      state over assertion+justification topology and never "believes" anything — the
      non-epistemic stance Umbral requires has a 40-year-old formal precedent.
      doi:10.1016/0004-3702(86)90082-2
RM-6  (EVIDENCE-PASS, OBSERVED) USLM versioning = exactly @startPeriod + @endPeriod +
      @status per provision, with an explicit warning that the period is not the effective
      or repeal date. VERIFIED FIRST-HAND by Hermes in the official USLM User Guide.
      github.com/usgpo/uslm
RM-7  (EVIDENCE-PASS, OBSERVED) PREMIS models provenance as structured event records
      (event + agent + role + time + outcome), and the agent's role is a property of the
      EVENT, not the agent. loc.gov/standards/premis/v3
RM-8  (EVIDENCE-PASS, OBSERVED) ISAD(G)'s six essential exchange elements: reference code,
      title, creator, date(s), extent, level — the smallest formally specified archival
      minimum found. ica.org (ISAD(G) 2nd ed.)
RM-9  (EVIDENCE-PASS, OBSERVED) MADR/ADR practice: mandatory header = status, date,
      decision-makers; "superseded by ADR-n" is rendered INSIDE the status value; records
      are append-only ("supersede rather than modify"). adr.github.io/madr
RM-10 (EVIDENCE-PASS, OBSERVED) ALCOA+/21-CFR-11 lab-notebook practice: author + timestamp
      + status captured by the system; corrections are new linked addenda, never edits.
      casrai.org ELN guide
RM-11 (EVIDENCE-PASS, OBSERVED) Preregistration: evidentiary meaning depends on
      time-of-record relative to outcome; corrections become new documents. OSF/Sage/PLOS
      sources via pass.
RM-12 (EVIDENCE-PASS, OBSERVED) TSQL2/SQL:2011: valid time and transaction time are
      orthogonal dimensions. Snodgrass TSQL2 spec.
RM-13 (EVIDENCE-PASS, OBSERVED) Zep/Graphiti (arXiv:2501.13956) implements bi-temporal
      facts (valid_at/invalid_at + ingestion order) with non-lossy source pointers and
      invalidation by writing, never deleting: a production AI-memory system that already
      converged on this record shape.
RM-14 (EVIDENCE-PASS, OBSERVED) PROV-DM core = Entity/Activity/Agent + a handful of
      relations; time is an attribute of occurrences, not a core construct.
      w3.org/TR/prov-dm
RM-15 (EVIDENCE-PASS, OBSERVED) Append-only correction practice: Kafka tombstones,
      accounting reversal entries ("corrected only by adding new reversing and correcting
      entries" — MS Dynamics/India audit law), GDPR Art. 16 supplementary statements.
      Retraction is uniformly a NEW record referencing the old.
RM-16 (INFERENCE, pass synthesis) Across law, science, archives and decision records the
      convergent minimum is: identified record + attributed agent with role + time
      (recorded + validity) + lifecycle status, with all change as append-only linked
      records. Decision is derivable everywhere: assertion + authority attribution +
      ratification status. No domain makes Decision or Supersession a primitive.
RM-17 (INFERENCE, Hermes, from scenarios + MIN-1) Scope/applicability is load-bearing and
      ABSENT from M. The Windows decision binds the project but not the user's fork (S2b);
      the schema-ownership boundary binds services A and B (S5); the customer contract
      binds one module (S7); delegation is scoped (pass-4 independently). MIN-1 ablation
      confirmed: removing `scope` breaks required queries.
RM-18 (EXPERIMENT) MIN-1 ablation: ALL EIGHT toy-model fields load-bearing — subj, pred,
      obj, src, kind(status), t_valid, t_record, scope. Caveat stated honestly: the
      fixtures were built from the same scenarios that motivated the fields, so the
      experiment demonstrates internal consistency, not independence; it falsifies only
      the claim that any of these fields is droppable. experiments/min1/run1.log

Pass reports also produced two explicit UNKNOWNs carried into §9.

================================================================================
4. FINDINGS — VERDICT ON EACH PRIMITIVE OF M
================================================================================
The four passes disagree on two primitives and agree on three. Where they disagree, the
disagreement is about representation, not information, and is recorded rather than forced.

ASSERTION — PRIMITIVE (unanimous). Present in every surveyed system under some name; the
  only abstraction that supports the self-referential structure all collapses depend on.
  Renamed for Umbral: RECORD (see §6 — an actor asserts, a tool observes; Umbral only records).
ENTITY — DISPUTED, resolved as: not a primitive TYPE, but identity is a REQUIREMENT.
  Pass-1: entities are just subject positions (RDF, Dung, TMS). Pass-2/3: identity is
  never derivable from content (FRBR WORK, PREMIS objectIdentifier, Kafka key). Both are
  right: no entity record type is needed, but addressable identity (TERM) is, and
  identity-across-change must itself be maintained by records (scenario S11: an assertion
  recorded against src/core must survive the rename to src/kernel).
RELATION — DERIVABLE (unanimous). A relation is a record whose subject is a pair.
  RM-1, RM-4. Making relations typed first-class objects is a storage/navigation
  optimization (property graphs), not a representational necessity — and §11 of the
  mandate forbids storage selection, so the question does not arise here.
PROVENANCE — STRUCTURALLY MERGEABLE, MATERIALLY IRREDUCIBLE. Pass-1: mergeable (RM-2,
  RM-3). Pass-3: mergeable to two pointers (agent, source). Pass-2: every domain records
  it as a distinct mandatory record class (RM-7, RM-8, RM-10). Pass-4: the capture is the
  point — outside Git, provenance exists only if written at event time; it is not
  derivable later. Resolution: provenance is not a peer primitive TYPE, but its content
  is mandatory on every record and capture-at-event-time is a hard requirement (R1 below).
TIME — DERIVABLE AS A PEER PRIMITIVE, REQUIRED AS TWO FIELDS (unanimous on the split).
  Not one time but two roles: recorded-at (transaction/assertion time) and valid-time
  (when true in reality) (RM-6, RM-12, RM-13). MIN-1: both fields load-bearing (S3b,
  S13 — "what did we believe in 2025-05" is unanswerable from valid-time alone).
MISSING FROM M: SCOPE/APPLICABILITY (RM-17, RM-18) and STATUS as a first-class field
  (lifecycle state: proposed/accepted/disputed/retracted — USLM @status, ADR status,
  PREMIS outcomes; MIN-1: load-bearing in 5 of 6 scenarios).

================================================================================
5. FALSIFICATIONS — THE COLLAPSE LIST
================================================================================
Three of the seven proposed collapses FAIL as stated (pass-4, corroborated by own
scenarios; class INFERENCE):

FAL-1  SUPERSESSION -> RELATION is lossy. A later decision can REPEAL (old rule no longer in
    force), OVERRIDE (old decision defeated but stands as record), REFINE (partially
    change), or REINSTATE (A supersedes B supersedes A — VISION.md's own N1-N3 history is
    a local instance). A bare "supersedes" edge cannot distinguish these, and the failure
    mode — an AI reading a repealed rule as binding — is exactly the hazard Umbral exists to
    prevent. Repair: supersession is a typed validity EVENT recorded between records
    (which the record model represents naturally), not an undifferentiated relation.
FAL-2  DECISION AUTHORITY -> ATTRIBUTION QUALIFIER conflates who MADE a record with who may
    BIND by it. Orthogonal (an artifact attributed to Alice can carry Bob's delegated
    authority; scenario S9: the editor's authority outranks the file owner's). Cannot
    express scoped delegation, shared/quorum authority, or revocation — and revocation
    needs standing-over-time, which silently smuggles Time back in. Repair: standing is
    first-class record content (who may decide, within what scope, until when), never
    derived from attribution.
FAL-3  FRESHNESS -> DERIVED STATE is circular exactly where Umbral most needs it. Derivation
    needs a change-log; outside Git-tracked corpora none exists, and MC §51 (via
    VISION.md) REQUIRES Umbral to work without Git. Repair: freshness is derived only where
    a change-log exists and CAPTURED at write time everywhere else. The same pass also
    demoted two prior claims: currency-as-stored-fact (staleness vs change history is
    mechanically derivable where history exists) and unknown-actor-inside-Git (the
    absence of attribution IS re-derivable at read time). The write-time discipline
    survives for non-Git corpora only.

The four collapses that survive scrutiny: conflicting records -> status + topology (with
the caveat that the conflict record itself is a record about records — RM-5, RM-15),
disproven hypothesis -> record with status=retracted + scope/strength qualifier (S10:
"failed under load Y" is not "false"; the strength loss noted by pass-4), handoff context
-> transient state (excluded from durable reality; §8), domain invariant -> constraint
record (nothing in the evidence elevates it).

================================================================================
6. MINIMAL MODEL v0 (candidate; HYPOTHESIS, unratified)
================================================================================
One primitive type, one supporting concept, two discipline rules:

  P1  RECORD — the only primitive type. An append-only, attributed, time-stamped
      statement. Self-referential: a record's subject may be another record (this is what
      makes relations, supersession events, conflicts and provenance expressible without
      new types). Mandatory content: subject(s), the statement, source + role, recorded-at;
      per-case content: valid-time, status (closed vocabulary), scope.
  P2  TERM — an addressable identity records refer to. Entities are EMERGENT: the things
      that records attach to. Identity across change (renames, moves) is itself
      maintained by records (identity links), not by a stored entity registry.
  R1  CAPTURE-AT-EVENT-TIME — provenance, time and status exist only if written when the
      event happens. Retroactive reconstruction of any of them is forbidden as a source
      of truth (a reconstructed rationale is byte-indistinguishable from a real one yet
      wrongly inherits the decision's authority — worse than recorded ignorance).
  R2  DERIVED-ONLY — no stored truth. "Current", "fresh", "accepted", "in force" are
      computed projections over record topology (ATMS precedent, RM-5). Correction and
      retraction are new records referencing old ones (RM-9, RM-10, RM-15); nothing is
      edited in place.

Anthropomorphism check (mandate §5): the model contains no belief, knowledge, truth or
confidence term. Records carry status vocabulary and source roles; "observed" vs
"asserted" distinguishes tool-captured from actor-stated; Umbral computes projections and
reports them as projections. Epistemic information without epistemic agency — with the
ATMS as formal precedent that status-as-derived-state does not require a believing system.

What the model is NOT: not a schema, not a storage format, not a graph, not an API.
Mandate §11: the information requirement is the finding; the realization question
(plain-file conventions vs anything else) is deliberately untouched — except for the one
boundary pass-4 established: the model must be REALIZABLE as plain-file conventions,
because this repository itself is the existence proof that conventions carry M's content
in single-writer steady state (IDs, status lines, dated UPDATE blocks, supersession
pointers inside status values — RM-9 is literally this repo's practice).

================================================================================
7. THE IRREDUCIBLE RESIDUE — WHAT FILES + GIT + A SMART AI CANNOT RECONSTRUCT
================================================================================
Pass-4's classification, with own-scenario corroboration (classes: INFERENCE unless
noted; this is the sharpest output of the cycle):

  genuinely unrecoverable:
    standing        — who may DECIDE is a social fact with no physical trace; when two
                      humans committed conflicting decisions and nobody recorded who
                      overrules whom, the corpus is complete and the answer absent.
                      (Q21's operational-value question remains open; this finding is
                      about representability, not value.)
    contest state   — "disputed, human informed, UNDECIDED" is destroyed by
                      last-write-wins; absence-of-decision is indistinguishable from
                      absence-of-record after the fact (S8).
    rationale       — lived in the deciding conversation (S1); reconstruction is worse
                      than absence (R1).
    private external constraints — legal/client/personal facts with no world-readable
                      trace (S4, S7). Public ones are recoverable with web access.
    write-time provenance/time for non-Git corpora — the event exists only if captured
                      when it happens (FAL-3).
  demoted (NOT irreducible, contra prior claims):
    currency-as-stored-fact (derivable where a change-log exists),
    unknown-actor-inside-Git (re-derivable at read time),
    rejected alternatives (efficiency cost to lose, not correctness — an AI re-proposing
    a rejected option wastes a cycle but corrupts nothing).

This residue IS the model's entire justified content. Everything machine-checkable
leaves the model and becomes a test/lint/policy (pass-4's executable-specification
verdict: that attack absorbs negative-constraint INSTANCES, CI-enforceable rules, capture
mechanics — and has NO form for social facts, historical causes, or resolution status).

================================================================================
8. INFORMATION HYGIENE — WHAT Umbral DELIBERATELY FORGETS
================================================================================
Not durable project reality: chain-of-thought, raw prompts, transient chat, tool
invocation logs, passive telemetry, routine operational noise, participant-local
currency (an agent's staleness belongs to the agent's session, not the project).
Transformation boundary: transient activity becomes a record ONLY when (a) a human
ratifies it, or (b) the event changed files. Rejected alternatives: recorded when
recorded at decision time (they arrive as part of rationale), never chased
retroactively. (Class: INFERENCE, from mandate §12 analysis + pass-4.)

================================================================================
9. REMAINING UNCERTAINTY
================================================================================
UNKNOWN-1 (pass-1): no primary source verifies that a self-contained plain-file record
  model survives Umbral's disappearance as readable files without Umbral — nanopublications
  are the closest prior art (self-contained, hash-addressed), but the file-durability
  requirement is untested. -> Q22.
UNKNOWN-2 (pass-3): whether single-writer causal markers (HLC / writer+sequence) suffice
  for multi-AI concurrent writes, or per-record version vectors become necessary, is
  unsettled — though record topology already preserves concurrency as coexisting
  records, so this affects derived merging only, not the durable model. -> Q23.
UNKNOWN-3 (own): status vocabularies are domain-closed (USLM legal set, ADR set); whether
  Umbral needs a universal status set, per-scope sets, or none is open. -> Q24.
UNKNOWN-4 (own): MIN-1 demonstrates field necessity only within its own fixtures
  (circularity caveat, RM-18); independent confirmation requires E-MIN-1 (§11).

================================================================================
10. STRONGEST OBJECTIONS TO MODEL v0 (kept with the model)
================================================================================
OBJ-A (hygiene, pass-4): files + Git + hardened conventions suffice for single-writer
  steady state; this repo proves it. v0 must therefore earn its keep ONLY at multi-AI
  concurrency and durable undecided-state, or it is over-building. (Untested: E-MIN-1 /
  E-CO-3 territory.)
OBJ-B (anthropomorphism risk): "status" and "stance-like" fields invite readers to treat
  computed projections as the system's opinions. Mitigated by R2, not eliminated.
OBJ-C (capture burden): R1 makes write-time capture mandatory; that is a cost on every
  participant and may push casual users away. Unquantified.
OBJ-D (vocabulary drift): closed status vocabularies rot (USLM's set is legal-specific);
  Umbral would need its own governance for the vocabulary it records. (DOC-FRICTION
  precedent: the project's own ID-namespace drift.)

================================================================================
11. NEXT EXPERIMENT (one)
================================================================================
E-MIN-1 — FIELD-ABLATION WITH FRESH READERS. Take this repository's own durable records
(DECISIONS.md entries, tension records, friction-log incidents). Produce degraded
variants with single fields removed (status line, attribution, date, supersession
pointer, scope qualifier). Give fresh readers (the EXP-DOC-1 instrument) real
interpretation questions per variant. Measures which fields are load-bearing for ACTUAL
interpretation by actual readers — closing the circularity gap of MIN-1 (RM-18) — at no
user authority, no model access, no prototype. Highest information value: it tests the
model's core claim (fields are load-bearing) against readers instead of against its own
fixtures, and doubles as documentation dogfooding.

================================================================================
12. DECISION BOUNDARY
================================================================================
This research does NOT justify a project decision. It reduces a user-supplied hypothesis
and proposes a candidate model. No invariant changes, no architecture selection, no
storage choice, no schema. UD-001..UD-011 untouched; A1-A11 untouched. If the user later
wants Minimal Model v0 considered as a hypothesis of record, that is a separate explicit
act.
