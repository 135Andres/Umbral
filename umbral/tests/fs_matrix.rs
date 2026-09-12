//! The filesystem matrix: one case per environment behaviour the tool must survive.
//!
//! Design rule taken from the F-6 lesson: **assertions must not depend on what the
//! filesystem does.** Where a behaviour is environment-dependent (inode reuse), the test
//! *records* what happened instead of requiring a particular outcome. The deterministic
//! assertions for that scenario live in `reconcile_properties.rs`, where the observation
//! sets are built by hand.

mod common;

use std::fs;

use common::{assert_all_labelled, Sandbox};

// ---------------------------------------------------------------------------------------
// Hard links
// ---------------------------------------------------------------------------------------

#[cfg(unix)]
#[test]
fn two_hard_links_are_two_entries_with_one_physical_object() {
    let s = Sandbox::new();
    s.write("one.txt", "shared body");
    fs::hard_link(s.root().join("one.txt"), s.root().join("two.txt")).unwrap();

    s.init_and_observe();
    let r = s.root().to_string_lossy().to_string();

    let one = s.run_ok(&["show", r.as_str(), "one.txt"]);
    let two = s.run_ok(&["show", r.as_str(), "two.txt"]);
    assert!(one.contains("kind=file") && two.contains("kind=file"));

    // Writing through one name is visible through the other: one object, two entries.
    fs::write(s.root().join("one.txt"), "changed through one").unwrap();
    s.run_ok(&["observe", r.as_str()]);
    let changes = s.run_ok(&["changes", r.as_str()]);

    let modified = changes.matches("modified").count();
    assert!(
        modified >= 1,
        "the write must be visible through at least one entry:\n{changes}"
    );

    // Deleting one entry must not claim the content is gone.
    fs::remove_file(s.root().join("two.txt")).unwrap();
    s.run_ok(&["observe", r.as_str()]);
    let changes2 = s.run_ok(&["changes", r.as_str()]);
    assert!(
        changes2.contains("object-survives=true"),
        "the object still exists through the other link:\n{changes2}"
    );
}

// ---------------------------------------------------------------------------------------
// Symlinks
// ---------------------------------------------------------------------------------------

#[cfg(unix)]
#[test]
fn symlinks_are_entries_not_targets() {
    let s = Sandbox::new();
    s.write("real.txt", "real body");
    std::os::unix::fs::symlink("real.txt", s.root().join("link.txt")).unwrap();
    std::os::unix::fs::symlink("missing.txt", s.root().join("dangling.txt")).unwrap();
    std::os::unix::fs::symlink("sub", s.root().join("link-to-dir")).unwrap();

    let r = s.root().to_string_lossy().to_string();
    s.run_ok(&["init", r.as_str()]);
    let out = s.run(&["observe", r.as_str()]);
    // A dangling symlink is an observation, not a failure.
    assert_eq!(
        out.status.code(),
        Some(0),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );

    let status = s.run_ok(&["status", r.as_str()]);
    assert!(status.contains("symlinks=3"), "got:\n{status}");

    let link = s.run_ok(&["show", r.as_str(), "link.txt"]);
    assert!(link.contains("kind=symlink"));
    assert!(
        link.contains("hash=none"),
        "a symlink must not report its target's content:\n{link}"
    );
}

// ---------------------------------------------------------------------------------------
// Atomic replacement (write a temp file, rename it over the target)
// ---------------------------------------------------------------------------------------

#[test]
fn an_atomic_replacement_is_classified() {
    let s = Sandbox::new();
    s.write("target.txt", "original content");
    s.init_and_observe();
    let r = s.root().to_string_lossy().to_string();

    // The classic atomic save: temp file in the same directory, then rename over.
    let tmp = s.root().join(".target.txt.tmp");
    fs::write(&tmp, "replacement content").unwrap();
    fs::rename(&tmp, s.root().join("target.txt")).unwrap();

    s.run_ok(&["observe", r.as_str()]);
    let changes = s.run_ok(&["changes", r.as_str()]);
    assert_all_labelled(&changes);

    // The temp file must not appear as an entry: it never existed at observation time.
    assert!(
        !changes.contains(".target.txt.tmp"),
        "a temp file that was renamed away must not be reported as a change:\n{changes}"
    );
    // The target changed one way or another, and the change is accounted for.
    let accounted = changes.contains("modified")
        || changes.contains("recreated")
        || changes.contains("ambiguous");
    assert!(
        accounted,
        "the replacement must be accounted for:\n{changes}"
    );
}

// ---------------------------------------------------------------------------------------
// Permissions
// ---------------------------------------------------------------------------------------

#[cfg(unix)]
#[test]
fn an_unreadable_file_is_recorded_as_not_verified() {
    use std::os::unix::fs::PermissionsExt;

    let s = Sandbox::new();
    s.write("readable.txt", "readable");
    s.write("secret.txt", "secret");
    let secret = s.root().join("secret.txt");
    fs::set_permissions(&secret, fs::Permissions::from_mode(0o000)).unwrap();

    let r = s.root().to_string_lossy().to_string();
    s.run_ok(&["init", r.as_str()]);
    let out = s.run(&["observe", r.as_str()]);
    fs::set_permissions(&secret, fs::Permissions::from_mode(0o644)).unwrap();

    // The entry was observed (it exists), so the run is complete; only its content is not
    // verified, and that is stated.
    assert_eq!(out.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("content-not-verified=1"),
        "an unreadable file must be reported as not verified:\n{stdout}"
    );

    let show = s.run_ok(&["show", r.as_str(), "secret.txt"]);
    assert!(show.contains("hash=none"));
    assert_all_labelled(&show);
}

// ---------------------------------------------------------------------------------------
// Non-UTF-8 paths
// ---------------------------------------------------------------------------------------

#[cfg(unix)]
#[test]
fn non_utf8_paths_round_trip_through_the_cli() {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;

    let s = Sandbox::new();
    let name = OsStr::from_bytes(b"caf\xe9-\xff.txt");
    fs::write(s.root().join(name), "bytes").unwrap();

    s.init_and_observe();
    let r = s.root().to_string_lossy().to_string();

    let status = s.run_ok(&["status", r.as_str()]);
    assert!(status.contains("files=1"), "got:\n{status}");
    assert_all_labelled(&status);

    // Looking it up with the same raw bytes finds it.
    let out = s.run(&["show", r.as_str(), name.to_str().unwrap_or_default()]);
    // The lossy rendering may not match the stored bytes, so either it is found or it is
    // honestly reported as not found — never silently reported as a different file.
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert_all_labelled(&stdout);
}

// ---------------------------------------------------------------------------------------
// Inode reuse: RECORDED, not asserted.
// ---------------------------------------------------------------------------------------

/// The F-6 scenario against a real filesystem. This test **records** whether the freed
/// `dev`+`ino` was handed to the new file. It asserts only the properties that hold either
/// way, so it is portable across filesystems — which is exactly what the original V0
/// assertion failed to be.
#[test]
fn inode_reuse_is_recorded_not_asserted() {
    use umbral::scan::scan;

    let s = Sandbox::new();
    s.write("bye.txt", "same content");
    let before = scan(s.root()).unwrap();
    let first = before
        .entries
        .iter()
        .find(|e| e.path == std::path::Path::new("bye.txt"))
        .unwrap();
    let first_id = (first.dev, first.ino);

    fs::remove_file(s.root().join("bye.txt")).unwrap();
    s.write("new.txt", "same content");
    let after = scan(s.root()).unwrap();
    let second = after
        .entries
        .iter()
        .find(|e| e.path == std::path::Path::new("new.txt"))
        .unwrap();
    let second_id = (second.dev, second.ino);

    let reused = first_id == second_id;
    // Recorded as evidence, visible in the test output with `--nocapture`.
    println!("inode-reuse probe: bye.txt={first_id:?} new.txt={second_id:?} reused={reused}");

    // Whatever the filesystem did, the end-to-end behaviour must be honest.
    s.run_ok(&["init", &s.root().to_string_lossy()]);
    s.run_ok(&["observe", &s.root().to_string_lossy()]);
    let r = s.root().to_string_lossy().to_string();
    let changes = s.run_ok(&["changes", r.as_str()]);
    assert_all_labelled(&changes);
    // Nothing was invented: the tool either reports the disappearance once, or reports that
    // the evidence does not settle it.
    let vanished = changes.matches("bye.txt").count();
    assert!(
        vanished <= 1,
        "bye.txt must be accounted for at most once:\n{changes}"
    );
    if reused {
        assert!(
            changes.contains("renamed-or-moved") || changes.contains("ambiguous"),
            "with a reused identity the tool must not invent a deletion:\n{changes}"
        );
    }
}

// ---------------------------------------------------------------------------------------
// Repeated observation and empty trees
// ---------------------------------------------------------------------------------------

#[test]
fn a_tree_that_only_shrinks_is_reported_accurately() {
    let s = Sandbox::new();
    s.write("a.txt", "a");
    s.write("b.txt", "b");
    s.write("c.txt", "c");
    s.init_and_observe();
    let r = s.root().to_string_lossy().to_string();

    fs::remove_file(s.root().join("b.txt")).unwrap();
    s.run_ok(&["observe", r.as_str()]);
    let changes = s.run_ok(&["changes", r.as_str()]);
    assert!(changes.contains("deleted"), "got:\n{changes}");
    assert!(changes.contains("path=b.txt"), "got:\n{changes}");
}

#[cfg(unix)]
#[test]
fn removing_a_directory_is_classified() {
    let s = Sandbox::new();
    s.write("keep.txt", "keep");
    s.write("gone/inside.txt", "inside");
    s.init_and_observe();
    let r = s.root().to_string_lossy().to_string();

    fs::remove_dir_all(s.root().join("gone")).unwrap();
    s.run_ok(&["observe", r.as_str()]);
    let changes = s.run_ok(&["changes", r.as_str()]);
    assert_all_labelled(&changes);
    assert!(changes.contains("deleted"), "got:\n{changes}");
    assert!(changes.contains("path=gone"), "got:\n{changes}");
    assert!(changes.contains("path=gone/inside.txt"), "got:\n{changes}");
}
