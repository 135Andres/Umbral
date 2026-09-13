# Documentation

This directory holds the project's working documentation, grouped by function. Each file
has exactly one role and one home; this page is only a map. The repository's full
authority ladder and reading rules live in the [root README](../README.md).

| Category | Files | For |
|---|---|---|
| [Where the project stands](canonical/) | [`PROJECT-DIRECTION`](canonical/PROJECT-DIRECTION.md) · [`VISION`](canonical/VISION.md) | the one-screen compass; what Umbral is meant to become and what it refuses to be |
| [Rules and bounds](canonical/) | [`PRINCIPLES`](canonical/PRINCIPLES.md) · [`INVARIANTS`](canonical/INVARIANTS.md) · [`REQUIREMENTS`](canonical/REQUIREMENTS.md) · [`CONSTRAINTS`](canonical/CONSTRAINTS.md) | how decisions are made; what must hold; what it must do; what bounds every candidate |
| [Commitments](decisions/) | [`DECISIONS`](decisions/DECISIONS.md) | **the only place decisions live** (`UD-nnn`, immutable records) |
| [Candidates and open questions](candidates/) | [`ARCHITECTURE-HYPOTHESES`](candidates/ARCHITECTURE-HYPOTHESES.md) · [`OPEN-QUESTIONS`](candidates/OPEN-QUESTIONS.md) · [`RESEARCH-AGENDA`](candidates/RESEARCH-AGENDA.md) | candidate mechanisms — all explicitly *not selected*; what is unresolved; what must be investigated |
| [Product-level hypotheses](candidates/) | [`COEXISTENCE-STRATEGIES`](candidates/COEXISTENCE-STRATEGIES.md) · [`ENVIRONMENT-INTELLIGENCE`](candidates/ENVIRONMENT-INTELLIGENCE.md) | proposed, unratified hypothesis sets from the research cycles |
| [Version scope](candidates/) | [`V0.2-SCOPE-PROPOSAL`](candidates/V0.2-SCOPE-PROPOSAL.md) | a proposed scope for a version that has not started — proposal, not commitment |
| [V0](v0/) | [`V0-IMPLEMENTATION-PLAN`](v0/V0-IMPLEMENTATION-PLAN.md) | the frozen prototype's plan, invariants, criteria |
| [Versions](versions/) | [`versions/README`](versions/README.md) | what each development version did — goal, evidence, known limitations. Frozen once the version closes |

Elsewhere in the repository:

- [`research/`](../research/) — evidence: research artifacts, external briefs, source
  index, frozen history
- [`experiments/`](../experiments/) — experiments and results, including V0's closeout in
  [`experiments/v0-harness/V0-CLOSEOUT.md`](../experiments/v0-harness/V0-CLOSEOUT.md)
- [`fsp-check/`](../fsp-check/) — the frozen V0 prototype (experimental infrastructure)
- [`DOCUMENTATION-ARCHITECTURE.md`](DOCUMENTATION-ARCHITECTURE.md) — how this repository
  itself is organized, and the routing rule for new information (proposed)

Nothing in this directory is a decision except
[`decisions/DECISIONS.md`](decisions/DECISIONS.md). Hypotheses are not selected; research
is not validation; V0 is not the architecture.
