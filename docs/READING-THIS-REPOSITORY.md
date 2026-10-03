# Reading this repository

Status: CURRENT — the epistemic contract of this repository: how its claims are ranked, how its
sources are named, and how it is developed. Moved here from the root README on 2026-10-03, when
the README became a short introduction for newcomers; the text is unchanged.

This page is for anyone who wants to rely on what the repository says — contributors,
researchers, reviewers and AI agents. For a first look at the project, the
[README](../README.md) is enough.

## How to read claims here

This repository distinguishes, on purpose and in writing, between what is established and
what is not. The authority ladder, highest first — a lower level never overrides a higher
one:

1. An explicit current instruction from the project owner
2. A decision record — [`DECISIONS.md`](decisions/DECISIONS.md), `UD-nnn`
3. The founding charter (cited as `MC §n`; see [Naming and sources](#naming-and-sources))
4. Canonical knowledge — `PROJECT-DIRECTION`, `VISION`, `INVARIANTS`, `PRINCIPLES`,
   `REQUIREMENTS`, `CONSTRAINTS`
5. An experiment result — evidence, not a decision
6. A research artifact — attributed, unratified
7. A hypothesis or interpretation — `ARCHITECTURE-HYPOTHESES`, `OPEN-QUESTIONS`
8. Session history, agent memory, model inference

Rules that follow from it:

- **Model output is never authority** — no matter which model produced it, including the
  agents that maintain this repository.
- **Research does not override the owner.** A recommendation stays a recommendation.
- **Conflicts are preserved**, never silently resolved.
- **Every claim cites a source** — an ID from one of the project's namespaces, or an
  explicit `UNKNOWN` where the origin cannot be identified. Which namespaces exist, and
  which document owns each one, are recorded in one place:
  [`docs/IDENTIFIERS.md`](IDENTIFIERS.md).
- **Evidence is kept when it is unflattering.** Negative results, rejected hypotheses and
  superseded decisions stay in the record.

## Naming and sources

**Umbral** is the public name of the project. It was previously developed under the
working name **FSP** (*File System Pro*), and `fsp-check` — the frozen V0 prototype — keeps
that name. Historical records (`research/history/`, experiment logs) and dated research
artifacts retain the working name as written; they are not rewritten.

The founding charter is the project owner's own document. It is cited throughout as
`MC §n` and is **not part of the public edition of this repository**; the canonical
documents in [`docs/`](README.md) carry its load-bearing content, each with its
`MC §n` citation preserved. Where a citation cannot be checked publicly, that is stated
rather than hidden. The same applies to the owner's reasoning behind some decisions: a record
cites it as `owner instruction (private, date)` and states what was decided and its scope, not
why.

This repository's history was rewritten once, before its first public release, to remove a
document that is not part of the public edition. That is why no commit hash from before
that rewrite is cited anywhere in this repository: the rewrite changed all of them. A few
records written afterwards do cite later commits. On 2026-10-03 the history was rewritten a
second time, only to change the owner's author name from `Andres` to `135Andres`; that changed
those hashes too, and each old one resolves to its new one in
[`research/history/COMMIT-REWRITE-2026-10-03.md`](../research/history/COMMIT-REWRITE-2026-10-03.md).
The development trail — what was done, in what order, with what evidence — is preserved.

## How this project is developed

Umbral is developed by its owner together with AI agents. The commit history carries two
identities: the earlier commits were authored by an agent identity (`Hermes`), and the later
ones by the owner's identity (`135Andres`), under which agent-assisted work is also committed.
The project has not rewritten that history to look otherwise; the 2026-10-03 rewrite changed only
the owner's name.

The project's own rules exist precisely because of this. Agent output — including the
output of the agents that wrote and maintain these documents — is **never authority**:
research is attributed and unratified, proposals stay proposals, and only the project owner
can record a decision. Where an agent's claim and a document here disagree, the document
wins; where neither is established, the answer is recorded as unknown.
