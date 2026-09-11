//! fsp-check binary — thin CLI over the library seams (scan / observe / store).
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("usage: fsp-check <command> <args>");
        eprintln!("  scan <root>            observe a tree, print counts (no store)");
        eprintln!("  init <db>              create an empty store");
        eprintln!("  record <db> <root>     scan <root> and record observations (+ stable hashes)");
        eprintln!("  inspect <db>           print active projection summary");
        eprintln!("  rebuild <db>           rebuild the projection from history");
        return ExitCode::from(2);
    }
    let cmd = args[1].as_str();
    match cmd {
        "scan" => {
            let root = std::path::PathBuf::from(&args[2]);
            match fsp_check::scan::scan(&root) {
                Ok(scan) => {
                    let (mut f, mut d, mut l) = (0usize, 0usize, 0usize);
                    for e in &scan.entries {
                        match e.kind {
                            fsp_check::EntryKind::File => f += 1,
                            fsp_check::EntryKind::Dir => d += 1,
                            fsp_check::EntryKind::Symlink => l += 1,
                            _ => {}
                        }
                    }
                    println!(
                        "entries: {} (files {f}, dirs {d}, symlinks {l}, other {}); unobservable: {}",
                        scan.entries.len(),
                        scan.entries.len() - f - d - l,
                        scan.errors.len()
                    );
                    ExitCode::SUCCESS
                }
                Err(e) => {
                    eprintln!("error: {e}");
                    ExitCode::FAILURE
                }
            }
        }
        "init" => match fsp_check::store::Store::open(std::path::Path::new(&args[2])) {
            Ok(_) => {
                println!("store ready: {}", args[2]);
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("error: {e}");
                ExitCode::FAILURE
            }
        },
        "record" => {
            if args.len() != 4 {
                eprintln!("usage: fsp-check record <db> <root>");
                return ExitCode::from(2);
            }
            record_cmd(&args[2], std::path::Path::new(&args[3]))
        }
        "inspect" => inspect_cmd(std::path::Path::new(&args[2])),
        "rebuild" => rebuild_cmd(std::path::Path::new(&args[2])),
        _ => {
            eprintln!("unknown command: {cmd}");
            ExitCode::from(2)
        }
    }
}

fn record_cmd(db: &str, root: &std::path::Path) -> ExitCode {
    let scan = match fsp_check::scan::scan(root) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::FAILURE;
        }
    };
    let store = match fsp_check::store::Store::open(std::path::Path::new(db)) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::FAILURE;
        }
    };
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as i64;
    let mut n = 0u32;
    for entry in &scan.entries {
        let content = if entry.kind == fsp_check::EntryKind::File {
            Some(fsp_check::hash_obs::observe_content(
                &root.join(&entry.path),
            ))
        } else {
            None
        };
        if store.record(entry, now, content.as_ref()).is_ok() {
            n += 1;
        }
    }
    println!("recorded {n} observations from {}", root.display());
    ExitCode::SUCCESS
}

fn inspect_cmd(db: &std::path::Path) -> ExitCode {
    match fsp_check::store::Store::open(db) {
        Ok(store) => {
            let active = store.active_projection().unwrap_or_default();
            println!(
                "history: {} observations",
                store.observation_count().unwrap_or(0)
            );
            println!("active paths: {}", active.len());
            for r in active.iter().take(20) {
                println!(
                    "  {} kind={} size={:?} ino={:?} hash={}",
                    r.path.display(),
                    fsp_check::store_kind_name(r.kind),
                    r.size,
                    r.ino,
                    match &r.hash {
                        Some(h) => format!("{}…", hex_head(h)),
                        None => "none".into(),
                    }
                );
            }
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

fn rebuild_cmd(db: &std::path::Path) -> ExitCode {
    match fsp_check::store::Store::open(db) {
        Ok(mut store) => match store.rebuild_projection() {
            Ok(n) => {
                println!("replayed {n} observations into the projection");
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("error: {e}");
                ExitCode::FAILURE
            }
        },
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

fn hex_head(h: &[u8]) -> String {
    h.iter().take(4).map(|b| format!("{b:02x}")).collect()
}
