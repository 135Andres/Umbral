//! Deterministic, read-only scanning. Covers acceptance criterion A4.

use std::fs;
use std::path::Path;

use tempfile::TempDir;
use umbral::scan::{scan, EntryKind, ScanError};

fn write(root: &Path, rel: &str, body: &str) {
    let p = root.join(rel);
    if let Some(parent) = p.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(p, body).unwrap();
}

fn tree() -> TempDir {
    let t = TempDir::new().unwrap();
    write(t.path(), "a.txt", "a");
    write(t.path(), "sub/b.txt", "b");
    write(t.path(), "sub/deep/c.txt", "c");
    write(t.path(), "z.log", "z");
    t
}

/// A4: two scans of an unchanged tree are equivalent in observable state.
#[test]
fn rescan_of_unchanged_tree_is_equivalent() {
    let t = tree();
    let a = scan(t.path()).unwrap();
    let b = scan(t.path()).unwrap();
    assert!(
        a.equivalent_observable_state(&b),
        "observable state differed between two scans"
    );
    assert!(a.complete() && b.complete());
}

/// A4: the order entries are created in does not change the observed order.
#[test]
fn entry_order_is_independent_of_creation_order() {
    let t1 = TempDir::new().unwrap();
    for name in ["m", "a", "z", "b"] {
        write(t1.path(), name, "x");
    }
    let t2 = TempDir::new().unwrap();
    for name in ["z", "b", "a", "m"] {
        write(t2.path(), name, "x");
    }
    let s1 = scan(t1.path()).unwrap();
    let s2 = scan(t2.path()).unwrap();
    let p1: Vec<_> = s1.entries.iter().map(|e| e.path.clone()).collect();
    let p2: Vec<_> = s2.entries.iter().map(|e| e.path.clone()).collect();
    assert_eq!(p1, p2);
}

/// The root itself is not an entry, and an empty root is a complete scan with no entries.
#[test]
fn empty_root_yields_no_entries_and_is_complete() {
    let t = TempDir::new().unwrap();
    let s = scan(t.path()).unwrap();
    assert!(s.entries.is_empty());
    assert!(s.errors.is_empty());
    assert!(s.complete());
}

#[test]
fn missing_root_is_reported_not_invented() {
    let t = TempDir::new().unwrap();
    let missing = t.path().join("nope");
    match scan(&missing) {
        Err(ScanError::RootMissing(p)) => assert_eq!(p, missing),
        other => panic!("expected RootMissing, got {other:?}"),
    }
}

#[test]
fn a_file_root_is_not_a_directory() {
    let t = TempDir::new().unwrap();
    write(t.path(), "file.txt", "x");
    match scan(&t.path().join("file.txt")) {
        Err(ScanError::NotADirectory(_)) => {}
        other => panic!("expected NotADirectory, got {other:?}"),
    }
}

/// Symlinks are observed as themselves and never traversed.
#[test]
fn symlinks_are_observed_not_traversed() {
    let t = TempDir::new().unwrap();
    write(t.path(), "real/file.txt", "content");
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink("real", t.path().join("link-to-dir")).unwrap();
        std::os::unix::fs::symlink("nowhere", t.path().join("dangling")).unwrap();
    }

    let s = scan(t.path()).unwrap();
    let link = s
        .entries
        .iter()
        .find(|e| e.path == Path::new("link-to-dir"))
        .unwrap();
    assert_eq!(link.kind, EntryKind::Symlink);
    // Not traversed: the target's children do not appear under the link.
    //
    // NOTE: `Path::new("link-to-dir/")` has ONE component and matches the entry
    // `link-to-dir` itself, so the containment test must compare against the bare path and
    // exclude the entry explicitly. This is the third occurrence of this trap in this
    // project; it is called out here because it is exactly the kind of mistake that looks
    // like a passing assertion.
    let link_path = Path::new("link-to-dir");
    assert!(
        !s.entries
            .iter()
            .any(|e| e.path != link_path && e.path.starts_with(link_path)),
        "the symlink target must not be traversed"
    );

    let dangling = s
        .entries
        .iter()
        .find(|e| e.path == Path::new("dangling"))
        .unwrap();
    assert_eq!(dangling.kind, EntryKind::Symlink);
    // A dangling symlink is an observation, not an error.
    assert!(s.complete());
}

/// Non-UTF-8 filenames survive byte-exactly.
#[cfg(unix)]
#[test]
fn non_utf8_names_are_preserved_byte_exact() {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;

    let t = TempDir::new().unwrap();
    let raw = OsStr::from_bytes(b"caf\xe9-\xff\xfe.txt");
    let path = t.path().join(raw);
    fs::write(&path, "x").unwrap();

    let s = scan(t.path()).unwrap();
    assert_eq!(s.entries.len(), 1);
    assert_eq!(
        s.entries[0].path.as_os_str().as_bytes(),
        b"caf\xe9-\xff\xfe.txt"
    );
}

/// An unreadable directory makes the scan incomplete, and the failure is represented.
#[cfg(unix)]
#[test]
fn unreadable_directory_makes_the_scan_incomplete() {
    use std::os::unix::fs::PermissionsExt;

    let t = TempDir::new().unwrap();
    write(t.path(), "visible.txt", "x");
    write(t.path(), "blocked/hidden.txt", "y");
    let blocked = t.path().join("blocked");
    fs::set_permissions(&blocked, fs::Permissions::from_mode(0o000)).unwrap();

    let s = scan(t.path()).unwrap();

    // Restore before asserting so a failure cannot leave an unremovable tree behind.
    fs::set_permissions(&blocked, fs::Permissions::from_mode(0o755)).unwrap();

    assert!(
        !s.complete(),
        "a scan with an unobservable path must not report complete"
    );
    assert!(s.errors.iter().any(|e| e.path == Path::new("blocked")));
    assert!(s.entries.iter().any(|e| e.path == Path::new("visible.txt")));
}

/// A4 / F-6 lesson: a rescan after create+delete is still deterministic, whatever the
/// filesystem does with inode numbers. The assertion is about observable state, not about
/// inode allocation.
#[test]
fn rescan_after_delete_is_deterministic() {
    let t = TempDir::new().unwrap();
    write(t.path(), "keep.txt", "keep");
    write(t.path(), "gone.txt", "gone");
    let before = scan(t.path()).unwrap();

    fs::remove_file(t.path().join("gone.txt")).unwrap();
    let after1 = scan(t.path()).unwrap();
    let after2 = scan(t.path()).unwrap();

    assert!(after1.equivalent_observable_state(&after2));
    assert_eq!(before.entries.len(), 2);
    assert_eq!(after1.entries.len(), 1);
}
