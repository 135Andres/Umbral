# AGENTS.md

Entry point for AI agents working in this repository. Deliberately short — it routes, it
does not explain. **Read [`docs/READING-THIS-REPOSITORY.md`](docs/READING-THIS-REPOSITORY.md)
first**; it holds the authority ladder, the rules for reading claims and the naming of sources,
and it is canonical for this repository. [`README.md`](README.md) is the short introduction for
newcomers.

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
- **Preserve the public/private boundary.** The owner keeps a private record outside this
  repository: deliberations, the reasoning behind decisions, working reports, full audits and
  product intent not yet decided. Some private files also sit, ignored by git, in this working
  tree (`research/sources/PROJECT-MASTER-CONTEXT.md`, `PUBLICATION-PROPOSAL.md`,
  `hermes-reports/`). Public documents cite private material as `MC §n` or
  `owner instruction (private, YYYY-MM-DD)` and never reproduce it.
  - **Public** is what a reader needs to interpret this repository: what binds the code, the
    licence and the documentation (decision records state *what* and *scope*, not the owner's
    reasoning); a definition for every identifier a public document cites; defects, evidence
    and negative results.
  - **Private** is everything else, until the owner publishes it. Promotion from private to
    public is the owner's decision and takes the form of a minimal extract, never a copy of
    working material.
  - **Quotations from the charter.** The short, labelled verbatim quotations already present
    in the canonical documents and in `DECISIONS.md` are authorized by the owner (2026-10-02).
    Any new quotation, copy or republication of non-public material needs the owner's explicit
    authorization.

## Process

The working process for this project — authority handling, research intake, experiment
discipline, hard boundaries — is documented in the `fsp-project-intelligence` skill of the
agent environment that maintains this repository. That skill is tooling; **this repository
is the record.**
