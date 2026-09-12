//! Shared helpers for the CLI-level tests.
//!
//! Every test runs the real binary as a child process with its own `XDG_DATA_HOME`, so the
//! tool's state can never land in the developer's real data directory and parallel tests
//! cannot race on a process-wide environment variable.

#![allow(dead_code)]

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use tempfile::TempDir;

pub const BIN: &str = env!("CARGO_BIN_EXE_umbral");

pub struct Sandbox {
    /// Stands in for `$XDG_DATA_HOME`.
    pub data: TempDir,
    /// The directory the user owns and points the tool at.
    pub root: TempDir,
}

impl Sandbox {
    pub fn new() -> Self {
        Sandbox {
            data: TempDir::new().unwrap(),
            root: TempDir::new().unwrap(),
        }
    }

    pub fn root(&self) -> &Path {
        self.root.path()
    }

    /// Run the binary with this sandbox's isolated state directory.
    pub fn run(&self, args: &[&str]) -> Output {
        Command::new(BIN)
            .args(args)
            .env("XDG_DATA_HOME", self.data.path())
            .env("HOME", self.data.path())
            .output()
            .expect("failed to run the umbral binary")
    }

    /// Run and require success, returning stdout.
    pub fn run_ok(&self, args: &[&str]) -> String {
        let out = self.run(args);
        assert!(
            out.status.success(),
            "command {:?} failed with {:?}\nstdout:\n{}\nstderr:\n{}",
            args,
            out.status.code(),
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8_lossy(&out.stdout).into_owned()
    }

    /// `init` then `observe`, both required to succeed.
    pub fn init_and_observe(&self) -> String {
        self.run_ok(&["init", &self.root().to_string_lossy()]);
        self.run_ok(&["observe", &self.root().to_string_lossy()])
    }

    pub fn write(&self, rel: &str, body: &str) {
        let p = self.root().join(rel);
        if let Some(parent) = p.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(p, body).unwrap();
    }
}

/// An observable fingerprint of a tree: relative path -> (kind, size, mtime, content hash).
/// Used by A3, which must show the user's tree is byte-identical before and after the tool
/// runs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeSnapshot(pub BTreeMap<String, (String, u64, i64, [u8; 32])>);

pub fn snapshot(dir: &Path) -> TreeSnapshot {
    let mut map = BTreeMap::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        for entry in std::fs::read_dir(&d).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let meta = std::fs::symlink_metadata(&path).unwrap();
            let rel = path
                .strip_prefix(dir)
                .unwrap()
                .to_string_lossy()
                .into_owned();
            let kind = if meta.is_symlink() {
                "symlink"
            } else if meta.is_dir() {
                stack.push(path.clone());
                "dir"
            } else if meta.is_file() {
                "file"
            } else {
                "other"
            };
            let hash = if kind == "file" {
                *blake3::hash(&std::fs::read(&path).unwrap()).as_bytes()
            } else {
                [0u8; 32]
            };
            let mtime = meta
                .modified()
                .ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d| d.as_nanos() as i64)
                .unwrap_or(0);
            map.insert(rel, (kind.to_string(), meta.len(), mtime, hash));
        }
    }
    TreeSnapshot(map)
}

/// Every line of a rendered output, with its declared label.
pub fn lines(out: &str) -> Vec<(String, String)> {
    out.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| {
            let mut it = l.splitn(2, char::is_whitespace);
            let label = it.next().unwrap_or("").to_string();
            let rest = it.next().unwrap_or("").trim().to_string();
            (label, rest)
        })
        .collect()
}

/// The set of labels used in a rendered output.
pub fn labels_used(out: &str) -> std::collections::BTreeSet<String> {
    lines(out).into_iter().map(|(l, _)| l).collect()
}

/// Assert that every line declares one of the four contract labels.
pub fn assert_all_labelled(out: &str) {
    let allowed = ["observed", "derived", "ambiguous", "unknown"];
    for (label, _) in lines(out) {
        assert!(
            allowed.contains(&label.as_str()),
            "line does not declare a contract label: {:?}\nfull output:\n{out}",
            label
        );
    }
}

pub fn path_of(dir: &Path) -> PathBuf {
    dir.to_path_buf()
}
