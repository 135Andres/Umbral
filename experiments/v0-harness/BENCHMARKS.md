# V0 HARNESS — BENCHMARKS AND FALSIFICATION EVIDENCE

Status: EXPERIMENT RECORD (evidence). Produced by increment 6 (harness/falsification).
Inputs: `fsp-check/examples/bench.rs` (benchmarks), `fsp-check/tests/harness_falsification.rs`
(oracle + generated sequences + crash-during-sequence).

Environment of record: Fedora 44, kernel 7.1.13-200.fc44.x86_64, x86_64 (Intel i7-8850H),
workspace on btrfs, benchmark corpus in a tempfile dir (tmpfs). Rust 1.98.1, release build.

Per the user's standing rule: benchmarks are EXPERIMENTS producing EVIDENCE. No numeric
threshold is a product requirement; none was used to make a decision.

---

## BENCHMARKS (plan §8)

## V0 BENCHMARKS (evidence — no thresholds)

Environment: Fedora 44, kernel 7.1.13, btrfs workspace; corpus on tmpfs (tempfile).
Corpus: 2000 files × 4 KiB = 7 MiB, 40 directories.
Method: single process, sequential, release build, wall-clock via std::time::Instant.

| metric | result | notes |
|---|---|---|
| corpus build | 79.238124ms | fixture setup, not a product metric |
| cold scan (stat only) | 22.166125ms | 2040 entries |
| warm scan | 23.518315ms | same tree, page cache warm |
| hash all files (guarded, streaming) | 89.698565ms | 2000 files hashed, 87.1 MiB/s |
| reconcile, zero changes | 5.131541ms | 2040 mutations |
| mutation burst (200 rewrites) | 3.07647ms write + 2.764254ms reconcile | 2040 mutations |
| store writes (all observations) | 135.315559ms | 2040 observations, WAL + synchronous=FULL |
| rebuild projection | 86.899493ms | 2040 observations replayed |
| reader latency (100 active_projection) | 276.924354ms | sequential, single connection |
| store size | 323584 B (+ 4132392 B WAL) | after 2040 observations |
| FTS5 index | NOT APPLICABLE | V0 has no full-text index (no FTS table exists) |
| memory | NOT APPLICABLE | not instrumented in V0; hash streams in 64 KiB chunks by construction |

Interpretation limits: tmpfs corpus (not btrfs), one machine, one process shape,
page cache warm for warm-scan numbers. These are EVIDENCE, not requirements.

