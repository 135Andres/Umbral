//! The output contract `umbral-output/1` — how stdout is written and read back.
//!
//! The specification is `CONTRACT.md` (accepted by the owner, `UD-030`); the grammar family,
//! error semantics and versioning mechanism are `UD-022`. This module is the single place that
//! turns a value into text and text back into a value, so the two cannot drift apart:
//!
//! - [`write_value`] is the only escaping function. Its output is a function of the value
//!   alone (one value, one rendering), and it never produces a separator, a line boundary or a
//!   raw control byte.
//! - [`read_value`] decodes and then re-encodes, and refuses any text that is not exactly what
//!   [`write_value`] would have produced. Strictness is not a separate set of rules that could
//!   miss a case: it is the round-trip itself.
//! - [`parse`] reads a whole output: the header, the label column, the items. It never repairs,
//!   skips or resynchronises; the only thing it tolerates is a field or token it does not know,
//!   which it keeps as opaque data.
//!
//! This is a library reader, not a command (`UD-030`). Nothing here gives the tool a new
//! surface; it exists so that the contract can be tested, and read by others.

use crate::report::Label;

/// The edition this build writes and reads. Its own number: not the log schema, not the crate
/// version, not a build identifier (`UD-022`).
pub const EDITION: &str = "umbral-output/1";

/// The key of the header's only field.
const HEADER_KEY: &str = "contract";

/// The first line of every output.
pub fn header_line() -> String {
    format!("{:<9} {HEADER_KEY}={EDITION}", Label::Derived.as_str())
}

/// Why a byte was escaped. The order of [`EscapeClass::ALL`] is the order reasons are listed in
/// the encoding annotation (`CONTRACT.md` §4–§5).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EscapeClass {
    Backslash,
    NotValidUtf8,
    ControlCharacter,
    AmbiguousSpace,
    DeceptiveUnicode,
}

impl EscapeClass {
    pub const ALL: [EscapeClass; 5] = [
        EscapeClass::Backslash,
        EscapeClass::NotValidUtf8,
        EscapeClass::ControlCharacter,
        EscapeClass::AmbiguousSpace,
        EscapeClass::DeceptiveUnicode,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            EscapeClass::Backslash => "backslash",
            EscapeClass::NotValidUtf8 => "not-valid-utf8",
            EscapeClass::ControlCharacter => "control-character",
            EscapeClass::AmbiguousSpace => "ambiguous-space",
            EscapeClass::DeceptiveUnicode => "deceptive-unicode",
        }
    }
}

/// A value as written, with the classes of escape that occurred in it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Written {
    pub text: String,
    /// Each class at most once, in [`EscapeClass::ALL`] order. Empty when nothing was escaped.
    pub classes: Vec<EscapeClass>,
}

impl Written {
    /// The value of the encoding annotation (`CONTRACT.md` §5), or `None` when the value was
    /// written as itself.
    pub fn annotation(&self) -> Option<String> {
        if self.classes.is_empty() {
            return None;
        }
        let reasons: Vec<&str> = self.classes.iter().map(|c| c.as_str()).collect();
        Some(format!("escaped:{}", reasons.join(",")))
    }

    /// `key=value`, followed on the same line by `key-encoding=…` when anything was escaped.
    pub fn field(&self, key: &str) -> String {
        match self.annotation() {
            Some(a) => format!("{key}={}  {key}-encoding={a}", self.text),
            None => format!("{key}={}", self.text),
        }
    }
}

/// Characters that could make a written value look like something else on a terminal.
fn is_deceptive(c: char) -> bool {
    matches!(c as u32,
        0x0080..=0x009F            // C1 controls
        | 0x061C | 0x200E | 0x200F // bidirectional marks
        | 0x202A..=0x202E          // bidirectional embeddings and overrides
        | 0x2066..=0x2069          // bidirectional isolates
        | 0x2028 | 0x2029          // line and paragraph separators
        | 0x200B..=0x200D | 0xFEFF // zero-width characters
    )
}

fn push_hex(out: &mut String, b: u8) {
    out.push_str(&format!("\\x{b:02X}"));
}

/// Write one value (`CONTRACT.md` §4).
pub fn write_value(bytes: &[u8]) -> Written {
    let mut text = String::with_capacity(bytes.len());
    let mut seen = [false; EscapeClass::ALL.len()];
    let mut mark = |c: EscapeClass| seen[c as usize] = true;

    // Byte offsets are tracked so the space rule can look at the neighbours in the value.
    let mut offset = 0usize;
    for chunk in bytes.utf8_chunks() {
        for c in chunk.valid().chars() {
            let at = offset;
            offset += c.len_utf8();
            match c {
                '\\' => {
                    text.push_str("\\\\");
                    mark(EscapeClass::Backslash);
                }
                ' ' => {
                    let first = at == 0;
                    let last = at + 1 == bytes.len();
                    let after_space = at > 0 && bytes[at - 1] == b' ';
                    let before_space = bytes.get(at + 1) == Some(&b' ');
                    if first || last || after_space || before_space {
                        push_hex(&mut text, b' ');
                        mark(EscapeClass::AmbiguousSpace);
                    } else {
                        text.push(' ');
                    }
                }
                c if (c as u32) < 0x20 || c as u32 == 0x7F => {
                    push_hex(&mut text, c as u8);
                    mark(EscapeClass::ControlCharacter);
                }
                c if is_deceptive(c) => {
                    let mut buf = [0u8; 4];
                    for b in c.encode_utf8(&mut buf).bytes() {
                        push_hex(&mut text, b);
                    }
                    mark(EscapeClass::DeceptiveUnicode);
                }
                c => text.push(c),
            }
        }
        for &b in chunk.invalid() {
            push_hex(&mut text, b);
            mark(EscapeClass::NotValidUtf8);
            offset += 1;
        }
    }

    let classes = EscapeClass::ALL
        .into_iter()
        .filter(|c| seen[*c as usize])
        .collect();
    Written { text, classes }
}

/// Why a written value could not be read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValueError {
    /// A raw control byte, at this byte offset of the text.
    RawControl(usize),
    /// A `\` not followed by `\` or `x` and two hexadecimal digits, at this byte offset.
    MalformedEscape(usize),
    /// Well-formed, but not what [`write_value`] produces for the value it denotes.
    NonCanonical,
}

/// Read one written value back to its bytes, refusing anything non-canonical.
pub fn read_value(text: &str) -> Result<Vec<u8>, ValueError> {
    let raw = text.as_bytes();
    if let Some(i) = raw.iter().position(|&b| b < 0x20 || b == 0x7F) {
        return Err(ValueError::RawControl(i));
    }
    let mut out = Vec::with_capacity(raw.len());
    let mut i = 0;
    while i < raw.len() {
        if raw[i] != b'\\' {
            out.push(raw[i]);
            i += 1;
            continue;
        }
        match raw.get(i + 1) {
            Some(b'\\') => {
                out.push(b'\\');
                i += 2;
            }
            Some(b'x') => {
                let hex = raw
                    .get(i + 2..i + 4)
                    .and_then(|h| std::str::from_utf8(h).ok())
                    .filter(|h| h.bytes().all(|b| b.is_ascii_hexdigit()))
                    .and_then(|h| u8::from_str_radix(h, 16).ok());
                match hex {
                    Some(b) => {
                        out.push(b);
                        i += 4;
                    }
                    None => return Err(ValueError::MalformedEscape(i)),
                }
            }
            _ => return Err(ValueError::MalformedEscape(i)),
        }
    }
    if write_value(&out).text != text {
        return Err(ValueError::NonCanonical);
    }
    Ok(out)
}

/// A reference to one observation: `<run>:<path>` (`CONTRACT.md` §6a, `UD-033`). The run is
/// written in decimal without leading zeros; the path is its bytes. The whole value is then
/// written like any other.
pub fn write_reference(run: u64, path: &[u8]) -> Written {
    let mut value = format!("{run}:").into_bytes();
    value.extend_from_slice(path);
    write_value(&value)
}

/// Why a decoded value is not an observation reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReferenceError {
    /// No `:` at all.
    MissingSeparator,
    /// Nothing before the first `:`.
    EmptyRun,
    /// The run is not made of decimal digits only.
    NotDecimal,
    /// The run has a leading zero: not the one way it is written.
    LeadingZero,
    /// The run does not fit the log's signed 64-bit identifier.
    OutOfRange,
}

/// Read a decoded value as an observation reference. It splits at the **first** `:` — a run
/// never contains one, so a path may.
pub fn read_reference(value: &[u8]) -> Result<(u64, Vec<u8>), ReferenceError> {
    let sep = value
        .iter()
        .position(|&b| b == b':')
        .ok_or(ReferenceError::MissingSeparator)?;
    let (run, path) = (&value[..sep], &value[sep + 1..]);
    if run.is_empty() {
        return Err(ReferenceError::EmptyRun);
    }
    if !run.iter().all(u8::is_ascii_digit) {
        return Err(ReferenceError::NotDecimal);
    }
    if run.len() > 1 && run[0] == b'0' {
        return Err(ReferenceError::LeadingZero);
    }
    let run: u64 = std::str::from_utf8(run)
        .ok()
        .and_then(|r| r.parse().ok())
        .filter(|&r| r <= i64::MAX as u64)
        .ok_or(ReferenceError::OutOfRange)?;
    Ok((run, path.to_vec()))
}

/// One item of a line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item {
    /// A bare word: `count`, `modified`, … or one this edition does not define.
    Token(String),
    /// `key=value`, with the value read back to its bytes.
    Field { key: String, value: Vec<u8> },
}

/// One result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedLine {
    pub label: Label,
    pub items: Vec<Item>,
}

impl ParsedLine {
    /// The value of the first field with this key.
    pub fn field(&self, key: &str) -> Option<&[u8]> {
        self.items.iter().find_map(|i| match i {
            Item::Field { key: k, value } if k == key => Some(value.as_slice()),
            _ => None,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseErrorKind {
    MissingHeader,
    UnknownEdition(String),
    UnknownLabel,
    EmptyItem,
    EmptyKey,
    MalformedEscape,
    NonCanonical,
    RawControl,
    /// The output does not end with a line feed.
    Truncated,
}

/// A refusal to read, with its position: `line` counts from 1 (the header is line 1) and
/// `column` counts characters from 1.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub line: usize,
    pub column: usize,
    pub kind: ParseErrorKind,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "line {}, column {}: {:?}",
            self.line, self.column, self.kind
        )
    }
}

impl std::error::Error for ParseError {}

const LABEL_COLUMN: usize = 10;

fn column_of(line: &str, byte: usize) -> usize {
    line[..byte].chars().count() + 1
}

fn parse_line(line: &str, n: usize) -> Result<ParsedLine, ParseError> {
    let err = |byte: usize, kind| ParseError {
        line: n,
        column: column_of(line, byte.min(line.len())),
        kind,
    };
    let label = Label::all()
        .into_iter()
        .find(|l| line.starts_with(&format!("{:<9} ", l.as_str())))
        .ok_or_else(|| err(0, ParseErrorKind::UnknownLabel))?;

    let value_error = |e: ValueError, at: usize| match e {
        ValueError::RawControl(i) => err(at + i, ParseErrorKind::RawControl),
        ValueError::MalformedEscape(i) => err(at + i, ParseErrorKind::MalformedEscape),
        ValueError::NonCanonical => err(at, ParseErrorKind::NonCanonical),
    };

    let mut items = Vec::new();
    let mut start = LABEL_COLUMN;
    let body = &line[LABEL_COLUMN..];
    for piece in body.split("  ") {
        if piece.is_empty() || piece.starts_with(' ') {
            return Err(err(start, ParseErrorKind::EmptyItem));
        }
        let item = match piece.split_once('=') {
            Some((key, text)) => {
                if key.is_empty() {
                    return Err(err(start, ParseErrorKind::EmptyKey));
                }
                let value_at = start + key.len() + 1;
                let value = read_value(text).map_err(|e| value_error(e, value_at))?;
                Item::Field {
                    key: key.to_string(),
                    value,
                }
            }
            None => {
                read_value(piece).map_err(|e| value_error(e, start))?;
                Item::Token(piece.to_string())
            }
        };
        items.push(item);
        start += piece.len() + 2;
    }
    Ok(ParsedLine { label, items })
}

/// Read a whole output (`CONTRACT.md` §6). Returns the results after the header.
pub fn parse(output: &str) -> Result<Vec<ParsedLine>, ParseError> {
    let Some(body) = output.strip_suffix('\n') else {
        let kind = if output.is_empty() {
            ParseErrorKind::MissingHeader
        } else {
            ParseErrorKind::Truncated
        };
        return Err(ParseError {
            line: output.split('\n').count(),
            column: 1,
            kind,
        });
    };
    let mut lines = body.split('\n');

    let first = lines.next().unwrap_or("");
    let header = parse_line(first, 1).map_err(|e| ParseError {
        kind: ParseErrorKind::MissingHeader,
        ..e
    })?;
    match header.items.as_slice() {
        [Item::Field { key, value }] if header.label == Label::Derived && key == HEADER_KEY => {
            if value != EDITION.as_bytes() {
                return Err(ParseError {
                    line: 1,
                    column: 1,
                    kind: ParseErrorKind::UnknownEdition(String::from_utf8_lossy(value).into()),
                });
            }
        }
        _ => {
            return Err(ParseError {
                line: 1,
                column: 1,
                kind: ParseErrorKind::MissingHeader,
            })
        }
    }

    lines
        .enumerate()
        .map(|(i, l)| parse_line(l, i + 2))
        .collect()
}
