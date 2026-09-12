//! `umbral` — the v0.1 command-line surface.
//!
//! This CLI is the output of an observation instrument. It is **not** a decision about
//! Umbral's interface: no API, web UI, daemon or other surface is designed here, and the
//! output format is declared unstable.
//!
//! # Commands
//!
//! Mutating the tool's own state (never the user's tree):
//!
//! - `init <root>`    — create the workspace record
//! - `observe <root>` — record one observation run
//!
//! Strictly read-only — these open the state file with no write capability:
//!
//! - `status <root>`  — what is known now
//! - `changes <root>` — what changed between the last two runs
//! - `show <root> <path>` — the history of one path
//! - `workspaces`     — which workspaces exist on this machine
//! - `check <root>`   — recompute derived state and verify the log is self-consistent
//!
//! # Exit codes
//!
//! - `0` success. Includes `ambiguous` and "no results": they are results, not errors.
//! - `1` runtime error (I/O, state, schema).
//! - `2` usage error.
//! - `3` `observe` only: the run was recorded but incomplete, because some paths could not
//!   be observed. Present so a script can see incompleteness instead of missing it.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use umbral::log::sqlite::SqliteLog;
use umbral::log::{NewObservation, NewRun, ObservationLog};
use umbral::report;
use umbral::{content, scan, workspace};

const USAGE: &str = "\
usage: umbral <command> [args]

  init <root>            create the workspace record for <root>
  observe <root>         record one observation run over <root>
  status <root>          read what is known now (read-only)
  changes <root>         read what changed between the last two runs (read-only)
  show <root> <path>     read the history of one path (read-only)
  workspaces             list workspaces on this machine (read-only)
  check <root>           recompute derived state and verify the log (read-only)

State lives outside <root>, under $XDG_DATA_HOME/umbral/ (fallback ~/.local/share/umbral/).
Nothing is ever written inside <root>.";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().collect();
    let code = run(&args);
    ExitCode::from(code)
}

fn run(args: &[String]) -> u8 {
    if args.len() < 2 {
        eprintln!("{USAGE}");
        return 2;
    }
    let cmd = args[1].as_str();

    match cmd {
        "init" => {
            let Some(root) = arg(args, 2) else {
                return usage("init <root>");
            };
            match workspace::init(Path::new(root)) {
                Ok(ws) => {
                    // Only the canonical root is the filesystem's answer. The root as given
                    // is the caller's own argument echoed back, and the workspace
                    // identifier and state path are computed from it.
                    println!(
                        "{}",
                        report::render(&[
                            report::Line::observed(format!("canonical={}", ws.canonical.display())),
                            report::Line::derived(format!("root={}", ws.root.display())),
                            report::Line::derived(format!("workspace-id={}", ws.id)),
                            report::Line::derived(format!("state-dir={}", ws.state_dir.display())),
                            report::Line::derived("initialised=true"),
                        ])
                    );
                    0
                }
                Err(e) => runtime_error(&e),
            }
        }

        "observe" => {
            let Some(root) = arg(args, 2) else {
                return usage("observe <root>");
            };
            observe(root)
        }

        "status" => {
            let Some(root) = arg(args, 2) else {
                return usage("status <root>");
            };
            read_only(root, |ws, log| report::status(ws, log))
        }

        "changes" => {
            let Some(root) = arg(args, 2) else {
                return usage("changes <root>");
            };
            read_only(root, |ws, log| report::changes(ws, log))
        }

        "show" => {
            let (Some(root), Some(rel)) = (arg(args, 2), arg(args, 3)) else {
                return usage("show <root> <path>");
            };
            read_only(root, |ws, log| report::show(ws, log, Path::new(rel)))
        }

        "check" => {
            let Some(root) = arg(args, 2) else {
                return usage("check <root>");
            };
            read_only(root, |ws, log| report::check(ws, log))
        }

        "workspaces" => match workspace::list() {
            Ok(list) => {
                let mut lines = Vec::new();
                if list.is_empty() {
                    lines.push(report::Line::unknown("workspaces=none"));
                }
                for w in &list {
                    // Everything in a workspace record except the canonical root was
                    // computed by the tool when the record was written: the identifier, the
                    // creation timestamp, the tool version.
                    lines.push(report::Line::observed(format!(
                        "canonical={}",
                        w.canonical.display()
                    )));
                    lines.push(report::Line::derived(format!("workspace-id={}", w.id)));
                    lines.push(report::Line::derived(format!(
                        "created={}  tool-version={}",
                        report::format_unix_ns(w.created_at_ns),
                        w.tool_version
                    )));
                }
                println!("{}", report::render(&lines));
                0
            }
            Err(e) => runtime_error(&e),
        },

        _ => {
            eprintln!("unknown command: {cmd}");
            eprintln!("{USAGE}");
            2
        }
    }
}

fn arg(args: &[String], i: usize) -> Option<&str> {
    args.get(i).map(String::as_str)
}

fn usage(expected: &str) -> u8 {
    eprintln!("usage: umbral {expected}");
    2
}

fn runtime_error(e: &dyn std::fmt::Display) -> u8 {
    eprintln!("error: {e}");
    1
}

/// Open a workspace read-only and render one of the read-only views.
///
/// A workspace that has been initialised but never observed has no log file yet. That is a
/// legitimate state — "nothing has been observed" — not an I/O failure, so it is answered
/// with an empty log rather than an error. The alternative would make `status` fail on a
/// freshly initialised workspace, which would be a confusing way to say "no observations".
fn read_only<F>(root: &str, f: F) -> u8
where
    F: Fn(
        &workspace::Workspace,
        &dyn ObservationLog,
    ) -> Result<Vec<report::Line>, umbral::log::LogError>,
{
    let ws = match workspace::open(Path::new(root)) {
        Ok(ws) => ws,
        Err(e) => return runtime_error(&e),
    };
    let log: Box<dyn ObservationLog> = if ws.log_path().exists() {
        match SqliteLog::open_read_only(&ws.log_path()) {
            Ok(l) => Box::new(l),
            Err(e) => return runtime_error(&e),
        }
    } else {
        match SqliteLog::open_in_memory() {
            Ok(l) => Box::new(l),
            Err(e) => return runtime_error(&e),
        }
    };
    match f(&ws, log.as_ref()) {
        Ok(lines) => {
            println!("{}", report::render(&lines));
            0
        }
        Err(e) => runtime_error(&e),
    }
}

fn observe(root: &str) -> u8 {
    let ws = match workspace::open(Path::new(root)) {
        Ok(ws) => ws,
        Err(e) => return runtime_error(&e),
    };

    let started_at_ns = now_ns();
    let scan = match scan::scan(Path::new(root)) {
        Ok(s) => s,
        Err(e) => return runtime_error(&e),
    };

    let mut observations: Vec<NewObservation> = Vec::with_capacity(scan.entries.len());
    let mut verified: u64 = 0;
    let mut not_verified: u64 = 0;

    for entry in &scan.entries {
        let content_obs = if entry.kind == umbral::EntryKind::File {
            let c = content::observe_content(&Path::new(root).join(&entry.path));
            if c.is_content_verified() {
                verified += 1;
            } else {
                not_verified += 1;
            }
            Some(c)
        } else {
            None
        };
        observations.push(NewObservation {
            entry: entry.clone(),
            content: content_obs,
            error: None,
        });
    }

    // Paths that could not be observed at all are recorded as evidence, not dropped.
    //
    // A path can be BOTH an entry and an error: a directory whose metadata is readable but
    // whose contents are not yields an entry (it exists) and an error (it could not be
    // descended into). One path has one row per run, so the error is attached to the
    // existing observation rather than duplicated into a second row.
    let mut by_path: std::collections::BTreeMap<PathBuf, usize> = std::collections::BTreeMap::new();
    for (i, o) in observations.iter().enumerate() {
        by_path.insert(o.entry.path.clone(), i);
    }
    for err in &scan.errors {
        match by_path.get(&err.path) {
            Some(&i) => {
                observations[i].error = Some(err.message.clone());
            }
            None => {
                let idx = observations.len();
                observations.push(NewObservation {
                    entry: umbral::Entry {
                        path: err.path.clone(),
                        kind: umbral::EntryKind::Other,
                        dev: None,
                        ino: None,
                        size: None,
                        mtime: None,
                    },
                    content: None,
                    error: Some(err.message.clone()),
                });
                by_path.insert(err.path.clone(), idx);
            }
        }
    }

    let finished_at_ns = now_ns();
    let run = NewRun {
        started_at_ns,
        finished_at_ns,
        root: ws.canonical.clone(),
        observations,
    };

    let mut log = match SqliteLog::open(&ws.log_path()) {
        Ok(l) => l,
        Err(e) => return runtime_error(&e),
    };
    let run_id = match log.append_run(run) {
        Ok(id) => id,
        Err(e) => return runtime_error(&e),
    };

    let meta = match log.run(run_id) {
        Ok(Some(m)) => m,
        Ok(None) => {
            eprintln!("error: run {run_id} was not recorded");
            return 1;
        }
        Err(e) => return runtime_error(&e),
    };

    println!(
        "{}",
        report::render(&report::observe_summary(&ws, &meta, verified, not_verified))
    );

    if meta.complete() {
        0
    } else {
        // The run is recorded; it just did not see everything. Say so to the shell.
        eprintln!(
            "note: run {} is incomplete: {} path(s) could not be observed",
            meta.id, meta.errors
        );
        3
    }
}

fn now_ns() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as i64)
        .unwrap_or(0)
}
