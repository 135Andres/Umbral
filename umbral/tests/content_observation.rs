//! Content observation: the guarded read, and the one normative statement it implements —
//! a persisted hash represents exactly the bytes read during one valid observation.

use std::fs;
use std::path::Path;

use tempfile::TempDir;
use umbral::content::{observe_content, ContentError, Stability};

fn blake3_of(bytes: &[u8]) -> [u8; 32] {
    *blake3::hash(bytes).as_bytes()
}

#[test]
fn a_stable_read_hashes_exactly_the_bytes_on_disk() {
    let t = TempDir::new().unwrap();
    let p = t.path().join("f.txt");
    let body = b"the bytes that were read\n";
    fs::write(&p, body).unwrap();

    let c = observe_content(&p);
    assert_eq!(c.stability, Some(Stability::Stable));
    assert_eq!(c.error, None);
    assert_eq!(c.valid_hash(), Some(&blake3_of(body)));
    assert_eq!(c.hashed_len, Some(body.len() as u64));
    assert!(c.is_content_verified());
}

#[test]
fn an_empty_file_hashes_to_the_empty_input() {
    let t = TempDir::new().unwrap();
    let p = t.path().join("empty");
    fs::write(&p, b"").unwrap();

    let c = observe_content(&p);
    assert_eq!(c.valid_hash(), Some(&blake3_of(b"")));
    assert_eq!(c.hashed_len, Some(0));
}

#[test]
fn different_bytes_produce_different_hashes() {
    let t = TempDir::new().unwrap();
    let a = t.path().join("a");
    let b = t.path().join("b");
    fs::write(&a, b"one").unwrap();
    fs::write(&b, b"two").unwrap();
    assert_ne!(
        observe_content(&a).valid_hash(),
        observe_content(&b).valid_hash()
    );
}

#[test]
fn identical_bytes_produce_identical_hashes() {
    let t = TempDir::new().unwrap();
    let a = t.path().join("a");
    let b = t.path().join("b");
    fs::write(&a, b"same").unwrap();
    fs::write(&b, b"same").unwrap();
    assert_eq!(
        observe_content(&a).valid_hash(),
        observe_content(&b).valid_hash()
    );
}

/// Content larger than the streaming buffer is hashed correctly — the read never loads the
/// whole file, so this checks the streaming path end to end.
#[test]
fn large_content_streams_correctly() {
    let t = TempDir::new().unwrap();
    let p = t.path().join("big.bin");
    let body: Vec<u8> = (0..300_000u32).map(|i| (i % 251) as u8).collect();
    fs::write(&p, &body).unwrap();

    let c = observe_content(&p);
    assert_eq!(c.valid_hash(), Some(&blake3_of(&body)));
    assert_eq!(c.hashed_len, Some(body.len() as u64));
}

#[test]
fn a_directory_is_not_a_regular_file() {
    let t = TempDir::new().unwrap();
    let c = observe_content(t.path());
    assert_eq!(c.error, Some(ContentError::NotARegularFile));
    assert_eq!(c.valid_hash(), None);
    assert!(!c.is_content_verified());
}

#[cfg(unix)]
#[test]
fn a_symlink_is_never_resolved_to_its_target_content() {
    let t = TempDir::new().unwrap();
    let target = t.path().join("target.txt");
    fs::write(&target, b"target body").unwrap();
    let link = t.path().join("link");
    std::os::unix::fs::symlink(&target, &link).unwrap();

    let c = observe_content(&link);
    assert_eq!(c.error, Some(ContentError::NotARegularFile));
    assert_eq!(
        c.valid_hash(),
        None,
        "a symlink must not report its target's bytes"
    );
}

#[test]
fn a_missing_path_is_reported_as_missing() {
    let t = TempDir::new().unwrap();
    let c = observe_content(&t.path().join("nope"));
    assert_eq!(c.error, Some(ContentError::NotFound));
    assert_eq!(c.hash, None);
    assert_eq!(c.stability, None);
}

/// Only `Stable` exposes a hash. This is the single gate that keeps an invalid read from
/// becoming content evidence.
#[test]
fn valid_hash_is_none_for_every_non_stable_outcome() {
    let t = TempDir::new().unwrap();

    let missing = observe_content(&t.path().join("nope"));
    assert_eq!(missing.valid_hash(), None);

    let dir = observe_content(t.path());
    assert_eq!(dir.valid_hash(), None);

    let file = t.path().join("f");
    fs::write(&file, b"x").unwrap();
    let mut unstable = observe_content(&file);
    unstable.stability = Some(Stability::Unstable);
    assert_eq!(
        unstable.valid_hash(),
        None,
        "an unstable read must not expose a hash"
    );
}

#[test]
fn absent_metadata_is_absent_not_defaulted() {
    let t = TempDir::new().unwrap();
    let p = t.path().join("f");
    fs::write(&p, b"x").unwrap();
    // A path that cannot be read has no length and no stability, and neither is invented.
    let c = observe_content(&t.path().join("missing"));
    assert_eq!(c.hashed_len, None);
    assert_eq!(c.hash, None);
    assert_eq!(c.deltas, Vec::new());
}

/// The guard's documented blind spot, asserted so it cannot be forgotten: a rewrite that
/// restores size and exact mtime inside the read window is invisible to a stat guard.
/// This is a limitation of the mechanism, not a bug — the test records it.
#[test]
fn a_stat_guard_cannot_see_a_rewrite_that_restores_size_and_mtime() {
    let t = TempDir::new().unwrap();
    let p = t.path().join("f");
    fs::write(&p, b"AAAAAAAA").unwrap();

    let first = observe_content(&p);
    let original_mtime = fs::metadata(&p).unwrap().modified().unwrap();

    // Same length, different bytes, mtime restored.
    fs::write(&p, b"BBBBBBBB").unwrap();
    let times = filetime_restore(&p, original_mtime);
    if !times {
        // Restoring mtime is not possible here; skip rather than assert something untrue.
        eprintln!("note: could not restore mtime on this platform; blind spot not exercised");
        return;
    }

    let second = observe_content(&p);
    assert_eq!(second.stability, Some(Stability::Stable));
    assert_ne!(
        first.valid_hash(),
        second.valid_hash(),
        "the bytes differ, so comparing hashes across observations is what separates them"
    );
    // The point: the guard reported Stable, because size and mtime were unchanged.
    assert!(second.is_content_verified());
}

/// Best-effort mtime restore via `std::fs::File::set_modified` (Rust 1.75+).
fn filetime_restore(path: &Path, when: std::time::SystemTime) -> bool {
    let Ok(f) = fs::File::options().write(true).open(path) else {
        return false;
    };
    f.set_modified(when).is_ok()
}
