# Contributing to Umbral

Umbral is an early research project. The most valuable contribution right now is usually
**not code** — it is a well-founded argument, a counter-example, or evidence that
contradicts something the project currently assumes.

Before anything else, read [`README.md`](README.md) — especially
[How to read claims here](README.md#how-to-read-claims-here). It takes a few minutes and
prevents most of the confusion this project is designed to avoid.

---

## 1. Where to read before you change anything

| If you want to change… | Read first |
|---|---|
| a decision | [`DECISIONS.md`](DECISIONS.md) — and see §3 below |
| an architecture idea | [`ARCHITECTURE-HYPOTHESES.md`](ARCHITECTURE-HYPOTHESES.md), [`CONSTRAINTS.md`](CONSTRAINTS.md), [`OPEN-QUESTIONS.md`](OPEN-QUESTIONS.md) |
| a requirement or principle | [`REQUIREMENTS.md`](REQUIREMENTS.md), [`PRINCIPLES.md`](PRINCIPLES.md), [`INVARIANTS.md`](INVARIANTS.md) |
| an experiment | the experiment's own directory under [`experiments/`](experiments/) |
| the V0 prototype | [`V0-IMPLEMENTATION-PLAN.md`](V0-IMPLEMENTATION-PLAN.md) and [`experiments/v0-harness/V0-CLOSEOUT.md`](experiments/v0-harness/V0-CLOSEOUT.md) |
| this repository's structure | [`DOCUMENTATION-ARCHITECTURE.md`](DOCUMENTATION-ARCHITECTURE.md) |

**One fact has one home.** If a fact already exists somewhere, change it there and let
other documents link to it. Do not copy paragraphs between files.

## 2. The four categories — keep them separate

This is the single most important rule in the project.

| Category | What it is | Where it lives |
|---|---|---|
| **Decision** | a choice made by the project owner, with authority | `DECISIONS.md`, `UD-nnn` — immutable |
| **Proposal** | a recommendation with no authority yet | research artifacts, or a "proposed" section of the relevant document |
| **Hypothesis** | a falsifiable idea, explicitly not selected | `ARCHITECTURE-HYPOTHESES.md`, `Hn` |
| **Experiment / evidence** | a measured result | `experiments/`, experiment logs, benchmark records |

A contribution that blurs these is harder to accept than one that disagrees with them.
If your change would promote a hypothesis into a decision, or present research as
validation, it will be sent back — not because the idea is wrong, but because the
project's record would become misleading.

Concretely, do not:

- add a decision record on your own authority (propose it instead, and say who must decide);
- describe a hypothesis as "the architecture" or "the design";
- present a research artifact or a model's output as evidence of correctness;
- delete or rewrite a superseded decision — supersede it with a pointer;
- edit a historical record to make it look better.

## 3. Proposing a change

1. **Open an issue first** for anything non-trivial: a decision, a hypothesis change, a new
   experiment, or a structural change to this repository.
2. **State the evidence class** of what you are adding — observed, measured, reasoned,
   assumed, unknown. Say what would falsify it.
3. **Preserve conflicts.** If your change contradicts an existing document, say so
   explicitly in the issue or pull request. Do not resolve it silently.
4. **Keep it small.** One concern per pull request.

## 4. Pull requests

A pull request is expected to state:

- **what changed** and in which files;
- **why** it changed;
- **evidence** — a link, a measurement, a reproducible command, or an explicit "no
  evidence, this is reasoning";
- **documentation impact** — which canonical document now needs to agree;
- **whether it changes a decision, a hypothesis or an invariant** — and if so, on whose
  authority;
- **how it was verified**.

Small, semantic commits are preferred over one large commit. Documentation-only changes
and code changes should not be mixed when they can be separated.

## 5. New evidence

If you produce a measurement, an experiment or a survey:

- put it under [`experiments/`](experiments/) (or `research/` for literature and sources);
- state the question, method, what was measured, the result, and the limitations;
- record negative and inconclusive results too — they are kept;
- say what it would take to falsify it;
- do not edit an existing experiment's results; add a new record that supersedes it.

## 6. Code

There is one prototype, `fsp-check/`, and it is **frozen**. It is kept as evidence of what
was tested, not as a foundation. New product code is not accepted before the project has
an accepted architecture.

If you change the prototype anyway (for example to reproduce a result), run:

```sh
cd fsp-check
cargo fmt --check
cargo clippy --all-targets
cargo test
```

and state in the pull request what the change demonstrates.

## 7. Documentation style

Plain Markdown, readable without any tooling. Short paragraphs, small tables, relative
links, one canonical home per fact, stable IDs, explicit status lines. Avoid duplication,
generated indexes, and decoration that has to be maintained.

## 8. Conduct and security

- [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md) applies to all project spaces.
- Report vulnerabilities through the channel in [`SECURITY.md`](SECURITY.md) — never in a
  public issue.

## 9. License

By contributing you agree that your contribution is licensed under the
[Apache License 2.0](LICENSE), the license of this repository.
