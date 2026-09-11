# REQUIREMENTS

Status: CURRENT (living) — canonical knowledge (T2).

Category C: what the product should enable. Deliberately unprioritised and undated.
Every item carries a status. Nothing here has been accepted as a requirement — the corpus
is research, so most items are RESEARCH OPINION awaiting ratification.

-------------------------------------------------------------------------------
CORE (the product is not the product without these)
-------------------------------------------------------------------------------
R1  Operate directly on arbitrary existing directories, with no migration step.
    Source: S1 §10.3, S5 §Problem, S8 §5.9. Status: USER INTENT.

R2  Attribute semantics to files without relocating them: type, state, links, provenance.
    Source: S1 §10.4, S4 §5, S9 §Separation. Status: STRONG HYPOTHESIS.

R3  Retrieve across the whole corpus: exact/lexical search first, conceptual search as a
    selectively applied layer.
    Source: S2 §5.1/§6, S9 §Recommendations, S11 §7. Status: STRONG HYPOTHESIS.

R4  Assemble curated, budgeted context for a model from the corpus.
    Source: S2 §16, S3 §Context Packages. Status: STRONG HYPOTHESIS.

R5  Present the same underlying files through multiple views without duplicating state.
    Source: S1 §10.6, S8 §5.6. Status: STRONG HYPOTHESIS.

R6  Trace where a claim, relationship or artifact came from, and by which agent.
    Source: S1 §10.8, S4 §5, S6 §AI-Provenance. Status: STRONG HYPOTHESIS.
    Note: [CORRECTED 2026-09-10 — the previous note here said "no user-facing requirement
    for it is stated anywhere", which was true of the research corpus but is contradicted
    by the user's own charter: MC §40 states that it should be possible to understand
    where information came from, who/what changed it, which AI/session produced a result,
    and what context was used.] Provenance is therefore USER INTENT (MC §40), not merely a
    research differentiator. Not yet a decision record; the residual open question is how
    prominent it is in the UX (Q13).

R7  Keep the user in control of what leaves the machine and what may be mutated.
    Source: S1 §15, S7 §17, S9 §11. Status: STRONG HYPOTHESIS.

-------------------------------------------------------------------------------
FORMATIVE (the corpus argues for these; contested or unresolved)
-------------------------------------------------------------------------------
R8  Undo and recovery that span multi-file AI operations, not per-file history.
    Source: S6 §AI-Specific Provenance, S11 §17. Status: RESEARCH OPINION.
    Contested: S6 recommends building it; S11 recommends deferring to an existing engine.

R9  Interoperate with Git without exposing Git's failure modes to non-developer users.
    Source: S6 §Exec/§Recommendations, S11 §17-5. Status: RESEARCH OPINION. Contested.

R10 Sessions that can be resumed across sessions, hosts and model providers.
    Source: S3 §Session Portability, S9 §Recommendations, S11 §17-4 (keep, simplified).
    Status: WEAK HYPOTHESIS — no evidence that mid-flight execution state crosses
    providers (S3 §8).

R11 Extensibility for third-party and user-authored logic under an explicit capability
    boundary.
    Source: S10 §16/§17. Status: RESEARCH OPINION (WEAK HYPOTHESIS).

R12 Declare and enforce permission scope, with modes that go beyond one binary switch.
    Source: S7 §6/§17. Status: USER INTENT (MC §24; UD-008) — upgraded 2026-09-10.
    The user's own words require the layered model (NORMAL / CONFIRMATION REQUIRED /
    SEMI-BYPASS / FULL BYPASS, with semi-bypass authorizing edit/move/delete), so this is
    no longer an unsettled research question: S11's two-mode simplification is superseded
    (tension T3 resolved by user). What remains RESEARCH is the mechanism — the exact
    semantics of each layer and how they are enforced (Q7, MC §43-Q10/Q11).

R13 Progressive independence from any single AI provider.
    Source: S0 §9 index, S3 §Session Portability. Status: USER INTENT (indexed).

-------------------------------------------------------------------------------
CANDIDATE (corpus possibilities; not requirements at this stage -> class J)
-------------------------------------------------------------------------------
  J1  Desktop, web, CLI and API surfaces over one core (S9 §Recommendations-3/4).
  J2  Self-hosted server / headless deployment (S9 Migration Path).
  J3  Optional hosted multi-tenant services (S0 §9 index: "optional hosted services").
  J4  Messaging-platform integration, e.g. Telegram (S0 §9 index; S10 explicitly defers
      it as an external process rather than a core daemon).
  J5  A plugin ecosystem with a published extension surface (S10).
  J6  Multiple visualisations: table, board, timeline, local graph (S8 §5).
      Note tension: S8 wants these as lenses; S11 rejects global graph views outright.
  J7  Binary and media assets as first-class tracked objects (S6, S9).
  J8  Onboarding that yields immediate utility with zero configuration (S8 §5.9/§17).

These are recorded so they are not lost or rediscovered as novelties. They are not
commitments and must not appear as requirements in any downstream document.
