# INVARIANTS

Status: CURRENT (living) — canonical knowledge (T2).

Category A only: properties that must remain true regardless of implementation.
Nothing here is CONFIRMED — no implementation exists to confirm anything.

AUTHORITY UPGRADE (2026-09-10 audit): the Project Master Context (MC) was recovered from
the project's original working session. It is the owner's founding charter — a private
document, not part of this repository, so its `MC §n` citations cannot be checked here —
and it directly states most of these invariants.
Statuses below now read USER INTENT with an MC section reference wherever MC states the
property. Still no architecture follows from any of them.

  A1  <- MC §29 ("the underlying user data should remain accessible as ordinary files"),
         MC §4, MC §39.
  A2  <- MC §29, MC §40 (Portability lesson).
  A3  <- MC §4 ("no mandatory global taxonomy ... unless the user chooses to create it"),
         MC §5, MC §39.
  A4  <- MC §6 (verbatim user property) — note the user themselves marks it as needing
         deeper architectural research before implementation.
  A5  <- MC §15 (AI discovery -> candidate -> user review -> accepted/rejected),
         MC §39 ("a generated summary is authoritative" must not be assumed).
  A6  <- MC §40 ("Capability != Authorization", Sonora lesson the user endorsed),
         MC §23 (dangerous actions require permissions/confirmation).
  A7  <- MC §29 (fully local; internet only where a service needs it), MC §34 ("setup
         is not ingestion"; the user decides what gets indexed).
  A8  <- MC §3 (local-first), MC §29 (works fully locally).
  A9  <- MC §10, §32, §39 (product should not depend on one AI provider; no canonical
         provider), MC §51.
  A10 <- MC §28 ("highly desirable"), MC §43 item 29 (portability if the app disappears).
  A11 <- MC §11 (verbatim: the system must NOT inject hidden prompts that override the
         user's/developer's instructions; capabilities are exposed as documentation/API,
         not behavioral override).

Original source references (S1-S11) remain valid as the evidence base for each; they are
listed on each invariant below.

-------------------------------------------------------------------------------
A1 — The user's files on disk are the authoritative store of durable user data
No proprietary container may be required to read, edit, version or port the user's work.
The product must never be the only way to reach the user's content.
Source: S1 §1, S5 §Problem, S9 §Exec, S11 §16. Status: USER INTENT.
Boundary: this invariant does not by itself settle where non-derivable state lives
(sessions, provenance, UI-asserted metadata). See Q2.

-------------------------------------------------------------------------------
A2 — Survivability without the application (the Sunset Test)
If the application disappears, user content and its human-readable semantics survive:
prose, links and declared attributes remain usable. Only derived artifacts may be lost.
Source: S1 §9, S5 §Separation of Storage Primitives, S9. Status: USER INTENT.

-------------------------------------------------------------------------------
A3 — No mandated taxonomy
Classification is opt-in and additive. A directory of plain, unclassified, arbitrarily
named files must remain a fully functional workspace.
Source: S1 §10.3, S8 Trap 3, S11 §17-1. Status: USER INTENT.

-------------------------------------------------------------------------------
A4 — External reorganisation must not destroy semantic structure
Moving, renaming, duplicating or restoring files outside the application must not sever
the user's entities and relationships.
Source: S1 §7, S4 §5. Status: STRONG HYPOTHESIS (product property; the mechanisms that
would deliver it are unresolved — see H3).

-------------------------------------------------------------------------------
A5 — Machine inference never silently becomes authoritative user state
AI output enters the user's operational reality only through explicit acceptance.
Observation is not assertion; capta is not data.
Source: S4 §2/§5/§16, S7 §17, S11 §9-2. Status: STRONG HYPOTHESIS.

-------------------------------------------------------------------------------
A6 — Delegated, bounded authority for automation
Operational reach is not authorization. Automation acts only inside scope that was
explicitly granted, and delegation can only narrow, never widen, that scope.
Source: S7 §17 ("Capability != Authorization"), S11 §11. Status: STRONG HYPOTHESIS.

-------------------------------------------------------------------------------
A7 — No egress without explicit, scoped authorization
Nothing leaves the machine unless the specific context was authorised, and local
inference must remain possible.
Source: S1 §15, S9 §7/§11, S8 §11. Status: USER INTENT (MC §29, §34).

-------------------------------------------------------------------------------
A11 — No hidden prompt injection into connected AIs
The system must not inject hidden prompts that override or replace the user's/developer's
instructions. It exposes capabilities and documentation to connected AIs — an API/SDK
documentation layer, not a behavioral override.
Source: MC §11 (verbatim user statement). Status: USER INTENT.

-------------------------------------------------------------------------------
A8 — Local-first operation
Core functions — organise, read, write, search, retrieve, assist with a local model —
work offline, with no mandatory service, account or network.
Source: S0 §9 index, S9 §Recommendations-1. Status: USER INTENT (indexed, low confidence).

-------------------------------------------------------------------------------
A9 — Provider neutrality
No lock-in to a single AI provider, model family or vendor-specific protocol.
Source: S0 §9 index, S3 §Exec. Status: USER INTENT (indexed, low confidence).

-------------------------------------------------------------------------------
A10 — Rebuildable derived state
No durable user data and no user-asserted semantics may exist only inside a disposable
index or cache. Destroying internal derived state must be recoverable.
Source: S4 §16-1, S5 §Rebuildability, S9 §Architectural Implications. Status: USER INTENT.
Caveat: whether this is achievable depends on resolving Q2 — what is genuinely
non-derivable, and therefore where it must live.

-------------------------------------------------------------------------------
INTERACTION TO WATCH
-------------------------------------------------------------------------------
A1 + A10 versus the two-sources-of-truth problem: filesystem authority plus an internal
state store reintroduces exactly the concurrency failure documented in Logseq (S9
Evidence 1; S11 §14 "The Dual Source of Truth"). The invariants stand. The mechanism that
satisfies both is an open question (Q1, Q2) and must not be assumed away.
