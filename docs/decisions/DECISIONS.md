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
UD-019 — v0.2 basis: applicability must be determinable; the mechanism is NOT chosen
Authority: USER (2026-09-13 project decision following EXP-AI-01, "QUÉ APRENDEMOS DE
EXP-AI-01 PARA v0.2").
Statement: `basis` must be traceable to the entry or object it applies to. The mechanism of
that traceability remains a design question.
What this record does NOT say — and the distinction is the point: it does not say that every
`basis` must explicitly carry a subject. It says that the applicability of `basis` must be
traceable and determinable. That is what the evidence permits.
Relationship to UD-017: `UD-017` already establishes that `basis` must be traceable to the
evidence that produced the result. This record refines HOW that traceability requirement must
be satisfied, and deliberately does not select the representation. `UD-017` is not amended;
records are immutable, so this is a new record referencing it.
Evidence: EXP-AI-01 (`8a50eb2`), `experiments/exp-ai-01/COMPARISON-AI-01-AI-02.md` §8, §10.
Two independent AI readers, under the same frozen material, could not resolve the
applicability of a derived declaration from positional adjacency alone: the
`path-encoding=escaped` line names no path and follows two `modified` lines (verified against
`material.txt` line 64). Mere positional adjacency therefore does not provide sufficient
traceability for external readers under this material.
What follows for v0.2: v0.2 must guarantee that the relation between `basis` and that to which
it applies is determinable under the contract.
The mechanism stays open, and none of these is excluded in advance:
  - an explicit field;
  - a declared convention;
  - another verifiable representation.
Do NOT assume, in any later design: that it must be a string; that it must be an independent
field; that it must be repeated on every line; or that any particular format is implied.
The format is not designed here and is not designed by this record.
Alternatives considered: (a) require an explicit `subject` on every derived line — NOT
adopted: it fixes the mechanism before the design work, and the experiment supports
determinability, not one particular representation; (b) leave UD-017 as it stands and treat the
adjacency problem as a legibility debt like F-V01-8/F-V01-9 — REJECTED: `basis` is new output
whose applicability is part of what makes its traceability claim meaningful, so the gap would
be introduced by v0.2 rather than inherited by it.
Consequences:
  - `basis` must not introduce an undefined semantic vocabulary. `F-V01-9` records that the
    output's vocabulary is defined nowhere in the output; v0.2 adds a term to that output and
    must not deepen the debt. `F-V01-9` is NOT reopened or resolved by this record.
  - Non-determinability must remain operational, not anthropomorphic: `unknown` means a
    property is not determinable from the available evidence within the contract. The output
    and the documentation describe evidence, observations, derivations and limits of
    determination, and attribute no mental state to the software. The enforced
    `BANNED_LEXICON` in `umbral/src/report.rs`, and the test that applies it to the rendered
    output of every command, continue to apply unchanged.
  - A contractual test is required that FAILS if a `basis` cannot be attributed to the entry or
    object it describes under the chosen mechanism.
Scope guard: this record does NOT authorize implementation, does NOT select a format, does NOT
redesign `show`, and does NOT redesign the path encoding. The `show` question raised by the same
experiment (CTF-2) remains DEFERRED, with one design condition: if v0.2 changes `show` to expose
`basis`, the new representation must not reproduce the unresolved subject/applicability
ambiguity observed in EXP-AI-01.
Reversibility: medium. Weaker than a format choice, but narrowing it later would itself have to
be recorded.
Status: DECIDED (scoped to v0.2).

================================================================================
UD-020 — v0.2 `show`: an entry-scoped value must name its entry
Authority: USER (2026-09-13 technical-design decision, "S1").
Statement: where `show` exposes a `basis` associated with an entry, `show` must identify that
entry unambiguously. The change is the minimum that makes `UD-019`'s applicability requirement
satisfiable on `show`'s surface, and nothing more.
Relationship to UD-017 and UD-019, which this record references and does not amend (records are
immutable):
  - `UD-017` requires that `show` expose `basis` where it is relevant.
  - `UD-019` requires that the relation between a `basis` and what it applies to be determinable
    under the contract, and left the mechanism open.
  - `UD-019` also recorded CTF-2 (`show` names no path) as DEFERRED, with one design condition:
    if v0.2 changes `show` to expose `basis`, the new representation must not reproduce the
    unresolved applicability ambiguity.
  - Verified in the technical design: `show`'s output names its entry **nowhere** — its
    `observed kind=… size=… mtime=…` line carries neither a path nor a run. So `show` cannot
    satisfy `UD-017` and `UD-019` together without naming the entry. This record resolves that
    impasse in favour of naming it.
Why this is a decision and not a design detail: it resolves a question `UD-019` left explicitly
OPEN (S1 vs S2), and it authorizes a scope item that was deferred — pulling the minimum of CTF-2
into v0.2. Neither can be done implicitly: `V0.2-SCOPE-PROPOSAL.md` and
`V0.2-TECHNICAL-DESIGN.md` are candidates (DT4) and cannot hold a commitment.
Alternatives considered: (a) S2 — `show` does not expose a per-entry `basis` in v0.2 —
REJECTED by the user: it would leave `UD-017`'s obligation partially unmet. (b) Leave `show`
unchanged and treat the impasse as a legibility debt — REJECTED: the same reasoning as `UD-019`
alternative (b), the gap would be introduced by v0.2 rather than inherited by it.
Scope guard — this record does NOT:
  - redesign `show`; its existing lines, labels and fields are unchanged;
  - resolve `F-V01-8` or `F-V01-9`, in general or in part;
  - become a general legibility cleanup, or authorize one;
  - authorize the path-encoding change (still `RESEARCH FOLLOW-UP`);
  - authorize implementation.
CTF-2 itself remains a deferred legibility debt: what this record takes into v0.2 is the minimum
needed for `basis` applicability, not the debt's own resolution.
Consequences:
  - `show` gains, for each entry it reports, a field naming that entry (the mechanism is a
    design question, exactly as `UD-019` left it for `basis`).
  - A contractual test is required that FAILS if a `basis` in `show`'s output cannot be
    attributed to an entry on the same line.
Reversibility: medium. Reverting means `show` returns to exposing no per-entry `basis`, which
would re-open the `UD-017`/`UD-019` impasse.
Status: DECIDED (scoped to v0.2).

================================================================================
UD-021 — v0.2 verdict preservation is bounded by the evidence each observation holds
Authority: USER (2026-09-13 technical-design closure mandate, "F-TD-4").
Statement: the verdict-equivalence clause of `UD-016` is refined. v0.2 preserves the verdicts,
the evidence, the ambiguity and the reconstruction capability **within the limits of the
evidence each observation actually holds**. It does NOT guarantee that an observation which
reused prior evidence reproduces the verdict a full re-read would have produced.
What this record does NOT do: it does not reconsider v0.2's objective, and it does not remove or
weaken the skip. The objective stands. What is corrected is one clause of its formulation.
Relationship to UD-016, which this record refines and does not amend (records are immutable):
`UD-016` states the objective and adds "an optimisation that changes a verdict fails v0.2".
That sentence is **no longer accurate as written**, and the discrepancy is resolved here rather
than by editing the record. See the partial supersession pointer below.
The four things this record keeps distinct, because conflating them is how a limitation becomes a
false guarantee:
  - **verdict equivalence** — that v0.2's classification of a change equals what a full re-read
    would produce. **NOT guaranteed.** It holds only where the two observations hold the same
    evidence.
  - **reuse of prior evidence** — that a skip may carry a previous run's content reading forward.
    Permitted, and it must be attributable (see `hash_read_run` below).
  - **content verification** — that the bytes were read in this run. Only a fresh read qualifies.
  - **metadata-stable** — that the metadata the predicate rests on matched. It is not content
    verification and may never be presented as one (`UD-017`).
Accepted known limitation (`F-TD-4`, recorded in
`docs/candidates/V0.2-TECHNICAL-DESIGN.md` §G.3):
  - the skip may retain a previous content reading;
  - a mutation that preserves the metadata the skip predicate uses may go undetected in that
    observation;
  - this is a known and **reachable** limitation of the mechanism, not a hypothetical;
  - it must **not** be presented as a guarantee of v0.2, and it must not be hidden as a test
    exception.
Consequence for persistence: where v0.2 reuses a content result without re-reading bytes, the
reconstruction must be able to distinguish that result from a new content verification. The
evidence needed to distinguish at least these three states must be persisted:
  - the content was actually read and verified;
  - prior evidence was reused through metadata;
  - the result is UNKNOWN / error.
The concrete persistence schema is NOT decided here; the requirement is the distinguishability.
Scope guard: this record does NOT authorize implementation, does NOT change the objective, does
NOT remove the skip, does NOT select a persistence schema, and does NOT resolve `D-PEND-2`.
Reversibility: low in practice. Accepting the limitation is what makes the skip admissible at
all; withdrawing it would return the objective to an unachievable form.
Status: DECIDED (scoped to v0.2).

================================================================================
UD-022 — v0.2 output grammar and contract versioning
Authority: USER (2026-09-13 product-decision mandate "DECISIÓN DE PRODUCTO — GRAMÁTICA Y
VERSIONADO", confirming the record after Hermes stopped to explain why no existing record
could host the decisions).
Statement: the v0.2 output contract adopts, as one related decision set:

1. **G-1 + G-5 as the output grammar family.** G-1 is delimiter + escaping. It is adopted
   together with the full G-5 canonical-form discipline:
   - canonical rendering per value (one value, exactly one serialization);
   - a formally defined escape;
   - the escape character escaped to itself;
   - **coverage of the whole valid value domain** — newline, carriage return, tab, other
     control characters, consecutive whitespace, backslash, non-UTF-8 bytes, delimiter
     bytes, and their combinations — explicitly NOT limited to the current fixtures;
   - round-trip as the contractual property: for every valid value V,
     `parse(serialize(V)) == V`, and for every valid result R, the parse preserves the
     semantic associations;
   - explicit rejection of invalid representations, and fail-closed behaviour under
     ambiguity or malformed representation: if the input cannot be reconstructed
     unambiguously, no association may be invented. Error semantics are part of the
     contract: reject on malformed escape, truncated representation, invalid structure; no
     silent resynchronization; no best-effort reinterpretation; unknown future fields are
     preserved without interpreting (where the grammar allows); an unknown grammar version
     is rejected with an explicit reason.
   The semantic unit of the output is the **result**; the rendered line is only a textual
   representation, and result boundaries are defined by the grammar, never by terminal
   rendering.

2. **V2 as the contract-versioning mechanism.** An explicit contract/grammar version in the
   output header, so that a reader can determine which grammar an output follows from the
   output/header alone; standalone outputs are interpretable without the external
   environment. The version functions stay strictly separated — output contract/grammar
   version ≠ durable schema version ≠ project version ≠ implementation version — and one
   number must not serve all four.

3. **G-3 is NOT adopted for v0.2** as the primary output. It remains a technically valid
   alternative that this version does not select; the reason is the product decision to
   keep the existing textual/human-facing surface.

4. **G-4 (field-per-line) remains ELIMINATED**, on principle: it restores positional
   association, the failure the EXP-AI-01 evidence recorded.

Why one record: G-1+G-5 and V2 are related contract decisions about the same output
surface — the grammar states how values are written, and the version mark states how a
reader knows which grammar it is reading. They were authorized together, they are scoped
together, and recording them apart would suggest either can vary independently of the
other, which the design does not support.

Explicitly still OPEN — this record closes none of them: the exact escape specification
(the concrete escape function, its coverage table, canonical rendering rules); the exact
header representation; the literal contract version value; M1 vs M2 (the field-set
carrier); the basis field name; the evidence reference representation; the persistence
schema; whether B1 is the final applicability mechanism; whether applicability and
provenance share a representation; whether `ctime` enters the skip condition (D-PEND-2);
and Q25.

Relationship to existing records, none amended (records are immutable): `UD-017` requires
that `basis` be traceable and that the output distinguish four outcomes — this record
fixes how output values are written so such distinctions survive parsing; it does not
select the basis mechanism. `UD-019` leaves applicability mechanisms open — untouched.
`UD-020` is untouched. Evidence: the technical design's grammar analysis
(`docs/candidates/V0.2-TECHNICAL-DESIGN.md` §E.25–E.32b), including the defects found
there (a two-space delimiter appearing unescaped in values; a newline-bearing path
destroying the line unit; no contract version anywhere in the output).
Consequences:
  - A contractual test suite must demonstrate the round-trip property over the whole
    domain, with combination cases, and rejection of invalid representations.
  - The exact escape specification is deliberately a separate technical specification; a
    change to it does not reopen the family decision unless it violates the properties
    recorded here.
  - G-3's non-adoption is version-scoped: it does not prejudice any later version.
Scope guard: this record does NOT authorize implementation, does NOT select a field name,
a schema, an encoding beyond the family level, or the version value, and does NOT close
any item in the OPEN list above.
Reversibility: medium. The family choice is reversible only by a recorded narrowing, which
would itself have to be recorded.
Status: DECIDED (scoped to v0.2).

================================================================================
UD-023 — Historical guarantees and minimum-materiality rule
Authority: USER (2026-09-13 mandate "Umbral v0.2 — Derivación del núcleo histórico mínimo",
followed by explicit authorization to begin converting the approved conceptual decisions into
versioned changes).
Statement: v0.2 adopts the following historical-contract guarantees as one related decision set:

1. **Historical emission and support.** Umbral preserves what it emitted and the historical
   support available to it, with the material conditions and limitations known at the time. A
   later interpretation never replaces the original emission. Historical code behaviour,
   intended historical rules and later evaluation remain distinguishable when known; absence
   of enough retained support may legitimately make a later question indeterminable.
2. **Observational appearance and disappearance.** `Created` and `Deleted` are relative claims
   over compared observations and their effective scope. They do not assert physical creation
   or deletion. Materially different cases — confirmed absence, incomplete reference,
   effective-scope change, unknown coverage and other known limitations — must not collapse.
3. **Effective scope and contemporary explanation.** The effective historical scope, achieved
   coverage and the material explanation recorded during the run are preserved. A partial
   explanation remains marked partial, and a later explanation remains distinguishable from
   one recorded contemporaneously. This is not a commitment to retain the complete policy
   resolution chain.
4. **Evidence acquisition.** Known, material facts about how evidence was or was not obtained
   must survive: fresh acquisition, reuse, failed attempt, known non-attempt, historical
   uncertainty about whether an attempt occurred, and known diagnostics. Metadata evidence is
   not content evidence; a diagnostic is not automatically a root cause; equal values do not
   imply equal acquisition histories.
5. **Re-evaluation.** Re-evaluation is a derived, conditional act. Its evidence, evaluator,
   applicable rules, material limitations and use of information acquired after the historical
   run remain attributable. Insufficient evidence, evaluator incapability and an evaluation
   attempt that failed remain distinguishable. A re-evaluation neither substitutes for the
   historical emission nor establishes historical code behaviour, intended rules or physical
   events by itself.

Minimum-materiality rule: historical support is mandatory exactly where information known to
Umbral during observation, acquisition, effective-scope determination or emission would, if
lost, collapse two histories that the guarantees above require to remain distinguishable. This
rule defines a minimum, not an instruction to retain every available or potentially useful fact.
It never requires Umbral to invent a distinction it could not observe.

Relationship to existing records: this record extends the historical contract without amending
`UD-017`, `UD-019`, `UD-020`, `UD-021` or `UD-022`. In particular, `UD-021`'s distinction between
a fresh content reading, reused prior evidence and UNKNOWN/error remains mandatory; its concrete
persistence mechanism remains open.

Alternatives rejected by the user: treating later reconstruction as the historical emission;
treating `Created`/`Deleted` as physical claims; collapsing effective scope into achieved
coverage; collapsing diagnostics into causes; treating reused evidence as a fresh reading; and
defining historical support by future reconstructibility.

Consequences:
  - the association between an emission and its material historical support must remain
    recoverable, but this record does not require co-location or any particular carrier;
  - known material acquisition diagnostics must not be discarded before persistence merely
    because the final result can honestly say `unknown`;
  - the historical conditions on both sides of a comparative claim must remain interpretable;
  - later evaluations are new, attributable acts and cannot silently rewrite prior emissions;
  - implementation proceeds only through version-scoped, test-first increments whose behaviour
    is already decided; an unresolved semantic choice remains a stop condition.

Scope guard: this record does NOT select M1, M2 or M6; define a schema, table, struct, API,
serialization, exact output vocabulary or identity model; adopt `ctime`; decide retention or
TTL; guarantee reconstruction or replay; settle the names or full semantics of every mutation
kind; authorize changes to the frozen V0 prototype; or declare v0.2 complete. The exact reduced
set of irreducible conceptual distinctions remains analysis until separately accepted.

Reversibility: low for the semantic boundaries; high for every mechanism left open.
Status: DECIDED (scoped to v0.2).

================================================================================
UD-024 — Typed association and comparison-side identity (D6)
Authority: USER (2026-09-13, explicit adoption "D6 = Sí" after the adversarial
audit of the N3→N8 reduction).
Statement: the historical association between an evaluative act and its support
must be able to preserve, where material: the role the support played in the act,
the identity of each comparison side it belongs to, and the known historical
conditions of each side. This is a semantic requirement about what must remain
distinguishable; it selects no representation, structure or names.
Purpose: this record fixes the semantic condition under which the comparative
anchoring distinction (N3) is absorbed into the typed association instead of
remaining an independent core distinction. If a future representation binds only
individual observations with roles and cannot identify to which comparison side
an observation belonged, stories that differ only in their reference would
collapse, violating the observational relativity required by the second
historical guarantee (UD-023 §2). The canonical adversarial case: a `Created`
claim relative to reference run R1 versus the same claim relative to reference
run R2, with identical retained observations on both sides, must remain
distinguishable.
Relationship to existing records: a semantic precision of the association
requirement already adopted in UD-023 (its first consequence: the association
between an emission and its material historical support must remain recoverable,
and the historical conditions on both sides of a comparative claim must remain
interpretable). It amends no earlier record.
Scope guard: this record does NOT define how a comparison side is identified
concretely (run, capture, derived set), the closed vocabulary of roles, any
carrier, schema or storage for the association, event names, or retention of the
linked support.
Reversibility: low for the semantic requirement; high for every mechanism left
open.
Status: DECIDED (scoped to v0.2).

================================================================================
UD-025 — Acquisition granularity and traversal facts (D7)
Authority: USER (2026-09-13, explicit adoption "D7 = Sí" after the adversarial
audit of the N5→derivable reduction).
Statement: the acquisition history must be able to preserve, where material: the
acquisition state distinguished per material component of the evidence (for
example, metadata versus content), and known facts about the traversal process
that explain which part of the scope came to be observed, or why it ceased to be
observed. This defines what must remain distinguishable; achieved coverage
remains a consequence derivable from these facts together with the effective
scope — not an entity and not an additional guarantee.
Purpose: this record fixes the semantic conditions under which achieved coverage
remains derivable instead of an independent core distinction. Acquisition states
must not collapse metadata and content when the difference is material (UD-023 §4
already requires metadata ≠ content), because coverage of the metadata component
does not imply coverage of the content component. Traversal facts are material
even when no individual element exists to attach them to: when enumeration
stopped at a known point, the un-enumerated remainder leaves coverage unknown,
and D2/D3 require unknown coverage to stay distinguishable from verified-zero
change.
Relationship to existing records: a semantic precision of the acquisition
requirement already adopted in UD-023 §4 (how evidence was or was not obtained,
including failed attempt, known non-attempt and historical uncertainty). It
specifies the minimum granularity that preserves the differences D4 already
guarantees; it introduces no guarantee D4 did not carry. It amends no earlier
record.
Scope guard: this record does NOT define the closed vocabulary of acquisition
states, the exact set of material components, how coverage is computed or
presented (including the fate of the current `complete_scan` flag), the semantics
of `unknown` (UD-019 unchanged), observation identity, or retention of acquisition
records.
Reversibility: low for the semantic requirement; high for every mechanism left
open.
Status: DECIDED (scoped to v0.2).

================================================================================
UD-026 — Contextual identity of comparison sides (D8)
Authority: USER (2026-09-13, explicit adoption of the weakened formulation after the
semantic investigation "Identidad de los lados de comparación", which rejected the
stronger phrasing "anclado en la determinación histórica de observación" as
presupposing an observation model not yet decided).
Statement: the historical identity of a comparison side is contextual: it must allow
distinguishing the stories the adopted guarantees require to remain separate, without
constituting a claim of physical or ontological identity of the observed objects. A side
is anchored in the historical fact of observation that constitutes it — what was
observed, in which act, to the extent Umbral knew it — and in the material conditions
known of that observation. Equality of content, hash, observed values, coverage or
acquisition modality does not by itself imply identity of the side; the run participates
in the individuation of the observation, but does not by itself define the side. This
decision fixes semantic meaning, not an identification mechanism or a concrete
representation. It does not presuppose how many observations exist, how they are
individuated, or how they are identified.
What this record fixes conceptually:
  - a comparison side is not a physical object, a value, a hash, a run or a role;
  - equality of content is not historical identity; equality of acquisition is not
    historical identity;
  - identity must be sufficient to preserve the differences D1–D7 require;
  - there is no obligation to assert physical continuity between observations.
Relationship to existing records: a semantic precision of the comparison-side identity
requirement already adopted in UD-024 (D6). It amends no earlier record.
Scope guard: this record does NOT decide which identifier to use, whether to use a UUID
or a hash, `(run_id, observation_id)`, how observations are identified, the structure of
a run, whether a side is stored or derived, schemas, tables, structs, serialization, API,
architecture, role vocabulary, acquisition-state vocabulary, `unknown` (UD-019
unchanged), the identity of rules/evaluators, or retention/TTL. It does not close the
`Observation` model and adds no ontological property to it. It does not turn achieved
coverage into an independent guarantee.
Reversibility: low for the semantic requirement; high for every mechanism left open.
Status: DECIDED (scoped to v0.2).

================================================================================
UD-027 — Historical attribution of the evaluative act (D9)
Authority: USER (2026-09-13, explicit adoption "D9 = Sí" with one minimal modification:
the record must state its compatibility with the three strata D1 already distinguishes).
Statement: D1–D8 do not require a stable or absolute identity of rules or of the
evaluator. They require that the evaluative act be historically attributable with
sufficient context to distinguish the original emission from later interpretations or
re-evaluations, including the rules Umbral knew and recorded as applied in that act, their
partiality where it applies, and the temporal provenance of that information.
Declared rules, rules recorded as applied, and rules inferred afterwards are distinct
categories and must not be presented as equivalent. Absence of knowledge about the
historical rules may remain explicitly indeterminate.
The strata D1 distinguishes — historical code behaviour, intended rules at the time, and
later evaluation — remain distinguishable when known; "declared" designates what was
declared at the moment of the act, not a later reconstruction.
This decision defines no evaluator identity and no mechanism for identifying rules, and
does not guarantee that the historical record describes the executable's internal
behaviour with independent truth.
Why the strata clause is part of the record: D1 (UD-023 §1) already requires those three
strata to remain distinguishable when known. Read without it, "declared rules" and "rules
inferred afterwards" leave the intended-rules stratum unhosted, and intended rules would
collapse into a present-day inference — which would weaken an obligation UD-023 already
carries. The clause preserves an adopted distinction; it creates no new reconstruction
capability.
Relationship to existing records: a semantic precision of the stratum attribution already
required by UD-023 §1, and consistent with UD-024 (D6), UD-025 (D7) and UD-026 (D8). It
amends no earlier record.
Scope guard: this record does NOT decide evaluator identity, absolute rule identity, a
rule hash, a commit SHA, an implementation version, an executable fingerprint, a
configuration format, representation, schema, serialization, replay, reproducibility,
equivalence between implementations, or any identification mechanism. It is not a
guarantee that the recorded rules equal the executable's actual behaviour, and it does not
make a later re-evaluation correct or complete.
Reversibility: low for the semantic requirement; high for every mechanism left open.
Status: DECIDED (scoped to v0.2).

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
                         (UD-016, UD-017, UD-019, UD-020, UD-021, UD-022, UD-023, UD-024,
                         UD-025, UD-026, UD-027.) A scoped commitment expires with its version
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
  UD-016 ("v0.2 objective: O(changes) for content read and hashing ONLY")
      superseded **in part** by UD-021 (2026-09-13). UD-016's objective stands unchanged: the
      reading/hashing of content is what v0.2 reduces, and O(changes) is not claimed for the
      traversal. What is superseded is one clause of its formulation — "an optimisation that
      changes a verdict fails v0.2" — which the accepted limitation F-TD-4 shows to be
      inaccurate as written. Verdict preservation is now bounded by the evidence each
      observation holds (UD-021). UD-016's text is preserved as written and is not edited.

================================================================================
NOT DECIDED (explicitly, by the user's own record — MC §52): final name, final
architecture, database, indexing technology, AI protocol, versioning model, permission
mechanism, UI architecture, semantic model. The v0.1 crate uses SQLite; that is a scoped
implementation choice under UD-014, and it settles none of these. Also undecided: surface sequence (0.7), MVP
boundary (0.8), licence, governance, first target user sequence.
