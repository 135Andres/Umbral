# VERSION RECORDS

Status: CURRENT (living index) — version records (DT11, added 2026-09-12).

One line per version. The record itself is the release note; there is no separate release-note
tree, and a version's known limitations live in that version's file rather than in a
cross-cutting document that would decay.

| Version | Goal | Status | Record |
|---|---|---|---|
| v0.1 | A person can point the tool at a directory they own, record what is there, and read back what is known, when it was known, and what changed. | Reader protocol executed; **A1 not satisfied**; one finding corrected, one fixed by amending the protocol; a second run is pending. **Not declared complete** | [`v0.1.md`](v0.1.md) |
| v0.2 | Reduce content reading and hashing during re-observation to O(changes), within the evidence and historical-contract bounds of `UD-021` and `UD-023`. | **Evidence complete — declared by the owner 2026-10-03** (`UD-038`); A2-V13 (independent human reader) not satisfied | [`v0.2.md`](v0.2.md) |

## What "status" means here

- **Evidence complete** — every criterion in the version's own record has been verified and
  the evidence is registered. This is a statement about evidence, not a declaration that the
  version is closed.
- **Declared complete** — the project owner has accepted the version's completion. Only the
  owner can do this.

## Naming

Two names that look alike and are not:

- **V0** — the frozen experiment `fsp-check/`. Closed 2026-09-11 at status PARTIAL. Never
  modified again; see [`experiments/v0-harness/V0-CLOSEOUT.md`](../../experiments/v0-harness/V0-CLOSEOUT.md).
- **v0.1, v0.2, …** — progressive development of Umbral in new code, starting 2026-09-12.

The repository writes the first as "V0 (fsp-check, frozen)" and the second as "v0.1, v0.2, …"
to keep them apart. V0 is evidence for v0.x; it is not a dependency of it.

## Before the versions: V0, the first experiment

*(Moved here from the root README on 2026-10-03; text unchanged.)*

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

Full evidence: [`experiments/v0-harness/V0-CLOSEOUT.md`](../../experiments/v0-harness/V0-CLOSEOUT.md)
(scope statement, criteria, limitations, open questions, V1 handoff).

## What comes next is not planned here

The next version is decided after each version closes, on the evidence that exists then. No
sequence of future versions is committed, and no version is a feature list. See
[`docs/candidates/ARCHITECTURE-HYPOTHESES.md`](../candidates/ARCHITECTURE-HYPOTHESES.md) for
what remains explicitly unselected.

## Audit

V1 is reserved for a version that has passed a separate **AUDIT READINESS REVIEW** and then
an external audit, whose findings have been resolved or explicitly accepted. No version is
currently designated as the audit candidate; that decision is made when a version is
sufficiently complete, not in advance.
