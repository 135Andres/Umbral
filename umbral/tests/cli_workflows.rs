//! End-to-end CLI workflows, exit codes, and repeated-observation determinism (A4).

mod common;

use common::{assert_all_labelled, Sandbox};

#[test]
fn init_then_observe_then_read_is_the_whole_contract() {
    let s = Sandbox::new();
    s.write("notes.md", "hello");
    s.write("src/main.rs", "fn main() {}");

    let r = s.root().to_string_lossy().to_string();

    let init = s.run_ok(&["init", r.as_str()]);
    assert_all_labelled(&init);
    assert!(init.contains("workspace-id="));
    assert!(init.contains("state-dir="));

    let obs = s.run_ok(&["observe", r.as_str()]);
    assert_all_labelled(&obs);
    assert!(
        obs.contains("content-fresh=2"),
        "both files are regular files:\n{obs}"
    );

    let status = s.run_ok(&["status", r.as_str()]);
    assert_all_labelled(&status);
    assert!(
        status.contains("entries=3"),
        "two files and one directory:\n{status}"
    );
    assert!(status.contains("complete=true"));

    let check = s.run_ok(&["check", r.as_str()]);
    assert_all_labelled(&check);
    assert!(check.contains("consistent=true"), "got:\n{check}");
}

/// A4: repeating the read-only commands over an unchanged tree produces identical output.
#[test]
fn repeated_reads_of_an_unchanged_tree_are_identical() {
    let s = Sandbox::new();
    s.write("a.txt", "alpha");
    s.write("sub/b.txt", "beta");
    s.init_and_observe();
    let r = s.root().to_string_lossy().to_string();

    for cmd in [
        vec!["status", r.as_str()],
        vec!["changes", r.as_str()],
        vec!["show", r.as_str(), "a.txt"],
        vec!["check", r.as_str()],
    ] {
        let first = s.run_ok(&cmd);
        let second = s.run_ok(&cmd);
        let third = s.run_ok(&cmd);
        assert_eq!(first, second, "output of {cmd:?} was not stable");
        assert_eq!(second, third, "output of {cmd:?} was not stable");
    }
}

/// A4: two observations of an unchanged tree report the same observable state.
#[test]
fn two_observations_of_an_unchanged_tree_agree() {
    let s = Sandbox::new();
    s.write("a.txt", "alpha");
    s.write("sub/b.txt", "beta");
    s.init_and_observe();
    let r = s.root().to_string_lossy().to_string();

    let second = s.run_ok(&["observe", r.as_str()]);
    // Nothing changed, so nothing is read again: both readings are carried (`UD-036`).
    assert!(
        second.contains("content-fresh=0  content-reused=2"),
        "got:\n{second}"
    );
    assert!(
        second.contains("content-read-entries=0  content-read-bytes=0"),
        "got:\n{second}"
    );

    let changes = s.run_ok(&["changes", r.as_str()]);
    assert!(changes.contains("count  unchanged=3"), "got:\n{changes}");
    assert!(changes.contains("count  modified=0"), "got:\n{changes}");
    assert!(changes.contains("count  created=0"), "got:\n{changes}");
    assert!(changes.contains("count  deleted=0"), "got:\n{changes}");
}

/// Both runs are recorded. A re-observation with no changes still records a run, because
/// otherwise "verified just now" would be indistinguishable from "not verified since
/// Tuesday".
#[test]
fn a_no_change_observation_still_records_a_run() {
    let s = Sandbox::new();
    s.write("a.txt", "alpha");
    s.init_and_observe();
    let r = s.root().to_string_lossy().to_string();
    s.run_ok(&["observe", r.as_str()]);
    s.run_ok(&["observe", r.as_str()]);

    let check = s.run_ok(&["check", r.as_str()]);
    assert!(check.contains("log-runs=3"), "got:\n{check}");
}

// ---------------------------------------------------------------------------------------
// Exit codes
// ---------------------------------------------------------------------------------------

#[test]
fn a_missing_root_is_a_runtime_error() {
    let s = Sandbox::new();
    let missing = s.root().join("does-not-exist");
    let out = s.run(&["init", &missing.to_string_lossy()]);
    assert_eq!(out.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&out.stderr).contains("does not exist"));
}

#[test]
fn observing_an_uninitialised_workspace_is_a_runtime_error() {
    let s = Sandbox::new();
    s.write("a.txt", "alpha");
    let r = s.root().to_string_lossy().to_string();
    let out = s.run(&["observe", r.as_str()]);
    assert_eq!(out.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&out.stderr).contains("not initialised"));
}

#[test]
fn initialising_twice_refuses_rather_than_silently_resetting() {
    let s = Sandbox::new();
    s.write("a.txt", "alpha");
    let r = s.root().to_string_lossy().to_string();
    s.run_ok(&["init", r.as_str()]);
    s.run_ok(&["observe", r.as_str()]);

    let out = s.run(&["init", r.as_str()]);
    assert_eq!(out.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&out.stderr).contains("already initialised"));

    // And the history is still there, which is the point of refusing.
    assert!(s.run_ok(&["status", r.as_str()]).contains("last-run=1"));
}

#[test]
fn a_root_that_is_a_file_is_refused() {
    let s = Sandbox::new();
    s.write("a.txt", "alpha");
    let out = s.run(&["init", &s.root().join("a.txt").to_string_lossy()]);
    assert_eq!(out.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&out.stderr).contains("not a directory"));
}

#[test]
fn missing_arguments_and_unknown_commands_are_usage_errors() {
    let s = Sandbox::new();
    assert_eq!(s.run(&[]).status.code(), Some(2));
    assert_eq!(s.run(&["status"]).status.code(), Some(2));
    assert_eq!(s.run(&["show", "/tmp"]).status.code(), Some(2));
    assert_eq!(s.run(&["not-a-command", "/tmp"]).status.code(), Some(2));
}

/// An empty root is a legitimate, complete observation with zero entries.
#[test]
fn an_empty_root_observes_cleanly() {
    let s = Sandbox::new();
    let r = s.root().to_string_lossy().to_string();
    s.run_ok(&["init", r.as_str()]);
    let obs = s.run_ok(&["observe", r.as_str()]);
    assert!(obs.contains("entries=0"), "got:\n{obs}");
    assert!(obs.contains("complete=true"), "got:\n{obs}");

    let status = s.run_ok(&["status", r.as_str()]);
    assert!(status.contains("entries=0"));
    assert_all_labelled(&status);
}

/// `show` on a directory works, and reports its observation without inventing a hash.
#[test]
fn show_reports_a_directory_without_inventing_content() {
    let s = Sandbox::new();
    s.write("sub/b.txt", "beta");
    s.init_and_observe();
    let r = s.root().to_string_lossy().to_string();

    let out = s.run_ok(&["show", r.as_str(), "sub"]);
    assert_all_labelled(&out);
    assert!(out.contains("kind=dir"), "got:\n{out}");
    assert!(
        out.contains("hash=none"),
        "a directory has no content hash:\n{out}"
    );
    assert!(out.contains("stability=none"), "got:\n{out}");
}

/// D-V01-12. `workspaces` reads the canonical root back from the stored workspace record, so
/// it is not something the filesystem reported in this run: it is labelled `derived`. And it
/// must come back exactly — before the fix the record was written lossily and read without
/// unescaping, so a root containing `"` or `\` (or a byte that is not UTF-8) was listed wrong.
#[cfg(unix)]
#[test]
fn workspaces_lists_the_recorded_root_exactly_and_as_derived() {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;

    let s = Sandbox::new();
    let dir = s.root().join(OsStr::from_bytes(b"we\"ird\\name-\xFF"));
    std::fs::create_dir(&dir).unwrap();
    s.run_os_ok(&[OsStr::new("init"), dir.as_os_str()]);

    let canonical = std::fs::canonicalize(&dir).unwrap();
    let expected = umbral::report::render_path(&canonical).field("canonical");
    let out = s.run_ok(&["workspaces"]);
    assert_all_labelled(&out);
    assert!(
        out.contains(&format!("derived   {expected}\n")),
        "expected the exact root {expected}, got:\n{out}"
    );
    // And it reads back to the exact bytes (`umbral-output/1`).
    let parsed = umbral::contract::parse(&out).unwrap();
    let listed = parsed.iter().find_map(|l| l.field("canonical")).unwrap();
    assert_eq!(listed, canonical.as_os_str().as_bytes());
    assert!(
        !out.contains("observed"),
        "a stored value is not observed:\n{out}"
    );
}

/// D-V01-12. A record without a creation time says so instead of inventing 1970.
#[test]
fn workspaces_does_not_invent_a_missing_creation_time() {
    let s = Sandbox::new();
    let r = s.root().to_string_lossy().to_string();
    s.run_ok(&["init", r.as_str()]);

    let umbral_dir = s.data.path().join("umbral");
    let ws_dir = std::fs::read_dir(&umbral_dir)
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    let record = ws_dir.join("workspace.json");
    let text = std::fs::read_to_string(&record).unwrap();
    let edited: String = text
        .lines()
        .filter(|l| !l.contains("\"created_at_ns\""))
        .map(|l| format!("{l}\n"))
        .collect();
    std::fs::write(&record, edited).unwrap();

    let out = s.run_ok(&["workspaces"]);
    assert!(out.contains("created=unknown"), "got:\n{out}");
    assert!(!out.contains("1970"), "got:\n{out}");
}

/// D-V01-14. One workspace record that cannot be read or interpreted must not hide the others,
/// and must not vanish silently either: before the fix, a record that was not UTF-8 made
/// `workspaces` fail outright, and a record without a root was skipped without a trace.
#[test]
fn workspaces_reports_an_unreadable_record_and_still_lists_the_others() {
    let s = Sandbox::new();
    let r = s.root().to_string_lossy().to_string();
    s.run_ok(&["init", r.as_str()]);

    let umbral_dir = s.data.path().join("umbral");
    let garbled = umbral_dir.join("ws-garbled");
    std::fs::create_dir(&garbled).unwrap();
    std::fs::write(
        garbled.join("workspace.json"),
        b"{\"canonical\": \"\xFF\xFE\"}",
    )
    .unwrap();
    let rootless = umbral_dir.join("ws-rootless");
    std::fs::create_dir(&rootless).unwrap();
    std::fs::write(rootless.join("workspace.json"), "{}\n").unwrap();

    let out = s.run_ok(&["workspaces"]);
    assert_all_labelled(&out);
    umbral::contract::parse(&out).unwrap();
    let canonical = std::fs::canonicalize(s.root()).unwrap();
    let expected = umbral::report::render_path(&canonical).field("canonical");
    assert!(
        out.contains(&expected),
        "the readable workspace is still listed:\n{out}"
    );
    assert!(
        out.contains("unknown   workspace-id=garbled  reason=record-unreadable"),
        "got:\n{out}"
    );
    assert!(
        out.contains("unknown   workspace-id=rootless  reason=record-without-root"),
        "got:\n{out}"
    );
}
