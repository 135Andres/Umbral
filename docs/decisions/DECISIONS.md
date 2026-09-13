# DECISIONS

Status: CURRENT (living log of FROZEN records) — decision records (T3). Individual records are immutable; supersede, never rewrite.

Created 2026-09-10 by the project-intelligence audit, after user decisions were
recovered from session history. Before that date no decision record existed because no
decision existed.

NAMING: this file predates the project's public name. Records are quoted as written and
retain the earlier working name (FSP / "File System Pro"); the project is now **Umbral**.
Records are not rewritten for a rename. The transition is documented in README.md
("Naming and sources").

SCOPE OF THIS FILE: product commitments the user has actually made — stated as
absolutes ("must", "will", "is NOT") in the Project Master Context (MC) or in session
history. MC's own header says no final ARCHITECTURE is adopted; these are product
commitments, not architecture choices. Architecture decisions do not exist yet and will
be appended here only when the user makes them.

Format per §17 of the audit mandate: ID, decision, authority, source, supporting
statement, alternatives, evidence, consequences, reversibility, status.
Where a field was not recoverable: RATIONALE UNKNOWN. No dates are fabricated — all
recovered items date from 2026-09-10 (the only FSP session date on record).

================================================================================
UD-001 — The product is open source
Authority: USER (MC §36). Statement: "The intended product direction is: Open source."
If hosted infrastructure is eventually offered: paid hosting possible, donations
accepted, no advertisements; local usage remains possible.
Alternatives: not discussed. RATIONALE UNKNOWN (partially inferable: ecosystem trust).
Evidence: none needed (user commitment).
Consequences: licence choice still open; governance model still open (gap G3).
Reversibility: low once a community forms — treat as durable.
Status: DECIDED (product direction).

UD-002 — No mandatory global taxonomy
Authority: USER (MC §4). Statement: "There must be no mandatory global taxonomy ...
unless the user chooses to create it. Users may organize their filesystem however they
want." Reinforced by MC §5 (semantic objects need not correspond to folders) and §39.
Alternatives considered: PARA-style structures (rejected by the user's framing).
Evidence: S1, S8, S11 independently confirm the failure mode of forced taxonomies.
Consequences: every architecture candidate must satisfy A3; indexes must handle
arbitrary layouts.
Reversibility: near-irreversible — it defines the product.
Status: DECIDED.

UD-003 — No hidden prompt injection into connected AIs
Authority: USER (MC §11). Statement: "The system must NOT inject hidden prompts that
override or replace the instructions of the project developer/user." Capabilities are
exposed as an integration/documentation layer (API/SDK style), not behavioral override.
Alternatives: none stated; the framing explicitly rejects system-prompt override.
Evidence: consistent with S7's "the prompt is not a boundary" (different problem, same
instinct: no silent authority).
Consequences: recorded as invariant A11.
Reversibility: treat as irreversible (trust property).
Status: DECIDED.

UD-004 — Sonora is not an architectural authority for FSP
Authority: USER. Statement: the project keeps architectural independence from the
applications that consume it. A consumer may influence it through use cases, requirements
and feedback; no individual consumer determines its architecture, and its architecture does
not constrain its consumers. The project's first intended consumer is one such application,
not an authority over it.
Alternatives: not discussed.
Evidence: the boundary is asserted from both sides — the consuming project's own governance
records state the same independence.
Consequences: no concept originating in a consuming application enters the project without
passing the Lesson / Invariant / Implementation test: a lesson learned elsewhere is an
input; an invariant must be justified on this project's own terms; an existing
implementation is never evidence that an architecture is right.
Reversibility: durable by design.
Status: DECIDED. Rationale restated 2026-09-11 in the project's own terms: the private
source document this decision was derived from is not published, and its wording is not
reproduced here. The decision is unchanged; only the citation form changed. The record's
title is left as written (see the NAMING note above).

UD-005 — Local-first with file survivability
Authority: USER (MC §3, §29). Statement: "The application should work fully locally";
"If the application is uninstalled: the underlying user data should remain accessible as
ordinary files." Internet only where a service needs it. Recorded as invariants A1, A2,
A8.
Alternatives: cloud-first is implicitly rejected.
Evidence: S9's local-first analysis; Logseq failure evidence supports it.
Consequences: sync/hosting are secondary tiers (MC §29 supports local desktop, home
server, VPS, eventually hosted).
Reversibility: near-irreversible — it defines the product.
Status: DECIDED (product direction; mechanisms remain research).

UD-006 — Research before implementation; no code yet
Authority: USER (MC §48, §52; bootstrap brief; repeated in session). Statement: "The
research phase should NOT automatically produce code"; sequence is requirements ->
research -> candidate architectures -> red-team -> choose -> MVP -> implement.
Alternatives: not discussed.
Evidence: the corpus exists; no implementation exists.
Consequences: any implementation work before an accepted architecture is out of order.
Reversibility: process rule, revisitable when the research phase completes.
Status: DECIDED (process).

UD-007 — Research method: Gemini Deep Research first, Hermes as independent
cross-verifier, not summarizer
Authority: USER (MC §46; confirmed by actual behavior — the user supplied the Gemini
reports S1-S11 and required ingestion/critique rather than new primary research).
Statement: "Hermes must NOT simply summarize Gemini"; per-claim verification ->
evidence -> agreement/disagreement -> gaps -> red-team -> synthesis.
Alternatives: Hermes-first, parallel (offered in the kickoff questions; timed out
unanswered; the user's later behavior resolved it).
Evidence: the delivered corpus.
Consequences: Hermes research output is verification/critique/gap analysis until the
user requests primary research.
Reversibility: method choice, revisitable.
Status: DECIDED (process, inferred from behavior — flag: confirm once).

UD-008 — Layered permission intent (NORMAL / CONFIRMATION / SEMI-BYPASS / FULL BYPASS)
Authority: USER (MC §24). Statement: "The user wants a layered permission model";
"The user explicitly wants a semi-bypass that can authorize: edit; move; delete."
BUT MC §24 itself says "The exact permission architecture should be researched rather
than assumed."
Alternatives: per-action-only, two-mode-only (S11) — both inconsistent with the user's
statement.
Evidence: S7's independent convergence on the same four tiers.
Consequences: intent-level decision; design-level open (MC §43-Q10/Q11).
Reversibility: high (researchable).
Status: DECIDED AS INTENT-DIRECTION; mechanism RESEARCH.

UD-009 — Views are wanted: dashboard, graph, kanban, timeline, filesystem
Authority: USER (MC §7, §8). Statement: the user wants multiple views over the same
information and a customizable command-center home. Overrides S11's REJECT verdicts
(recorded in VISION.md).
Consequences: view layer is in scope; S11's evidence becomes design constraints
(ego-scoped graphs, declarative zero-maintenance views, user-controlled auto-generation).
Status: DECIDED (intent); implementation RESEARCH.

UD-010 — Proactive intelligence is wanted, non-autonomous, and disablable
Authority: USER (MC §18). Statement: "The user wants this functionality strongly";
"Suggestions should not automatically become actions"; disable/reduce controls wanted.
Overrides S11's REJECT verdict (recorded in VISION.md).
Status: DECIDED (intent); mechanism RESEARCH.

GAPS IN THIS LOG (recorded so a reader does not infer absence of a record means absence
of intent):
  - PROVENANCE has no UD. The user's charter states it (MC §40: it should be possible to
    understand where information came from, who/what changed it, which AI/session produced
    a result, what context was used), and the repository records it as USER INTENT, but it
    was never written as a decision record. It is also the one item carrying an unresolved
    caveat: MC §40's own header calls its section "research inputs, not final
    requirements". Deliberately NOT resolved here — resolving it would be this repository
    deciding for the user. See RESEARCH-AGENDA 0.5.
  - VERSIONING has no UD either, and that is correct: the user stated an intent ("Git
    without Git", MC §26) and explicitly left the mechanism to research (MC §52). The
    intent is recorded under NOT DECIDED below and in OPEN-QUESTIONS Q4.

UD-011 — AI-native coexistence is the current central research direction
Authority: USER (2026-09-10 stage mandate, §1/§16). Statement: "FSP must demonstrate that
it can meaningfully coexist with an environment full of AI systems"; the central question
is what FSP must provide so that humans and multiple AI systems can safely, coherently and
continuously coexist around the same filesystem without forcing the human to reorganize the
project around any one AI's assumptions.
Explicit scope limit set by the user in the same instruction: "This is NOT yet a product
decision. It is a research direction authorized by the user."
Alternatives: the prior framing (semantic projection over an arbitrary filesystem as the
minimum proof) remains FUNDAMENTAL and is explicitly preserved, not replaced.
Evidence: research/COEXISTENCE-RESEARCH.md (external); S1-S11 (corpus); the problem is named
and studied externally (stale agent memory, arXiv:2609.03340; multi-agent concurrency,
arXiv:2608.23740) and is unsolved at the level FSP would need.
Consequences: coexistence becomes the reference problem for architecture comparison
(criteria K1-K9 still apply); the earlier MVP framing is widened, not discarded.
Reversibility: high (a research direction).
Status: DECIDED (research direction, not product).

UD-012 — V0 prototype authorized (fsp-check); research phase provisionally concluded
Authority: USER (2026-09-11 synchronization-and-transition mandate). The user states the
initial conceptual-research phase has reached its provisional conclusion and authorizes
the transition to implementation, starting with a minimal technical prototype provisionally
named fsp-check (observe a real filesystem, catalog observable state, detect mutations,
reconcile, persist, reconstruct, maintain verifiable invariants, recover from reasonable
failures). Explicit limits in the same instruction: fsp-check is NOT FSP; it resolves none
of the out-of-scope items (sync, P2P, multi-AI authority, semantic graph, UI, MCP, ...);
no code was written at authorization time (documentation-preparation stage first).
Consequences: PROJECT-DIRECTION enters an implementation-transition phase; pending
experiments (E-CO-1, E-CO-6a, E-MIN-1) are NOT cancelled — they remain open evidence lines.
Reversibility: high (a prototype, not a release).
Status: DECIDED.

UD-013 — V0 stack provisionally adopted, V0 scope only, explicitly non-irreversible
Authority: USER (same mandate): Rust · SQLite (rusqlite) · walkdir · BLAKE3 · UUIDv7 ·
tempfile · proptest, as "PROVISIONALMENTE ADOPTADAS PARA V0", with the explicit statement
that these are NOT irreversible architectural decisions. The same mandate lists
technologies OUT OF V0 SCOPE (Tokio, Rayon, notify, Iroh, MCP, HTTP, Tauri, React,
cryptographic identity, P2P, plugins, mobile, Tree-sitter-in-core) — out of scope is NOT
rejected for FSP.
Provenance caveat (recorded, unresolved): the supporting research (technology-selection
report, adversarial stack audit, prototype specification) exists in Gemini/Hermes research
sessions and has NOT been archived in this corpus — same defect class as the S1-S11 gap
(RESEARCH-INDEX GAPS item 4). The adoption rests on user authority, which suffices; the
evidence base is missing until archived.
Scope guard: this record does NOT settle any MC §52 item (final architecture, database,
versioning, etc.). Home of the working plan: V0-IMPLEMENTATION-PLAN.md.
Reversibility: high by design.
Status: DECIDED (scoped adoption).

UD-014 — v0.1 stack adopted, v0.1 scope only, explicitly non-irreversible
Authority: USER (2026-09-12 v0.1 implementation mandate, decision D1): Rust, rusqlite,
walkdir, blake3, proptest, tempfile for the v0.1 development crate `umbral/`.
Statement: the same provisional set the V0 mandate named, adopted again for a DIFFERENT
scope. This is a new adoption, not an inheritance: UD-013 covers the frozen experiment
`fsp-check/` and nothing else, and this record covers `umbral/` and nothing else. Neither
extends the other, and neither is a technology decision for the product.
Alternatives: none evaluated in this mandate. RATIONALE UNKNOWN beyond "the user named this
set for this version".
Evidence: none supplied with the mandate. The provenance caveat recorded under UD-013
applies unchanged — the research that would justify a stack choice exists in research
sessions and is NOT archived in this corpus (research/RESEARCH-INDEX.md GAPS item 4). The
adoption rests on user authority, which suffices; the evidence base is missing until
archived.
Consequences: `fsp-check/` remains independent, frozen, excluded from the Cargo workspace,
and unmodified. Any change to a v0.1 dependency updates this record.
Scope guard: this record does NOT settle Q25 (SQLite-only vs SQLite + external log), does
not select a persistence model, and does not decide anything MC §52 leaves open.
Reversibility: high by design.
Status: DECIDED (scoped adoption, v0.1 only).

================================================================================
UD-015 — Evidence classes for validation, and the separation of A1 from A1-AI
Authority: USER (2026-09-12 mandate, "NUEVA FASE DE TRABAJO — v0.2 + VALIDACIÓN POR
MÚLTIPLES INTELIGENCIAS", §2 and §3; resolved in the follow-up mandate of the same date,
D-PEND-1).
Statement: validation evidence is classified by WHO produced it, and the classes are never
converted into one another:

  HUMAN-INDEPENDENT  a person other than the author executes a protocol without
                     interpretive help from the author.
  AI-INDEPENDENT     an artificial intelligence executes a protocol with controlled
                     context, without receiving the expected interpretation or conclusion.
  AI-CROSS-CHECK     a second AI reviews evidence produced by another AI, without
                     unnecessarily sharing the earlier interpretation.
  AUTHOR             evidence produced by the author/developer.
  AGENT-INTERNAL     evidence produced by Hermes during its own work.

A1 keeps its exact meaning: **HUMAN-INDEPENDENT**. It is not redefined and not weakened.
A NEW auxiliary criterion is created: **A1-AI = AI-INDEPENDENT**. A1-AI can supply
legibility evidence for another intelligence, surface ambiguities, and unblock technical
decisions. It **cannot satisfy A1**.

Authority for the change: USER. The multi-intelligence track is a USER decision.
Supporting statement: "Yo, Andrés, seré quien delegue manualmente los experimentos a otras
inteligencias artificiales" and "Una IA no debe convertirse artificialmente en 'humano
independiente'. Pero una IA independiente tampoco debe ser tratada como evidencia inútil."
Alternatives considered: (a) replace A1 with a general legibility criterion — REJECTED by
the user; (b) leave A1 as the only criterion and treat AI runs as informal — REJECTED, it
would discard usable evidence; (c) keep A1 and add A1-AI alongside it — ADOPTED.
Evidence: none is claimed for the classes themselves; they are a user commitment about how
evidence is classified. The first AI-INDEPENDENT run already exists (EVIDENCIA-B).
Consequences:
  - EVIDENCIA-B stays classified AI-INDEPENDENT. It is NOT retroactive satisfaction of A1.
  - The state of A1 in v0.1 is unchanged: NOT SATISFIED. No retrospective rewrite.
  - A1-AI evidence may unblock technical decisions and discover defects during a version,
    but the formal satisfaction of a version's A1 requires HUMAN-INDEPENDENT.
  - No AI is made an automatic judge of another AI; AI-CROSS-CHECK finds discrepancies and
    is not an authority.
Scope guard: this record does NOT change A1's wording, does NOT declare A1 satisfied, does
NOT close Q25, and does NOT authorize the delegation of any specific experiment (each is
authorized separately by the owner, who delivers it personally).
Reversibility: high. Removing A1-AI would restore the prior state without touching A1.
Status: DECIDED (process — how the project validates, not what the product is).

================================================================================
UD-016 — v0.2 objective: O(changes) for content read and hashing ONLY
Authority: USER (2026-09-12 v0.2 planning mandate, D1).
Statement: the objective of v0.2 is to reduce to **O(changes) the reading/hashing of
content during re-observation**, preserving the verdicts, the evidence, the ambiguity and
the reconstruction capability of V0/v0.1. It is **not** permitted to claim O(changes) for
the whole filesystem traversal.
Supporting statement, verbatim: "Reducir a O(changes) la lectura/hash de contenido durante
la re-observación, preservando los veredictos, la evidencia, la ambigüedad y la capacidad
de reconstrucción de V0/v0.1." and "No afirmar O(changes) para el recorrido completo del
filesystem."
The distinction the record exists to preserve:
  O(corpus)   = traversal / metadata inspection of every entry. v0.2 does NOT optimise it.
  O(changes)  = reading and hashing of content. This is the only thing v0.2 promises.
Alternatives considered: (a) claim full O(changes) for the process — REJECTED by the user as
a formulation stronger than what is demonstrable; (b) optimise the traversal too — NOT
v0.2 (it is a separate concern and would need its own evidence).
Evidence: v0.1's own measurement (2000 files x 4 KiB): scan 23.9 ms, content read+hash
102.5 ms, full observation 209.6 ms. The optimisable share is the read/hash part, and its
proportion varies with the corpus: hashing dominates for large files, traversal for very
many small ones. Both directions are to be stated in the version record.
Consequences:
  - Any claim of the form "cost proportional to change" must name the read/hash scope.
  - A benchmark that is merely faster is not evidence of the objective; the objective needs
    counters showing which work was avoided.
  - Preserving verdicts, evidence, ambiguity and reconstruction is part of the objective,
    not a side condition: an optimisation that changes a verdict fails v0.2.
Scope guard: this record does NOT select an architecture, does NOT settle Q25, and does NOT
authorize any implementation. Implementation is authorized separately.
Reversibility: high. The objective is a target, not a commitment about the product's shape.
Status: DECIDED (scoped to v0.2).

================================================================================
UD-017 — v0.2 observation basis: a skip is never content verification
Authority: USER (2026-09-12 v0.2 planning mandate, D2 and D3).
Statement: the contract grows so that the output distinguishes four outcomes, and `basis`
is the field that expresses it:
  - content read and verified in this run;
  - result obtained through metadata evidence;
  - content not verified;
  - errors / UNKNOWN.
`status` must be able to expose the difference and `show` must expose `basis` where it is
relevant. `basis` must be traceable to the evidence that produced the result.
**Mandatory rule:** if Umbral did not read the bytes, it may not present the result as
content-verified.
Supporting statement, verbatim: "Si Umbral no leyó los bytes, no puede presentar el
resultado como content-verified."
The epistemic chain this record fixes, which no later change may shorten:
  bytes read            -> content-verified
  metadata sufficient   -> metadata-stable / equivalent -> NOT content-verified
  evidence insufficient -> UNKNOWN
Alternatives considered: (a) express the basis inside the existing labels without a new
field — REJECTED by the user: it would overload `observed`, which by UD-014's correction
(F-V01-2) is reserved for what the filesystem itself states per entry; (b) leave the
contract unchanged and optimise silently — REJECTED: it would narrow the contract silently,
which this project forbids.
Evidence: the basis is derived, not observed — it is produced by Umbral. Under the F-V01-2
rule (observed = only what the filesystem states: canonical, kind, size, mtime), `basis`
must be labelled `derived` and must remain traceable to the observed fields it rests on.
Consequences:
  - The growth of the output contract is declared NOW, before v0.2's acceptance criteria are
    fixed — deliberately, to avoid the v0.1 pattern of finding a needed distinction after
    the criteria were frozen (F-V01-1, F-V01-3).
  - A contractual test is required that FAILS if an entry obtained by skip is counted again
    as content-verified.
  - The log must persist whatever the skip decision needs; any schema change is technical
    and version-scoped (UD-014), and does not settle Q25.
Scope guard: this record does NOT specify the mechanism, does NOT decide the skip condition
(see UD-018), and does NOT authorize implementation.
Reversibility: medium. The contract growth is reversible only by an explicit narrowing,
which would itself have to be recorded.
Status: DECIDED (scoped to v0.2).

================================================================================
UD-018 — ctime is an optimisation heuristic, never a guarantee of content
Authority: USER (2026-09-12 v0.2 planning mandate, D3).
Statement: ctime is approved **only** as a possible optimisation heuristic. It is not a
guarantee of content equality. It may not be used to assert content-verified. If it is used
to avoid hashing, the result must explicitly retain that the bytes were not verified.
Supporting statement, verbatim: "No conviertas una heurística de metadata en una garantía de
integridad."
What the evidence supports and what it does not (recorded so the limits travel with the
decision):
  - ctime is the inode change time, stamped whenever the inode's metadata changes, and it is
    NOT settable from userland (Linux kernel documentation). mtime is settable; ctime is not.
    Therefore a writer that restores size and mtime exactly still moves ctime — which closes
    the specific false negative v0.1 documents as its `stat` guard's blind spot.
  - Measured locally (tmpfs and btrfs, kernel 7.1.13): ctime changed in every one of the
    tested cases — same size with mtime restored, chmod only, hard link created, atomic
    replacement — and in 600 consecutive writes there was not one collision.
  - NOT established: that ctime always changes. The kernel's own documentation records that
    coarse-grained timestamps can make a change invisible within a jiffy, and that
    multigrain timestamps (which reduce this) are an opt-in per filesystem. ctime is
    Unix-only and its granularity is filesystem-dependent. ext4 was not measured locally.
  - Therefore: ctime narrows a known false negative. It does not create an integrity
    guarantee, and it does not convert metadata into content evidence.
Alternatives considered: (a) treat ctime as sufficient evidence of content stability —
REJECTED by the user as a false guarantee of integrity; (b) omit ctime entirely — left open
as the other legitimate option; (c) admit ctime into the skip condition while the result
keeps stating that the bytes were not read — ADOPTED as permitted, not required.
Consequences:
  - The analysis of ctime's limits and portability must be preserved alongside the mechanism.
  - Any test that relies on ctime must be classified: if it depends on filesystem behaviour,
    it is demonstrated against hand-built observation sets, never required of the filesystem.
  - Adding ctime to a skip condition can only make it stricter (more re-reads), never more
    permissive. A metadata change that does not alter content (chmod, chown, hard link) will
    force a re-read, and that cost is accepted.
Scope guard: this record does NOT decide that ctime enters the skip condition. That question
(D-PEND-2) remains OPEN. This record fixes only what ctime may and may not be used to claim.
Reversibility: high.
Status: DECIDED (process/epistemic constraint), with the mechanism still OPEN.

================================================================================
STATUS VOCABULARY (so no reader has to infer a record's state)
================================================================================
  DECIDED              — the user has committed; the record stands.
  DECIDED (scoped)     — committed for a named, limited scope; explicitly reversible and
                         explicitly NOT a project-wide or architectural choice
                         (UD-013 for the V0 experiment, UD-014 for v0.1).
  DECIDED (intent)     — the intent is committed; the mechanism is still research.
  DECIDED (process)    — a commitment about how the project works, not about the product.
  DECIDED (scoped to v0.2)
                       — committed for the named version; explicitly not project-wide.
                         (UD-016, UD-017.) A scoped commitment expires with its version
                         unless a later record extends it.
  SUPERSEDED           — replaced by a later record; the record itself is never edited,
                         a pointer is added here instead.

================================================================================
SUPERSESSION POINTERS (records are immutable; this is where replacement is recorded)
================================================================================
  UD-006 ("research before implementation; no code yet")
      superseded in part by UD-012 (2026-09-11), which authorized the scoped V0
      prototype fsp-check. UD-006's rule still governs the product: no implementation
      before an accepted architecture. The "no code yet" condition applied to the state
      of the project on 2026-09-10 and is preserved as written; it is no longer a
      description of the current tree.
  UD-007 ("research method: Gemini Deep Research first, Hermes as independent verifier")
      status unchanged (DECIDED, process), still carrying its original flag: it was
      inferred from observed behaviour rather than stated, and was awaiting one
      confirmation (recorded as D8). Still awaiting it.

================================================================================
NOT DECIDED (explicitly, by the user's own record — MC §52): final name, final
architecture, database, indexing technology, AI protocol, versioning model, permission
mechanism, UI architecture, semantic model. The v0.1 crate uses SQLite; that is a scoped
implementation choice under UD-014, and it settles none of these. Also undecided: surface sequence (0.7), MVP
boundary (0.8), licence, governance, first target user sequence.
