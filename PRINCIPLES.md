# PRINCIPLES

Status: CURRENT (living) — canonical knowledge (T2).

Category D: how the product and its architecture should be decided. These are decision
heuristics, not implementation rules. Several conflict with each other; where they do, the
conflict is stated and left open.

P1 — Start from the user's existing files
   Adopt the user's current directory structure rather than migrating them into ours.
   Source: S1 §1, S8 §5. Term applied by synthesis from S1 §10.3, S8 §2.
   Rationale note: grouped from statements about non-mandatory taxonomy (S1), unmodified
   user hierarchies (S5), and zero-configuration discovery (S8). Basis: these restate the
   same constraint on the product from three angles.
   Status: USER INTENT.

P2 — Rebuildability over durability of derived state
   Any derived artifact — index, embedding, summary, cache, score — must be cheap to
   discard and safe to lose.
   Source: S1 §20, S4 §16-1, S5 §Rebuildability. Status: STRONG HYPOTHESIS.

P3 — Composition over invention (build only the differentiator)
   Reuse mature engines for storage, parsing, search, vector math, versioning primitives,
   protocol transport. Build only the semantic projection and context assembly layer.
   Source: S1 §17, S3 §13, S4 §13, S9 §Build-vs-Buy, S10 §13, S11 §13.
   Status: STRONG HYPOTHESIS. Conflict to watch: this principle is stated at least five
   times but never with the same "what to build" answer. See T2.

P4 — Determinism before probabilism
   Where a deterministic mechanism achieves the task (lexical lookup, syntax structure,
   change detection, exact-match identity), prefer it over a probabilistic or generative
   one. Model inference is for the tasks nothing deterministic can do.
   Source: S2 §6, S4 §16, S10 §5.2, S11 §7. Status: STRONG HYPOTHESIS.

P5 — Model output is a proposal, never a fact
   Inferences carry provenance, a confidence signal and an authority classification, and
   are promotable to authoritative state only by the user.
   Source: S4 §5/§16-3, S7 §17, S11 §8. Status: STRONG HYPOTHESIS.

P6 — Containment and reversibility over gatekeeping
   Action must be safe to attempt: bounded scope, atomic operations, universal undo.
   Prompting for permission on every action is not a security mechanism (see C12).
   Source: S7 §17, S8 §5.5, S11 §11. Status: STRONG HYPOTHESIS.

P7 — Progressive disclosure
   Zero configuration at first contact; complexity appears only when requested.
   Source: S8 §5.1/§5.9/§16. Status: STRONG HYPOTHESIS.

P8 — Context economy
   What is sent to a model is a curated, budgeted artifact, never a dump of the corpus.
   Source: S2 §3, S3 §3.2, S11 §7/§10. Status: STRONG HYPOTHESIS.

P9 — Human organisation is the primary interface; the semantic layer is a projection
   The physical layout keeps its spatial meaning. Semantic views are lenses computed over
   it, and mutating a lens mutates the underlying files.
   Source: S8 §5.2/§5.6/§16, S11 §16. Status: STRONG HYPOTHESIS.

-------------------------------------------------------------------------------
DOCUMENTATION PRINCIPLES — MOVED
-------------------------------------------------------------------------------
The six documentation principles that previously lived here (minimum viable structure,
no duplication, explicit uncertainty, no premature decisions, traceability, portability
without tooling) are now DP-1..DP-10 in DOCUMENTATION-ARCHITECTURE.md, which owns that
subject. Mapping, so nothing is lost: minimum viable structure -> DP-10; no duplication ->
DP-6; explicit uncertainty -> DP-5; no premature decisions -> the authority ladder in
README §AUTHORITY LADDER; traceability -> DP-3; portable/tool-independent -> DP-9.
This file now covers product decision heuristics only (one audience, one authority).

-------------------------------------------------------------------------------
UNRESOLVED PRINCIPLE CONFLICT
-------------------------------------------------------------------------------
P6 (containment over gatekeeping) versus A6 (bounded, explicitly granted authority). One
says make action safe and reversible rather than interrogating the user; the other says
authority must be granted deliberately in advance. They are reconcilable in principle —
pre-granted scoped authority plus strong containment — but the reconciliation is not
demonstrated and the corpus contains both framings. Recorded as Q7.
