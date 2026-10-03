<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="../assets/brand/mark-dark.svg">
    <img src="../assets/brand/mark-light.svg" alt="" width="48">
  </picture>
</p>

<h1 align="center">Umbral documentation</h1>

<p align="center"><a href="../README.md">Project home</a> · <a href="../umbral/README.md">Tool guide</a> · <a href="READING-THIS-REPOSITORY.md">Reading this repository</a></p>

---

This directory is the project's working memory, grouped by function. Each file has exactly one
role and one home; this page is only a map. The authority ladder and the rules for reading
claims live in [Reading this repository](READING-THIS-REPOSITORY.md).

## Start here

| I want to… | Read |
|---|---|
| know where the project stands, in one screen | [`PROJECT-DIRECTION`](canonical/PROJECT-DIRECTION.md) |
| understand what Umbral is meant to become | [`VISION`](canonical/VISION.md) |
| see what has actually been decided | [`DECISIONS`](decisions/DECISIONS.md) — the only place decisions live |
| see what each version did, and its evidence | [Version records](versions/README.md) · latest: [`v0.2`](versions/v0.2.md) |
| see what is still open | [`OPEN-QUESTIONS`](candidates/OPEN-QUESTIONS.md) · [`ARCHITECTURE-HYPOTHESES`](candidates/ARCHITECTURE-HYPOTHESES.md) |
| find where an identifier (`UD-031`, `A2-V3`, …) is defined | [`IDENTIFIERS`](IDENTIFIERS.md) |

## The full map

| Category | Files | For |
|---|---|---|
| [Where the project stands](canonical/) | [`PROJECT-DIRECTION`](canonical/PROJECT-DIRECTION.md) · [`VISION`](canonical/VISION.md) | the one-screen compass; what Umbral is meant to become and what it refuses to be |
| [Rules and bounds](canonical/) | [`PRINCIPLES`](canonical/PRINCIPLES.md) · [`INVARIANTS`](canonical/INVARIANTS.md) · [`REQUIREMENTS`](canonical/REQUIREMENTS.md) · [`CONSTRAINTS`](canonical/CONSTRAINTS.md) | how decisions are made; what must hold; what it must do; what bounds every candidate |
| [Commitments](decisions/) | [`DECISIONS`](decisions/DECISIONS.md) | **the only place decisions live** (`UD-nnn`, immutable records) |
| [Candidates and open questions](candidates/) | [`ARCHITECTURE-HYPOTHESES`](candidates/ARCHITECTURE-HYPOTHESES.md) · [`OPEN-QUESTIONS`](candidates/OPEN-QUESTIONS.md) · [`RESEARCH-AGENDA`](candidates/RESEARCH-AGENDA.md) | candidate mechanisms — all explicitly *not selected*; what is unresolved; what must be investigated |
| [Product-level hypotheses](candidates/) | [`COEXISTENCE-STRATEGIES`](candidates/COEXISTENCE-STRATEGIES.md) · [`ENVIRONMENT-INTELLIGENCE`](candidates/ENVIRONMENT-INTELLIGENCE.md) | proposed, unratified hypothesis sets from the research cycles |
| [Latest version](versions/v0.2.md) | [`v0.2`](versions/v0.2.md) · [`V0.2-SCOPE-PROPOSAL`](candidates/V0.2-SCOPE-PROPOSAL.md) · [`V0.2-TECHNICAL-DESIGN`](candidates/V0.2-TECHNICAL-DESIGN.md) | v0.2, declared evidence complete (`UD-038`); its accepted criteria are in the scope proposal §9 |
| [V0](v0/) | [`V0-IMPLEMENTATION-PLAN`](v0/V0-IMPLEMENTATION-PLAN.md) | the frozen prototype's plan, invariants, criteria |
| [Versions](versions/) | [`versions/README`](versions/README.md) | what each development version did — goal, evidence, known limitations. Frozen once the version closes |

Elsewhere in the repository:

- [`IDENTIFIERS.md`](IDENTIFIERS.md) — registry of the identifier namespaces in use, their owning
  documents and their ranges, with the known collisions recorded. A derived index: it points at the
  owners and is not authority.
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
