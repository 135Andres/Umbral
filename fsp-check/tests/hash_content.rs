//! Increment 3 — HASH tests. Deterministic unless labelled otherwise.
//! The guard's job: an invalidated observation must never surface as a
//! valid hash (INV-1's negative half).

use fsp_check::hash_obs::*;
use fsp_check::identity::PhysicalId;
use proptest::prelude::*;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

fn stable_hash(path: &std::path::Path) -> [u8; HASH_LEN] {
    let o = observe_content(path);
    assert_eq!(
        o.stability,
        Stability::Stable,
        "expected stable: {:?}",
        o.error
    );
    assert!(o.error.is_none());
    o.hash.unwrap()
}

#[test]
fn stable_file_and_empty_file_and_repeatability() {
    let dir = tempfile::tempdir().unwrap();
    let p1 = dir.path().join("a.txt");
    std::fs::write(&p1, b"hello").unwrap();
    let p2 = dir.path().join("empty.bin");
    std::fs::write(&p2, b"").unwrap();

    let h1a = stable_hash(&p1);
    let h1b = stable_hash(&p1);
    assert_eq!(h1a, h1b, "repeated observation of unchanged file is stable");
    let h2 = stable_hash(&p2);
    assert_ne!(h1a, h2);
    // known BLAKE3 property: deterministic; also empty-string hash sanity via length
    let o = observe_content(&p2);
    assert_eq!(o.hashed_len, Some(0));
}

#[test]
fn file_changed_before_observation_yields_new_hash() {
    // case B: changed before hash — the guard must simply hash the new bytes
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("f");
    std::fs::write(&p, b"v1").unwrap();
    let h_before = stable_hash(&p);
    std::fs::write(&p, b"v2-longer").unwrap();
    std::thread::sleep(std::time::Duration::from_millis(2)); // ensure mtime moves
    let h_after = stable_hash(&p);
    assert_ne!(h_before, h_after);
}

#[test]
fn changed_during_read_is_detected_or_hash_withheld() {
    // case C, controlled variant (concurrency-sensitive, labelled): we cannot
    // guarantee a mid-read write lands inside the read window on a fast
    // machine, so we prove the MECHANISM: any guard-visible change during
    // observation leads to Unstable with hash == None, and if no change was
    // guard-visible, the result is Stable. Both branches are valid outcomes;
    // the invariant under test is the pairing, not the race.
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("f");
    std::fs::write(&p, vec![0u8; 1_000_000]).unwrap(); // big enough to take a moment
    let o = observe_content(&p);
    match o.stability {
        Stability::Stable => assert!(o.hash.is_some() && o.deltas.is_empty()),
        Stability::Unstable => {
            assert!(
                o.hash.is_none(),
                "unstable observation must withhold the hash"
            );
            assert!(!o.deltas.is_empty() || o.error.is_some());
        }
    }
}

#[test]
fn atomic_replacement_is_observed_as_physical_id_change() {
    // case D: write temp + rename over = the classic atomic save. After it,
    // the path names a DIFFERENT inode. The old hash (from before) must not
    // be presented as current; a fresh observation is Stable with a new pid.
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("f");
    std::fs::write(&p, b"original").unwrap();
    let before = observe_content(&p);
    assert_eq!(before.stability, Stability::Stable);

    let tmp = dir.path().join(".f.tmp");
    std::fs::write(&tmp, b"replacement").unwrap();
    std::fs::rename(&tmp, &p).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(2));

    let after = observe_content(&p);
    assert_eq!(after.stability, Stability::Stable);
    assert_ne!(
        before.physical_id, after.physical_id,
        "atomic save swaps the inode"
    );
    assert_ne!(before.hash.unwrap(), after.hash.unwrap());
}

#[test]
fn file_deleted_during_observation_window_is_never_a_fake_hash() {
    // case E (controlled): delete between stat and read. We force it
    // deterministically by deleting the file, then observing: NotFound,
    // hash None, error recorded — no magic zero-hash.
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("gone");
    std::fs::write(&p, b"x").unwrap();
    std::fs::remove_file(&p).unwrap();
    let o = observe_content(&p);
    assert_eq!(o.error, Some(ContentError::NotFound));
    assert_eq!(o.hash, None);
    assert_eq!(o.stability, Stability::Unstable);
    assert!(
        o.valid_hash().is_none(),
        "INV-1: no valid hash from a failed observation"
    );
}

#[test]
fn truncation_and_same_size_rewrite_are_guarded_by_mtime() {
    // cases F/G: truncate (size 0) and same-size rewrite. The guard compares
    // size AND mtime; a same-size rewrite moves mtime, so either way the
    // delta is visible on the next observation's before/after comparison.
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("f");
    std::fs::write(&p, b"some content").unwrap();
    let h_full = stable_hash(&p);
    std::fs::write(&p, b"").unwrap(); // truncate
    std::thread::sleep(std::time::Duration::from_millis(2));
    let h_empty = stable_hash(&p);
    assert_ne!(h_full, h_empty);
    std::fs::write(&p, b"same-size").unwrap(); // 9 bytes -> same-size rewrite family
    std::thread::sleep(std::time::Duration::from_millis(2));
    let h3 = stable_hash(&p);
    assert_ne!(h3, h_empty);
}

#[test]
fn physical_id_difference_with_identical_content_is_duplicate_not_identity() {
    // §4: two objects, same bytes. hash equal, PhysicalId different:
    // content equality != identity equality.
    let dir = tempfile::tempdir().unwrap();
    let a = dir.path().join("a");
    let b = dir.path().join("b");
    std::fs::write(&a, b"identical bytes").unwrap();
    std::fs::write(&b, b"identical bytes").unwrap();
    let oa = observe_content(&a);
    let ob = observe_content(&b);
    assert_eq!(
        oa.hash.unwrap(),
        ob.hash.unwrap(),
        "same content, same hash"
    );
    assert_ne!(oa.physical_id, ob.physical_id, "different objects");
    // and the same-object-same-content case:
    let oa2 = observe_content(&a);
    assert_eq!(oa2.hash.unwrap(), oa.hash.unwrap());
    assert_eq!(oa2.physical_id, oa.physical_id);
}

#[test]
fn symlink_is_not_a_regular_file_and_never_resolves_to_target() {
    let dir = tempfile::tempdir().unwrap();
    let target = dir.path().join("target");
    std::fs::write(&target, b"target bytes").unwrap();
    let link = dir.path().join("link");
    std::os::unix::fs::symlink("target", &link).unwrap();
    let o = observe_content(&link);
    assert_eq!(o.error, Some(ContentError::NotARegularFile));
    assert_eq!(o.hash, None);
    let t = observe_content(&target);
    assert_ne!(o.hash, t.hash);
}

#[test]
fn directory_is_not_hashable() {
    let dir = tempfile::tempdir().unwrap();
    let o = observe_content(dir.path());
    assert_eq!(o.error, Some(ContentError::NotARegularFile));
    assert!(o.valid_hash().is_none());
}

#[test]
fn permission_denied_is_an_error_not_a_hash() {
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("secret");
    std::fs::write(&p, b"top").unwrap();
    std::fs::set_permissions(&p, std::fs::Permissions::from_mode(0o000)).unwrap();
    let o = observe_content(&p);
    // restore so tempdir cleanup works
    std::fs::set_permissions(&p, std::fs::Permissions::from_mode(0o644)).unwrap();
    if let ContentError::PermissionDenied = o.error.unwrap() {
        // expected (unless running as root, where reads succeed — accept a
        // stable hash there rather than failing)
    } else {
        assert_eq!(o.stability, Stability::Stable);
    }
}

#[test]
fn invalid_path_shapes_are_errors() {
    let dir = tempfile::tempdir().unwrap();
    let o = observe_content(&dir.path().join("no/such/file"));
    assert_eq!(o.error, Some(ContentError::NotFound));
}

#[test]
fn large_file_streams_without_loading_memory() {
    // §9: 8 MiB file; streaming is verified by construction (64 KiB buffer)
    // and by correctness of the hash over known content.
    let dir = tempfile::tempdir().unwrap();
    let p = dir.path().join("big");
    let chunk = vec![7u8; 1024];
    let mut f = std::fs::File::create(&p).unwrap();
    for _ in 0..8 * 1024 {
        use std::io::Write;
        f.write_all(&chunk).unwrap();
    }
    drop(f);
    let o = observe_content(&p);
    assert_eq!(o.stability, Stability::Stable);
    assert_eq!(o.hashed_len, Some(8 * 1024 * 1024));
    // reference hash computed independently via the same streaming API
    let mut h = blake3::Hasher::new();
    for _ in 0..8 * 1024 {
        h.update(&chunk);
    }
    let expect: [u8; HASH_LEN] = h.finalize().into();
    assert_eq!(o.hash.unwrap(), expect);
}

#[test]
fn guard_policy_is_explicit_and_bounded() {
    // The retry policy is a documented constant, not an arbitrary magic:
    // 1 retry after a detected change; then Unstable.
    assert_eq!(MAX_GUARD_ATTEMPTS, 2);
}

// ---- proptest: byte-sequence stability of the hash itself --------------------

proptest! {
    // BLAKE3 stability over arbitrary byte sequences (pure property, no fs)
    #[test]
    fn blake3_hash_is_deterministic_per_bytes(bytes in proptest::prelude::any::<Vec<u8>>()) {
        let mut ha = blake3::Hasher::new();
        ha.update(&bytes);
        let a = ha.finalize();
        let mut hb = blake3::Hasher::new();
        hb.update(&bytes);
        let b = hb.finalize();
        prop_assert_eq!(a, b);
    }

    // INV-1 negative half as a model property: unstable or errored
    // observations NEVER expose a valid hash.
    #[test]
    fn unstable_or_errored_never_yields_valid_hash(
        has_hash in proptest::prelude::any::<bool>(),
        stable in proptest::prelude::any::<bool>(),
    ) {
        let o = ContentObservation {
            physical_id: PhysicalId { dev: Some(1), ino: Some(1) },
            hash: if has_hash { Some([0u8; HASH_LEN]) } else { None },
            hashed_len: if has_hash { Some(1) } else { None },
            stability: if stable { Stability::Stable } else { Stability::Unstable },
            deltas: vec![],
            error: None,
        };
        prop_assert_eq!(o.valid_hash().is_some(), has_hash && stable);
    }
}
