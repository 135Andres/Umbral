# umbral

The v0.1 workspace observation instrument, with v0.2 in development on branch `v0.2` (slice 1,
the output contract `umbral-output/1`, is in place).

## What this is

A local, read-only tool. You point it at a directory you own, it records what is there, and
later it tells you what is known, when it was known, and what changed — labelling every
statement as `observed`, `derived`, `ambiguous` or `unknown`.

The distinction is drawn at **who produced the value**, so a reader can tell from the output
alone what came from their filesystem and what the tool produced:

- **`observed`** — the filesystem reported it for an entry during this run: `canonical`, `kind`,
  `size`, `mtime`. Those four, and no others.
- **`derived`** — the tool produced it: content fingerprints, stability verdicts, run
  identifiers, the run's own timestamps, workspace identifiers, composed paths, counts, and
  configuration echoed back.
- **`ambiguous`** — a classification the evidence leaves open between more than one outcome
  (renamed, or recreated?), with the reason named. None is chosen.
- **`unknown`** — a value that is not determinable from the evidence available: not observed,
  not obtainable, not comparable. An error is a reason given on an `unknown` line, not a label
  of its own (`UD-031`).

The set of fields allowed on an `observed` line is a constant (`OBSERVED_FIELDS` in
`src/report.rs`) and is enforced by a test, so the distinction cannot decay by inattention.

### The output format: `umbral-output/1`

Every output begins with a header naming the contract edition it follows, and every value is
written by one escaping rule, specified in [`CONTRACT.md`](CONTRACT.md) (`UD-022`, `UD-030`):

```
derived   contract=umbral-output/1
observed  canonical=/tmp/ws-\xFF\xFE  canonical-encoding=escaped:not-valid-utf8
```

A path is a byte string; the output is text. What cannot stand in a line — a backslash (written
`\\`), bytes that are not UTF-8, control characters such as a line feed, spaces that would be
ambiguous, and characters that could disguise a name on a terminal — is written as `\xNN`. The
writing is reversible and has exactly one form per value; names in any script, with single
spaces, are written as themselves. When anything was escaped, the field is followed **on the same
line** by `<field>-encoding=escaped:<reasons>`, so a reader is never left taking the escaped form
for the name on disk, and never has to guess which value the note is about.

The library reads it back (`umbral::contract::parse`), refusing anything it cannot reconstruct
exactly — an unknown edition, a malformed or non-canonical escape, a truncated output — and
keeping fields it does not know. Standard error is not part of the contract.

### How each value was obtained

Every observation has two components: its **metadata** (kind, size, mtime, physical identity),
read for every entry, and — for a regular file only — its **content**, read and hashed. For each
component the output states how it was obtained, in a closed vocabulary (`UD-031`, `UD-033`):

| State | Meaning |
|---|---|
| `fresh` | obtained in this run; a value exists |
| `reused` | carried from an earlier observation, which is named; never content verification. Not emitted yet: it arrives with the skip (v0.2 slice 3) |
| `failed` | attempted, and no value was obtained — including a reading that kept changing |
| `not-attempted` | it is recorded that no attempt was made (a path whose metadata failed: its kind is unknown) |
| `not-recorded` | it is not recorded whether an attempt was made (a row written by an older build) |

`show` writes `metadata=` and `content=` on each observation's line; a directory, symlink or
special file has no `content=` because it has no content component. Every line of `show` names
the observation it reports, `observation=<run>:<path>` ([`CONTRACT.md`](CONTRACT.md) §6a), so no
line depends on the one above it.

`observe` and `status` count by state, on lines that name their run: `metadata-fresh`,
`metadata-failed`; `content-fresh`, `content-reused`, `content-failed`, `content-not-attempted`,
`content-not-recorded` (they sum to `files + kind-unknown`); and the failed content readings by
diagnostic (`unstable-observation`, `not-found`, `permission-denied`, `not-a-regular-file`,
`read-error`).

### What a verdict rests on

Each line of `changes` (and the comparison lines of `show`) states, besides the verdict and its
subject `path=` (`UD-034`):

- `reference=<run>:<path>` and `compared=<run>:<path>` — the observation the verdict relates on
  each side, each followed by `reference-fields=` / `compared-fields=`: the fields the rules
  consulted on it (`dev,ino,kind,size,mtime,hash`, a subset of them, or `none`);
- `reference-absent=<run>` / `compared-absent=<run>` instead, where the verdict rests on the
  path having no observation in that run (`created`, `deleted`, `unobserved`) — written only when
  that is so;
- `counterpart=<run>:<path>`, once per entry, with `counterpart-fields=` — other entries the
  verdict rests on: every surviving hard link of a `deleted` entry, every other member of a group
  of conflicting candidates;
- `reference-complete=` and `compared-complete=` — how complete each run was. A verdict is
  relative to both.

The header line names the two runs: `compared  reference-run=…  compared-run=…`.

## What this is NOT

- **Not the product.** This is development code for one version. Nothing here is a selected
  design, and no interface decision is made by it.
- **Not V1.** V1 is reserved for a version that has passed an external audit.
- **Not a foundation to build on.** Each version is re-evaluated after it closes.
- **Not `fsp-check`.** That is the frozen V0 experiment, a separate crate in the same
  repository. This crate does not import, depend on, or modify it. V0 is cited here as
  *evidence*, never as a dependency.

## Deliberately absent

Each absence is a decision, not an oversight: watcher, daemon, FTS, embeddings, API, UI,
MCP, AI of any kind, sync, multi-device, multi-writer, permissions, semantic layer,
Project Reality.

## Commands

```
umbral init <root>            create the workspace record for <root>
umbral observe <root>         record one observation run over <root>
umbral status <root>          what is known now                      (read-only)
umbral changes <root>         what changed between the last two runs (read-only)
umbral show <root> <path>     the history of one path                (read-only)
umbral workspaces             which workspaces exist on this machine (read-only)
umbral check <root>           verify the log: references, stored values, no derived state (read-only)
```

Exit codes: `0` success (including `ambiguous` and "no results" — they are results, not
errors), `1` runtime error, `2` usage error, `3` `observe` recorded a run that was
incomplete because some paths could not be observed.

## Where state lives

`$XDG_DATA_HOME/umbral/ws-<id>/`, falling back to `~/.local/share/umbral/ws-<id>/`.

**Nothing is ever written inside the root you point it at** — not even a marker file to
identify it. That restriction is why the identifier is derived from the canonical path
instead:

1. the root is canonicalised (absolute, symlinks resolved);
2. the canonical path's **raw OS bytes** are taken (no lossy UTF-8 conversion);
3. BLAKE3 is computed over exactly those bytes;
4. the first 8 bytes, as 16 hex characters, name the directory.

Consequences, stated plainly: the same canonical root always produces the same identifier;
moving or renaming the root produces a *different* workspace with an empty history, because
there is no marker to follow.

This is v0.1's mechanism for locating a workspace's local record. It is **not** the
definitive global identity of a workspace in Umbral, and it is replaceable.

The state directory contains paths and content fingerprints of the observed files. It is
local, nothing is transmitted, and **no file content is ever stored** — only hashes.

## The persistence seam

Persistence goes through a small trait (`umbral::log::ObservationLog`). The v0.1
implementation is SQLite. The trait exists so that an open question stays open: whether
SQLite alone is the right substrate, or whether an external append-only log is also needed
(**Q25** in the repository's open questions). v0.1 does not answer it, and the SQLite choice
here is not a statement about Umbral's persistence model.

The log stores observations and the runs that produced them. It stores **no derived state**:
there is no projection table, no cached current state, no stored reconciliation result.
Everything derived is recomputed on read, so nothing can drift out of agreement with the log.
`umbral check` verifies the structural half of that claim — the stored tables are the log and
nothing else — together with referential integrity and that every stored value is one this
build can interpret. Each of its verifications is shown failing by a test that corrupts a log on
purpose; verifications that could not fail were removed (D-V01-11).

## Building and testing

```
cargo test                 # from umbral/
cargo clippy --all-targets
cargo fmt --check
cargo run --release --example bench   # baseline measurements; evidence, not thresholds
```

The suite is run on tmpfs and btrfs locally, and on ext4 by CI. Assertions must not depend
on filesystem behaviour: where behaviour is environment-dependent (inode reuse), the test
*records* what happened and the deterministic assertion lives elsewhere, against observation
sets built by hand. See `../docs/versions/v0.1.md`.

## Security boundary

No writes to the observed tree; no network code and no sockets; no AI; no daemon; no
watcher; no telemetry; no credential access; no file content stored. Every effect is the
result of a command the user typed.
