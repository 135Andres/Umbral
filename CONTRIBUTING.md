# Contributing to Umbral

Umbral is an early research project, and **contributions are open** — from people and from
AI agents working alongside them.

The most valuable contribution right now is usually **not code**. It is a well-founded
argument, a counter-example, a source, or an experiment that contradicts something the
project currently assumes.

Before anything else, read [`README.md`](README.md) — especially
[How to read claims here](README.md#how-to-read-claims-here). It takes a few minutes and
prevents most of the confusion this project is designed to avoid.

---

## 1. How to orient yourself

| If you want to change… | Read first |
|---|---|
| a decision | [`DECISIONS.md`](docs/decisions/DECISIONS.md) — and see §3 below |
| an architecture idea | [`ARCHITECTURE-HYPOTHESES.md`](docs/candidates/ARCHITECTURE-HYPOTHESES.md), [`CONSTRAINTS.md`](docs/canonical/CONSTRAINTS.md), [`OPEN-QUESTIONS.md`](docs/candidates/OPEN-QUESTIONS.md) |
| a requirement or principle | [`REQUIREMENTS.md`](docs/canonical/REQUIREMENTS.md), [`PRINCIPLES.md`](docs/canonical/PRINCIPLES.md), [`INVARIANTS.md`](docs/canonical/INVARIANTS.md) |
| an experiment | the experiment's own directory under [`experiments/`](experiments) |
| the V0 prototype | [`V0-IMPLEMENTATION-PLAN.md`](docs/v0/V0-IMPLEMENTATION-PLAN.md) and [`experiments/v0-harness/V0-CLOSEOUT.md`](experiments/v0-harness/V0-CLOSEOUT.md) |
| this repository's structure | [`DOCUMENTATION-ARCHITECTURE.md`](docs/DOCUMENTATION-ARCHITECTURE.md) |
| where the project stands | [`PROJECT-DIRECTION.md`](docs/canonical/PROJECT-DIRECTION.md) |

**One fact has one home.** If a fact already exists somewhere, change it there and let
other documents link to it. Do not copy paragraphs between files.

## 2. The four categories — keep them separate

This is the single most important rule in the project.

| Category | What it is | Where it lives |
|---|---|---|
| **Decision** | a choice made by the project owner, with authority | [`DECISIONS.md`](docs/decisions/DECISIONS.md), `UD-nnn` — immutable |
| **Proposal** | a recommendation with no authority yet | research artifacts, or a "proposed" section of the relevant document |
| **Hypothesis** | a falsifiable idea, explicitly not selected | [`ARCHITECTURE-HYPOTHESES.md`](docs/candidates/ARCHITECTURE-HYPOTHESES.md), `Hn` |
| **Experiment / evidence** | a measured result | [`experiments/`](experiments), experiment logs, benchmark records |

A contribution that blurs these is harder to accept than one that disagrees with them.
If your change would promote a hypothesis into a decision, or present research as
validation, it will be sent back — not because the idea is wrong, but because the
project's record would become misleading.

Concretely, do not:

- add a decision record on your own authority — propose it instead, and say who must decide;
- describe a hypothesis as "the architecture" or "the design";
- present a research artifact or a model's output as evidence of correctness;
- delete or rewrite a superseded decision — supersede it with a pointer;
- edit a historical record to make it look better.

## 3. How to propose a change

1. **Open an issue first** for anything non-trivial: a decision, a hypothesis change, a
   new experiment, or a structural change to this repository. Use the
   [proposal template](https://github.com/135Andres/Umbral/issues/new/choose) and pick
   whether it is a feature, architectural or research proposal.
2. **State the evidence class** of what you are adding — observed, measured, reasoned,
   assumed, unknown. Say what would falsify it.
3. **Preserve conflicts.** If your change contradicts an existing document, say so
   explicitly. Do not resolve it silently.
4. **Keep it small.** One concern per pull request.

## 4. Reporting bugs

Use the [bug template](https://github.com/135Andres/Umbral/issues/new/choose). Include
what you observed, what you expected, how to reproduce it, and the commit you tested.
Synthetic fixtures only — never real personal data.

Security issues do **not** go in the issue tracker: see [`SECURITY.md`](SECURITY.md).

## 5. Proposing research

Research proposals are welcome and are a first-class contribution. A useful research
proposal states:

- the question, and why it matters for a decision the project has to make;
- what evidence would settle it, and what would falsify the current assumption;
- how it could be measured or gathered — the cheaper the better;
- which existing document would change as a result.

Open it as a research proposal in the issue tracker. If you then run it, the result goes
under [`experiments/`](experiments) with its method, result and limitations — including
negative results, which are kept.

## 6a. The development crate (`umbral/`)

`umbral/` holds the version-by-version development of Umbral. It is new code and does not
depend on `fsp-check/`.

```
cd umbral
cargo test
cargo clippy --all-targets
cargo fmt --check
```

It is a Cargo workspace member declared in the root `Cargo.toml`. `fsp-check/` is
deliberately **excluded** from that workspace so it keeps its own lockfile and target
directory — do not add it as a member.

Rules that apply to changes here:

- **One version at a time.** A version is defined in [`docs/versions/`](docs/versions/README.md)
  with its goal, non-goals and acceptance criteria. Work outside that scope is a separate
  proposal, not a pull request.
- **Assertions must not depend on filesystem behaviour.** The suite runs on tmpfs, btrfs and
  ext4. Where behaviour is environment-dependent (inode reuse is the known case), *record*
  what happened and put the deterministic assertion against observation sets built by hand.
  A test that only passes on the filesystem it was written on is the defect class this rule
  exists to prevent.
- **A test that cannot fail proves nothing.** When a check asserts a contract, show that the
  check rejects a violation of it.
- **Never adapt the implementation to make a test pass.** When the implementation and an
  independent model disagree, decide which of the two is wrong against reality and fix that
  one.
- **Documented limits stay documented.** A known limitation is not removed by making it
  quieter.

## 6b. The V0 prototype is frozen

`fsp-check/` is **closed and frozen**. It is kept as evidence of what was tested, not as
a foundation. New product code is not accepted before the project has an accepted
architecture.

What that means in practice:

- **Do not** add features to it, or extend it toward a product.
- **You may** change it to reproduce a result, to fix a defect in the harness or a test,
  or to add a failing case that demonstrates a real problem — say clearly in the pull
  request what the change demonstrates.
- A change to the prototype does **not** change Umbral's architecture, and must not be
  described as if it did. Its results are evidence, not decisions.

If you want to change the prototype's behaviour rather than its evidence, open a proposal
first.

## 7. Pull requests

A pull request is expected to state: what changed, why, the evidence or tests, the
documentation impact, whether a decision / invariant / principle / hypothesis changes, and
whether it introduces a new architectural commitment. The
[PR template](.github/PULL_REQUEST_TEMPLATE.md) asks for exactly that — nothing more.

Small, semantic commits are preferred over one large commit. Documentation-only changes
and code changes should not be mixed when they can be separated.

New commits follow `type(scope): concise imperative summary`, with the why in the body
when it is not obvious. Types: `feat`, `fix`, `docs`, `test`, `refactor`, `chore`,
`research`. Keep the subject specific — no "updates", "misc changes" or "cleanup" — and
include identifiers (`F-6`, `Q25`, `UD-nnn`) only when they add real traceability. No
emojis, no promotional language. Existing commits are left as they were written.

## 8. Running the prototype's tests

If you touch `fsp-check/`:

```sh
cd fsp-check
cargo fmt --check
cargo clippy --all-targets
cargo test
```

These same three commands run in CI (see
[`.github/workflows/fsp-check.yml`](.github/workflows/fsp-check.yml)).

## 9. Documentation style

Plain Markdown, readable without any tooling. Short paragraphs, small tables, relative
links, one canonical home per fact, stable IDs, explicit status lines. Avoid duplication,
generated indexes, and decoration that has to be maintained.

## 10. Conduct and license

- [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md) applies to all project spaces.
- By contributing you agree that your contribution is licensed under the
  [Apache License 2.0](LICENSE), the license of this repository.
