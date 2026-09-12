# fsp-check

V0 observation prototype (UD-012; stack UD-013, V0-scoped). NOT Umbral; the V0 store is
NOT Project Reality (H25). Plan: ../V0-IMPLEMENTATION-PLAN.md.

## Status

Increment 1 — SCAN: **implemented, tested** (2026-09-11).

Increment 2 — IDENTITY (physical evidence): **implemented, tested** (2026-09-11).

Increment 3 — HASH (content evidence + re-read guard): **implemented, tested**
(2026-09-11).

Increment 4 — STORE (SQLite history + projection, crash-tested): **implemented, tested**
(2026-09-11).

Increment 5 — RECONCILE (mutation classification over observations): **implemented,
tested** (2026-09-11).

- `cargo check` clean · `cargo test` 60/60 pass (16 reconcile + 9 store + 15 hash +
  11 identity + 8 scan + crash worker) · `cargo clippy --all-targets` clean ·
  `cargo fmt` applied.

Increment 6 — HARNESS (independent oracle, generated sequences, falsification,
benchmarks): **done** (2026-09-11). See `../experiments/v0-harness/`.

V0 CLOSED AND FROZEN (2026-09-11): STATUS PARTIAL — core demonstrated within declared
scope, no implementation failure produced by falsification; SC-5 (reader surface) is the
recorded gap, handed to V1. Formal scope statement and freeze:
`../experiments/v0-harness/V0-CLOSEOUT.md`. No further functional changes to this crate
absent contradicting evidence.

- `cargo test` 68/68 pass · clippy clean · fmt clean.
- `examples/bench.rs`: benchmarks (evidence only, no thresholds);
  `tests/harness_falsification.rs`: independent ReferenceState oracle (models
  existence/content/aliasing directly, never calls fsp-check's reconcile), generated
  op sequences with explicit precondition gating, 512 proptest cases, crash-during-
  sequence, hard-link group survival, path-swap-as-property, out-of-order semantics.
- Findings: four class D (harness/oracle defects, none in fsp-check) — including the
  second occurrence of the `Path::starts_with("dir/")` trap and an oracle that failed to
  model hard-link aliasing (the implementation was right, the oracle wrong) — **plus one
  class B coverage gap (F-5): the generator never emitted `Op::DeleteDir`, so directory
  deletion was not falsified by V0.** The three-way distinction between operations
  modelled, operations generated, and operations exercised is stated in
  `../experiments/v0-harness/FALSIFICATION-REPORT.md`.

Reconcile (`src/reconcile.rs`): pure function over two ObservationSets (entries +
valid-hash evidence + completeness flag). Matching rule, in priority order:
1. same-path + equal dev+ino -> Unchanged/Modified (size/mtime, refined by hash);
2. leftovers matched globally by PhysicalId against "vacated" previous paths — exactly
   one candidate -> RenamedOrMoved (this makes path SWAPS two renames, not two
   modifications); several -> Ambiguous(ConflictingCandidates), never an arbitrary pick;
3. same-path without shared identity: different valid content -> Recreated; identical
   content or no content evidence -> Ambiguous (duplicate content is NOT identity);
4. new paths -> Created; 5. vanished previous paths -> Deleted ONLY when the current
scan is complete, else Unobserved ("not seen" is not "deleted"). Hard-link entry
deletions carry object_survives evidence. Directory-rename children are flagged
under_dir_rename. Deleted/Unobserved are the tombstone answer for V0: the
reconciliation result IS the deletion record; the store needs no second mechanism
(its projection keeps the last observation until a reconcile consumer applies it —
H25 boundary kept: store records, reconcile interprets).

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
