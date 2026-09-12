# AGENTS.md

Entry point for AI agents working in this repository. Deliberately short — it routes, it
does not explain. **Read [`README.md`](README.md) first**; it holds the orientation, the
authority ladder and the rules for reading claims, and it is canonical for this repository.

This repository is the project's durable memory. Agent memory, session history and skills
are operational tooling only. If they disagree with the files here, the files win — and a
fact that exists only inside an agent does not exist.

## Authority, in one line

An explicit instruction from the project owner > a decision record (`DECISIONS.md`,
`UD-nnn`) > the canonical documents > experiment results > research artifacts >
hypotheses > your own inference. **Never promote a lower level into a higher one.**

## Read before acting

| Need | File |
|---|---|
| where the project stands | [`PROJECT-DIRECTION.md`](docs/canonical/PROJECT-DIRECTION.md) |
| what has been committed to | [`DECISIONS.md`](docs/decisions/DECISIONS.md) |
| what must stay true | [`INVARIANTS.md`](docs/canonical/INVARIANTS.md) · [`PRINCIPLES.md`](docs/canonical/PRINCIPLES.md) · [`CONSTRAINTS.md`](docs/canonical/CONSTRAINTS.md) |
| what is only a candidate | [`ARCHITECTURE-HYPOTHESES.md`](docs/candidates/ARCHITECTURE-HYPOTHESES.md) — **nothing here is selected** |
| what is unresolved | [`OPEN-QUESTIONS.md`](docs/candidates/OPEN-QUESTIONS.md) |
| what was measured | [`experiments/`](experiments) · [`experiments/v0-harness/V0-CLOSEOUT.md`](experiments/v0-harness/V0-CLOSEOUT.md) |
| how this repository is organized | [`DOCUMENTATION-ARCHITECTURE.md`](docs/DOCUMENTATION-ARCHITECTURE.md) — before creating any file, apply its routing rule (§10) |
| how to contribute | [`CONTRIBUTING.md`](CONTRIBUTING.md) |

## Rules

- **Do not invent.** No fabricated files, measurements, decisions, requirements or
  architecture. If something is unknown, write `UNKNOWN`; if suspected, write it as a
  hypothesis; if disputed, preserve the dispute.
- **Do not decide.** Decisions belong to the project owner. Record, research, question,
  propose, verify — never ratify.
- **Classify every claim** by its source (`MC §n`, `UD-nnn`, `Hn`, `Qn`, experiment,
  external source) and keep observed / recorded / derived / inferred / assumed / unknown
  distinct.
- **One home per fact.** Before adding a file, check whether the fact already has one.
- **Never rewrite history.** Decision records and frozen records in
  [`research/history/`](research/history) are immutable; supersede with a pointer.
- **No architecture selection**, no schema, no protocol, no technology choice presented as
  settled. The V0 prototype's stack is scoped, provisional and reversible
  (`UD-012`, `UD-013`), and **V0 is not Umbral's architecture**.
- **Preserve the boundary.** `research/sources/` may contain material that is not part of
  the public edition of this repository; see the root README, "Naming and sources". Do not
  quote, copy or republish anything from a non-public source into a public document.

## Process

The working process for this project — authority handling, research intake, experiment
discipline, hard boundaries — is documented in the `fsp-project-intelligence` skill of the
agent environment that maintains this repository. That skill is tooling; **this repository
is the record.**
