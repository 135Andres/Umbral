# Umbral

**Umbral is a research project about one question: when humans and AI systems work on the
same files, how does anyone — human or artificial — tell what is actually known about the
project, and what is only inference?**

> **Research project.** No architecture selected. No product. What exists today: a frozen
> proof-of-concept ([`fsp-check/`](fsp-check/)) and a small working command-line
> instrument ([`umbral/`](umbral/)). V1 has not started and is not authorized.

## The problem

Anyone joining a project late — a human collaborator, or an AI agent — can read the
current state of the files. What the files alone do not say is how things got this way:
what changed, when, whether a claim rests on actually reading the bytes or only on
metadata, and what is simply not known.

That distinction matters more as AI takes part in more of the work. A reader that cannot
tell "I read this file" apart from "this file looks unchanged" will present guesses with
the confidence of evidence. Humans do it too, when they are guessing in good faith.

## The idea

Keep the files where they are, and keep a record of observations that is honest about its
own evidence. Umbral points at a directory you own and:

- records what existed and what changed between observations, in an append-only local log;
- labels every statement by where it came from — **`observed`** (the filesystem reported
  it), **`derived`** (the tool computed it), **`ambiguous`** (the evidence permits more
  than one reading, with the reason named) and **`unknown`** (it does not have the
  information);
- never writes inside the directory it observes, never stores file content — only content
  fingerprints — and has no network, daemon or watcher.

The labels are not philosophy. They are what let a *later* reader — including an AI that
cannot ask you questions — tell evidence apart from inference.

## A real session

Run against a small demo directory (`docs/`, `research/`, three files). Between the two
observation runs, `docs/api.md` was edited, `docs/faq.md` was added and
`research/latency-notes.md` was removed:

```
$ umbral init /tmp/umbral-demo
derived   contract=umbral-output/1
observed  canonical=/tmp/umbral-demo
derived   root=/tmp/umbral-demo
derived   workspace-id=b8e1aa4d4ff5946a
derived   state-dir=/tmp/umbral-state/umbral/ws-b8e1aa4d4ff5946a
derived   initialised=true

$ umbral observe /tmp/umbral-demo
derived   contract=umbral-output/1
observed  canonical=/tmp/umbral-demo
derived   run=1  root=/tmp/umbral-demo
derived   run=1  entries=5  files=3  dirs=2  symlinks=0  other=0  kind-unknown=0
derived   run=1  metadata-fresh=5  metadata-failed=0
derived   run=1  content-fresh=3  content-reused=0  content-failed=0  content-not-attempted=0  content-not-recorded=0
derived   run=1  content-failed-diagnostics  unstable-observation=0  not-found=0  permission-denied=0  not-a-regular-file=0  read-error=0
derived   run=1  traversal-complete=true  traversal-not-descended=0  traversal-metadata-failed=0  traversal-not-recorded=0  root-not-descended=false
derived   run=1  tool-version=0.1.0  scope=recursive,symlinks-not-followed,no-exclusions
derived   run=1  content-read-entries=3  content-read-bytes=82
derived   run=1  started=2026-10-03T07:19:41.928Z  finished=2026-10-03T07:19:41.928Z
derived   run=1  complete=true

# ... api.md edited, faq.md added, latency-notes.md removed ...

$ umbral observe /tmp/umbral-demo
derived   contract=umbral-output/1
observed  canonical=/tmp/umbral-demo
derived   run=2  root=/tmp/umbral-demo
derived   run=2  entries=5  files=3  dirs=2  symlinks=0  other=0  kind-unknown=0
derived   run=2  metadata-fresh=5  metadata-failed=0
derived   run=2  content-fresh=2  content-reused=1  content-failed=0  content-not-attempted=0  content-not-recorded=0
derived   run=2  content-failed-diagnostics  unstable-observation=0  not-found=0  permission-denied=0  not-a-regular-file=0  read-error=0
derived   run=2  traversal-complete=true  traversal-not-descended=0  traversal-metadata-failed=0  traversal-not-recorded=0  root-not-descended=false
derived   run=2  tool-version=0.1.0  scope=recursive,symlinks-not-followed,no-exclusions
derived   run=2  content-read-entries=2  content-read-bytes=53
derived   run=2  started=2026-10-03T07:19:41.934Z  finished=2026-10-03T07:19:41.934Z
derived   run=2  complete=true

$ umbral changes /tmp/umbral-demo
derived   contract=umbral-output/1
derived   compared  reference-run=1  compared-run=2  reference-complete=true  compared-complete=true
derived   count  unchanged=3
derived   count  modified=1
derived   count  created=1
derived   count  deleted=1
derived   count  unobserved=0
derived   count  renamed-or-moved=0
derived   count  recreated=0
derived   count  ambiguous=0
derived   modified  path=docs/api.md  reference=1:docs/api.md  reference-fields=dev,ino,kind,size,mtime,hash  compared=2:docs/api.md  compared-fields=dev,ino,kind,size,mtime,hash  content-changed=true  reference-complete=true  compared-complete=true
derived   created  path=docs/faq.md  reference-absent=1  compared=2:docs/faq.md  compared-fields=dev,ino  reference-complete=true  compared-complete=true
derived   deleted  path=research/latency-notes.md  reference=1:research/latency-notes.md  reference-fields=dev,ino  compared-absent=2  object-survives=false  reference-complete=true  compared-complete=true

$ umbral status /tmp/umbral-demo
derived   contract=umbral-output/1
observed  canonical=/tmp/umbral-demo
derived   root=/tmp/umbral-demo
derived   workspace-id=b8e1aa4d4ff5946a
derived   last-run=2  started=2026-10-03T07:19:41.934Z
derived   last-run=2  complete=true
derived   run=2  entries=5  files=3  dirs=2  symlinks=0  other=0  kind-unknown=0
derived   run=2  metadata-fresh=5  metadata-failed=0
derived   run=2  content-fresh=2  content-reused=1  content-failed=0  content-not-attempted=0  content-not-recorded=0
derived   run=2  content-failed-diagnostics  unstable-observation=0  not-found=0  permission-denied=0  not-a-regular-file=0  read-error=0
derived   run=2  traversal-complete=true  traversal-not-descended=0  traversal-metadata-failed=0  traversal-not-recorded=0  root-not-descended=false
derived   run=2  tool-version=0.1.0  scope=recursive,symlinks-not-followed,no-exclusions
derived   log-runs=2  log-observations=10

$ umbral show /tmp/umbral-demo docs/overview.md
derived   contract=umbral-output/1
derived   observation=1:docs/overview.md  hash=139e3fda7011  stability=stable  metadata=fresh  content=fresh
observed  observation=1:docs/overview.md  kind=file  size=30  mtime=2026-10-03T07:19:41.921Z  ctime=2026-10-03T07:19:41.921Z
derived   observation=2:docs/overview.md  hash=139e3fda7011  stability=stable  metadata=fresh  content=reused  content-source=1:docs/overview.md
observed  observation=2:docs/overview.md  kind=file  size=30  mtime=2026-10-03T07:19:41.921Z  ctime=2026-10-03T07:19:41.921Z
derived   unchanged  path=docs/overview.md  reference=1:docs/overview.md  reference-fields=dev,ino,kind,size,mtime,hash  compared=2:docs/overview.md  compared-fields=dev,ino,kind,size,mtime,hash  content-changed=false  reference-complete=true  compared-complete=true
```

The second `observe` read only what could have changed — the edited file and the new one,
`content-read-entries=2` — and carried the third file's reading forward: `show` reports it as
`content=reused` and names the observation that actually read its bytes
(`content-source=1:docs/overview.md`). A reused reading is never presented as verified in that
run.

Every output begins with the edition of the output contract it follows
(`contract=umbral-output/1`, specified in [`umbral/CONTRACT.md`](umbral/CONTRACT.md)), so a
saved or piped output can be read correctly later without anything else.

Every verdict says what it rests on: the observation it relates on each side
(`reference=1:docs/api.md`, `compared=2:docs/api.md` — a run and a path), the fields the rules
consulted on it, and how complete each of the two runs was. Even `deleted` is stated carefully:
it is a claim about the comparison between two runs — `compared-absent=2` says the path had no
observation in run 2, and `object-survives=false` that its object was not seen elsewhere — not a
claim about physical deletion. In the same way, `created` rests on the path's absence from a
reference run whose completeness it states: a path missing from an incomplete run may have
existed unseen. Where the evidence would permit several readings, the output says so instead of
choosing silently.

## What exists today

| | |
|---|---|
| Product | **does not exist.** There is no usable application to install, no UI, no AI integration |
| Architecture | **not selected.** No database, protocol, versioning engine or semantic model has been chosen |
| V0 | **frozen proof-of-concept**, status `PARTIAL` — see below |
| v0.1 | **working observation instrument** — [`umbral/`](umbral/), the CLI shown above. Not declared complete; its independent-human reader criterion (A1) is unsatisfied |
| Development | **v0.2 is active** on branch `v0.2`, currently at contract integration; no production code yet |
| V1 | not started and not authorized |

Two names that look alike and are not. **V0** is the frozen experiment `fsp-check/`.
**v0.1** and the active **v0.2** are development versions in new code that does not depend
on it. V0 is evidence for v0.x; it is never a dependency of it. And neither `fsp-check/`
nor `umbral/` is the product: both are instruments and evidence on the way to a design
that has not been chosen.

## V0 — the first experiment

`fsp-check/` is a small Rust prototype that asked one narrow question:

> Can a filesystem be observed, identified, content-verified, reconciled and persisted
> deterministically and safely — **without pretending to know what the files mean?**

- **Tested:** 68 tests; 512 generated property-test cases checked against an independent
  oracle; real process-kill crash trials; benchmarks recorded as evidence. One test
  originally encoded an assumption about the filesystem (that a freed inode is never
  reused) and failed on ext4 — found by the CI runner, corrected in the test, and recorded
  as finding F-6 rather than quietly fixed.
- **Result:** no implementation failure was produced inside the tested scope. The findings
  were in the harness, the oracle, or the tests — never in the prototype. They are
  enumerated, with their class, in the falsification report.
- **Frozen state:** `PARTIAL`. The only gap is a reader-facing surface that would let a
  new reader reconstruct what the prototype recorded and when — deliberately outside V0's
  mandate.

> **V0 evidence ≠ Umbral architecture.** `fsp-check` is an experiment with a result. It is
> not "the Umbral engine", it is not the product's design, and nothing in it is a
> commitment about how Umbral will be built.

Full evidence: [`experiments/v0-harness/V0-CLOSEOUT.md`](experiments/v0-harness/V0-CLOSEOUT.md)
(scope statement, criteria, limitations, open questions, V1 handoff).

## Where Umbral is going

Everything above is what exists. The rest is what the project is *for* — recorded as
intent and research, not as working software.

Umbral is intended as an open-source, local-first, AI-native project environment whose
human mental model is *"my files and folders"*. Projects, tasks, decisions, knowledge and
AI capabilities are meant to live as an **optional, derived, rebuildable overlay** over a
filesystem the user already owns — not as a mandated taxonomy, and not inside a
proprietary database. The files stay the user's. The overlay is a projection that can be
discarded and rebuilt.

The research direction currently being worked towards ([`UD-011`](docs/decisions/DECISIONS.md)):
what must Umbral provide so that a human and several AI systems — from different vendors,
some not yet existing — can safely, coherently and continuously share one filesystem,
without the human reorganizing the project around any one AI's assumptions.

Four commitments define the shape of that goal:

- **Filesystem sovereign.** Files are the source of truth; any index is a disposable
  projection.
- **No mandatory taxonomy.** Existing, messy, arbitrary structure must remain workable.
- **Intelligence as a participant, not a feature.** Umbral's job is to make the shared
  environment legible to its participants, not to think on the user's behalf.
- **User authority.** The project records; it does not decide what is true. Its tools
  observe, derive, and report — including reporting that something is unknown, ambiguous,
  or disputed.

None of this is built. What it requires, what could falsify it, and what has been tested
so far are tracked in the documentation below.

## Documentation

The project's memory is its documentation: what it intends, what must stay true, what is
required, what constrains it, what is hypothesised, what is unknown, and what has been
decided. Each file has one role; nothing duplicates another.

| Where | What |
|---|---|
| [`docs/`](docs/README.md) | the working documentation, grouped by function — with a map of its own |
| [`docs/canonical/`](docs/canonical/PROJECT-DIRECTION.md) | where the project stands; vision, principles, invariants, requirements, constraints |
| [`docs/decisions/DECISIONS.md`](docs/decisions/DECISIONS.md) | **the only place commitments live** (`UD-nnn`) |
| [`docs/candidates/`](docs/candidates/ARCHITECTURE-HYPOTHESES.md) | candidate architectures, open questions, research agenda — all explicitly *not selected* |
| [`docs/v0/`](docs/v0/V0-IMPLEMENTATION-PLAN.md) | the V0 plan, invariants and criteria |
| [`docs/versions/`](docs/versions/README.md) | what each development version did, with its evidence and its known limitations |
| [`research/`](research/) | evidence: research artifacts, external briefs, frozen history |
| [`experiments/`](experiments/) | experiments and their results, including V0 |

### How to read claims here

This repository distinguishes, on purpose and in writing, between what is established and
what is not. The authority ladder, highest first — a lower level never overrides a higher
one:

1. An explicit current instruction from the project owner
2. A decision record — [`DECISIONS.md`](docs/decisions/DECISIONS.md), `UD-nnn`
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
  [`docs/IDENTIFIERS.md`](docs/IDENTIFIERS.md).
- **Evidence is kept when it is unflattering.** Negative results, rejected hypotheses and
  superseded decisions stay in the record.

## Naming and sources

**Umbral** is the public name of the project. It was previously developed under the
working name **FSP** (*File System Pro*), and `fsp-check` — the frozen V0 prototype — keeps
that name. Historical records (`research/history/`, experiment logs) and dated research
artifacts retain the working name as written; they are not rewritten.

The founding charter is the project owner's own document. It is cited throughout as
`MC §n` and is **not part of the public edition of this repository**; the canonical
documents in [`docs/`](docs/README.md) carry its load-bearing content, each with its
`MC §n` citation preserved. Where a citation cannot be checked publicly, that is stated
rather than hidden. The same applies to the owner's reasoning behind some decisions: a record
cites it as `owner instruction (private, date)` and states what was decided and its scope, not
why.

This repository's history was rewritten once, before its first public release, to remove a
document that is not part of the public edition. That is why no commit hash from before
that rewrite is cited anywhere in this repository: the rewrite changed all of them. A few
records written afterwards do cite later commits; those hashes are real, but a hash on a
development branch only stays valid if that branch is merged without rewriting it. The development trail — what was done, in what order,
with what evidence — is preserved.

## How this project is developed

Umbral is developed by its owner together with AI agents. The commit history carries two
identities: the earlier commits were authored by an agent identity (`Hermes`), and the later
ones by the owner's identity, under which agent-assisted work is also committed. The project
has not rewritten that history to look otherwise.

The project's own rules exist precisely because of this. Agent output — including the
output of the agents that wrote and maintain these documents — is **never authority**:
research is attributed and unratified, proposals stay proposals, and only the project owner
can record a decision. Where an agent's claim and a document here disagree, the document
wins; where neither is established, the answer is recorded as unknown.

## Contributing

Contributions are welcome. Read [`CONTRIBUTING.md`](CONTRIBUTING.md) first — in
particular, the rules that keep decisions, hypotheses, evidence and experiments from being
confused with one another.

## Security

See [`SECURITY.md`](SECURITY.md). Please do not open a public issue for a vulnerability.

## License

[GNU General Public License v3.0 or later](LICENSE) (`GPL-3.0-or-later`), recorded as
[`UD-028`](docs/decisions/DECISIONS.md). A work based on Umbral that is distributed must stay open
source under the same terms. Earlier copies obtained under the Apache License 2.0 keep that
licence. Dependency licenses are their own; nothing here claims otherwise.

## Project status

This is an early research project, published for transparency rather than for use. It has
no release, no stability promise, and no support commitment. Interfaces, documents and
even names may change. The prototype under `fsp-check/` is experimental infrastructure:
it is kept as evidence of what was tested, not as a foundation to build on. The instrument
under `umbral/` is development code for one version, subject to the same rule.

If you are looking for a product to install, there isn't one yet.
