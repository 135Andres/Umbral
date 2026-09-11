//! fsp-check binary — thin CLI over the scan library seam.
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: fsp-check <root-directory>");
        return ExitCode::from(2);
    }
    let root = std::path::PathBuf::from(&args[1]);
    match fsp_check::scan::scan(&root) {
        Ok(scan) => {
            let files = scan
                .entries
                .iter()
                .filter(|e| e.kind == fsp_check::EntryKind::File)
                .count();
            let dirs = scan
                .entries
                .iter()
                .filter(|e| e.kind == fsp_check::EntryKind::Dir)
                .count();
            let links = scan
                .entries
                .iter()
                .filter(|e| e.kind == fsp_check::EntryKind::Symlink)
                .count();
            let other = scan.entries.len() - files - dirs - links;
            println!("root: {}", scan.root.display());
            println!(
                "entries: {} (files {}, dirs {}, symlinks {}, other {})",
                scan.entries.len(),
                files,
                dirs,
                links,
                other
            );
            println!("unobservable paths: {}", scan.errors.len());
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
