//! Baseline measurements for v0.1. **Evidence only — no thresholds, no optimisation.**
//!
//! Per the project's standing rule, a measurement is an experiment that produces evidence;
//! no number here is a product requirement, and none was used to make a decision.
//!
//! Run with: `cargo run --release --example bench`
//!
//! The corpus shape matches the frozen V0 benchmark (2000 files x 4 KiB in 40 directories)
//! so the numbers are comparable with `experiments/v0-harness/BENCHMARKS.md`. v0.1
//! re-observes the whole tree on every run, so the cost is O(corpus) per run — that is
//! stated in the version record as a known limitation, and it is what v0.2 would address.

use std::path::Path;
use std::time::Instant;

use umbral::content::observe_content;
use umbral::log::sqlite::SqliteLog;
use umbral::log::{NewObservation, NewRun, ObservationLog};
use umbral::reconcile::{reconcile, ObservationSet, ObservedPath};
use umbral::scan::{scan, EntryKind};
use umbral::workspace::Workspace;

const FILES: usize = 2000;
const DIRS: usize = 40;
const FILE_BYTES: usize = 4096;

fn build_corpus(root: &Path) {
    for d in 0..DIRS {
        std::fs::create_dir_all(root.join(format!("d{d:02}"))).unwrap();
    }
    let body = vec![b'x'; FILE_BYTES];
    for i in 0..FILES {
        std::fs::write(root.join(format!("d{:02}/f{i:04}.bin", i % DIRS)), &body).unwrap();
    }
}

fn ms(d: std::time::Duration) -> String {
    format!("{:.3}ms", d.as_secs_f64() * 1000.0)
}

fn observe_set(root: &Path) -> ObservationSet {
    let s = scan(root).unwrap();
    let complete = s.complete();
    let paths = s
        .entries
        .iter()
        .map(|e| {
            let valid_hash = if e.kind == EntryKind::File {
                observe_content(&root.join(&e.path)).valid_hash().copied()
            } else {
                None
            };
            ObservedPath {
                path: e.path.clone(),
                kind: e.kind,
                dev: e.dev,
                ino: e.ino,
                size: e.size,
                mtime: e.mtime,
                valid_hash,
            }
        })
        .collect();
    ObservationSet::new(paths, complete)
}

fn main() {
    let tmp = tempfile::TempDir::new().unwrap();
    let root = tmp.path();

    let t = Instant::now();
    build_corpus(root);
    let build = t.elapsed();

    let t = Instant::now();
    let first = scan(root).unwrap();
    let cold_scan = t.elapsed();

    let t = Instant::now();
    let _ = scan(root).unwrap();
    let warm_scan = t.elapsed();

    let t = Instant::now();
    let mut hashed = 0usize;
    let mut bytes = 0u64;
    for e in &first.entries {
        if e.kind == EntryKind::File {
            let c = observe_content(&root.join(&e.path));
            if c.is_content_verified() {
                hashed += 1;
                bytes += c.hashed_len.unwrap_or(0);
            }
        }
    }
    let hash_all = t.elapsed();

    let t = Instant::now();
    let set_a = observe_set(root);
    let set_b = observe_set(root);
    let full_observe = t.elapsed();

    let t = Instant::now();
    let r = reconcile(&set_a, &set_b);
    let reconcile_no_change = t.elapsed();

    // Persist one run, then measure a second run and the size of the state.
    let ws = Workspace {
        root: root.to_path_buf(),
        canonical: root.to_path_buf(),
        id: "bench".into(),
        state_dir: tmp.path().to_path_buf(),
    };
    let log_path = ws.log_path();
    let t = Instant::now();
    {
        let mut log = SqliteLog::open(&log_path).unwrap();
        for (n, set) in [&set_a, &set_b].iter().enumerate() {
            let observations = set
                .paths
                .iter()
                .map(|p| NewObservation {
                    entry: umbral::Entry {
                        path: p.path.clone(),
                        kind: p.kind,
                        dev: p.dev,
                        ino: p.ino,
                        size: p.size,
                        mtime: p.mtime,
                    },
                    content: None,
                    error: None,
                })
                .collect();
            log.append_run(NewRun {
                started_at_ns: n as i64,
                finished_at_ns: n as i64,
                root: root.to_path_buf(),
                observations,
            })
            .unwrap();
        }
    }
    let store_writes = t.elapsed();

    let t = Instant::now();
    let log = SqliteLog::open_read_only(&log_path).unwrap();
    let _ = log.observations_for_run(2).unwrap();
    let read_run = t.elapsed();

    let t = Instant::now();
    let _ = log
        .observations_for_path(Path::new("d00/f0000.bin"))
        .unwrap();
    let read_one_path = t.elapsed();

    let db_bytes = std::fs::metadata(&log_path).map(|m| m.len()).unwrap_or(0);

    println!("umbral v0.1 — baseline (evidence only, no thresholds)");
    println!("corpus: {FILES} files x {FILE_BYTES} B in {DIRS} dirs");
    println!();
    println!("{:<44} result", "metric");
    println!(
        "{:<44} {}",
        "corpus build (fixture, not a product metric)",
        ms(build)
    );
    println!("{:<44} {}", "cold scan (stat only)", ms(cold_scan));
    println!("{:<44} {}", "warm scan", ms(warm_scan));
    println!(
        "{:<44} {}",
        format!("hash all files (guarded, streaming) [{hashed} files]"),
        ms(hash_all)
    );
    println!(
        "{:<44} {}",
        "full observation (scan + hash), twice",
        ms(full_observe)
    );
    println!(
        "{:<44} {}",
        "reconcile, zero changes",
        ms(reconcile_no_change)
    );
    println!(
        "{:<44} {}",
        "store writes (2 runs, no content)",
        ms(store_writes)
    );
    println!("{:<44} {}", "read one run from the log", ms(read_run));
    println!("{:<44} {}", "read one path's history", ms(read_one_path));
    println!("{:<44} {} bytes", "state file size after 2 runs", db_bytes);
    println!();
    println!(
        "mutations in the zero-change comparison: {}",
        r.mutations.len()
    );
    println!(
        "all of them Unchanged: {}",
        r.count(umbral::reconcile::MutationKind::Unchanged) == r.mutations.len()
    );
    if bytes > 0 {
        println!(
            "hash throughput: {:.1} MiB/s",
            (bytes as f64 / (1024.0 * 1024.0)) / hash_all.as_secs_f64()
        );
    }
    println!();
    println!("Known limitation this baseline exposes: v0.1 re-observes the whole tree on every");
    println!("run, so the cost is O(corpus) per run. That is accepted for v0.1's contract and is");
    println!("the subject of a later version. No optimisation is performed here.");
}
