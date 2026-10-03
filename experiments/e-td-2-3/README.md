# E-TD-2 and E-TD-3 — the skip condition against hidden changes

Status: **SPECIFIED 2026-10-02, BEFORE THE RUN; RESULTS IN §5.** This file fixes the execution parameters of the
two experiments pre-registered in [`V0.2-TECHNICAL-DESIGN.md`](../../docs/candidates/V0.2-TECHNICAL-DESIGN.md)
§H.1 (E-TD-2, the metadata-restoring writer) and §H.2 (E-TD-3, inode reuse with matching
metadata). The protocols, hypotheses and falsifiers are those sections, **unchanged**; nothing
here edits them. Results will be recorded in §5 of this file after the run, and the parameters
below are not changed after it.

Why now: `D-PEND-2` — does `ctime` enter the skip condition — is decided by the owner on this
evidence (`UD-018`, A2-V12). Execution parameters chosen by the owner on 2026-10-02 (private
instruction): a probe here, run locally and in CI on ext4; 50 repetitions per variant; an
E-TD-3 budget of 1000 attempts.

## 1. What is run

[`probe.py`](probe.py), one process per filesystem under test, writing one JSON document to
standard output. It touches nothing but a temporary directory it creates under the directory it
is given, and removes it.

- **The skip condition** is §C.1 written as a pure function over two `lstat` results: conditions
  1–5 (metadata-only) and 1–6 (with `ctime`). No `umbral` build implements a skip yet, so the
  decision is this function's — the experiment asks whether the **information** the condition
  reads can see the change, which does not depend on where the function lives.
- **The verdict** (H2-d, E-TD-3 step 8) is the real one: the probe runs the `umbral` binary
  (`init`, `observe`, `observe`, `changes`) on the same directory and records the line for the
  file. That is v0.1's implementation, which reads every file. The v0.2 verdict under a skip is
  **derived**, not run: a skip carries the previous hash forward (§C.2), and the reconciliation
  rule for a same-path pair with equal identity, size and mtime does not consult the hash
  (`same_observable`), so the derived verdict is stated from that rule and labelled as derived.

## 2. Parameters — fixed before the run

| Parameter | Value |
|---|---|
| E-TD-2 variants | `restore` (same length, mtime restored with `utimensat`; the case), `atomic` (`rename` over the target), `touch` (mtime moves, bytes equal), `chmod` (mode only) — §H.1 steps 5 and 9 |
| E-TD-2 repetitions | **50** per variant per filesystem (§H.1 asks ≥ 20) |
| E-TD-2 granularity wait | **1 s** between the two writes (§H.1 step 4) |
| E-TD-2 validity | in `restore`, a repetition where size, mtime, `dev` or `ino` differ from step 2 is **invalid** and counted as such, not as a result (§H.1 step 6) |
| E-TD-3 budget | **1000** attempts per filesystem; the run stops at the budget, not at a success (§H.2 step 5) |
| E-TD-3 verdicts | the `umbral` verdict is recorded for **every** attempt in which the identity was reused |
| H2-c | checked exhaustively over every combination of equal/different for `kind`, `stability`, `dev`, `ino`, `size`, `mtime`, `ctime` — no filesystem needed (§H.1) |
| Filesystems | tmpfs and btrfs locally; **ext4** in CI (`.github/workflows/experiments.yml`, `ubuntu-latest`). Kernel, filesystem type and mount options are recorded by the probe |

## 3. How each outcome will be reported — fixed before the run

- **H2-b dies** on one `restore` repetition with bytes different, size/mtime/`dev`/`ino` equal and
  `ctime` equal. One is enough. It is a result, not a failed run.
- **E-TD-3, reuse never observed within the budget:** "not provoked in 1000 attempts on <fs>" —
  never "does not occur".
- **E-TD-3, reuse observed and the metadata-only condition skips:** L1 is **demonstrated** on that
  filesystem.
- A filesystem not probed is reported **NOT EXECUTED**.
- None of the outcomes can show that a false skip is impossible.

## 4. What this cannot establish

That `ctime` always moves; that the behaviour holds on other kernels, filesystems or mount
options; anything about filesystems that do not generate their own timestamps (§D.4 case 1).

## 5. Results

Status: **RESULT RECORDED 2026-10-02/03.** Raw output: [`results/`](results) — `local-tmpfs.json`,
`local-btrfs.json` (kernel 7.2.7-arch1-1) and `ci-ext4.json` (kernel 6.17.0-1022-azure,
`ubuntu-latest`, mount options in the file; run
https://github.com/135Andres/Umbral/actions/runs/37093458222). The protocol and parameters of
§1–§3 were not changed.

**Provenance note.** The local arms ran twice. The first run's raw output was written to a
session-temporary directory that was removed before it was archived; only its printed summary
survived. The archived files are the second run, under the same protocol. Every count in the
printed summary of the first run equals the corresponding count of the second.

### 5.1 E-TD-2 — the metadata-restoring writer (50 repetitions per variant)

Identical on all three filesystems; no repetition was invalid.

| Variant | bytes differ | `ctime` equal | metadata-only: skip | with `ctime`: skip | v0.1 verdict (real) |
|---|---|---|---|---|---|
| `restore` | 50 | **0** | **50** | 0 | `unchanged` ×50 |
| `atomic` | 50 | 0 | 0 | 0 | `recreated` ×50 |
| `touch` | 0 | 0 | 0 | 0 | `unchanged` ×50 |
| `chmod` | 0 | 0 | 50 | 0 | `unchanged` ×50 |

- **H2-a — not falsified.** Without `ctime`, the restored writer is invisible to the condition:
  50/50 skips on each filesystem.
- **H2-b — not falsified.** No repetition had the bytes changed and `ctime` equal (smallest
  `ctime` movement 1.00 s locally, 1.005 s on ext4 — the 1 s wait of step 4). One counterexample
  would have sufficed; none was found. This does not show that `ctime` always moves (§4).
- **H2-c — not falsified.** 128 of 128 combinations: adding `ctime` never enlarged the skipped set.
- **H2-d — not falsified**, and for a reason the protocol did not anticipate: see 5.3.
- **Controls.** `atomic` and `touch` force a read under both conditions. `chmod` forces a read
  only with `ctime` (50/50) — the cost `UD-018` accepted; the metadata-only condition skips it,
  correctly, since the bytes did not change.

### 5.2 E-TD-3 — inode reuse with matching metadata (budget 1000 attempts)

| Filesystem | Outcome | metadata-only: skip | with `ctime`: skip | `ctime` equal | v0.1 verdict (real) |
|---|---|---|---|---|---|
| tmpfs | **not provoked in 1000 attempts** | — | — | — | — |
| btrfs | **not provoked in 1000 attempts** | — | — | — | — |
| **ext4** | **reuse observed in 1000 of 1000 attempts**, from the first | **1000** | 0 | 0 | `unchanged` ×1000 |

- **L1 is demonstrated on ext4.** A file deleted and recreated at the same path with different
  bytes of the same length and the original mtime receives the same `dev`+`ino` — on every
  attempt — and the metadata-only condition skips it. Under that condition the new bytes would
  never be read.
- With `ctime` in the condition, every one of those 1000 cases is read.
- tmpfs and btrfs: "not provoked in 1000 attempts". Not "does not occur".

### 5.3 Finding not anticipated by the protocol — v0.1's own verdict

In **every** case where the bytes changed but identity, size and mtime were equal — `restore`
on all three filesystems (150), and inode reuse on ext4 (1000) — the **real v0.1 verdict is
`unchanged`, although v0.1 read both versions and their hashes differ.** The cause is in
`reconcile` phase 1 (`umbral/src/reconcile.rs`): when identity is shared and `same_observable`
holds, the verdict is `unchanged` **without consulting the hashes**, even when both are valid and
different. Slice 2b already makes this visible — such a line lists `dev,ino,kind,size,mtime` and
no `hash` — but the verdict contradicts evidence the observations hold.

Consequences, stated without deciding anything:

- This is why H2-d did not fail: under both conditions the verdict is `unchanged` — under the
  skip because the hash is carried, under a read because the rule does not look at it.
- Adding `ctime` to the skip condition makes the **stored** reading truthful (the new bytes are
  read and hashed) but, with the current rule, does **not** change the verdict of `changes`.
- Whether phase 1 should consult two valid hashes before answering `unchanged` changes v0.1
  verdicts, so it is an owner decision, not a correction this experiment can make.
