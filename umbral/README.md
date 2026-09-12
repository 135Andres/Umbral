# umbral

The v0.1 workspace observation instrument.

## What this is

A local, read-only tool. You point it at a directory you own, it records what is there, and
later it tells you what is known, when it was known, and what changed — labelling every
statement as `observed`, `derived`, `ambiguous` or `unknown`.

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
umbral check <root>           recompute derived state and verify the log (read-only)
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
`umbral check` demonstrates that.

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
