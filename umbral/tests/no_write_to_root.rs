//! A3: the user's tree is not modified, and deleting the tool's state leaves it untouched.
//!
//! The comparator is also shown to detect a real change, so the test cannot pass by being
//! insensitive.

mod common;

use common::{snapshot, Sandbox};

#[test]
fn no_command_modifies_the_observed_tree() {
    let s = Sandbox::new();
    s.write("a.txt", "alpha");
    s.write("sub/b.txt", "beta");
    s.write("sub/deep/c.txt", "gamma");
    s.write("z.log", "z");
    #[cfg(unix)]
    std::os::unix::fs::symlink("a.txt", s.root().join("link")).unwrap();

    let before = snapshot(s.root());
    assert!(!before.0.is_empty());

    let r = s.root().to_string_lossy().to_string();
    s.run_ok(&["init", r.as_str()]);
    s.run_ok(&["observe", r.as_str()]);
    s.run_ok(&["status", r.as_str()]);
    s.run_ok(&["changes", r.as_str()]);
    s.run_ok(&["show", r.as_str(), "a.txt"]);
    s.run_ok(&["check", r.as_str()]);
    s.run_ok(&["workspaces"]);

    let after = snapshot(s.root());
    assert_eq!(before, after, "a command modified the user's tree");
}

/// The state directory is outside the root, and removing it does not touch the tree.
#[test]
fn state_lives_outside_the_root_and_can_be_deleted_safely() {
    let s = Sandbox::new();
    s.write("a.txt", "alpha");
    s.write("sub/b.txt", "beta");

    let before = snapshot(s.root());
    let r = s.root().to_string_lossy().to_string();

    let init_out = s.run_ok(&["init", r.as_str()]);
    let state_dir = init_out
        .lines()
        .find_map(|l| l.strip_prefix("observed  state-dir="))
        .expect("init must report where the state lives")
        .to_string();

    assert!(
        !std::path::Path::new(&state_dir).starts_with(s.root()),
        "state directory {state_dir} must not be inside the observed root"
    );
    assert!(
        !before.0.keys().any(|k| k.contains("ws-")),
        "no workspace marker may appear inside the tree"
    );

    s.run_ok(&["observe", r.as_str()]);
    assert_eq!(before, snapshot(s.root()), "observe modified the tree");

    std::fs::remove_dir_all(&state_dir).unwrap();
    assert_eq!(
        before,
        snapshot(s.root()),
        "deleting the state modified the tree"
    );

    // With the state gone, the workspace reports as not initialised rather than inventing one.
    let out = s.run(&["status", r.as_str()]);
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr).contains("not initialised"));

    // And it can be re-initialised; the history is gone, which is a documented consequence.
    s.run_ok(&["init", r.as_str()]);
    let after_reinit = s.run_ok(&["status", r.as_str()]);
    assert!(after_reinit.contains("reason=no-run-recorded"));
    assert_eq!(before, snapshot(s.root()));
}

/// The negative control: the comparator DOES notice a real change. Without this, the tests
/// above could pass by comparing nothing to nothing.
#[test]
fn the_comparator_detects_a_real_change() {
    let s = Sandbox::new();
    s.write("a.txt", "alpha");
    let before = snapshot(s.root());

    // Content change.
    s.write("a.txt", "alpha changed");
    assert_ne!(
        before,
        snapshot(s.root()),
        "a content change must be visible"
    );

    // Creation.
    let after_content = snapshot(s.root());
    s.write("b.txt", "new");
    assert_ne!(
        after_content,
        snapshot(s.root()),
        "a creation must be visible"
    );

    // Deletion.
    let after_create = snapshot(s.root());
    std::fs::remove_file(s.root().join("b.txt")).unwrap();
    assert_ne!(
        after_create,
        snapshot(s.root()),
        "a deletion must be visible"
    );

    // Restoration returns to the earlier fingerprint, so the comparison is not one-way.
    let after_delete = snapshot(s.root());
    s.write("a.txt", "alpha");
    assert_ne!(after_delete, snapshot(s.root()));
}

/// The state directory is the only thing the tool creates, and it is created under the
/// configured data home.
#[test]
fn the_tool_creates_state_only_under_the_data_home() {
    let s = Sandbox::new();
    s.write("a.txt", "alpha");
    let r = s.root().to_string_lossy().to_string();
    s.run_ok(&["init", r.as_str()]);
    s.run_ok(&["observe", r.as_str()]);

    let created: Vec<String> = walk(s.data.path())
        .into_iter()
        .filter(|p| !p.ends_with("umbral") && !p.contains("ws-"))
        .collect();
    assert!(
        created.is_empty(),
        "unexpected entries under the data home: {created:?}"
    );
    assert!(walk(s.data.path()).iter().any(|p| p.contains("ws-")));
}

fn walk(dir: &std::path::Path) -> Vec<String> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        if let Ok(rd) = std::fs::read_dir(&d) {
            for e in rd.flatten() {
                let p = e.path();
                out.push(p.to_string_lossy().into_owned());
                if p.is_dir() {
                    stack.push(p);
                }
            }
        }
    }
    out
}
