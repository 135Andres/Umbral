//! v0.2 slice 1 — the output contract `umbral-output/1` (`CONTRACT.md`, `UD-030`).
//!
//! One test (or group) per criterion `A2-T1-n` of `docs/candidates/V0.2-SCOPE-PROPOSAL.md` §9.3.
//! Each is written so that it fails when the property it names is removed.

mod common;

use std::collections::BTreeSet;

use common::Sandbox;
use proptest::prelude::*;
use umbral::contract::{
    self, parse, read_value, write_value, EscapeClass, Item, ParseErrorKind, EDITION,
};
use umbral::report::Label;

const HEADER: &str = "derived   contract=umbral-output/1";

fn first_line(out: &str) -> &str {
    out.lines().next().unwrap_or("")
}

// ---------------------------------------------------------------------------------------
// A2-T1-1 — the header
// ---------------------------------------------------------------------------------------

#[test]
fn every_command_begins_with_the_contract_header_and_stderr_never_carries_it() {
    let s = Sandbox::new();
    s.write("a.txt", "alpha");
    let r = s.root().to_string_lossy().to_string();
    let runs: Vec<Vec<&str>> = vec![
        vec!["init", &r],
        vec!["observe", &r],
        vec!["observe", &r],
        vec!["status", &r],
        vec!["changes", &r],
        vec!["show", &r, "a.txt"],
        vec!["workspaces"],
        vec!["check", &r],
    ];
    for args in runs {
        let out = s.run(&args);
        let stdout = String::from_utf8_lossy(&out.stdout);
        let stderr = String::from_utf8_lossy(&out.stderr);
        assert_eq!(first_line(&stdout), HEADER, "{args:?}:\n{stdout}");
        assert!(!stderr.contains("contract="), "{args:?} stderr:\n{stderr}");
        assert!(
            parse(&stdout).is_ok(),
            "{args:?} is not readable:\n{stdout}"
        );
    }
    // A usage error writes only to stderr: no header, no stdout at all.
    let out = s.run(&["status"]);
    assert!(out.stdout.is_empty());
}

#[test]
fn the_reader_refuses_an_output_without_a_header_or_with_another_edition() {
    let missing = parse("derived   run=1\n").unwrap_err();
    assert_eq!(missing.kind, ParseErrorKind::MissingHeader);

    let other = parse("derived   contract=umbral-output/2\nderived   run=1\n").unwrap_err();
    assert_eq!(
        other.kind,
        ParseErrorKind::UnknownEdition("umbral-output/2".into())
    );

    let empty = parse("").unwrap_err();
    assert_eq!(empty.kind, ParseErrorKind::MissingHeader);

    assert_eq!(EDITION, "umbral-output/1");
    assert_eq!(contract::header_line(), HEADER);
}

// ---------------------------------------------------------------------------------------
// A2-T1-2 — value round-trip over the whole domain
// ---------------------------------------------------------------------------------------

/// Bytes biased towards every class of `CONTRACT.md` §4, so combinations actually occur.
fn interesting_byte_string() -> impl Strategy<Value = Vec<u8>> {
    let piece = prop_oneof![
        Just(b"\\".to_vec()),
        Just(b" ".to_vec()),
        Just(b"  ".to_vec()),
        Just(b"\n".to_vec()),
        Just(b"\r".to_vec()),
        Just(b"\t".to_vec()),
        Just(b"\x7f".to_vec()),
        Just(b"\xff".to_vec()),
        Just(b"\xc3".to_vec()),
        Just("ñ".as_bytes().to_vec()),
        Just("\u{202e}".as_bytes().to_vec()),
        Just("\u{200b}".as_bytes().to_vec()),
        Just("\u{85}".as_bytes().to_vec()),
        Just(b"\\x41".to_vec()),
        Just(b"=".to_vec()),
        Just(b"a".to_vec()),
        prop::collection::vec(any::<u8>(), 0..4),
    ];
    prop::collection::vec(piece, 0..8).prop_map(|v| v.concat())
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(2048))]

    #[test]
    fn every_value_round_trips(v in interesting_byte_string()) {
        let w = write_value(&v);
        prop_assert_eq!(read_value(&w.text).unwrap(), v.clone());
        // A written value never contains a separator, a boundary or a raw control byte.
        prop_assert!(!w.text.contains("  "));
        prop_assert!(!w.text.starts_with(' ') && !w.text.ends_with(' '));
        prop_assert!(!w.text.bytes().any(|b| b < 0x20 || b == 0x7f));
    }

    #[test]
    fn arbitrary_bytes_round_trip(v in prop::collection::vec(any::<u8>(), 0..64)) {
        prop_assert_eq!(read_value(&write_value(&v).text).unwrap(), v);
    }
}

/// The generator is only evidence if it reaches every class, alone and combined.
#[test]
fn the_generator_produces_every_escape_class_and_combinations() {
    use proptest::strategy::ValueTree;
    use proptest::test_runner::TestRunner;
    let mut runner = TestRunner::deterministic();
    let mut seen: BTreeSet<EscapeClass> = BTreeSet::new();
    let mut combined = 0;
    for _ in 0..2048 {
        let v = interesting_byte_string()
            .new_tree(&mut runner)
            .unwrap()
            .current();
        let classes = write_value(&v).classes;
        if classes.len() > 1 {
            combined += 1;
        }
        seen.extend(classes);
    }
    assert_eq!(seen.len(), EscapeClass::ALL.len(), "seen: {seen:?}");
    assert!(combined > 100, "only {combined} values combined classes");
}

// ---------------------------------------------------------------------------------------
// A2-T1-3 and A2-T1-6 — association round-trip on real outputs; no raw control bytes
// ---------------------------------------------------------------------------------------

#[cfg(unix)]
#[test]
fn real_outputs_read_back_with_every_name_intact() {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;

    let s = Sandbox::new();
    let names: Vec<&[u8]> = vec![
        b"line1\nline2.txt",
        b"two  spaces.txt",
        b" leading.txt",
        b"back\\slash.txt",
        b"bytes-\xff\xfe.txt",
        "rev\u{202e}txt.exe".as_bytes(),
        "zero\u{200b}width.txt".as_bytes(),
        "Mis documentos ño.txt".as_bytes(),
    ];
    let r = s.root().to_string_lossy().to_string();
    s.run_ok(&["init", &r]);
    s.run_ok(&["observe", &r]);
    for n in &names {
        std::fs::write(s.root().join(OsStr::from_bytes(n)), b"x").unwrap();
    }
    s.run_ok(&["observe", &r]);

    let out = s.run_ok(&["changes", &r]);
    for line in out.lines() {
        assert!(
            !line.bytes().any(|b| b < 0x20 || b == 0x7f),
            "raw control in {line:?}"
        );
    }
    let parsed = parse(&out).unwrap_or_else(|e| panic!("{e:?}\n{out}"));
    let created: BTreeSet<Vec<u8>> = parsed
        .iter()
        .filter(|l| l.items.first() == Some(&Item::Token("created".into())))
        .map(|l| l.field("path").expect("created names its path").to_vec())
        .collect();
    let expected: BTreeSet<Vec<u8>> = names.iter().map(|n| n.to_vec()).collect();
    assert_eq!(created, expected, "\n{out}");

    // Every other command also reads back.
    for args in [
        vec!["status", r.as_str()],
        vec!["workspaces"],
        vec!["check", r.as_str()],
    ] {
        let out = s.run_ok(&args);
        parse(&out).unwrap_or_else(|e| panic!("{args:?}: {e:?}\n{out}"));
    }
    let show = s.run_os_ok(&[
        OsStr::new("show"),
        s.root().as_os_str(),
        OsStr::from_bytes(b"line1\nline2.txt"),
    ]);
    parse(&show).unwrap_or_else(|e| panic!("{e:?}\n{show}"));
}

// ---------------------------------------------------------------------------------------
// A2-T1-4 — fail-closed reading
// ---------------------------------------------------------------------------------------

fn kind_of(body: &str) -> ParseErrorKind {
    parse(&format!("{HEADER}\n{body}")).unwrap_err().kind
}

#[test]
fn malformed_and_non_canonical_input_is_rejected_never_repaired() {
    use ParseErrorKind::*;
    assert_eq!(kind_of("derived   path=a\\q\n"), MalformedEscape);
    assert_eq!(kind_of("derived   path=a\\x4\n"), MalformedEscape);
    assert_eq!(kind_of("derived   path=a\\\n"), MalformedEscape);
    assert_eq!(kind_of("derived   path=a\\xff\n"), NonCanonical);
    assert_eq!(kind_of("derived   path=\\x41\n"), NonCanonical);
    assert_eq!(kind_of("derived   path=a\\x5Cb\n"), NonCanonical);
    assert_eq!(kind_of("derived   path=\u{202e}x\n"), NonCanonical);
    assert_eq!(kind_of("derived   path=a\tb\n"), RawControl);
    assert_eq!(kind_of("derived   a   b\n"), EmptyItem);
    assert_eq!(kind_of("derived   a    b\n"), EmptyItem);
    assert_eq!(kind_of("derived   a  \n"), EmptyItem);
    assert_eq!(kind_of("derived   =x\n"), EmptyKey);
    assert_eq!(kind_of("guessed   a\n"), UnknownLabel);
    assert_eq!(kind_of("\n"), UnknownLabel);
    let truncated = parse(&format!("{HEADER}\nderived   a")).unwrap_err();
    assert_eq!(truncated.kind, Truncated);

    let e = parse(&format!("{HEADER}\nderived   ok=1\nderived   path=a\\q\n")).unwrap_err();
    assert_eq!((e.line, e.column), (3, 17), "the error names its position");
}

// ---------------------------------------------------------------------------------------
// A2-T1-5 — unknown fields and tokens are preserved
// ---------------------------------------------------------------------------------------

#[test]
fn unknown_fields_and_tokens_are_preserved_opaque() {
    let out = format!("{HEADER}\nderived   future-token  future-field=v\\x0Aw  path=a\n");
    let lines = parse(&out).unwrap();
    assert_eq!(
        lines[0].items,
        vec![
            Item::Token("future-token".into()),
            Item::Field {
                key: "future-field".into(),
                value: b"v\nw".to_vec()
            },
            Item::Field {
                key: "path".into(),
                value: b"a".to_vec()
            },
        ]
    );
}

// ---------------------------------------------------------------------------------------
// A2-T1-7 and A2-T1-8 — readable stays readable; deceptive characters are escaped
// ---------------------------------------------------------------------------------------

#[test]
fn ordinary_text_is_written_as_itself() {
    for s in [
        "Mis documentos/año.txt",
        "canción — résumé 日本語 🎵.txt",
        "a b c",
        "plain",
        "",
    ] {
        let w = write_value(s.as_bytes());
        assert_eq!(w.text, s);
        assert!(w.classes.is_empty(), "{s:?}: {:?}", w.classes);
    }
}

#[test]
fn every_deceptive_character_is_escaped() {
    let set: Vec<u32> = (0x80..=0x9f)
        .chain([0x061c, 0x200e, 0x200f])
        .chain(0x202a..=0x202e)
        .chain(0x2066..=0x2069)
        .chain([0x2028, 0x2029])
        .chain(0x200b..=0x200d)
        .chain([0xfeff])
        .collect();
    for cp in set {
        let c = char::from_u32(cp).unwrap();
        let v = format!("a{c}b");
        let w = write_value(v.as_bytes());
        assert!(!w.text.contains(c), "U+{cp:04X} written literally");
        assert_eq!(w.classes, vec![EscapeClass::DeceptiveUnicode], "U+{cp:04X}");
        assert_eq!(read_value(&w.text).unwrap(), v.as_bytes());
    }
}

#[test]
fn the_escape_table_is_exactly_as_specified() {
    let cases: Vec<(&[u8], &str)> = vec![
        (b"C:\\temp", "C:\\\\temp"),
        (b"a  b", "a\\x20\\x20b"),
        (b" a", "\\x20a"),
        (b"a ", "a\\x20"),
        (b" ", "\\x20"),
        (b"line1\nline2", "line1\\x0Aline2"),
        (b"\x7f", "\\x7F"),
        (b"\xff\xfe", "\\xFF\\xFE"),
        (
            "invoice\u{202e}fdp.exe".as_bytes(),
            "invoice\\xE2\\x80\\xAEfdp.exe",
        ),
    ];
    for (v, text) in cases {
        assert_eq!(write_value(v).text, text, "{v:?}");
    }
}

// ---------------------------------------------------------------------------------------
// A2-T1-9 — the encoding annotation
// ---------------------------------------------------------------------------------------

#[test]
fn the_annotation_lists_exactly_the_classes_that_occurred_in_order() {
    let w = write_value(b" \xff\n\\");
    assert_eq!(
        w.annotation().as_deref(),
        Some("escaped:backslash,not-valid-utf8,control-character,ambiguous-space")
    );
    assert_eq!(write_value(b"plain").annotation(), None);
}

#[cfg(unix)]
#[test]
fn the_annotation_follows_its_field_on_the_same_line_on_any_label() {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;

    let s = Sandbox::new();
    let dir = s.root().join(OsStr::from_bytes(b"ws\nroot"));
    std::fs::create_dir(&dir).unwrap();
    let out = s.run_os_ok(&[OsStr::new("init"), dir.as_os_str()]);

    let line = out
        .lines()
        .find(|l| l.starts_with("observed  canonical="))
        .expect("the canonical line");
    assert!(
        line.contains("ws\\x0Aroot  canonical-encoding=escaped:control-character"),
        "annotation not right after its field:\n{out}"
    );
    assert!(
        umbral::report::label_contract_violations_in_text(&out).is_empty(),
        "the annotation must not count as an observed field:\n{out}"
    );
    let parsed = parse(&out).unwrap();
    let observed = parsed.iter().find(|l| l.label == Label::Observed).unwrap();
    assert!(observed.field("canonical").unwrap().ends_with(b"ws\nroot"));
}

/// The former note said `path-is-not-valid-utf8` for a valid path that merely contained `\`.
#[test]
fn a_backslash_is_not_reported_as_invalid_utf8() {
    let s = Sandbox::new();
    s.write("back\\slash.txt", "x");
    let r = s.root().to_string_lossy().to_string();
    s.run_ok(&["init", &r]);
    s.run_ok(&["observe", &r]);
    s.write("back\\slash.txt", "changed");
    s.run_ok(&["observe", &r]);
    let out = s.run_ok(&["changes", &r]);
    assert!(
        out.contains("path=back\\\\slash.txt  path-encoding=escaped:backslash"),
        "{out}"
    );
    assert!(!out.contains("not-valid-utf8"), "{out}");
    assert!(!out.contains("path-is-not-valid-utf8"), "{out}");
}
