# DECISIONS

Status: CURRENT (living log of FROZEN records) — decision records (T3). Individual records are immutable; supersede, never rewrite.

Created 2026-09-10 by the project-intelligence audit, after user decisions were
recovered from session history. Before that date no decision record existed because no
decision existed.

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
Authority: USER (MC header, §2). Statement: "Sonora ... must NOT be architecturally
constrained by [or constrain] ... The new tool"; "Sonora's domain ontology is NOT
automatically the ontology of the new tool." Sonora is the first intended consumer/use
case; its lessons (authority, provenance, uncertainty, sovereignty) are inputs.
Alternatives: not discussed.
Evidence: the Sonora governance corpus confirms the same boundary from the other side.
Consequences: no Sonora concept may enter FSP without passing the Lesson-vs-Invariant-
vs-Implementation test (MC §2).
Reversibility: durable by design.
Status: DECIDED.

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

================================================================================
NOT DECIDED (explicitly, by the user's own record — MC §52): final name, final
architecture, database, indexing technology, AI protocol, versioning model, permission
mechanism, UI architecture, semantic model. Also undecided: surface sequence (0.7), MVP
boundary (0.8), licence, governance, first target user sequence.
