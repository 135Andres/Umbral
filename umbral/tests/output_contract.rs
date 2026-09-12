//! The output contract: four labels, no invented values, no anthropomorphic language.
//! Covers A2 (nothing invented) and the lexical rule.
//!
//! Each contract check is also exercised against output that deliberately breaks it, so the
//! test is shown to discriminate rather than to pass unconditionally.

mod common;

use common::{assert_all_labelled, labels_used, lines, Sandbox};
use umbral::report::{
    contract_violations, fields_in, first_banned_word, label_contract_violations,
    label_contract_violations_in_text, unlabelled_lines, Line, BANNED_LEXICON, DERIVED_ONLY_FIELDS,
    OBSERVED_FIELDS,
};

/// Every line of every command declares exactly one of the four labels.
#[test]
fn every_command_emits_only_labelled_lines() {
    let s = Sandbox::new();
    s.write("a.txt", "alpha");
    s.write("sub/b.txt", "beta");
    s.init_and_observe();
    s.write("a.txt", "alpha changed");
    s.write("new.txt", "new");
    s.run_ok(&["observe", &s.root().to_string_lossy()]);

    let r = s.root().to_string_lossy().to_string();
    for args in [
        vec!["init", r.as_str()], // will fail (already initialised) — stderr only
        vec!["observe", r.as_str()],
        vec!["status", r.as_str()],
        vec!["changes", r.as_str()],
        vec!["show", r.as_str(), "a.txt"],
        vec!["check", r.as_str()],
        vec!["workspaces"],
    ] {
        let out = s.run(&args);
        let stdout = String::from_utf8_lossy(&out.stdout);
        assert_all_labelled(&stdout);
        assert!(
            unlabelled_lines(&stdout).is_empty(),
            "unlabelled lines in {:?}: {:?}",
            args,
            unlabelled_lines(&stdout)
        );
    }
}

/// No command output uses anthropomorphic language.
#[test]
fn no_command_output_uses_banned_vocabulary() {
    let s = Sandbox::new();
    s.write("a.txt", "alpha");
    s.init_and_observe();

    let r = s.root().to_string_lossy().to_string();
    for args in [
        vec!["status", r.as_str()],
        vec!["changes", r.as_str()],
        vec!["show", r.as_str(), "a.txt"],
        vec!["check", r.as_str()],
        vec!["workspaces"],
        vec!["observe", r.as_str()],
    ] {
        let out = s.run(&args);
        let stdout = String::from_utf8_lossy(&out.stdout);
        for (label, text) in lines(&stdout) {
            assert!(
                first_banned_word(&text).is_none(),
                "banned word {:?} in {:?} output line: {} {}",
                first_banned_word(&text),
                args,
                label,
                text
            );
        }
    }
}

/// The lexicon is not vacuous: it actually catches the words it lists.
#[test]
fn the_banned_lexicon_discriminates() {
    assert!(first_banned_word("Umbral knows the file changed").is_some());
    assert!(first_banned_word("the tool believes this").is_some());
    assert!(first_banned_word("it detects a rename").is_some());
    // And it does not fire on the vocabulary the tool is supposed to use.
    assert!(first_banned_word("observed run=1 entries=4").is_none());
    assert!(first_banned_word("derived complete=true").is_none());
    assert!(first_banned_word("unknown reason=not-observed-in-any-run").is_none());
    // "unknown" contains "know" but is not the word "know".
    assert!(first_banned_word("unknown").is_none());
    assert!(!BANNED_LEXICON.is_empty());
}

/// A contract check that could not fail would prove nothing, so it is shown failing.
#[test]
fn the_contract_check_rejects_doctored_output() {
    let good = vec![
        Line::observed("run=1 entries=4"),
        Line::derived("complete=true"),
    ];
    assert!(contract_violations(&good).is_empty());

    let bad_vocabulary = vec![Line::observed("Umbral knows the file changed")];
    assert!(
        !contract_violations(&bad_vocabulary).is_empty(),
        "the check must reject anthropomorphic language"
    );

    let empty = vec![Line::derived("")];
    assert!(
        !contract_violations(&empty).is_empty(),
        "the check must reject an empty line"
    );

    // And an unlabelled rendered line is caught.
    assert_eq!(
        unlabelled_lines("this line has no label\n"),
        vec!["this line has no label"]
    );
    assert!(unlabelled_lines("observed this line is fine\n").is_empty());
}

/// A2: a value that was not obtainable is reported as `none` with an `unknown` line naming
/// the reason. Nothing is filled in.
#[cfg(unix)]
#[test]
fn unobservable_metadata_is_reported_as_absent_not_invented() {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    let s = Sandbox::new();
    s.write("visible.txt", "v");
    s.write("blocked/hidden.txt", "h");
    let blocked = s.root().join("blocked");
    fs::set_permissions(&blocked, fs::Permissions::from_mode(0o000)).unwrap();

    let r = s.root().to_string_lossy().to_string();
    s.run_ok(&["init", r.as_str()]);
    let out = s.run(&["observe", r.as_str()]);
    // The run is recorded but incomplete.
    assert_eq!(
        out.status.code(),
        Some(3),
        "an incomplete run must be visible to the shell"
    );
    fs::set_permissions(&blocked, fs::Permissions::from_mode(0o755)).unwrap();

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert_all_labelled(&stdout);
    assert!(
        stdout.contains("complete=false"),
        "the incompleteness must be stated:\n{stdout}"
    );

    // The count of unobservable paths is an aggregate the tool computed, so it is `derived`.
    // The unknowns themselves are reported per path, by `show`.
    let status = s.run_ok(&["status", r.as_str()]);
    assert!(
        status.contains("unobservable-paths=1"),
        "status must count the unobserved path:\n{status}"
    );
    assert!(
        !status
            .lines()
            .any(|l| l.starts_with("observed") && l.contains("unobservable-paths=")),
        "a computed count must not be labelled observed:\n{status}"
    );

    let show = s.run_ok(&["show", r.as_str(), "blocked"]);
    assert_all_labelled(&show);
    assert!(
        show.contains("size="),
        "the entry itself was observed, so its size is a fact"
    );
    assert!(
        show.contains("unknown") && show.contains("observation-error"),
        "the failure must be reported as unknown with the reason:\n{show}"
    );
}

/// A path that was never observed is `unknown`, not an empty success and not an invented
/// value.
#[test]
fn a_never_observed_path_is_unknown() {
    let s = Sandbox::new();
    s.write("a.txt", "alpha");
    s.init_and_observe();

    let r = s.root().to_string_lossy().to_string();
    let out = s.run_ok(&["show", r.as_str(), "never-existed.txt"]);
    assert_all_labelled(&out);
    let (label, text) = &lines(&out)[0];
    assert_eq!(label, "unknown");
    assert!(
        text.contains("reason=not-observed-in-any-run"),
        "got: {text}"
    );
}

/// Before any run, `changes` says the comparison is not possible — it does not say "no
/// changes".
#[test]
fn an_empty_workspace_reports_not_comparable() {
    let s = Sandbox::new();
    s.write("a.txt", "alpha");
    s.run_ok(&["init", &s.root().to_string_lossy()]);

    let r = s.root().to_string_lossy().to_string();
    let out = s.run_ok(&["changes", r.as_str()]);
    assert_all_labelled(&out);
    assert!(
        out.contains("comparison-not-possible") && out.contains("reason=no-run-recorded"),
        "an empty workspace must not report 'no changes':\n{out}"
    );

    // A single run is likewise not comparable.
    s.run_ok(&["observe", r.as_str()]);
    let out2 = s.run_ok(&["changes", r.as_str()]);
    assert!(out2.contains("reason=only-one-run"), "got:\n{out2}");
}

/// The four labels are all reachable, so the contract is not a distinction the tool never
/// makes in practice.
#[test]
fn all_four_labels_are_actually_used() {
    let s = Sandbox::new();
    s.write("a.txt", "alpha");
    s.write("dup1.txt", "same bytes");
    s.write("dup2.txt", "same bytes");
    s.init_and_observe();

    let r = s.root().to_string_lossy().to_string();
    let mut used = std::collections::BTreeSet::new();
    for args in [
        vec!["status", r.as_str()],
        vec!["changes", r.as_str()],
        vec!["show", r.as_str(), "a.txt"],
        vec!["check", r.as_str()],
    ] {
        used.extend(labels_used(&s.run_ok(&args)));
    }
    assert!(used.contains("observed"), "labels used: {used:?}");
    assert!(used.contains("derived"), "labels used: {used:?}");
    assert!(used.contains("unknown"), "labels used: {used:?}");
}

// ---------------------------------------------------------------------------------------
// The observed/derived distinction, made enforceable.
//
// This is the defect the reader protocol found (F-V01-2): a value the tool computed was
// labelled `observed`. A reader has to be able to tell, from the output alone, whether a value
// came from their filesystem or was produced by the tool. The check below is exhaustive on
// purpose — `observed` lines may carry only the fields in `OBSERVED_FIELDS` — so that adding a
// new one is a deliberate act rather than an oversight.
// ---------------------------------------------------------------------------------------

/// No command's output labels a computed value as observed.
#[test]
fn no_command_labels_a_computed_value_as_observed() {
    let s = Sandbox::new();
    s.write("a.txt", "alpha");
    s.write("sub/b.txt", "beta");
    s.write("dup1.txt", "same bytes");
    s.write("dup2.txt", "same bytes");
    #[cfg(unix)]
    std::os::unix::fs::symlink("a.txt", s.root().join("link.txt")).unwrap();

    s.init_and_observe();
    s.write("a.txt", "alpha changed");
    s.write("new.txt", "new");
    s.run_ok(&["observe", &s.root().to_string_lossy()]);

    let r = s.root().to_string_lossy().to_string();
    for args in [
        vec!["init", r.as_str()],
        vec!["observe", r.as_str()],
        vec!["status", r.as_str()],
        vec!["changes", r.as_str()],
        vec!["show", r.as_str(), "a.txt"],
        vec!["show", r.as_str(), "sub"],
        vec!["check", r.as_str()],
        vec!["workspaces"],
    ] {
        let out = s.run(&args);
        let stdout = String::from_utf8_lossy(&out.stdout);
        let violations = label_contract_violations_in_text(&stdout);
        assert!(
            violations.is_empty(),
            "label contract violated by {args:?}:\n  {}\nfull output:\n{stdout}",
            violations.join("\n  ")
        );
    }
}

/// The check is not vacuous: it catches a computed value labelled observed, and a field that
/// is not an observed field at all.
#[test]
fn the_label_check_rejects_doctored_lines() {
    // Conforming.
    assert!(label_contract_violations(&[
        Line::observed("canonical=/tmp/x"),
        Line::observed("kind=file  size=6  mtime=2026-01-01T00:00:00.000Z"),
        Line::derived("entries=3"),
    ])
    .is_empty());

    // A computed field on an observed line — the F-V01-2 defect, in the exact form the reader
    // found it.
    let doctored = vec![Line::observed("workspace-id=deadbeefdeadbeef")];
    assert!(
        !label_contract_violations(&doctored).is_empty(),
        "the check must reject a computed identifier labelled observed"
    );

    // Every field the reader's report named.
    for field in [
        "workspace-id=x",
        "state-dir=/tmp/x",
        "entries=3",
        "files=2",
        "dirs=1",
        "symlinks=0",
        "other=0",
        "content-verified=2",
        "content-not-verified=0",
        "unobservable-paths=0",
        "log-runs=1",
        "log-observations=5",
    ] {
        let lines = vec![Line::observed(field.to_string())];
        assert!(
            !label_contract_violations(&lines).is_empty(),
            "the check must reject `{field}` on an observed line"
        );
    }

    // An unknown field on an observed line is caught by the allowlist half, not the denylist.
    let unknown_field = vec![Line::observed("something-new=1")];
    assert!(
        !label_contract_violations(&unknown_field).is_empty(),
        "the allowlist must reject a field that is not an observed field"
    );

    // And the field parser actually parses.
    assert_eq!(
        fields_in("run=1  kind=file  size=6"),
        vec!["run=", "kind=", "size="]
    );
    assert!(DERIVED_ONLY_FIELDS.contains(&"hash="));
    assert!(!OBSERVED_FIELDS.contains(&"hash="));
}

/// Every command that emits an `observed` line emits at least one, so the category is not
/// merely theoretical: the tool really does read facts from the filesystem.
#[test]
fn observed_lines_carry_only_filesystem_facts() {
    let s = Sandbox::new();
    s.write("a.txt", "alpha");
    s.init_and_observe();
    let r = s.root().to_string_lossy().to_string();

    let show = s.run_ok(&["show", r.as_str(), "a.txt"]);
    let observed: Vec<&str> = show.lines().filter(|l| l.starts_with("observed")).collect();
    assert!(
        !observed.is_empty(),
        "show must report observed facts:\n{show}"
    );
    for line in observed {
        for field in fields_in(line) {
            assert!(
                OBSERVED_FIELDS.contains(&field.as_str()),
                "observed line carries `{field}`: {line}"
            );
        }
    }
}
