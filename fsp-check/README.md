# fsp-check

V0 observation prototype (UD-012; stack UD-013, V0-scoped). NOT FSP; the V0 store is
NOT Project Reality (H25). Plan: ../V0-IMPLEMENTATION-PLAN.md.

## Status

Increment 1 — SCAN: **implemented, tested** (2026-09-11).

- `cargo check` clean · `cargo test` 8/8 pass · `cargo clippy --all-targets` clean ·
  `cargo fmt` applied.

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
