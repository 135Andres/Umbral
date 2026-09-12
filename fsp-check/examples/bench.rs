//! V0 benchmarks (plan §8) — executed as EXPERIMENTS, producing EVIDENCE.
//! No thresholds, no product requirements. Run: cargo run --release --bin bench
//! Output is a markdown fragment on stdout, captured to
//! experiments/v0-harness/BENCHMARKS.md.
//!
//! Metrics marked NOT APPLICABLE where V0 has no corresponding component
//! (FTS5: no full-text index exists in V0 — the store has no FTS table).

use fsp_check::hash_obs::observe_content;
use fsp_check::reconcile::{ObservationSet, ObservedPath, reconcile};
use fsp_check::scan::scan;
use fsp_check::store::Store;
use std::time::Instant;

fn main() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    let files = 2_000usize;
    let payload = vec![7u8; 4096];

    // corpus: 2000 files of 4 KiB in 40 dirs
    let t0 = Instant::now();
    for d in 0..40 {
        std::fs::create_dir_all(root.join(format!("d{d}"))).unwrap();
    }
    for i in 0..files {
        std::fs::write(root.join(format!("d{}/f{i}.bin", i % 40)), &payload).unwrap();
    }
    let corpus_build = t0.elapsed();

    // 1. cold scan (stat only, no hashing)
    let t = Instant::now();
    let s1 = scan(root).unwrap();
    let cold_scan = t.elapsed();

    // 2. warm scan
    let t = Instant::now();
    let _ = scan(root).unwrap();
    let warm_scan = t.elapsed();

    // 3. hash throughput (streaming, guarded)
    let t = Instant::now();
    let mut hashed = 0usize;
    for i in 0..files {
        let o = observe_content(&root.join(format!("d{}/f{i}.bin", i % 40)));
        if o.valid_hash().is_some() {
            hashed += 1;
        }
    }
    let hash_all = t.elapsed();
    let bytes = (files * payload.len()) as f64;
    let mib_s = bytes / 1_048_576.0 / hash_all.as_secs_f64();

    // 4. reconciliation (stat-only evidence: no hashes in the sets)
    let to_set = |s: &fsp_check::Scan| {
        ObservationSet::new(
            s.entries
                .iter()
                .map(|e| ObservedPath {
                    entry: e.clone(),
                    valid_hash: None,
                })
                .collect(),
            s.errors.is_empty(),
        )
    };
    let set1 = to_set(&s1);
    let set2 = to_set(&scan(root).unwrap());
    let t = Instant::now();
    let r = reconcile(&set1, &set2);
    let reconcile_zero_change = t.elapsed();

    // 5. mutation burst: 200 files rewritten, then scan + reconcile
    let t = Instant::now();
    for i in 0..200 {
        std::fs::write(root.join(format!("d{}/f{i}.bin", i % 40)), b"changed").unwrap();
    }
    let burst_write = t.elapsed();
    let set3 = to_set(&scan(root).unwrap());
    let t = Instant::now();
    let r3 = reconcile(&set2, &set3);
    let reconcile_burst = t.elapsed();

    // 6. SQLite writes: record the whole scan
    let store_path = dir.path().join("bench.db");
    let mut store = Store::open(&store_path).unwrap();
    let t = Instant::now();
    for e in &s1.entries {
        store.record(e, 1, None).unwrap();
    }
    let store_write_all = t.elapsed();

    // 7. rebuild projection from history
    let t = Instant::now();
    let replayed = store.rebuild_projection().unwrap();
    let rebuild = t.elapsed();

    // 8. reader latency while writing (same connection, sequential V0 model)
    let t = Instant::now();
    for _ in 0..100 {
        let _ = store.active_projection().unwrap();
    }
    let reader_100 = t.elapsed();

    // 9. store size on disk
    let size = std::fs::metadata(&store_path).map(|m| m.len()).unwrap_or(0);
    let wal = std::fs::metadata(store_path.with_extension("db-wal"))
        .map(|m| m.len())
        .unwrap_or(0);

    println!("## V0 BENCHMARKS (evidence — no thresholds)");
    println!();
    println!("Environment: Fedora 44, kernel 7.1.13, btrfs workspace; corpus on tmpfs (tempfile).");
    println!(
        "Corpus: {files} files × 4 KiB = {} MiB, 40 directories.",
        (files * 4096) / 1_048_576
    );
    println!(
        "Method: single process, sequential, release build, wall-clock via std::time::Instant."
    );
    println!();
    println!("| metric | result | notes |");
    println!("|---|---|---|");
    println!(
        "| corpus build | {:?} | fixture setup, not a product metric |",
        corpus_build
    );
    println!(
        "| cold scan (stat only) | {:?} | {} entries |",
        cold_scan,
        s1.entries.len()
    );
    println!(
        "| warm scan | {:?} | same tree, page cache warm |",
        warm_scan
    );
    println!(
        "| hash all files (guarded, streaming) | {:?} | {hashed} files hashed, {:.1} MiB/s |",
        hash_all, mib_s
    );
    println!(
        "| reconcile, zero changes | {:?} | {} mutations |",
        reconcile_zero_change,
        r.mutations.len()
    );
    println!(
        "| mutation burst (200 rewrites) | {:?} write + {:?} reconcile | {} mutations |",
        burst_write,
        reconcile_burst,
        r3.mutations.len()
    );
    println!(
        "| store writes (all observations) | {:?} | {} observations, WAL + synchronous=FULL |",
        store_write_all,
        s1.entries.len()
    );
    println!(
        "| rebuild projection | {:?} | {replayed} observations replayed |",
        rebuild
    );
    println!(
        "| reader latency (100 active_projection) | {:?} | sequential, single connection |",
        reader_100
    );
    println!(
        "| store size | {size} B (+ {wal} B WAL) | after {} observations |",
        s1.entries.len()
    );
    println!("| FTS5 index | NOT APPLICABLE | V0 has no full-text index (no FTS table exists) |");
    println!(
        "| memory | NOT APPLICABLE | not instrumented in V0; hash streams in 64 KiB chunks by construction |"
    );
    println!();
    println!("Interpretation limits: tmpfs corpus (not btrfs), one machine, one process shape,");
    println!("page cache warm for warm-scan numbers. These are EVIDENCE, not requirements.");
}
