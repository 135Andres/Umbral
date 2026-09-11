# fsp-check

V0 observation prototype (UD-012; stack UD-013, V0-scoped). NOT FSP; the V0 store is
NOT Project Reality (H25). Plan: ../V0-IMPLEMENTATION-PLAN.md.

## Status

Increment 1 — SCAN: **implemented, tested** (2026-09-11).

Increment 2 — IDENTITY (physical evidence): **implemented, tested** (2026-09-11).

Increment 3 — HASH (content evidence + re-read guard): **implemented, tested**
(2026-09-11).

Increment 4 — STORE (SQLite history + projection, crash-tested): **implemented, tested**
(2026-09-11).

- `cargo check` clean · `cargo test` 43/43 pass (8 store + 1 crash-worker + 15 hash +
  11 identity + 8 scan) · `cargo clippy --all-targets` clean · `cargo fmt` applied.

Store (`src/store.rs`): rusqlite `bundled`, WAL + synchronous=FULL. TWO tables, each
column justified: `observations` (append-only history — the only irreplaceable truth)
and `projection` (latest-observation-per-path — a pure cache of the history, INV-2).
One record = one transaction over both. INV-7 tested with REAL process kills at four
instrumented points (before_first / mid_history / after_commit_1 / after_all, fresh DB
per point, separate process via the ignored-test worker): after every kill the store
reopens, history and projection agree, and rebuild-from-history reproduces the
projection exactly. INV-1 at rest: only Stable hashes are ever persisted; unstable/
errored content persists evidence, never bytes. Honest semantics finding: the V0
projection has NO tombstones — a vanished path remains until a later observation
updates it; disappearance classification belongs to RECONCILE (documented, not hidden).
CLI: init / record / inspect / rebuild (development commands, not product UX).
Q25 evidence recorded — see V0-IMPLEMENTATION-PLAN §14 and OPEN-QUESTIONS Q25 note.

Hash module (`src/hash_obs.rs`): `observe_content(path) -> ContentObservation` —
streaming BLAKE3 (64 KiB buffer, never whole-file), guarded stat-before / read /
stat-after over size + ns-mtime + dev/ino. Policy: on a detected change, retry once
(`MAX_GUARD_ATTEMPTS = 2`, explicit documented constant); still changing → `Unstable`
with `hash: None` and the `deltas` that fired. `valid_hash()` returns a hash ONLY for
Stable observations (INV-1's two halves). Symlinks/dirs/specials: `NotARegularFile`,
never resolved to target content. Errors (`NotFound`, `PermissionDenied`, `ReadError`)
never become fake hashes — no magic values. Residual limitation, documented: a rewrite
that restores size AND exact mtime within the read window is invisible to any
stat-guard; content hash comparison across observations is the partial backstop later.

Identity module (`src/identity.rs`): `PhysicalId {dev, ino}` as physical evidence —
NEVER semantic identity, never eternal (inode reuse). `compare_same_path` /
`compare_cross_path` return verdicts with `Ambiguous` as a first-class outcome (INV-8):
path alone never yields a positive identity; missing evidence is never identity; hard
links share one PhysicalId (one object, several directory entries); symlink identity is
the link itself, not the target. Observed on Fedora (btrfs + tmpfs): rename preserves
dev+ino (observed AND rename(2)-documented, filesystem-family-local, not universal);
inode reuse after delete+recreate was NOT observed in the probe but is designed for:
same-path dev+ino change → Ambiguous(IdentityChangedAtSamePath), never a claim.

## What exists

- `src/scan.rs` — deterministic recursive scanner (walkdir, `follow_links(false)`):
  read-only; symlink-aware via `symlink_metadata`; output sorted by relative path
  (`PathBuf: Ord`, lossless); per-entry errors represented, never silently dropped.
- `src/lib.rs` — the seam: `Entry`/`Scan` observation records (pure data). The future
  reconciler consumes these, never walkdir/syscalls (plan §12 boundary). Observable
  state (A) is separated from observation time (B, `Scan::started_at`).
- `src/main.rs` — thin CLI over the seam; prints counts only.
- `tests/scan_determinism.rs` — 8 tests incl. the INV-5 seed (re-scan of an unchanged
  tree → equal observable state), creation-order independence, non-ASCII losslessness,
  non-UTF-8 byte-exact filenames (labelled Linux-specific), symlink observed-not-
  traversed, no-modification proof via raw metadata snapshots, missing-root errors.

## Increment order (plan §5)

scan (done) → identity → hash → store → reconcile → harness.
Not yet: rusqlite, blake3, uuid, proptest enter only with their increment.
