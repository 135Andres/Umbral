//! Task continuity end to end (T1.7): one process saves, another resumes, through the real
//! binary and a real git repository. Then the required edges of phase 1 (T1.8-T1.16).

mod common;

use std::io::Write;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
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
    umbral_env(args, stdin, &[])
}

fn umbral_env(
    args: &[&str],
    stdin: Option<&Value>,
    env: &[(&str, &std::ffi::OsStr)],
) -> (Output, Value) {
    let mut child = Command::new(BIN)
        .args(args)
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .envs(env.iter().copied())
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

// ---- Edges (T1.8-T1.16) ----

fn input(task: &str, expected: Value, sources: &[&str]) -> Value {
    json!({
        "task": task,
        "expected_revision": expected,
        "goal": format!("goal of {task}"),
        "next_step": "next",
        "blockers": [],
        "declarations": [],
        "sources": sources,
    })
}

fn save(root: &Path, v: &Value) -> (Option<i32>, Value) {
    let (out, doc) = umbral(&["task", "save", root.to_str().unwrap()], Some(v));
    (out.status.code(), doc)
}

fn run(args: &[&str], root: &Path) -> (Option<i32>, Value) {
    let mut a = vec!["task", args[0], root.to_str().unwrap()];
    a.extend(&args[1..]);
    let (out, doc) = umbral(&a, None);
    (out.status.code(), doc)
}

fn task_dir(root: &Path, task: &str) -> PathBuf {
    root.join(".umbral/continuity/tasks").join(task)
}

fn files(dir: &Path) -> Vec<String> {
    let mut v: Vec<String> = std::fs::read_dir(dir)
        .map(|r| {
            r.map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
                .collect()
        })
        .unwrap_or_default();
    v.sort();
    v
}

/// Save one revision and return its id.
fn saved(root: &Path, v: &Value) -> String {
    let (code, doc) = save(root, v);
    assert_eq!(code, Some(0), "{doc}");
    doc["revision"].as_str().unwrap().to_string()
}

/// Write a record file named by its own hash, as Umbral would name it.
fn plant(root: &Path, task: &str, bytes: &[u8]) -> PathBuf {
    let dir = task_dir(root, task);
    std::fs::create_dir_all(&dir).unwrap();
    let p = dir.join(format!("{}.json", &blake3::hash(bytes).to_hex()[..32]));
    std::fs::write(&p, bytes).unwrap();
    p
}

/// T1.8: a stale single id, a wrong list and a create over an existing task all conflict;
/// nothing is written.
#[test]
fn t1_8_stale_expected_revisions_conflict_without_writing() {
    let dir = repo();
    let root = dir.path();
    let first = saved(root, &input("t", Value::Null, &["a.txt"]));
    let second = saved(root, &input("t", json!(first), &["a.txt"]));
    for expected in [
        json!(first),
        Value::Null,
        json!([first, second]),
        json!("0".repeat(32)),
    ] {
        let (code, err) = save(root, &input("t", expected, &["a.txt"]));
        assert_eq!(code, Some(4), "{err}");
        assert_eq!(err["error"], "conflicto-revision");
        assert_eq!(err["heads"], json!([second]));
    }
    assert_eq!(files(&task_dir(root, "t")).len(), 2);
}

/// T1.8 / contract §5.2: two branches advance the same task and merge. The task forks;
/// resume and export refuse to choose; a save naming both heads closes the fork.
#[test]
fn t1_8_a_merge_forks_the_task_and_a_save_naming_both_heads_closes_it() {
    let dir = repo();
    let root = dir.path();
    let base = saved(root, &input("t", Value::Null, &["a.txt"]));
    git(root, &["add", "-A"]);
    git(root, &["commit", "-q", "-m", "base"]);
    git(root, &["checkout", "-q", "-b", "side"]);
    let mut v = input("t", json!(base), &["a.txt"]);
    v["goal"] = json!("side");
    let side = saved(root, &v);
    git(root, &["add", "-A"]);
    git(root, &["commit", "-q", "-m", "side"]);
    git(root, &["checkout", "-q", "-"]);
    let main = saved(root, &input("t", json!(base), &["a.txt"]));
    git(root, &["add", "-A"]);
    git(root, &["commit", "-q", "-m", "main"]);
    git(root, &["merge", "-q", "--no-edit", "side"]);

    let mut heads = vec![side.clone(), main.clone()];
    heads.sort();
    for op in ["resume", "export"] {
        let (code, err) = run(&[op, "t"], root);
        assert_eq!(code, Some(4), "{err}");
        assert_eq!(err["error"], "bifurcacion");
        let got: Vec<&str> = err["heads"]
            .as_array()
            .unwrap()
            .iter()
            .map(|h| h["revision"].as_str().unwrap())
            .collect();
        assert_eq!(got, heads);
    }
    let (code, one) = run(&["resume", "t", "--revision", &side], root);
    assert_eq!(code, Some(0));
    assert_eq!(one["declared"]["goal"], "side");
    let (_, listed) = run(&["list"], root);
    assert_eq!(listed["tasks"][0]["heads"].as_array().unwrap().len(), 2);

    // One head alone is not the current state.
    let (code, _) = save(root, &input("t", json!(main), &["a.txt"]));
    assert_eq!(code, Some(4));
    let closed = saved(root, &input("t", json!([main, side]), &["a.txt"]));
    let (code, resumed) = run(&["resume", "t"], root);
    assert_eq!(code, Some(0), "{resumed}");
    assert_eq!(resumed["revision"], closed.as_str());
    assert_eq!(resumed["parents"], json!(heads));
}

/// T1.9: an interrupted save leaves at most a temporary in `tmp/`, which no read sees and
/// Git ignores.
#[test]
fn t1_9_a_leftover_temporary_is_invisible_to_reads_and_to_git() {
    let dir = repo();
    let root = dir.path();
    let id = saved(root, &input("t", Value::Null, &["a.txt"]));
    std::fs::write(
        root.join(".umbral/continuity/tmp/123-456-0"),
        b"{\"format\": \"umbral-contin",
    )
    .unwrap();
    let (code, resumed) = run(&["resume", "t"], root);
    assert_eq!(code, Some(0));
    assert_eq!(resumed["revision"], id.as_str());
    let (_, listed) = run(&["list"], root);
    assert_eq!(listed["tasks"][0]["integrity"], "ok");
    let out = Command::new("git")
        .args(["status", "--porcelain", "--untracked-files=all"])
        .current_dir(root)
        .output()
        .unwrap();
    let status = String::from_utf8(out.stdout).unwrap();
    assert!(!status.contains("tmp"), "{status}");
    assert!(status.contains(&format!("{id}.json")), "{status}");
}

/// T1.9 / contract §5.4: a record changed by hand or by Git is reported, never repaired;
/// the other tasks still list.
#[test]
fn t1_9_an_altered_record_is_detected_and_left_as_it_is() {
    let dir = repo();
    let root = dir.path();
    saved(root, &input("ok", Value::Null, &["a.txt"]));
    let id = saved(root, &input("t", Value::Null, &["a.txt"]));
    let file = task_dir(root, "t").join(format!("{id}.json"));
    let original = std::fs::read_to_string(&file).unwrap();

    let edited = original.replace("goal of t", "goal of u");
    let conflicted = format!("<<<<<<< HEAD\n{original}=======\n{edited}>>>>>>> side\n");
    for (bytes, reason) in [(edited, "hash-mismatch"), (conflicted, "conflict-markers")] {
        std::fs::write(&file, &bytes).unwrap();
        for op in ["resume", "export"] {
            let (code, err) = run(&[op, "t"], root);
            assert_eq!(code, Some(5), "{err}");
            assert_eq!(err["error"], "registro-alterado");
            assert_eq!(err["reason"], reason);
        }
        let (code, err) = save(root, &input("t", json!(id), &["a.txt"]));
        assert_eq!((code, &err["reason"]), (Some(5), &json!(reason)));
        let (code, listed) = run(&["list"], root);
        assert_eq!(code, Some(0));
        assert_eq!(listed["tasks"][0]["integrity"], "ok");
        assert_eq!(listed["tasks"][1]["integrity"], "registro-alterado");
        assert_eq!(std::fs::read_to_string(&file).unwrap(), bytes);
        assert_eq!(files(&task_dir(root, "t")).len(), 1);
    }
}

/// Contract §5.4: a parent removed (a revert, a deletion) is an altered record.
#[test]
fn t1_9_a_missing_parent_is_an_altered_record() {
    let dir = repo();
    let root = dir.path();
    let first = saved(root, &input("t", Value::Null, &["a.txt"]));
    saved(root, &input("t", json!(first), &["a.txt"]));
    std::fs::remove_file(task_dir(root, "t").join(format!("{first}.json"))).unwrap();
    let (code, err) = run(&["resume", "t"], root);
    assert_eq!(code, Some(5), "{err}");
    assert_eq!(err["error"], "registro-alterado");
    assert_eq!(err["reason"], "missing-parent");
}

/// Contract §5.4 / P4: a file whose name matches its bytes but whose content is not a
/// record of this edition is invalid, and is not rewritten.
#[test]
fn t1_9_an_invalid_record_is_reported() {
    let dir = repo();
    let root = dir.path();
    let id = saved(root, &input("t", Value::Null, &["a.txt"]));
    let good: Value = serde_json::from_slice(
        &std::fs::read(task_dir(root, "t").join(format!("{id}.json"))).unwrap(),
    )
    .unwrap();
    let mut extra = good.clone();
    extra["unknown_field"] = json!(1);
    let mut wrong_task = good.clone();
    wrong_task["task"] = json!("other");
    for (i, bytes) in [
        b"not json\n".to_vec(),
        serde_json::to_vec_pretty(&extra).unwrap(),
        serde_json::to_vec_pretty(&wrong_task).unwrap(),
        // B-12: larger than any valid record; refused without being read whole.
        vec![b' '; 512 * 1024 + 1],
    ]
    .into_iter()
    .enumerate()
    {
        let task = format!("bad-{i}");
        let p = plant(root, &task, &bytes);
        let (code, err) = run(&["resume", &task], root);
        assert_eq!(code, Some(5), "{err}");
        assert_eq!(err["error"], "registro-invalido", "{err}");
        assert_eq!(std::fs::read(&p).unwrap(), bytes);
    }
}

/// T1.10: escapes, links, directories, absences and ignored files are rejected and nothing is
/// written; a malformed path is invalid input.
#[test]
fn t1_10_unsupported_sources_are_rejected_and_nothing_is_written() {
    let dir = repo();
    let root = dir.path();
    let outside = TempDir::new().unwrap();
    std::fs::write(outside.path().join("x"), "x").unwrap();
    std::os::unix::fs::symlink("a.txt", root.join("link")).unwrap();
    std::os::unix::fs::symlink(outside.path(), root.join("out")).unwrap();
    std::fs::write(root.join(".gitignore"), ".env\n").unwrap();
    std::fs::write(root.join(".env"), "TOKEN=1").unwrap();

    let (code, err) = save(
        root,
        &input(
            "t",
            Value::Null,
            &["a.txt", "link", "src", "missing", "out/x", ".env"],
        ),
    );
    assert_eq!(code, Some(2), "{err}");
    assert_eq!(err["error"], "fuente-rechazada");
    assert_eq!(
        err["sources"],
        json!([
            { "path": "link", "reason": "symlink" },
            { "path": "src", "reason": "not-a-regular-file" },
            { "path": "missing", "reason": "absent" },
            { "path": "out/x", "reason": "outside-root" },
            { "path": ".env", "reason": "git-ignored" },
        ])
    );
    for bad in [
        "../a.txt",
        "/etc/passwd",
        "./a.txt",
        "src//c.rs",
        ".git/config",
        ".umbral/x",
    ] {
        let (code, err) = save(root, &input("t", Value::Null, &[bad]));
        assert_eq!(code, Some(2), "{bad}: {err}");
        assert_eq!(err["error"], "entrada-invalida", "{bad}");
    }
    assert!(!root.join(".umbral").exists());

    // A tracked file that matches an ignore rule is versioned already: accepted.
    git(root, &["add", "-f", ".env"]);
    saved(root, &input("t", Value::Null, &[".env"]));
}

/// T1.10: after a save, absence, a denied read and a change are three distinct results. A
/// source that is only absent is not `all-match`, and a check depending on it needs review
/// (A M-1: mutants M14, M17). Only the denied read needs file modes to be enforced (A M-6).
#[test]
fn t1_10_absent_denied_and_differing_sources_are_distinct_at_resume() {
    use std::os::unix::fs::PermissionsExt;
    let dir = repo();
    let root = dir.path();
    let check = |id: &str, on: &str| {
        json!({ "id": id, "kind": "check", "claimed_origin": "agent", "text": "t",
                "command": null, "claimed_result": "passed", "evidence": null,
                "depends_on": [on] })
    };
    let mut v = input("t", Value::Null, &["a.txt", "b.txt", "src/c.rs"]);
    v["declarations"] = json!([check("on-a", "a.txt"), check("on-c", "src/c.rs")]);
    saved(root, &v);
    let states = |doc: &Value| -> Vec<(Value, Value)> {
        doc["sources"]
            .as_array()
            .unwrap()
            .iter()
            .map(|s| (s["state"].clone(), s["reason"].clone()))
            .collect()
    };

    std::fs::remove_file(root.join("a.txt")).unwrap();
    let (code, resumed) = run(&["resume", "t"], root);
    assert_eq!(code, Some(0), "{resumed}");
    assert_eq!(resumed["summary"], "not-all-match");
    assert_eq!(
        resumed["counts"]["states"],
        json!({ "absent": 1, "differs": 0, "match": 2, "unknown": 0 })
    );
    assert_eq!(resumed["checks_needing_review"], json!(["on-a"]));

    std::fs::write(root.join("b.txt"), "changed").unwrap();
    let c = root.join("src/c.rs");
    std::fs::set_permissions(&c, std::fs::Permissions::from_mode(0o000)).unwrap();
    if std::fs::read(&c).is_ok() {
        std::fs::set_permissions(&c, std::fs::Permissions::from_mode(0o644)).unwrap();
        eprintln!("skipped the denied read: running with privileges that ignore file modes");
        return;
    }
    let (code, resumed) = run(&["resume", "t"], root);
    std::fs::set_permissions(&c, std::fs::Permissions::from_mode(0o644)).unwrap();
    assert_eq!(code, Some(0), "{resumed}");
    assert_eq!(
        states(&resumed),
        [
            (json!("absent"), Value::Null),
            (json!("differs"), Value::Null),
            (json!("unknown"), json!("permission-denied")),
        ]
    );
    assert_eq!(
        resumed["counts"]["unknown_reasons"],
        json!({ "permission-denied": 1 })
    );
    assert_eq!(resumed["checks_needing_review"], json!(["on-a", "on-c"]));

    // At save, a denied read is recorded as uncheckable, not rejected.
    std::fs::set_permissions(&c, std::fs::Permissions::from_mode(0o000)).unwrap();
    let (code, doc) = save(root, &input("u", Value::Null, &["src/c.rs"]));
    std::fs::set_permissions(&c, std::fs::Permissions::from_mode(0o644)).unwrap();
    assert_eq!(code, Some(0), "{doc}");
    assert_eq!(
        doc["uncheckable"],
        json!([{ "path": "src/c.rs", "reason": "permission-denied" }])
    );
    let (_, resumed) = run(&["resume", "u"], root);
    assert_eq!(resumed["sources"][0]["reason"], "no-recorded-hash");
    assert_eq!(resumed["summary"], "not-all-match");
}

/// T1.11: `.umbral/` (or a directory under it) that is a symlink or not a directory is
/// unsafe for writes and for reads, and nothing is written through it.
#[test]
fn t1_11_an_unsafe_state_directory_is_refused() {
    let outside = TempDir::new().unwrap();
    let cases: [fn(&Path, &Path); 3] = [
        |root, out| std::os::unix::fs::symlink(out, root.join(".umbral")).unwrap(),
        |root, out| {
            std::fs::create_dir(root.join(".umbral")).unwrap();
            std::os::unix::fs::symlink(out, root.join(".umbral/continuity")).unwrap();
        },
        |root, _| std::fs::write(root.join(".umbral"), "").unwrap(),
    ];
    for make in cases {
        let dir = repo();
        let root = dir.path();
        make(root, outside.path());
        let (code, err) = save(root, &input("t", Value::Null, &["a.txt"]));
        assert_eq!(code, Some(1), "{err}");
        assert_eq!(err["error"], "estado-inseguro");
        for args in [&["resume", "t"][..], &["list"], &["export", "t"]] {
            let (code, err) = run(args, root);
            assert_eq!(code, Some(1), "{args:?}: {err}");
            assert_eq!(err["error"], "estado-inseguro");
        }
    }
    assert!(files(outside.path()).is_empty());
}

/// T1.11: a record of an unknown edition fails, naming the edition, and is not rewritten,
/// not even by a save.
#[test]
fn t1_11_an_unknown_format_fails_without_rewriting() {
    let dir = repo();
    let root = dir.path();
    let bytes = b"{\n  \"format\": \"umbral-continuity/2\"\n}\n";
    let p = plant(root, "t", bytes);
    for op in ["resume", "export"] {
        let (code, err) = run(&[op, "t"], root);
        assert_eq!(code, Some(5), "{err}");
        assert_eq!(err["error"], "formato-desconocido");
        assert_eq!(err["reason"], "umbral-continuity/2");
    }
    let id = p.file_stem().unwrap().to_str().unwrap();
    let (code, err) = save(root, &input("t", json!(id), &["a.txt"]));
    assert_eq!(
        (code, &err["error"]),
        (Some(5), &json!("formato-desconocido"))
    );
    let (_, listed) = run(&["list"], root);
    assert_eq!(listed["tasks"][0]["integrity"], "formato-desconocido");
    assert_eq!(std::fs::read(&p).unwrap(), bytes);
    assert_eq!(files(&task_dir(root, "t")).len(), 1);
}

/// T1.12: a repository without commits is accepted with an unborn HEAD; a folder without
/// Git and a subdirectory of a repository are refused, and nothing is written.
#[test]
fn t1_12_unborn_head_is_accepted_and_no_git_is_refused() {
    let dir = TempDir::new().unwrap();
    let root = dir.path();
    std::fs::write(root.join("a.txt"), "alpha").unwrap();

    let out = Command::new(BIN)
        .args(["task", "list", root.to_str().unwrap()])
        .env("GIT_CEILING_DIRECTORIES", root.parent().unwrap())
        .output()
        .unwrap();
    let err: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(out.status.code(), Some(1), "{err}");
    assert_eq!(err["error"], "sin-git");
    assert!(String::from_utf8_lossy(&out.stderr).contains("no git repository"));
    // B-09: git's own reason is kept (it is the only clue for `safe.directory`).
    assert!(
        err["message"].as_str().unwrap().contains("fatal: "),
        "{err}"
    );

    git(root, &["init", "-q"]);
    let (code, doc) = save(root, &input("t", Value::Null, &["a.txt"]));
    assert_eq!(code, Some(0), "{doc}");
    assert_eq!(doc["git"], json!({ "head_state": "unborn", "head": null }));
    assert_eq!(doc["parents"], json!([]));

    let sub = root.join("sub");
    std::fs::create_dir(&sub).unwrap();
    let (code, err) = save(&sub, &input("t", Value::Null, &["a.txt"]));
    assert_eq!((code, &err["error"]), (Some(2), &json!("entrada-invalida")));
    assert!(!sub.join(".umbral").exists());
}

/// T1.12: a folder without Git is refused on save, without writing.
#[test]
fn t1_12_save_in_a_folder_without_git_writes_nothing() {
    let dir = TempDir::new().unwrap();
    let root = dir.path();
    std::fs::write(root.join("a.txt"), "alpha").unwrap();
    let mut child = Command::new(BIN)
        .args(["task", "save", root.to_str().unwrap()])
        .env("GIT_CEILING_DIRECTORIES", root.parent().unwrap())
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let v = input("t", Value::Null, &["a.txt"]);
    child
        .stdin
        .take()
        .unwrap()
        .write_all(v.to_string().as_bytes())
        .unwrap();
    let out = child.wait_with_output().unwrap();
    let err: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(out.status.code(), Some(1), "{err}");
    assert_eq!(err["error"], "sin-git");
    assert_eq!(files(root), ["a.txt"]);
}

/// T1.13: exceeding a limit is an error naming the field, the limit and the size; a value at
/// the limit is kept whole.
#[test]
fn t1_13_limits_are_errors_and_nothing_is_truncated() {
    let dir = repo();
    let root = dir.path();
    let long = "x".repeat(250);
    std::fs::write(root.join(&long), "x").unwrap();

    let at_limit = "é".repeat(1024); // 2048 bytes
    let mut over = Vec::new();
    let mut v = input("t", Value::Null, &["a.txt"]);
    v["goal"] = json!(format!("{at_limit}x"));
    over.push((v, "goal", 2048, 2049));
    let mut v = input("t", Value::Null, &["a.txt"]);
    v["blockers"] = json!(vec!["b"; 21]);
    over.push((v, "blockers", 20, 21));
    let many: Vec<String> = (0..201).map(|i| format!("f{i}")).collect();
    let refs: Vec<&str> = many.iter().map(String::as_str).collect();
    over.push((input("t", Value::Null, &refs), "sources", 200, 201));
    let mut v = input("t", Value::Null, &[long.as_str()]);
    v["declarations"] = json!((0..100)
        .map(
            |i| json!({ "id": format!("d{i}"), "kind": "decision", "claimed_origin": "agent",
            "text": "t".repeat(2048), "depends_on": vec![long.as_str(); 50] })
        )
        .collect::<Vec<_>>());
    over.push((v, "revision file", 512 * 1024, 0));

    for (v, field, limit, actual) in over {
        let (code, err) = save(root, &v);
        assert_eq!(code, Some(2), "{field}: {err}");
        assert_eq!(err["error"], "limite-excedido");
        assert_eq!(err["field"], field);
        assert_eq!(err["limit"], limit);
        if actual > 0 {
            assert_eq!(err["actual"], actual);
        } else {
            assert!(err["actual"].as_u64().unwrap() > limit as u64);
        }
    }
    assert!(!root.join(".umbral").exists());

    let mut v = input("t", Value::Null, &["a.txt"]);
    v["goal"] = json!(at_limit);
    saved(root, &v);
    let (_, resumed) = run(&["resume", "t"], root);
    assert_eq!(resumed["declared"]["goal"], at_limit.as_str());
}

/// T1.5 / contract §6.1: list pages by task id with a cursor; bad limits are input errors.
#[test]
fn list_pages_with_a_cursor() {
    let dir = repo();
    let root = dir.path();
    for t in ["t-c", "t-a", "t-b"] {
        saved(root, &input(t, Value::Null, &["a.txt"]));
    }
    let names = |doc: &Value| -> Vec<String> {
        doc["tasks"]
            .as_array()
            .unwrap()
            .iter()
            .map(|t| t["task"].as_str().unwrap().to_string())
            .collect()
    };
    let (_, page) = run(&["list", "--limit", "2"], root);
    assert_eq!(names(&page), ["t-a", "t-b"]);
    assert_eq!(page["next_cursor"], "t-b");
    let (_, page) = run(&["list", "--limit", "2", "--cursor", "t-b"], root);
    assert_eq!(names(&page), ["t-c"]);
    assert_eq!(page["next_cursor"], Value::Null);
    // Owner decision (2026-10-17): an invalid cursor is no longer an input error, so the
    // input errors here are only the limits. A valid id past every task is an empty page.
    for bad in [&["list", "--limit", "0"][..], &["list", "--limit", "501"]] {
        let (code, err) = run(bad, root);
        assert_eq!((code, &err["error"]), (Some(2), &json!("entrada-invalida")));
    }
    let (_, page) = run(&["list", "--cursor", "zzz-none"], root);
    assert_eq!(names(&page), Vec::<String>::new());
    assert_eq!(page["next_cursor"], Value::Null);
}

/// Owner decision (2026-10-17, contract §6.1): the cursor is never an invalid name. An
/// entry whose directory name is not a task id is listed with its error, does not count
/// for `limit`, and pagination reaches the end without an input error.
#[test]
fn list_pages_past_invalid_names_without_an_invalid_cursor() {
    let dir = repo();
    let root = dir.path();
    std::fs::create_dir_all(task_dir(root, "!invalid")).unwrap();
    saved(root, &input("t-a", Value::Null, &["a.txt"]));
    let mut cursor: Option<String> = None;
    let mut seen = Vec::new();
    loop {
        let mut a = vec!["list", "--limit", "1"];
        if let Some(c) = &cursor {
            a.push("--cursor");
            a.push(c);
        }
        let (code, page) = run(&a, root);
        assert_eq!(code, Some(0), "{page}");
        seen.extend(
            page["tasks"]
                .as_array()
                .unwrap()
                .iter()
                .map(|t| t["task"].as_str().unwrap().to_string()),
        );
        match page["next_cursor"].as_str() {
            Some(c) => cursor = Some(c.to_string()),
            None => break,
        }
    }
    assert_eq!(seen, ["!invalid", "t-a"]);
}

/// Owner decision (2026-10-17): invalid names, interleaved or trailing, appear exactly
/// once across the whole traversal and never consume the page's limit.
#[test]
fn invalid_names_appear_once_and_do_not_consume_the_limit() {
    let dir = repo();
    let root = dir.path();
    for bad in ["!bad", "t-x!", "~~trailing"] {
        std::fs::create_dir_all(task_dir(root, bad)).unwrap();
    }
    for t in ["t-a", "t-b", "t-c"] {
        saved(root, &input(t, Value::Null, &["a.txt"]));
    }
    let mut cursor: Option<String> = None;
    let mut seen = Vec::new();
    loop {
        let mut a = vec!["list", "--limit", "1"];
        if let Some(c) = &cursor {
            a.push("--cursor");
            a.push(c);
        }
        let (code, page) = run(&a, root);
        assert_eq!(code, Some(0), "{page}");
        seen.extend(
            page["tasks"]
                .as_array()
                .unwrap()
                .iter()
                .map(|t| t["task"].as_str().unwrap().to_string()),
        );
        match page["next_cursor"].as_str() {
            Some(c) => cursor = Some(c.to_string()),
            None => break,
        }
    }
    assert_eq!(seen, ["!bad", "t-a", "t-b", "t-c", "t-x!", "~~trailing"]);
}

/// Owner decision (2026-10-17): with only invalid names, one page lists them all and the
/// cursor ends at null; they do not count for `limit`.
#[test]
fn only_invalid_names_list_in_one_page_with_a_null_cursor() {
    let dir = repo();
    let root = dir.path();
    for bad in ["!a", "!b"] {
        std::fs::create_dir_all(task_dir(root, bad)).unwrap();
    }
    let (code, page) = run(&["list", "--limit", "1"], root);
    assert_eq!(code, Some(0), "{page}");
    let tasks: Vec<&str> = page["tasks"]
        .as_array()
        .unwrap()
        .iter()
        .map(|t| t["task"].as_str().unwrap())
        .collect();
    assert_eq!(tasks, ["!a", "!b"]);
    assert_eq!(page["next_cursor"], Value::Null);
}

/// T1.14: export is the record file's bytes plus provenance, readable without Umbral.
#[test]
fn t1_14_export_is_the_record_as_it_is() {
    let dir = repo();
    let root = dir.path();
    let first = saved(root, &save_input(Value::Null));
    let second = saved(root, &save_input(json!(first)));
    for (args, id) in [
        (vec!["export", "fase1-nucleo"], &second),
        (vec!["export", "fase1-nucleo", "--revision", &first], &first),
    ] {
        let (code, doc) = run(&args, root);
        assert_eq!(code, Some(0), "{doc}");
        assert_eq!(doc["format"], "umbral-continuity/1");
        assert_eq!(doc["task"], "fase1-nucleo");
        assert_eq!(doc["revision"], id.as_str());
        assert_eq!(doc["integrity"], "ok");
        let record = doc["record"].as_str().unwrap();
        let file = task_dir(root, "fase1-nucleo").join(format!("{id}.json"));
        assert_eq!(record.as_bytes(), std::fs::read(file).unwrap());
        assert_eq!(&blake3::hash(record.as_bytes()).to_hex()[..32], id.as_str());
        let parsed: Value = serde_json::from_str(record).unwrap();
        assert_eq!(parsed["goal"], "Prove continuity across processes");
    }
    let (code, err) = run(
        &["export", "fase1-nucleo", "--revision", &"0".repeat(32)],
        root,
    );
    assert_eq!(
        (code, &err["error"]),
        (Some(5), &json!("revision-inexistente"))
    );
    let (code, err) = run(&["export", "nothing"], root);
    assert_eq!(
        (code, &err["error"]),
        (Some(5), &json!("revision-inexistente"))
    );
}

/// T1.15: a source path that is not UTF-8 is stored reversibly and found again.
#[test]
fn t1_15_a_non_utf8_path_round_trips() {
    let dir = repo();
    let root = dir.path();
    let name = std::ffi::OsStr::from_bytes(b"src/\xFF\xFEname.rs");
    std::fs::write(root.join(name), "bytes").unwrap();
    let written = "src/\\xFF\\xFEname.rs";
    let (code, doc) = save(root, &input("t", Value::Null, &[written]));
    assert_eq!(code, Some(0), "{doc}");
    assert_eq!(doc["sources"][0]["path"], written);
    assert_eq!(doc["sources"][0]["capture"], "captured");
    let (_, resumed) = run(&["resume", "t"], root);
    assert_eq!(resumed["sources"][0]["path"], written);
    assert_eq!(resumed["sources"][0]["state"], "match");
    std::fs::write(root.join(name), "other bytes").unwrap();
    let (_, resumed) = run(&["resume", "t"], root);
    assert_eq!(resumed["sources"][0]["state"], "differs");
}

/// T1.16: save, resume, list and export change nothing outside `.umbral/` (`.git/` included),
/// and run no git subcommand but `rev-parse` and `check-ignore`, neither of which uses the
/// network.
#[test]
fn t1_16_nothing_outside_umbral_changes_and_git_stays_local() {
    use std::os::unix::fs::PermissionsExt;
    let dir = repo();
    let root = dir.path();
    std::fs::write(root.join(".gitignore"), "ignored\n").unwrap();

    // A `git` on PATH that logs its subcommand and runs the real one.
    let real = std::env::split_paths(&std::env::var_os("PATH").unwrap())
        .map(|d| d.join("git"))
        .find(|p| p.is_file())
        .unwrap();
    let bin = TempDir::new().unwrap();
    let log = bin.path().join("log");
    let wrapper = bin.path().join("git");
    std::fs::write(
        &wrapper,
        format!(
            // The first argument that is neither `-c` nor a `-c` value is the subcommand.
            "#!/bin/sh\nfor a in \"$@\"; do case \"$a\" in -c|*=*) ;; *) printf '%s\\n' \"$a\" >> '{}'; break;; esac; done\nexec '{}' \"$@\"\n",
            log.display(),
            real.display()
        ),
    )
    .unwrap();
    std::fs::set_permissions(&wrapper, std::fs::Permissions::from_mode(0o755)).unwrap();
    let path = std::env::join_paths(
        std::iter::once(bin.path().to_path_buf())
            .chain(std::env::split_paths(&std::env::var_os("PATH").unwrap())),
    )
    .unwrap();
    let r = root.to_str().unwrap();
    let call = |args: &[&str], stdin: Option<&Value>| {
        let mut child = Command::new(BIN)
            .args(args)
            .env("PATH", &path)
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
        assert_eq!(out.status.code(), Some(0), "{args:?}: {out:?}");
        serde_json::from_slice::<Value>(&out.stdout).unwrap()
    };

    let outside = |s: common::TreeSnapshot| -> std::collections::BTreeMap<_, _> {
        s.0.into_iter()
            .filter(|(k, _)| k != ".umbral" && !k.starts_with(".umbral/"))
            .collect()
    };
    let before = outside(common::snapshot(root));
    let first = call(&["task", "save", r], Some(&save_input(Value::Null)));
    call(
        &["task", "save", r],
        Some(&save_input(first["revision"].clone())),
    );
    call(&["task", "resume", r, "fase1-nucleo"], None);
    call(&["task", "list", r], None);
    call(&["task", "export", r, "fase1-nucleo"], None);
    let after = outside(common::snapshot(root));
    assert_eq!(before, after, "a task command wrote outside .umbral/");
    assert!(root.join(".umbral/continuity/tasks/fase1-nucleo").is_dir());

    // The comparison can fail: a write outside .umbral/ is seen.
    std::fs::write(root.join("a.txt"), "changed").unwrap();
    assert_ne!(before, outside(common::snapshot(root)));

    let used: std::collections::BTreeSet<String> = std::fs::read_to_string(&log)
        .unwrap()
        .lines()
        .map(str::to_string)
        .collect();
    assert_eq!(used, ["check-ignore", "rev-parse"].map(String::from).into());
}

// ---- Phase 1 audit fixes ----

/// An executable script at `path` that appends a line to `witness` and fails.
fn witness_script(path: &Path, witness: &Path) {
    use std::os::unix::fs::PermissionsExt;
    std::fs::write(
        path,
        format!("#!/bin/sh\necho ran >> '{}'\nexit 1\n", witness.display()),
    )
    .unwrap();
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o755)).unwrap();
}

/// B-01: the observed repository's `core.fsmonitor` is a program; no task command runs it.
#[test]
fn b01_the_repository_fsmonitor_is_never_run() {
    let dir = repo();
    let root = dir.path();
    let bin = TempDir::new().unwrap();
    let witness = bin.path().join("witness");
    let evil = bin.path().join("fsmonitor");
    witness_script(&evil, &witness);
    git(root, &["config", "core.fsmonitor", evil.to_str().unwrap()]);
    let id = saved(root, &input("t", Value::Null, &["a.txt"]));
    saved(root, &input("t", json!(id), &["a.txt"]));
    for args in [&["resume", "t"][..], &["list"], &["export", "t"]] {
        let (code, doc) = run(args, root);
        assert_eq!(code, Some(0), "{doc}");
    }
    assert!(!witness.exists(), "core.fsmonitor was run");
}

/// B-02: a `git` planted in the repository is never run, even when `PATH` has an empty or a
/// relative entry (which would resolve inside the repository once git runs there).
#[test]
fn b02_a_git_planted_in_the_repository_is_never_run() {
    let dir = repo();
    let root = dir.path();
    let r = root.to_str().unwrap();
    let bin = TempDir::new().unwrap();
    let witness = bin.path().join("witness");
    witness_script(&root.join("git"), &witness);
    let path = std::env::var_os("PATH").unwrap();
    for prefix in [":", ".:", "./:"] {
        let mut p = std::ffi::OsString::from(prefix);
        p.push(&path);
        let env = [("PATH", p.as_os_str())];
        let (out, doc) = umbral_env(&["task", "list", r], None, &env);
        assert_eq!(out.status.code(), Some(0), "{prefix}: {doc}");
        let (out, doc) = umbral_env(
            &["task", "save", r],
            Some(&input("t", Value::Null, &["a.txt"])),
            &env,
        );
        assert_eq!(out.status.code(), Some(0), "{prefix}: {doc}");
        std::fs::remove_dir_all(root.join(".umbral")).unwrap();
    }
    assert!(!witness.exists(), "a planted git was run");
}

/// B-04: `GIT_*` variables inherited from the caller (a hook exports `GIT_DIR`) do not move
/// git to another repository, nor bring in config.
#[test]
fn b04_inherited_git_variables_are_ignored() {
    let dir = repo();
    let root = dir.path();
    let r = root.to_str().unwrap();
    std::fs::write(root.join(".gitignore"), ".env\n").unwrap();
    std::fs::write(root.join(".env"), "TOKEN=1").unwrap();
    let other = repo();
    std::fs::write(other.path().join(".env"), "x").unwrap();
    git(other.path(), &["add", "-f", ".env"]);
    git(other.path(), &["commit", "-q", "-m", "env"]);
    let head = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(root)
        .output()
        .unwrap()
        .stdout;
    let head = String::from_utf8(head).unwrap().trim().to_string();

    let bin = TempDir::new().unwrap();
    let witness = bin.path().join("witness");
    let evil = bin.path().join("fsmonitor");
    witness_script(&evil, &witness);
    let other_git = other.path().join(".git");
    let env = [
        ("GIT_DIR", other_git.as_os_str()),
        ("GIT_WORK_TREE", root.as_os_str()),
        ("GIT_CONFIG_COUNT", "1".as_ref()),
        ("GIT_CONFIG_KEY_0", "core.fsmonitor".as_ref()),
        ("GIT_CONFIG_VALUE_0", evil.as_os_str()),
    ];
    let (out, doc) = umbral_env(
        &["task", "save", r],
        Some(&input("t", Value::Null, &["a.txt"])),
        &env,
    );
    assert_eq!(out.status.code(), Some(0), "{doc}");
    assert_eq!(doc["git"]["head"], head.as_str());
    let (out, doc) = umbral_env(
        &["task", "save", r],
        Some(&input("e", Value::Null, &[".env"])),
        &env,
    );
    assert_eq!(out.status.code(), Some(2), "{doc}");
    assert_eq!(doc["sources"][0]["reason"], "git-ignored");
    assert!(
        !witness.exists(),
        "core.fsmonitor from the environment was run"
    );
}

/// B-03: a process that keeps swapping a directory of the store with a symlink to elsewhere
/// (an atomic exchange, so the check and the write can see different things) never gets a
/// save to create anything outside `.umbral/`.
#[cfg(target_os = "linux")]
#[test]
fn b03_a_symlink_swapped_in_during_saves_never_redirects_a_write() {
    use rustix::fs::{renameat_with, RenameFlags, CWD};
    use std::sync::atomic::{AtomicBool, Ordering};
    const SAVES: usize = 200;
    let dir = repo();
    let root = dir.path().to_path_buf();
    saved(&root, &input("t0", Value::Null, &["a.txt"]));
    let outside = TempDir::new().unwrap();
    let side = TempDir::new().unwrap();
    let link = side.path().join("link");
    std::os::unix::fs::symlink(outside.path(), &link).unwrap();
    let tasks = root.join(".umbral/continuity/tasks");

    let stop = AtomicBool::new(false);
    std::thread::scope(|s| {
        s.spawn(|| {
            while !stop.load(Ordering::Relaxed) {
                let _ = renameat_with(CWD, &tasks, CWD, &link, RenameFlags::EXCHANGE);
            }
        });
        for i in 1..=SAVES {
            // Any outcome but a write elsewhere is acceptable: refusing is the safe answer.
            save(&root, &input(&format!("t{i}"), Value::Null, &["a.txt"]));
        }
        stop.store(true, Ordering::Relaxed);
    });
    if std::fs::symlink_metadata(&tasks).unwrap().is_symlink() {
        renameat_with(CWD, &tasks, CWD, &link, RenameFlags::EXCHANGE).unwrap();
    }
    assert_eq!(files(outside.path()), Vec::<String>::new());
    assert!(!files(&tasks).is_empty());
}

/// B-08 / A M-2: a `.umbral/` made by someone else still gets its `.gitattributes` and
/// `.gitignore`, and the privacy warning is given; a later save does not repeat it.
#[test]
fn b08_an_existing_state_directory_gets_its_git_files_and_the_warning() {
    let dir = repo();
    let root = dir.path();
    std::fs::create_dir(root.join(".umbral")).unwrap();
    let (code, doc) = save(root, &input("t", Value::Null, &["a.txt"]));
    assert_eq!(code, Some(0), "{doc}");
    assert!(doc["privacy_warning"].is_string(), "{doc}");
    assert_eq!(
        std::fs::read(root.join(".umbral/.gitattributes")).unwrap(),
        b"* -text\n"
    );
    assert_eq!(
        std::fs::read(root.join(".umbral/.gitignore")).unwrap(),
        b"continuity/tmp/\n"
    );
    let (_, doc) = save(root, &input("u", Value::Null, &["a.txt"]));
    assert!(doc.get("privacy_warning").is_none(), "{doc}");
}

/// A-1: closing a fork of more than 8 heads would write a record with more parents than the
/// contract allows. It is refused before writing; 8 heads close.
#[test]
fn a1_closing_a_fork_of_more_than_8_heads_writes_nothing() {
    let dir = repo();
    let root = dir.path();
    let base = saved(root, &input("t", Value::Null, &["a.txt"]));
    let first = saved(root, &input("t", json!(base), &["a.txt"]));
    let record =
        std::fs::read_to_string(task_dir(root, "t").join(format!("{first}.json"))).unwrap();
    let mut heads = vec![first];
    for i in 1..9 {
        let p = plant(
            root,
            "t",
            record.replace("goal of t", &format!("goal {i}")).as_bytes(),
        );
        heads.push(p.file_stem().unwrap().to_str().unwrap().to_string());
    }
    let (code, err) = save(root, &input("t", json!(heads), &["a.txt"]));
    assert_eq!(code, Some(2), "{err}");
    assert_eq!(err["error"], "limite-excedido");
    assert_eq!(
        (&err["field"], &err["limit"], &err["actual"]),
        (&json!("parents"), &json!(8), &json!(9))
    );
    assert!(err["message"]
        .as_str()
        .unwrap()
        .contains("steps of up to 8"));
    assert_eq!(files(&task_dir(root, "t")).len(), 10);

    // On disk, a record with 9 parents is invalid (mutant M19).
    let mut big: Value = serde_json::from_str(&record).unwrap();
    big["task"] = json!("big");
    big["parents"] = json!(heads);
    plant(
        root,
        "big",
        serde_json::to_string_pretty(&big).unwrap().as_bytes(),
    );
    let (code, err) = run(&["resume", "big"], root);
    assert_eq!(
        (code, &err["error"]),
        (Some(5), &json!("registro-invalido")),
        "{err}"
    );

    // Closing eight of nine heads writes a valid revision and leaves a visible fork.
    let (code, partial) = save(root, &input("t", json!(&heads[..8]), &["a.txt"]));
    assert_eq!(code, Some(4), "{partial}");
    assert_eq!(partial["error"], "bifurcacion");
    let remaining: Vec<Value> = partial["heads"]
        .as_array()
        .unwrap()
        .iter()
        .map(|h| h["revision"].clone())
        .collect();
    assert_eq!(remaining.len(), 2);
    // A stale parent is still refused, even when another parent is a current head.
    let (code, stale) = save(root, &input("t", json!([base, remaining[0]]), &["a.txt"]));
    assert_eq!(code, Some(4), "{stale}");
    assert_eq!(stale["error"], "conflicto-revision");
    saved(root, &input("t", json!(remaining), &["a.txt"]));
    let (code, doc) = run(&["resume", "t"], root);
    assert_eq!(code, Some(0), "{doc}");
    assert_eq!(doc["parents"].as_array().unwrap().len(), 2);
}

/// B-05: a directory on a source's path swapped with a symlink to elsewhere while saves and
/// resumes read it never yields the fingerprint of the file outside the root.
#[cfg(target_os = "linux")]
#[test]
fn b05_a_symlink_swapped_in_during_reads_never_reads_outside() {
    use rustix::fs::{renameat_with, RenameFlags, CWD};
    use std::sync::atomic::{AtomicBool, Ordering};
    const READS: usize = 200;
    let dir = repo();
    let root = dir.path().to_path_buf();
    std::fs::create_dir(root.join("sub")).unwrap();
    std::fs::write(root.join("sub/f"), "inside\n").unwrap();
    saved(&root, &input("r", Value::Null, &["sub/f"]));
    let outside = TempDir::new().unwrap();
    std::fs::write(outside.path().join("f"), "OUTSIDE-SECRET\n").unwrap();
    let side = TempDir::new().unwrap();
    let link = side.path().join("link");
    std::os::unix::fs::symlink(outside.path(), &link).unwrap();
    let sub = root.join("sub");

    let stop = AtomicBool::new(false);
    let mut seen = Vec::new();
    std::thread::scope(|s| {
        s.spawn(|| {
            while !stop.load(Ordering::Relaxed) {
                let _ = renameat_with(CWD, &sub, CWD, &link, RenameFlags::EXCHANGE);
            }
        });
        for i in 0..READS {
            let (_, doc) = save(&root, &input(&format!("s{i}"), Value::Null, &["sub/f"]));
            seen.push(doc["sources"][0]["size"].clone());
            let (_, doc) = run(&["resume", "r"], &root);
            seen.push(doc["sources"][0]["current"]["size"].clone());
        }
        stop.store(true, Ordering::Relaxed);
    });
    assert!(
        !seen.contains(&json!(15)),
        "a file outside the root was read"
    );
    assert!(seen.contains(&json!(7)));
}

/// B-06, A M-4, A M-5: a nested repository's `.git` (any case) is invalid input; a source
/// inside a submodule is rejected as in-submodule instead of failing the save
/// with `io`; a symlinked directory that stays inside the root is reported as `symlink`.
#[test]
fn b06_m4_m5_nested_repositories_and_inner_symlinks_are_rejected_with_a_true_reason() {
    let dir = repo();
    let root = dir.path();
    for bad in ["src/.git/config", ".GIT/config", "src/.Git/x", ".UMBRAL/x"] {
        let (code, err) = save(root, &input("t", Value::Null, &[bad]));
        assert_eq!(
            (code, &err["error"]),
            (Some(2), &json!("entrada-invalida")),
            "{bad}: {err}"
        );
    }
    let sub = root.join("sub");
    std::fs::create_dir(&sub).unwrap();
    git(&sub, &["init", "-q"]);
    std::fs::write(sub.join("f"), "f").unwrap();
    git(&sub, &["add", "f"]);
    git(&sub, &["commit", "-q", "-m", "sub"]);
    git(root, &["add", "sub"]);
    std::os::unix::fs::symlink("src", root.join("alias")).unwrap();
    let (code, err) = save(root, &input("t", Value::Null, &["sub/f", "alias/c.rs"]));
    assert_eq!(code, Some(2), "{err}");
    assert_eq!(
        err["sources"],
        json!([
            { "path": "sub/f", "reason": "in-submodule" },
            { "path": "alias/c.rs", "reason": "symlink" },
        ])
    );
    assert!(!root.join(".umbral").exists());
}

/// Contract §5.2 (A M-1, mutant M6) and B-07: processes that save the same task at once
/// fork it. Every revision reported written is on disk, a save that forked says so with
/// exit 4, and the privacy warning reaches whoever created `.umbral/`, fork or not.
#[test]
fn b07_concurrent_saves_fork_visibly_and_keep_the_privacy_warning() {
    const PROCS: usize = 6;
    let mut forks = 0;
    for _round in 0..10 {
        let dir = repo();
        let root = dir.path().to_str().unwrap();
        let body = input("t", Value::Null, &["a.txt"]).to_string();
        let children: Vec<_> = (0..PROCS)
            .map(|_| {
                Command::new(BIN)
                    .args(["task", "save", root])
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .unwrap()
            })
            .collect();
        let mut children = children;
        for c in &mut children {
            c.stdin.take().unwrap().write_all(body.as_bytes()).unwrap();
        }
        let docs: Vec<(Option<i32>, Value)> = children
            .into_iter()
            .map(|c| {
                let out = c.wait_with_output().unwrap();
                (
                    out.status.code(),
                    serde_json::from_slice(&out.stdout).unwrap(),
                )
            })
            .collect();
        let mut written = Vec::new();
        for (code, doc) in &docs {
            match (code, doc["error"].as_str()) {
                (Some(0), None) => written.push(doc["revision"].as_str().unwrap()),
                (Some(4), Some("bifurcacion")) => {
                    forks += 1;
                    written.push(doc["revision"].as_str().unwrap());
                }
                (Some(4), Some("conflicto-revision")) => {}
                _ => panic!("{code:?}: {doc}"),
            }
        }
        let mut on_disk = files(&task_dir(dir.path(), "t"));
        on_disk.iter_mut().for_each(|f| f.truncate(32));
        // Two saves in the same millisecond can write the same bytes: the same revision.
        written.sort();
        written.dedup();
        assert_eq!(on_disk, written);
        let warned = docs
            .iter()
            .filter(|(_, d)| d["privacy_warning"].is_string())
            .count();
        assert!(warned >= 1, "{docs:?}");
    }
    assert!(forks > 0, "no save forked the task in 10 rounds of {PROCS}");
}

/// A M-3 / C-M4 / C-M3: usage errors of `task` are input errors with the JSON document on
/// stdout; a task id or revision that is not UTF-8 is reported as given (lossily), never as
/// an empty id, and extra arguments are not ignored.
#[test]
fn m3_task_usage_errors_are_json_and_name_what_was_given() {
    let dir = repo();
    let root = dir.path().to_str().unwrap();
    for args in [
        &["task"][..],
        &["task", "frob", root],
        &["task", "resume", root],
        &["task", "resume", root, "t", "extra"],
        &["task", "export", root, "t", "--revision"],
        &["task", "list", root, "--limit", "x"],
        &["task", "list", root, "--limit"],
        &["task", "save", root, "extra"],
        &["task", "resume", root, "t", "--revision", "garbage"],
    ] {
        let (out, err) = umbral(args, None);
        assert_eq!(out.status.code(), Some(2), "{args:?}: {err}");
        assert_eq!(err["error"], "entrada-invalida", "{args:?}");
    }
    let bad = std::ffi::OsStr::from_bytes(b"x\xff");
    for extra in [&[][..], &["junk"]] {
        let out = Command::new(BIN)
            .args(["task".as_ref(), "resume".as_ref(), root.as_ref(), bad])
            .args(extra)
            .output()
            .unwrap();
        let err: Value = serde_json::from_slice(&out.stdout).unwrap();
        assert_eq!(out.status.code(), Some(2), "{err}");
        assert_eq!(err["error"], "entrada-invalida");
        let message = err["message"].as_str().unwrap();
        assert!(!message.contains("\"\""), "{message}");
        if extra.is_empty() {
            assert!(message.contains("x\u{fffd}"), "{message}");
        }
    }
}

/// A M-7: a malformed `expected_revision` is invalid input, not a revision conflict (a
/// client that retries on conflict would loop).
#[test]
fn m7_a_malformed_expected_revision_is_invalid_input() {
    let dir = repo();
    let root = dir.path();
    let id = "0".repeat(32);
    for expected in [
        json!("garbage"),
        json!([]),
        json!([id]),
        json!([id, id]),
        json!([id, "x"]),
    ] {
        let (code, err) = save(root, &input("t", expected.clone(), &["a.txt"]));
        assert_eq!(
            (code, &err["error"]),
            (Some(2), &json!("entrada-invalida")),
            "{expected}: {err}"
        );
    }
    // Required even though it may be null.
    let mut v = input("t", Value::Null, &["a.txt"]);
    v.as_object_mut().unwrap().remove("expected_revision");
    let (code, err) = save(root, &v);
    assert_eq!(
        (code, &err["error"]),
        (Some(2), &json!("entrada-invalida")),
        "{err}"
    );
    assert!(!root.join(".umbral").exists());
}

/// A M-1: rules that no test watched. Forbidden characters (M8) and a `depends_on` outside
/// `sources` (M9) are invalid input; the file is the canonical serialization (M10); `list`
/// cuts a long goal at 120 characters and says so (M7).
#[test]
fn m1_forbidden_text_foreign_dependencies_canonical_bytes_and_goal_cut() {
    let dir = repo();
    let root = dir.path();
    for (field, bad) in [
        ("goal", "a\u{202E}b"),
        ("next_step", "a\u{1}b"),
        ("goal", "a\u{200B}b"),
        ("goal", "a\rb"),
    ] {
        let mut v = input("t", Value::Null, &["a.txt"]);
        v[field] = json!(bad);
        let (code, err) = save(root, &v);
        assert_eq!(
            (code, &err["error"]),
            (Some(2), &json!("entrada-invalida")),
            "{bad:?}: {err}"
        );
    }
    let mut v = input("t", Value::Null, &["a.txt"]);
    v["declarations"] = json!([{ "id": "d", "kind": "decision", "claimed_origin": "agent",
                                 "text": "t", "depends_on": ["b.txt"] }]);
    let (code, err) = save(root, &v);
    assert_eq!(
        (code, &err["error"]),
        (Some(2), &json!("entrada-invalida")),
        "{err}"
    );
    assert!(!root.join(".umbral").exists());

    // Canonical: two-space indent, the contract's key order, non-ASCII literal, final LF.
    let mut v = input("t", Value::Null, &["a.txt"]);
    v["goal"] = json!(format!("é{}", "x".repeat(120)));
    let id = saved(root, &v);
    let text = std::fs::read_to_string(task_dir(root, "t").join(format!("{id}.json"))).unwrap();
    assert!(
        text.starts_with(
            "{\n  \"format\": \"umbral-continuity/1\",\n  \"task\": \"t\",\n  \"parents\": [],\n  \"saved_at\": \""
        ),
        "{text}"
    );
    assert!(text.ends_with("\n  ]\n}\n"), "{text}");
    let keys: Vec<usize> = [
        "\n  \"emitter\": {\n    \"channel\": \"cli\"\n  },\n",
        &format!("\n  \"goal\": \"é{}\",\n", "x".repeat(120)),
        "\n  \"next_step\": ",
        "\n  \"blockers\": [],\n",
        "\n  \"declarations\": [],\n",
        "\n  \"git\": {\n    \"head_state\": \"present\",\n    \"head\": \"",
        "\n  \"sources\": [\n    {\n      \"path\": \"a.txt\",\n      \"capture\": \"captured\",\n      \"hash\": \"blake3:",
    ]
    .iter()
    .map(|k| text.find(k).unwrap_or_else(|| panic!("{k:?} in {text}")))
    .collect();
    assert!(keys.windows(2).all(|w| w[0] < w[1]), "{text}");

    let (_, listed) = run(&["list"], root);
    let head = &listed["tasks"][0]["heads"][0];
    assert_eq!(head["goal"], format!("é{}", "x".repeat(119)));
    assert_eq!(head["goal_truncated"], true);
    let mut v = input("u", Value::Null, &["a.txt"]);
    v["goal"] = json!(format!("é{}", "x".repeat(119)));
    saved(root, &v);
    let (_, listed) = run(&["list", "--cursor", "t"], root);
    assert_eq!(listed["tasks"][0]["heads"][0]["goal_truncated"], false);
}

/// B-07, deterministic: another revision of the task appears while a first save runs (a
/// `git` wrapper plants it when the save asks for HEAD). The save writes, reports the fork
/// with exit 4, and still gives the privacy warning, since it created the state.
#[test]
fn b07_a_first_save_that_forks_still_gives_the_privacy_warning() {
    use std::os::unix::fs::PermissionsExt;
    let donor = repo();
    let other = saved(donor.path(), &input("t", Value::Null, &["a.txt"]));
    let record = task_dir(donor.path(), "t").join(format!("{other}.json"));

    let dir = repo();
    let root = dir.path();
    let real = std::env::split_paths(&std::env::var_os("PATH").unwrap())
        .map(|d| d.join("git"))
        .find(|p| p.is_file())
        .unwrap();
    let bin = TempDir::new().unwrap();
    let tasks = task_dir(root, "t");
    std::fs::write(
        bin.path().join("git"),
        format!(
            "#!/bin/sh\ncase \"$*\" in *--verify*) mkdir -p '{t}' && cp '{r}' '{t}/' ;; esac\nexec '{g}' \"$@\"\n",
            t = tasks.display(),
            r = record.display(),
            g = real.display()
        ),
    )
    .unwrap();
    std::fs::set_permissions(
        bin.path().join("git"),
        std::fs::Permissions::from_mode(0o755),
    )
    .unwrap();
    let mut path = bin.path().as_os_str().to_os_string();
    path.push(":");
    path.push(std::env::var_os("PATH").unwrap());

    let (out, err) = umbral_env(
        &["task", "save", root.to_str().unwrap()],
        Some(&input("t", Value::Null, &["a.txt"])),
        &[("PATH", path.as_os_str())],
    );
    assert_eq!(out.status.code(), Some(4), "{err}");
    assert_eq!(err["error"], "bifurcacion");
    assert!(err["privacy_warning"].is_string(), "{err}");
    assert!(String::from_utf8_lossy(&out.stderr).contains("note: Records in .umbral/"));
    let mut expected = vec![format!("{}.json", err["revision"].as_str().unwrap())];
    expected.push(format!("{other}.json"));
    expected.sort();
    assert_eq!(files(&tasks), expected);
}

/// B-10: dangling objects, malformed refs and detached broken HEADs are not empty repos.
#[test]
fn b10_broken_heads_are_unknown_and_survive_resume_and_export() {
    for case in [
        "missing-object",
        "invalid-ref",
        "detached",
        "missing-branch",
    ] {
        let dir = repo();
        let root = dir.path();
        let branch = std::fs::read_to_string(root.join(".git/HEAD")).unwrap();
        let reference = root
            .join(".git")
            .join(branch.trim().strip_prefix("ref: ").unwrap());
        match case {
            "missing-object" => std::fs::write(reference, format!("{}\n", "1".repeat(40))).unwrap(),
            "invalid-ref" => std::fs::write(reference, "broken\n").unwrap(),
            "detached" => {
                std::fs::write(root.join(".git/HEAD"), format!("{}\n", "1".repeat(40))).unwrap()
            }
            "missing-branch" => std::fs::remove_file(reference).unwrap(),
            _ => unreachable!(),
        }
        let (code, doc) = save(root, &input("t", Value::Null, &[]));
        assert_eq!(code, Some(0), "{case}: {doc}");
        // An absent symbolic branch is Git's unborn state; it is indistinguishable from
        // deleting the sole branch ref. Other broken states must remain unknown.
        let expected = if case == "missing-branch" {
            "unborn"
        } else {
            "unknown"
        };
        assert_eq!(
            doc["git"],
            json!({"head_state": expected, "head": null}),
            "{case}"
        );
        let (code, resumed) = run(&["resume", "t"], root);
        assert_eq!(code, Some(0), "{resumed}");
        assert_eq!(resumed["git"], doc["git"]);
        let (code, exported) = run(&["export", "t"], root);
        assert_eq!(code, Some(0), "{exported}");
    }
}

/// A directory that becomes a submodule (a .git file, rather than a directory) is
/// uncheckable on resume, and is refused on the next save.
#[test]
fn in_submodule_is_reported_on_resume_and_save_with_a_git_file() {
    let dir = repo();
    let root = dir.path();
    let sub = root.join("src");
    saved(root, &input("t", Value::Null, &["src/c.rs"]));
    let metadata = TempDir::new().unwrap();
    git(
        &sub,
        &[
            "init",
            "-q",
            "--separate-git-dir",
            metadata.path().join("git").to_str().unwrap(),
        ],
    );
    assert!(sub.join(".git").is_file());
    let (code, resumed) = run(&["resume", "t"], root);
    assert_eq!(code, Some(0), "{resumed}");
    assert_eq!(resumed["sources"][0]["state"], "unknown");
    assert_eq!(resumed["sources"][0]["reason"], "in-submodule");
    let (code, err) = save(root, &input("u", Value::Null, &["src/c.rs"]));
    assert_eq!(code, Some(2), "{err}");
    assert_eq!(err["sources"][0]["reason"], "in-submodule");
}

/// Rehashing an invalid record must not bypass the schema checks on disk.
#[test]
fn rehashed_records_reject_duplicate_keys_and_invalid_captured_metadata() {
    let dir = repo();
    let root = dir.path();
    let id = saved(root, &input("t", Value::Null, &["a.txt"]));
    let good = std::fs::read_to_string(task_dir(root, "t").join(format!("{id}.json"))).unwrap();
    let value: Value = serde_json::from_str(&good).unwrap();
    let mut cases = vec![
        good.replace("\"goal\":", "\"goal\": \"shadowed\", \"goal\":"),
        good.replace("\"channel\":", "\"channel\": \"mcp\", \"channel\":"),
    ];
    for (field, bad) in [
        ("saved_at", json!("INVALID")),
        ("saved_at", json!("2026-02-29T12:00:00.000Z")),
        ("saved_at", json!("1900-02-29T12:00:00.000Z")),
        ("saved_at", json!("2026-13-01T12:00:00.000Z")),
        ("saved_at", json!("2026-01-01T24:00:00.000Z")),
        ("saved_at", json!("2026-01-01T00:00:00Z")),
        ("git", json!({"head_state": "present", "head": null})),
        (
            "git",
            json!({"head_state": "present", "head": "g".repeat(40)}),
        ),
        (
            "git",
            json!({"head_state": "unknown", "head": "a".repeat(40)}),
        ),
    ] {
        let mut v = value.clone();
        v[field] = bad;
        cases.push(v.to_string());
    }
    let mut v = value.clone();
    v["sources"][0]["captured_at"] = json!("2026-04-31T00:00:00.000Z");
    cases.push(v.to_string());
    for bad in cases {
        let file = task_dir(root, "t").join(format!("{id}.json"));
        std::fs::remove_file(&file).unwrap();
        let planted = plant(root, "t", bad.as_bytes());
        for operation in ["resume", "export"] {
            let (code, err) = run(&[operation, "t"], root);
            assert_eq!(code, Some(5), "{operation}: {err}; input: {bad}");
            assert_eq!(err["error"], "registro-invalido", "{err}");
        }
        assert_eq!(std::fs::read_to_string(&planted).unwrap(), bad);
        std::fs::remove_file(planted).unwrap();
        std::fs::write(file, &good).unwrap();
    }
    // Valid boundary dates and both Git object formats remain readable.
    for (stamp, state, head) in [
        ("2000-02-29T23:59:59.999Z", "present", json!("a".repeat(40))),
        ("2024-02-29T00:00:00.000Z", "present", json!("b".repeat(64))),
        ("2026-12-31T00:00:00.000Z", "unknown", Value::Null),
        ("2026-01-01T00:00:00.000Z", "unborn", Value::Null),
    ] {
        let file = task_dir(root, "t").join(format!("{id}.json"));
        std::fs::remove_file(&file).unwrap();
        let mut v = value.clone();
        v["saved_at"] = json!(stamp);
        v["sources"][0]["captured_at"] = json!(stamp);
        v["git"] = json!({"head_state":state,"head":head});
        let planted = plant(root, "t", v.to_string().as_bytes());
        let (code, doc) = run(&["resume", "t"], root);
        assert_eq!(code, Some(0), "{doc}");
        assert_eq!(doc["saved_at"], stamp);
        std::fs::remove_file(planted).unwrap();
        std::fs::write(file, &good).unwrap();
    }
}

/// A non-regular entry (Unix socket) in one task's directory is reported as unsafe state
/// for that task alone; opening other records never fails through it (ENXIO isolation).
#[test]
fn a_socket_revision_is_isolated_as_unsafe_while_other_tasks_remain_visible() {
    let dir = repo();
    let root = dir.path();
    saved(root, &input("valid", Value::Null, &["a.txt"]));
    let bad = task_dir(root, "bad");
    std::fs::create_dir_all(&bad).unwrap();
    let socket_path = bad.join(format!("{}.json", "a".repeat(32)));
    let _socket = std::os::unix::net::UnixListener::bind(socket_path).unwrap();
    let (code, listed) = run(&["list"], root);
    assert_eq!(code, Some(0), "{listed}");
    assert_eq!(listed["tasks"][0]["task"], "bad");
    assert_eq!(listed["tasks"][0]["integrity"], "estado-inseguro");
    assert_eq!(listed["tasks"][1]["task"], "valid");
    assert_eq!(listed["tasks"][1]["integrity"], "ok");
}
