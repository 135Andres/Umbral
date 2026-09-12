# Umbral

**Umbral is an open-source, local-first, AI-native project environment for making the
reality of an environment legible to multiple intelligences — human and artificial —
while keeping organization and authority in the user's hands, and providing tools that
make the work more ordered and efficient.**

> Status: **research project. No architecture selected. No product.** A frozen
> proof-of-concept (V0) exists under `fsp-check/`. See [Current status](#current-status).

---

## What is Umbral?

Umbral is an intended open-source, local-first workspace whose human mental model is
*"my files and folders"*. Projects, tasks, decisions, knowledge and AI capabilities are
meant to live as an **optional, derived, rebuildable overlay** over a filesystem the user
already owns — not as a mandated taxonomy, and not inside a proprietary database.

The files stay the user's. The overlay is a projection that can be discarded and rebuilt.

```
                 ┌──────────────────────────────────────────────┐
                 │  several intelligences — human and artificial │
                 └───────────────────┬──────────────────────────┘
                                     │  read / propose / act
                 ┌───────────────────▼──────────────────────────┐
                 │  derived overlay (optional, rebuildable)     │
                 │  meaning, relationships, provenance, state   │
                 └───────────────────┬──────────────────────────┘
                                     │  projected from, never owning
                 ┌───────────────────▼──────────────────────────┐
                 │  the user's real filesystem                  │
                 │  arbitrary files · arbitrary structure       │
                 └──────────────────────────────────────────────┘
```

## Why?

Today the human organizes files, and AI has to ingest them wholesale. Knowledge tools
demand a taxonomy up front, or swallow files into a database that outlives nothing. Both
put the maintenance burden on the person, and both make the environment harder — not
easier — for the *next* participant, human or artificial, to understand.

Umbral starts from the opposite assumption: the filesystem is already the shared reality,
and what is missing is a truthful, portable account of it.

## The idea

Four commitments define the shape of the project:

- **Filesystem sovereign.** Files are the source of truth; any index is a disposable
  projection.
- **No mandatory taxonomy.** Existing, messy, arbitrary structure must remain workable.
- **Intelligence as a participant, not a feature.** Several AI systems — from different
  vendors, some not yet existing — are expected to share one environment. Umbral's job is
  to make that environment legible to them, not to think on the user's behalf.
- **User authority.** The project records; it does not decide what is true. Its tools
  observe, derive, and report — including reporting that something is unknown, ambiguous,
  or disputed.

## Current status

| | |
|---|---|
| Phase | DISCOVERY → RESEARCH → ARCHITECTURE |
| Architecture | **not selected.** No database, protocol, versioning engine, UI or semantic model has been chosen |
| Product | **does not exist.** There is no usable application to install |
| Prototype | **V0 exists and is frozen**, status `PARTIAL` — see below |
| Next | V1 is **not started** and is not authorized |

The documentation in this repository is the project's memory: what it intends, what must
stay true, what is required, what constrains it, what is hypothesised, what is unknown,
and what has been decided. Decisions are recorded only in [`DECISIONS.md`](DECISIONS.md);
nothing else in this repository is a decision.

## V0 — the first experiment

`fsp-check/` is a small Rust prototype that asked one narrow question:

> Can a filesystem be observed, identified, content-verified, reconciled and persisted
> deterministically and safely — **without pretending to know what the files mean?**

```
  scan ──▶ identity ──▶ hash ──▶ reconcile ──▶ store
  (what    (physical    (content  (what         (append-only
   exists)  evidence)    bytes)    changed)      history +
                                                   rebuildable
                                                   projection)
```

- **Tested:** 68 tests; 512 generated property-test cases checked against an
  independent oracle; real process-kill crash trials; benchmarks recorded as evidence.
- **Falsification:** an oracle that models reality directly and never reuses the
  prototype's own reconciliation logic.
- **Result:** no implementation failure was produced inside the tested scope. Four defects
  were found, all in the harness or the oracle, **plus one coverage gap**: the property-test
  strategy never generated directory deletion, so that operation was modelled but never
  exercised. The three are distinguished explicitly in the falsification report — operations
  modelled, operations actually generated, operations exercised end to end.
- **Frozen state:** `PARTIAL`. The only gap is a reader-facing surface that would let a
  new reader reconstruct what the prototype recorded and when — deliberately outside V0's
  mandate.

Full evidence: [`experiments/v0-harness/V0-CLOSEOUT.md`](experiments/v0-harness/V0-CLOSEOUT.md)
(scope statement, criteria, limitations, open questions, V1 handoff).

> **V0 evidence ≠ Umbral architecture.** `fsp-check` is an experiment with a result. It is
> not "the Umbral engine", it is not the product's design, and nothing in it is a
> commitment about how Umbral will be built.

## Principles

The essentials, each with a canonical home:

- **The filesystem is the source of truth**; derived state must be rebuildable.
- **No mandatory organization.** The tool adapts to the user's structure.
- **The user decides.** Umbral records decisions, proposals, evidence and uncertainty; it
  does not hold opinions, and it never silently resolves a conflict.
- **Unknown is a valid answer.** Absence of evidence is represented as absence, not
  guessed.
- **Local-first, and survivable.** What the user wrote must remain readable if the
  software disappears.
- **No hidden instructions** injected into connected AI systems.
- **Research before implementation**; evidence before claims.

Detail: [`PRINCIPLES.md`](PRINCIPLES.md) · [`INVARIANTS.md`](INVARIANTS.md) ·
[`CONSTRAINTS.md`](CONSTRAINTS.md).

## Documentation

Read in this order; each file has exactly one role, and none duplicates another.

| Read | For |
|---|---|
| [`PROJECT-DIRECTION.md`](PROJECT-DIRECTION.md) | the one-screen compass: where the project is right now |
| [`VISION.md`](VISION.md) | what Umbral is meant to become, and what it refuses to be |
| [`PRINCIPLES.md`](PRINCIPLES.md) · [`INVARIANTS.md`](INVARIANTS.md) | how decisions are made; what must hold regardless of design |
| [`REQUIREMENTS.md`](REQUIREMENTS.md) · [`CONSTRAINTS.md`](CONSTRAINTS.md) | what it must do; what bounds every candidate |
| [`DECISIONS.md`](DECISIONS.md) | **the only place commitments live** (UD-001 …) |
| [`ARCHITECTURE-HYPOTHESES.md`](ARCHITECTURE-HYPOTHESES.md) | candidate architectures and mechanisms — all explicitly *not selected* |
| [`OPEN-QUESTIONS.md`](OPEN-QUESTIONS.md) | what is still unresolved |
| [`RESEARCH-AGENDA.md`](RESEARCH-AGENDA.md) | research phases and comparison criteria |
| [`research/`](research/) | evidence: sources, research artifacts, external briefs, frozen history |
| [`experiments/`](experiments/) | experiments and their results, including V0 |
| [`V0-IMPLEMENTATION-PLAN.md`](V0-IMPLEMENTATION-PLAN.md) | the V0 plan, invariants, criteria and closeout |
| [`DOCUMENTATION-ARCHITECTURE.md`](DOCUMENTATION-ARCHITECTURE.md) | how this repository itself is organized (proposed) |

### How to read claims here

This repository distinguishes, on purpose and in writing, between what is established and
what is not. The authority ladder, highest first — a lower level never overrides a higher
one:

1. An explicit current instruction from the project owner
2. A decision record — [`DECISIONS.md`](DECISIONS.md), `UD-nnn`
3. The founding charter (cited as `MC §n`; see [Naming and sources](#naming-and-sources))
4. Canonical knowledge — `PROJECT-DIRECTION`, `VISION`, `INVARIANTS`, `PRINCIPLES`,
   `REQUIREMENTS`, `CONSTRAINTS`
5. An experiment result — evidence, not a decision
6. A research artifact — attributed, unratified
7. A hypothesis or interpretation — `ARCHITECTURE-HYPOTHESES`, `OPEN-QUESTIONS`
8. Session history, agent memory, model inference

Rules that follow from it:

- **Repetition is not confirmation.** A claim appearing often was repeated, not validated.
- **Model output is never authority** — no matter which model produced it, including the
  agents that maintain this repository.
- **Research does not override the owner.** A recommendation stays a recommendation.
- **Conflicts are preserved**, never silently resolved.
- **Every claim cites a source**: `MC §n`, `UD-nnn`, `A-n`, `R-n`, `C-n`, `P-n`, `H-n`,
  `Q-n`, `T-n`, `EXP-n`. Unidentifiable origin is marked `UNKNOWN`.
- **Evidence is kept when it is unflattering.** Negative results, rejected hypotheses and
  superseded decisions stay in the record.

## Naming and sources

**Umbral** is the public name of the project. It was previously developed under the
working name **FSP** (*File System Pro*), and `fsp-check` — the frozen V0 prototype — keeps
that name. Historical records (`research/history/`, experiment logs) and dated research
artifacts retain the working name as written; they are not rewritten. This is the naming
note the rest of the repository refers to.

The founding charter is the project owner's own document. It is cited throughout as
`MC §n` and is **not part of the public edition of this repository**; the canonical
documents above carry its load-bearing content, each with its `MC §n` citation preserved.
Where a citation cannot be checked publicly, that is stated rather than hidden.

This repository's history was rewritten once, before its first public release, to remove a
document that is not part of the public edition. That is why no commit hash is cited
anywhere in this repository: the rewrite changed all of them, and a hash written down
afterwards would have been wrong. The development trail — what was done, in what order,
with what evidence — is preserved.

## How this project is developed

Umbral is developed by its owner together with AI agents, and the commit history says so:
the commits in this repository were authored by an agent identity, and the project has not
rewritten that history to look otherwise.

The project's own rules exist precisely because of this. Agent output — including the
output of the agents that wrote and maintain these documents — is **never authority**:
research is attributed and unratified, proposals stay proposals, and only the project owner
can record a decision. Where an agent's claim and a document here disagree, the document
wins; where neither is established, the answer is recorded as unknown.

## Contributing

Contributions are welcome once the repository is published. Read
[`CONTRIBUTING.md`](CONTRIBUTING.md) first — in particular, the rules that keep decisions,
hypotheses, evidence and experiments from being confused with one another.

## Security

See [`SECURITY.md`](SECURITY.md). Please do not open a public issue for a vulnerability.

## License

[Apache License 2.0](LICENSE). Dependency licenses are their own; nothing here claims
otherwise.

## Project status

This is an early research project, published for transparency rather than for use. It has
no release, no stability promise, and no support commitment. Interfaces, documents and
even names may change. The prototype under `fsp-check/` is experimental infrastructure:
it is kept as evidence of what was tested, not as a foundation to build on.

If you are looking for a product to install, there isn't one yet.
