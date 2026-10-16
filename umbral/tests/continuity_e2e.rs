//! Task continuity end to end (T1.7): one process saves, another resumes, through the real
//! binary and a real git repository.

use std::io::Write;
use std::path::Path;
use std::process::{Command, Output, Stdio};

use serde_json::{json, Value};
use tempfile::TempDir;

const BIN: &str = env!("CARGO_BIN_EXE_umbral");

fn git(root: &Path, args: &[&str]) {
    let out = Command::new("git")
        .args([
            "-c",
            "user.name=t",
            "-c",
            "user.email=t@t",
            "-c",
            "commit.gpgsign=false",
        ])
        .args(args)
        .current_dir(root)
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .output()
        .unwrap();
    assert!(out.status.success(), "git {args:?}: {out:?}");
}

fn repo() -> TempDir {
    let dir = TempDir::new().unwrap();
    git(dir.path(), &["init", "-q"]);
    for (rel, body) in [
        ("a.txt", "alpha"),
        ("b.txt", "beta"),
        ("src/c.rs", "fn c() {}"),
    ] {
        let p = dir.path().join(rel);
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, body).unwrap();
    }
    git(dir.path(), &["add", "."]);
    git(dir.path(), &["commit", "-q", "-m", "init"]);
    dir
}

fn umbral(args: &[&str], stdin: Option<&Value>) -> (Output, Value) {
    let mut child = Command::new(BIN)
        .args(args)
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let mut pipe = child.stdin.take().unwrap();
    if let Some(v) = stdin {
        pipe.write_all(v.to_string().as_bytes()).unwrap();
    }
    drop(pipe);
    let out = child.wait_with_output().unwrap();
    let doc = serde_json::from_slice(&out.stdout).unwrap_or_else(|e| {
        panic!("stdout is not JSON ({e}): {out:?}");
    });
    (out, doc)
}

fn save_input(expected: Value) -> Value {
    json!({
        "task": "fase1-nucleo",
        "expected_revision": expected,
        "emitter": { "client": "test" },
        "goal": "Prove continuity across processes",
        "next_step": "Resume in another process",
        "blockers": [],
        "declarations": [
            { "id": "tests-pass", "kind": "check", "claimed_origin": "agent",
              "text": "b compiles", "command": "true", "claimed_result": "passed",
              "evidence": null, "depends_on": ["b.txt"] },
            { "id": "no-sqlite", "kind": "constraint", "claimed_origin": "human-stated",
              "text": "No SQLite", "depends_on": [] }
        ],
        "sources": ["a.txt", "b.txt", "src/c.rs"]
    })
}

#[test]
fn a_source_edited_after_save_differs_and_the_others_match() {
    let dir = repo();
    let root = dir.path().to_str().unwrap();

    // Process A.
    let (out, saved) = umbral(&["task", "save", root], Some(&save_input(Value::Null)));
    assert_eq!(out.status.code(), Some(0), "{saved}\n{out:?}");
    let revision = saved["revision"].as_str().unwrap().to_string();
    assert_eq!(revision.len(), 32);
    assert_eq!(saved["git"]["head_state"], "present");
    assert!(saved["privacy_warning"].is_string());
    let file = dir
        .path()
        .join(".umbral/continuity/tasks/fase1-nucleo")
        .join(format!("{revision}.json"));
    let bytes = std::fs::read(&file).unwrap();
    assert_eq!(&blake3::hash(&bytes).to_hex()[..32], revision.as_str());

    // An edit without a commit.
    std::fs::write(dir.path().join("b.txt"), "beta, edited").unwrap();

    // Process B.
    let (out, resumed) = umbral(&["task", "resume", root, "fase1-nucleo"], None);
    assert_eq!(out.status.code(), Some(0), "{resumed}");
    assert_eq!(resumed["state"], "record");
    assert_eq!(resumed["revision"], revision.as_str());
    let states: Vec<(&str, &str)> = resumed["sources"]
        .as_array()
        .unwrap()
        .iter()
        .map(|s| (s["path"].as_str().unwrap(), s["state"].as_str().unwrap()))
        .collect();
    assert_eq!(
        states,
        [
            ("a.txt", "match"),
            ("b.txt", "differs"),
            ("src/c.rs", "match")
        ]
    );
    assert_eq!(resumed["summary"], "not-all-match");
    assert_eq!(resumed["checks_needing_review"], json!(["tests-pass"]));
    assert_eq!(
        resumed["declarations_without_dependencies"],
        json!(["no-sqlite"])
    );
    assert_eq!(
        resumed["declared"]["goal"],
        "Prove continuity across processes"
    );

    // Reverting the edit makes every source match again: the comparison is by content.
    std::fs::write(dir.path().join("b.txt"), "beta").unwrap();
    let (_, resumed) = umbral(&["task", "resume", root, "fase1-nucleo"], None);
    assert_eq!(resumed["summary"], "all-match");

    // A second save from the head extends the chain; the first file is untouched.
    let (out, second) = umbral(&["task", "save", root], Some(&save_input(json!(revision))));
    assert_eq!(out.status.code(), Some(0), "{second}");
    assert_eq!(second["parents"], json!([revision]));
    assert!(second.get("privacy_warning").is_none());
    assert_eq!(std::fs::read(&file).unwrap(), bytes);

    let (out, listed) = umbral(&["task", "list", root], None);
    assert_eq!(out.status.code(), Some(0));
    assert_eq!(listed["tasks"][0]["task"], "fase1-nucleo");
    assert_eq!(listed["tasks"][0]["integrity"], "ok");
    assert_eq!(
        listed["tasks"][0]["heads"][0]["revision"],
        second["revision"]
    );
    assert_eq!(listed["next_cursor"], Value::Null);
}

#[test]
fn an_empty_repository_has_no_tasks_and_reading_creates_nothing() {
    let dir = repo();
    let root = dir.path().to_str().unwrap();

    let (out, listed) = umbral(&["task", "list", root], None);
    assert_eq!(out.status.code(), Some(0));
    assert_eq!(listed["tasks"], json!([]));

    let (out, resumed) = umbral(&["task", "resume", root, "nothing-here"], None);
    assert_eq!(out.status.code(), Some(0));
    assert_eq!(resumed["state"], "no-record");

    assert!(!dir.path().join(".umbral").exists());
}

#[test]
fn a_stale_expected_revision_is_a_conflict_and_writes_nothing() {
    let dir = repo();
    let root = dir.path().to_str().unwrap();
    let (out, _) = umbral(&["task", "save", root], Some(&save_input(Value::Null)));
    assert_eq!(out.status.code(), Some(0));

    let (out, err) = umbral(&["task", "save", root], Some(&save_input(Value::Null)));
    assert_eq!(out.status.code(), Some(4), "{err}");
    assert_eq!(err["error"], "conflicto-revision");
    let n = std::fs::read_dir(dir.path().join(".umbral/continuity/tasks/fase1-nucleo"))
        .unwrap()
        .count();
    assert_eq!(n, 1);
}
