use fsp_check::EntryKind;
use fsp_check::scan::scan;

fn path_of(p: &std::path::Path) -> std::path::PathBuf {
    p.to_path_buf()
}

/// §4 deterministic re-scan: unchanged tree → equal observable state (A),
/// differing observation time (B), stable order (C). First step toward INV-5.
#[test]
fn rescan_of_unchanged_tree_is_equivalent() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    std::fs::write(root.join("b.txt"), b"bravo").unwrap();
    std::fs::write(root.join("empty.txt"), b"").unwrap();
    std::fs::create_dir(root.join("zdir")).unwrap();
    std::fs::create_dir(root.join("adir")).unwrap(); // empty dir
    std::fs::write(root.join("zdir").join("nested.md"), b"# nested").unwrap();
    std::fs::write(root.join("zdir").join("deep").join("x"), b"x").unwrap_or_else(|_| {
        std::fs::create_dir(root.join("zdir").join("deep")).unwrap();
        std::fs::write(root.join("zdir").join("deep").join("x"), b"x").unwrap()
    });

    let s1 = scan(root).unwrap();
    std::thread::sleep(std::time::Duration::from_millis(5));
    let s2 = scan(root).unwrap();

    assert!(s1.equivalent_observable_state(&s2));
    assert!(
        s1.started_at != s2.started_at,
        "observation time (B) must not leak into state (A)"
    );
    // order (C): ascending by relative path
    let mut expected: Vec<_> = s1.entries.clone();
    expected.sort_by(|a, b| a.path.cmp(&b.path));
    assert_eq!(s1.entries, expected);
}

#[test]
fn scan_sees_full_structure_with_counts_and_types() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    std::fs::write(root.join("empty.txt"), b"").unwrap();
    std::fs::write(root.join("full.bin"), [0u8, 1, 2, 3]).unwrap();
    std::fs::create_dir(root.join("sub")).unwrap();

    let s = scan(root).unwrap();
    assert_eq!(s.entries.len(), 3);
    assert_eq!(s.errors.len(), 0);
    assert!(
        s.entries
            .iter()
            .any(|e| e.path == path_of(std::path::Path::new("empty.txt"))
                && e.kind == EntryKind::File
                && e.size == Some(0))
    );
    assert!(
        s.entries
            .iter()
            .any(|e| e.path == path_of(std::path::Path::new("full.bin"))
                && e.kind == EntryKind::File
                && e.size == Some(4))
    );
    assert!(
        s.entries
            .iter()
            .any(|e| e.path == path_of(std::path::Path::new("sub")) && e.kind == EntryKind::Dir)
    );
    // physical metadata is present on Linux and mtime is a real epoch time
    assert!(
        s.entries
            .iter()
            .all(|e| e.ino.is_some() && e.mtime.is_some())
    );
}

#[test]
fn creation_order_does_not_affect_output_order() {
    // created in reverse-lexicographic order on purpose
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    for name in ["zeta", "mid", "alpha"] {
        std::fs::write(root.join(name), b"x").unwrap();
    }
    let s = scan(root).unwrap();
    let names: Vec<_> = s.entries.iter().map(|e| e.path.clone()).collect();
    let mut sorted = names.clone();
    sorted.sort();
    assert_eq!(names, sorted, "output must be sorted, not creation-ordered");
}

#[test]
fn non_ascii_names_survive_losslessly() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    let name = "reseña — 研究ノート.txt";
    std::fs::write(root.join(name), b"contenido").unwrap();

    let s = scan(root).unwrap();
    assert_eq!(s.entries.len(), 1);
    assert_eq!(s.entries[0].path, std::path::PathBuf::from(name));
}

#[test]
fn symlink_is_observed_as_link_entry_not_followed() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    std::fs::write(root.join("target.txt"), b"t").unwrap();
    #[cfg(unix)]
    std::os::unix::fs::symlink(root.join("target.txt"), root.join("link")).unwrap();

    #[cfg(unix)]
    {
        let s = scan(root).unwrap();
        let link = s.entries.iter().find(|e| e.path == *"link").unwrap();
        assert_eq!(link.kind, EntryKind::Symlink);
        // the link target dir is not traversed through the link (it is a file anyway);
        // with a dir symlink the walk must not descend:
        std::fs::create_dir(root.join("realdir")).unwrap();
        std::fs::write(root.join("realdir").join("inner"), b"i").unwrap();
        #[cfg(unix)]
        std::os::unix::fs::symlink(root.join("realdir"), root.join("dirlink")).unwrap();
        let s2 = scan(root).unwrap();
        assert!(
            s2.entries
                .iter()
                .any(|e| e.path == *"dirlink" && e.kind == EntryKind::Symlink)
        );
        assert!(
            !s2.entries.iter().any(|e| e.path == *"dirlink/inner"),
            "symlinked dirs must be observed as one entry, not traversed"
        );
    }
}

#[cfg(unix)]
#[test]
fn non_utf8_filename_is_preserved_bytewise() {
    // Linux-specific: paths are bytes on Unix; the scanner must not do a
    // lossy UTF-8 conversion (plan §2/§12). Labelled Linux-specific.
    use std::os::unix::ffi::OsStrExt;
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    let raw = std::ffi::OsStr::from_bytes(b"raw-\xFF-byte-name");
    std::fs::write(root.join(raw), b"z").unwrap();

    let s = scan(root).unwrap();
    assert_eq!(s.entries.len(), 1);
    assert_eq!(
        fsp_check::os_str_bytes(s.entries[0].path.as_os_str()),
        b"raw-\xFF-byte-name".as_slice()
    );
    // and the lossy conversion would NOT match
    assert_ne!(
        s.entries[0].path.to_string_lossy().as_bytes(),
        fsp_check::os_str_bytes(s.entries[0].path.as_os_str())
    );
}

#[test]
fn missing_root_is_an_error_not_a_panic_or_empty_scan() {
    let dir = tempfile::tempdir().unwrap();
    let gone = dir.path().join("does-not-exist");
    assert!(matches!(
        scan(&gone),
        Err(fsp_check::scan::ScanError::RootMissing(_))
    ));
    let file = dir.path().join("afile");
    std::fs::write(&file, b"x").unwrap();
    assert!(matches!(
        scan(&file),
        Err(fsp_check::scan::ScanError::RootNotADirectory(_))
    ));
}

#[test]
fn scan_does_not_modify_the_tree() {
    // raw metadata snapshot (not via the scanner) before and after three scans
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    std::fs::write(root.join("f.txt"), b"data").unwrap();
    std::fs::create_dir(root.join("d")).unwrap();
    std::fs::write(root.join("d").join("g"), b"g").unwrap();

    fn snapshot(root: &std::path::Path) -> Vec<(std::path::PathBuf, u64, i64, u32, u32)> {
        use std::os::unix::fs::PermissionsExt;
        let mut v = Vec::new();
        for e in walkdir_like(root) {
            let m = std::fs::symlink_metadata(&e).unwrap();
            let mt = m
                .modified()
                .unwrap()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap();
            v.push((
                e,
                m.permissions().mode() as u64,
                mt.as_secs() as i64,
                mt.subsec_nanos(),
                m.len() as u32,
            ));
        }
        v.sort();
        v
    }
    fn walkdir_like(root: &std::path::Path) -> Vec<std::path::PathBuf> {
        let mut out = vec![root.to_path_buf()];
        let mut stack = vec![root.to_path_buf()];
        while let Some(d) = stack.pop() {
            for entry in std::fs::read_dir(&d).unwrap() {
                let p = entry.unwrap().path();
                out.push(p.clone());
                if p.is_dir() {
                    stack.push(p);
                }
            }
        }
        out
    }

    let before = snapshot(root);
    let _ = scan(root).unwrap();
    let _ = scan(root).unwrap();
    let _ = scan(root).unwrap();
    assert_eq!(
        before,
        snapshot(root),
        "scanning must not modify the observed tree"
    );
}
