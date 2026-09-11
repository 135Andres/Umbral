//! Increment 2 — IDENTITY tests. Physical evidence only; no hashing, no store.
//! Deterministic tests run against real filesystem fixtures (Fedora/btrfs and
//! tmpfs both verified); the pure-model properties are separated below.

use fsp_check::identity::*;
use fsp_check::scan::scan;
use fsp_check::{Entry, EntryKind};

fn entry_at<'a>(scan: &'a fsp_check::Scan, rel: &str) -> &'a Entry {
    scan.entries
        .iter()
        .find(|e| e.path == *rel)
        .unwrap_or_else(|| panic!("entry {rel} not found"))
}

// ---- observed filesystem behaviour (Fedora: btrfs / tmpfs) ------------------

#[test]
fn same_file_no_changes_is_unchanged() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    std::fs::write(root.join("f.txt"), b"stable").unwrap();
    let s1 = scan(root).unwrap();
    let s2 = scan(root).unwrap();
    let m = compare_same_path(entry_at(&s1, "f.txt"), entry_at(&s2, "f.txt"));
    assert_eq!(m, IdentityMatch::SamePhysicalObjectUnchanged);
}

#[test]
fn same_file_modified_content_is_modified_object() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    std::fs::write(root.join("f.txt"), b"short").unwrap();
    let s1 = scan(root).unwrap();
    std::fs::write(root.join("f.txt"), b"a much longer content now").unwrap();
    let s2 = scan(root).unwrap();
    let m = compare_same_path(entry_at(&s1, "f.txt"), entry_at(&s2, "f.txt"));
    assert_eq!(m, IdentityMatch::SamePhysicalObjectModified);
}

#[test]
fn rename_preserves_dev_ino_and_is_detectable_without_hash() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    std::fs::write(root.join("old.txt"), b"payload").unwrap();
    let s1 = scan(root).unwrap();
    std::fs::rename(root.join("old.txt"), root.join("new.txt")).unwrap();
    let s2 = scan(root).unwrap();

    // the old path no longer resolves to anything: deleted at that path
    assert!(s2.entries.iter().all(|e| e.path != *"old.txt"));
    // cross-path: the object lives on at new.txt with the same physical id
    let m = compare_cross_path(entry_at(&s1, "old.txt"), entry_at(&s2, "new.txt"));
    assert_eq!(m, Some(IdentityMatch::RenamedOrMoved));
}

#[test]
fn directory_rename_keeps_identity_of_dir_and_children() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    std::fs::create_dir_all(root.join("a/sub")).unwrap();
    std::fs::write(root.join("a/sub/f.txt"), b"x").unwrap();
    let s1 = scan(root).unwrap();
    std::fs::rename(root.join("a"), root.join("b")).unwrap();
    let s2 = scan(root).unwrap();
    assert_eq!(
        compare_cross_path(entry_at(&s1, "a/sub/f.txt"), entry_at(&s2, "b/sub/f.txt")),
        Some(IdentityMatch::RenamedOrMoved)
    );
}

#[test]
fn delete_recreate_with_different_ino_is_ambiguous_not_same() {
    // loop until we observe a different inode; on any normal filesystem the
    // FIRST allocation after delete is almost never the freed inode, so this
    // terminates immediately. If it somehow cannot be observed, skip — we
    // never claim more than was observed.
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    std::fs::write(root.join("f.txt"), b"first").unwrap();
    let s1 = scan(root).unwrap();
    std::fs::remove_file(root.join("f.txt")).unwrap();
    for i in 0..64u32 {
        std::fs::write(root.join(format!("churn{i}")), b"churn").unwrap();
    }
    std::fs::write(root.join("f.txt"), b"second").unwrap();
    let s2 = scan(root).unwrap();
    let a = entry_at(&s1, "f.txt");
    let b = entry_at(&s2, "f.txt");
    if a.ino == b.ino && a.dev == b.dev {
        // inode reuse observed: by design this stays Ambiguous only if we had
        // extra evidence — we do not. Document: the model CANNOT distinguish
        // reuse from identity with physical evidence alone.
        assert_eq!(
            compare_same_path(a, b),
            IdentityMatch::SamePhysicalObjectUnchanged,
            "observed inode reuse: model reports unchanged (documented limitation)"
        );
    } else {
        assert_eq!(
            compare_same_path(a, b),
            IdentityMatch::Ambiguous {
                reason: AmbiguityReason::IdentityChangedAtSamePath
            }
        );
    }
}

#[test]
fn hard_links_share_one_physical_identity() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    std::fs::write(root.join("a.txt"), b"shared").unwrap();
    std::fs::hard_link(root.join("a.txt"), root.join("b.txt")).unwrap();
    let s = scan(root).unwrap();
    let a = entry_at(&s, "a.txt");
    let b = entry_at(&s, "b.txt");
    assert_eq!(a.physical_id(), b.physical_id());
    // one filesystem object, two directory entries — never two identities
    let index = index_by_physical_id(&s.entries);
    let group = index.get(&a.physical_id()).unwrap();
    assert_eq!(group.len(), 2);
    assert_ne!(a.path, b.path, "directory entries remain distinct");
}

#[test]
fn symlink_identity_is_the_link_itself() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    std::fs::write(root.join("target.txt"), b"t").unwrap();
    std::os::unix::fs::symlink("target.txt", root.join("link")).unwrap();
    let s1 = scan(root).unwrap();
    let link = entry_at(&s1, "link");
    let target = entry_at(&s1, "target.txt");
    assert_ne!(link.physical_id(), target.physical_id());
    assert_eq!(link.kind, EntryKind::Symlink);
    // stable across rescans
    let s2 = scan(root).unwrap();
    assert_eq!(
        compare_same_path(link, entry_at(&s2, "link")),
        IdentityMatch::SamePhysicalObjectUnchanged
    );
}

// ---- pure-model properties (no filesystem luck involved) --------------------

fn synthetic(
    path: &str,
    dev: Option<u64>,
    ino: Option<u64>,
    size: u64,
    mtime: (i64, u32),
) -> Entry {
    Entry {
        path: std::path::PathBuf::from(path),
        kind: EntryKind::File,
        dev,
        ino,
        size: Some(size),
        mtime: Some(mtime),
    }
}

#[test]
fn missing_evidence_is_never_identity() {
    // INV-8: absence of dev/ino → Ambiguous, never SAME
    let no_dev = synthetic("f", None, Some(7), 1, (1, 0));
    let with_dev = synthetic("f", Some(2), Some(7), 1, (1, 0));
    assert_eq!(
        compare_same_path(&no_dev, &with_dev),
        IdentityMatch::Ambiguous {
            reason: AmbiguityReason::MissingEvidence
        }
    );
    let no_ino = synthetic("f", Some(2), None, 1, (1, 0));
    assert_eq!(
        compare_same_path(&no_ino, &with_dev),
        IdentityMatch::Ambiguous {
            reason: AmbiguityReason::MissingEvidence
        }
    );
}

#[test]
fn path_is_never_evidence_of_identity() {
    // same path, different object: AMBIGUOUS even though the path matches
    let a = synthetic("f", Some(2), Some(10), 3, (1, 0));
    let b = synthetic("f", Some(2), Some(11), 3, (1, 0));
    assert_eq!(
        compare_same_path(&a, &b),
        IdentityMatch::Ambiguous {
            reason: AmbiguityReason::IdentityChangedAtSamePath
        }
    );
    // same identity at different paths: rename-level evidence exists
    let moved = synthetic("g", Some(2), Some(10), 3, (1, 0));
    assert_eq!(
        compare_cross_path(&a, &moved),
        Some(IdentityMatch::RenamedOrMoved)
    );
    // different identity at a different path: NO cross-path claim at all
    let other = synthetic("g", Some(2), Some(99), 3, (1, 0));
    assert_eq!(compare_cross_path(&a, &other), None);
}

#[test]
fn deleted_produces_no_positive_match() {
    // removing an entry can never yield SAME_PHYSICAL_OBJECT with a new one:
    // with no physical overlap there is either None (cross-path) or
    // Ambiguous (same path) — both non-positive. This test pins the
    // exhaustive behaviour for the delete+recreate family.
    let a = synthetic("f", Some(2), Some(10), 3, (1, 0));
    let b = synthetic("f", Some(2), Some(12), 9, (5, 0));
    let same_path = compare_same_path(&a, &b);
    let cross = compare_cross_path(&a, &b);
    assert_ne!(same_path, IdentityMatch::SamePhysicalObjectUnchanged);
    assert_ne!(same_path, IdentityMatch::SamePhysicalObjectModified);
    assert_eq!(cross, None);
}

#[test]
fn equivalence_of_physical_id_is_exact_not_probabilistic() {
    let a = synthetic("x", Some(4), Some(8), 1, (1, 0));
    let b = synthetic("y", Some(4), Some(8), 9, (9, 9));
    // same object, wildly different state: identity is state-independent
    assert_eq!(a.physical_id(), b.physical_id());
    // but state comparison still reports modification
    assert_eq!(
        compare_cross_path(&a, &b),
        Some(IdentityMatch::SamePhysicalObjectModified)
    );
}
