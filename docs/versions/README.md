# VERSION RECORDS

Status: CURRENT (living index) — version records (DT11, added 2026-09-12).

One line per version. The record itself is the release note; there is no separate release-note
tree, and a version's known limitations live in that version's file rather than in a
cross-cutting document that would decay.

| Version | Goal | Status | Record |
|---|---|---|---|
| v0.1 | A person can point the tool at a directory they own, record what is there, and read back what is known, when it was known, and what changed. | Evidence complete except the formal reader protocol; **not yet declared complete** | [`v0.1.md`](v0.1.md) |

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
