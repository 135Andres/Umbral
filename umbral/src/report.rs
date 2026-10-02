//! The output contract, and the read-only views that satisfy it.
//!
//! # The four labels
//!
//! Every line of user-facing output begins with exactly one of:
//!
//! - `observed` — a fact read from the filesystem during a named run.
//! - `derived` — a result computed from observations.
//! - `ambiguous` — insufficient or conflicting evidence, with a named reason.
//! - `unknown` — not observed, not observable, or not comparable.
//!
//! A line with no label is an implementation defect, and [`unlabelled_lines`] exists so that
//! a test can say so.
//!
//! # No invented values
//!
//! A value that was not obtainable is rendered as `none`, and the absence is stated on its
//! own `unknown` line with a reason. Nothing is filled in with a plausible-looking default.
//!
//! # No anthropomorphic language
//!
//! The tool does not think, know, believe, understand, want, decide, remember, learn,
//! notice, conclude or detect. It observes, classifies and reports. [`BANNED_LEXICON`] is
//! enforced by a test over the rendered output of every command, so the rule does not
//! depend on the discipline of whoever edits this file next.

use std::path::Path;

use crate::content::{hex_short, Stability};
use crate::log::{Observation, ObservationLog, RunMeta};
use crate::reconcile::{reconcile, Mutation, MutationKind, ObservationSet, ObservedPath};
use crate::scan::EntryKind;
use crate::workspace::Workspace;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Label {
    Observed,
    Derived,
    Ambiguous,
    Unknown,
}

impl Label {
    pub fn as_str(self) -> &'static str {
        match self {
            Label::Observed => "observed",
            Label::Derived => "derived",
            Label::Ambiguous => "ambiguous",
            Label::Unknown => "unknown",
        }
    }

    pub fn all() -> [Label; 4] {
        [
            Label::Observed,
            Label::Derived,
            Label::Ambiguous,
            Label::Unknown,
        ]
    }
}

/// One line of output: a label and the facts it applies to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Line {
    pub label: Label,
    pub text: String,
}

impl Line {
    pub fn observed(text: impl Into<String>) -> Self {
        Line {
            label: Label::Observed,
            text: text.into(),
        }
    }
    pub fn derived(text: impl Into<String>) -> Self {
        Line {
            label: Label::Derived,
            text: text.into(),
        }
    }
    pub fn ambiguous(text: impl Into<String>) -> Self {
        Line {
            label: Label::Ambiguous,
            text: text.into(),
        }
    }
    pub fn unknown(text: impl Into<String>) -> Self {
        Line {
            label: Label::Unknown,
            text: text.into(),
        }
    }

    pub fn render(&self) -> String {
        format!("{:<9} {}", self.label.as_str(), self.text)
    }
}

/// Words that must never appear in user-facing text. The tool reports; it does not have a
/// mental life.
pub const BANNED_LEXICON: &[&str] = &[
    "thinks",
    "think",
    "knows",
    "know",
    "believes",
    "believe",
    "understands",
    "understand",
    "intends",
    "intend",
    "wants",
    "want",
    "decides",
    "decide",
    "remembers",
    "remember",
    "learns",
    "learn",
    "realizes",
    "realize",
    "notices",
    "notice",
    "concludes",
    "conclude",
    "detects",
    "detect",
    "assumes",
    "assume",
    "expects",
    "expect",
];

fn tokens(text: &str) -> Vec<String> {
    text.split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|t| !t.is_empty())
        .map(|t| t.to_ascii_lowercase())
        .collect()
}

/// The first banned word found in `text`, if any.
pub fn first_banned_word(text: &str) -> Option<String> {
    tokens(text)
        .into_iter()
        .find(|t| BANNED_LEXICON.contains(&t.as_str()))
}

/// Contract violations in a rendered set of lines: banned vocabulary, or empty text.
/// Returns an empty vector for conforming output — and a non-empty one for output that
/// breaks the contract, which is what lets a test prove it can fail.
pub fn contract_violations(lines: &[Line]) -> Vec<String> {
    let mut out = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if line.text.trim().is_empty() {
            out.push(format!("line {}: empty text", i + 1));
        }
        if let Some(word) = first_banned_word(&line.text) {
            out.push(format!(
                "line {}: banned word in user-facing text: {word}",
                i + 1
            ));
        }
    }
    out
}

/// Render a complete output: the `umbral-output/1` header, then one line per result, each
/// ending with a line feed (`CONTRACT.md` §2).
pub fn render(lines: &[Line]) -> String {
    let mut s = crate::contract::header_line();
    s.push('\n');
    for l in lines {
        s.push_str(&l.render());
        s.push('\n');
    }
    s
}

/// The label a rendered line declares, or `None` if it declares none.
pub fn parse_label(line: &str) -> Option<Label> {
    let first = line.split_whitespace().next()?;
    Label::all().into_iter().find(|l| l.as_str() == first)
}

/// Every rendered line that declares no label.
pub fn unlabelled_lines(rendered: &str) -> Vec<String> {
    rendered
        .lines()
        .filter(|l| !l.trim().is_empty() && parse_label(l).is_none())
        .map(str::to_string)
        .collect()
}

// ---------------------------------------------------------------------------------------
// What may be labelled `observed`
// ---------------------------------------------------------------------------------------
//
// The rule, and the line it draws:
//
//   `observed` — the filesystem itself reported this value for an entry, during this run:
//                the entry's path, kind, size, mtime and physical identity, and the
//                canonical form of the observed root (the filesystem's answer to "where is
//                this really").
//
//   `derived`  — the tool produced it. That covers everything computed, counted,
//                aggregated, compared, identified, assigned or composed: content
//                fingerprints, stability verdicts, run identifiers, the run's own
//                timestamps, workspace identifiers, composed paths, and configuration
//                echoed back.
//
// The reason for drawing it here rather than at "does the tool know it": a reader has to be
// able to tell, from the output alone, whether a value came from their filesystem or was
// produced by the tool. A hash is a function of bytes the tool read; a stability verdict is
// the outcome of comparing two readings; a count is an aggregate. All three are the tool's
// output, not the filesystem's statement, so all three are `derived`.
//
// This was a real defect, found by the reader protocol's Q8 rather than by reading the code:
// `workspace-id` was labelled `observed` while being a fingerprint computed from the
// canonical path. See `experiments/v0.1-reader-protocol/` §5.5, finding F-V01-2.
//
// [`DERIVED_ONLY_FIELDS`] is the enforceable half of the rule, and
// [`derived_field_violations`] is the check. It exists so the distinction cannot decay:
// adding `foo=3` to an `observed` line is caught by a test, not by review.

/// Field names that only ever carry a computed value, so they may never appear on a line
/// labelled `observed`.
pub const DERIVED_ONLY_FIELDS: &[&str] = &[
    // Identifiers and composed paths
    "workspace-id=",
    "state-dir=",
    "state=",
    // Assigned identifiers
    "run=",
    "last-run=",
    "from-run=",
    "to-run=",
    // The tool's own clock readings
    "started=",
    "finished=",
    "created=",
    // Counts and aggregates
    "entries=",
    "files=",
    "dirs=",
    "symlinks=",
    "other=",
    "content-verified=",
    "content-not-verified=",
    "content-verification-not-applicable=",
    "unobservable-paths=",
    "log-runs=",
    "log-observations=",
    // Values computed from the bytes read
    "hash=",
    "stability=",
    // Configuration echoed back
    "root=",
    "tool-version=",
];

/// The complete set of fields a line labelled `observed` may carry. Everything else is the
/// tool's output and belongs on a `derived` line.
///
/// Deliberately exhaustive rather than a denylist: adding an `observed` field is then a
/// deliberate act that requires changing this list, instead of something that slips in.
pub const OBSERVED_FIELDS: &[&str] = &["canonical=", "kind=", "size=", "mtime="];

/// Every `name=` field present in a line of output: the key of each item that has one. Items
/// are separated by two spaces (`CONTRACT.md` §2), so a `=` inside a value is never mistaken
/// for a field.
pub fn fields_in(text: &str) -> Vec<String> {
    text.split("  ")
        .filter_map(|item| item.split_once('='))
        .map(|(key, _)| format!("{}=", key.trim()))
        .collect()
}

/// Whether a key is an encoding annotation: grammar, not a claim (`CONTRACT.md` §5), so it may
/// stand on a line of any label.
fn is_encoding_annotation(field: &str) -> bool {
    field.ends_with("-encoding=")
}

/// Contract violations of the observed/derived distinction. Two halves, both checked:
///
/// 1. an `observed` line must not carry a field the tool computes; and
/// 2. an `observed` line must not carry a field outside [`OBSERVED_FIELDS`].
///
/// Empty for conforming output and non-empty for output that breaks the rule, which is what
/// lets a test prove the check can fail.
pub fn label_contract_violations(lines: &[Line]) -> Vec<String> {
    let mut out = Vec::new();
    for (i, line) in lines.iter().enumerate() {
        if line.label != Label::Observed {
            continue;
        }
        for field in fields_in(&line.text) {
            if is_encoding_annotation(&field) {
                continue;
            }
            if DERIVED_ONLY_FIELDS.contains(&field.as_str()) {
                out.push(format!(
                    "line {}: labelled `observed` but carries the computed field `{field}`: {}",
                    i + 1,
                    line.text
                ));
            } else if !OBSERVED_FIELDS.contains(&field.as_str()) {
                out.push(format!(
                    "line {}: `observed` carries `{field}`, which is not an observed field: {}",
                    i + 1,
                    line.text
                ));
            }
        }
    }
    out
}

/// The same check over already-rendered output.
pub fn label_contract_violations_in_text(rendered: &str) -> Vec<String> {
    let lines: Vec<Line> = rendered
        .lines()
        .filter(|l| !l.trim().is_empty())
        .filter_map(|l| {
            let label = parse_label(l)?;
            let text = l
                .split_once(char::is_whitespace)
                .map(|(_, t)| t.trim().to_string())
                .unwrap_or_default();
            Some(Line { label, text })
        })
        .collect();
    label_contract_violations(&lines)
}

// ---------------------------------------------------------------------------------------
// Timestamps. Rendered in UTC without pulling in a date library.
// ---------------------------------------------------------------------------------------

fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = (z - era * 146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe as i64 + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

/// `1970-01-01T00:00:00.000Z`-style rendering of nanoseconds since the Unix epoch.
pub fn format_unix_ns(ns: i64) -> String {
    let secs = ns.div_euclid(1_000_000_000);
    let nanos = ns.rem_euclid(1_000_000_000);
    let days = secs.div_euclid(86_400);
    let sod = secs.rem_euclid(86_400);
    let (y, m, d) = civil_from_days(days);
    format!(
        "{y:04}-{m:02}-{d:02}T{:02}:{:02}:{:02}.{:03}Z",
        sod / 3600,
        (sod % 3600) / 60,
        sod % 60,
        nanos / 1_000_000
    )
}

// ---------------------------------------------------------------------------------------
// Building observation sets from persisted rows. Used by `changes` and `show`, so that the
// classification rules exist in exactly one place.
// ---------------------------------------------------------------------------------------

fn observed_path(o: &Observation) -> ObservedPath {
    ObservedPath {
        path: o.path.clone(),
        kind: o.kind,
        dev: o.dev,
        ino: o.ino,
        size: o.size,
        mtime: o.mtime,
        valid_hash: o.valid_hash().copied(),
    }
}

// ---------------------------------------------------------------------------------------
// Rendering a path as text.
//
// A path is a byte string, not text, and on Linux it need not be valid UTF-8. Every value is
// written by `contract::write_value` under `umbral-output/1` (`CONTRACT.md` §4): what cannot
// stand in a line — a backslash, bytes that are not UTF-8, control characters, ambiguous
// spaces, deceptive characters — is escaped reversibly, and the field is followed on the same
// line by `<field>-encoding=escaped:<reasons>`, so a reader is never left believing the
// rendered form is what is on disk and never has to guess which value the note is about.
// ---------------------------------------------------------------------------------------

/// A path rendered for text output: its written form under `umbral-output/1`, and the classes
/// of escape that occurred.
#[derive(Debug, Clone)]
pub struct RenderedPath {
    pub text: String,
    pub escaped: bool,
    pub classes: Vec<crate::contract::EscapeClass>,
    written: crate::contract::Written,
}

impl RenderedPath {
    /// `key=value`, followed on the same line by `key-encoding=escaped:…` when anything was
    /// escaped (`CONTRACT.md` §5).
    pub fn field(&self, key: &str) -> String {
        self.written.field(key)
    }
}

/// Render a path as text. Never fails: whatever cannot stand in text is escaped, not refused.
pub fn render_path(p: &Path) -> RenderedPath {
    let written = crate::contract::write_value(&path_key(p));
    RenderedPath {
        text: written.text.clone(),
        escaped: !written.classes.is_empty(),
        classes: written.classes.clone(),
        written,
    }
}

/// A text value that is not a path — an operating-system message, for instance — written
/// under the same rules, as `key=value` with its annotation when needed.
pub fn text_field(key: &str, value: &str) -> String {
    crate::contract::write_value(value.as_bytes()).field(key)
}

fn observation_set(
    log: &dyn ObservationLog,
    run: &RunMeta,
) -> Result<ObservationSet, crate::log::LogError> {
    let obs = log.observations_for_run(run.id)?;
    Ok(ObservationSet::new(
        obs.iter().map(observed_path).collect(),
        run.complete(),
    ))
}

fn kind_field(o: &Observation) -> String {
    o.kind.as_str().to_string()
}

fn opt_u64(v: Option<u64>) -> String {
    match v {
        Some(x) => x.to_string(),
        None => "none".to_string(),
    }
}

fn opt_hash(o: &Observation) -> String {
    match o.valid_hash() {
        Some(h) => hex_short(h),
        None => "none".to_string(),
    }
}

fn opt_stability(o: &Observation) -> String {
    match o.stability {
        Some(s) => s.as_str().to_string(),
        None => "none".to_string(),
    }
}

// ---------------------------------------------------------------------------------------
// observe
// ---------------------------------------------------------------------------------------

pub fn observe_summary(
    ws: &Workspace,
    run: &RunMeta,
    verified: u64,
    not_verified: u64,
) -> Vec<Line> {
    // Everything here describes the run the tool just performed, so all of it is the tool's
    // own output. The one exception is the canonical root: that is the filesystem's answer
    // about where the observed path really is.
    let canonical = render_path(&ws.canonical);
    let root = render_path(&ws.root);
    let out = vec![
        Line::observed(canonical.field("canonical")),
        Line::derived(format!("run={}  {}", run.id, root.field("root"))),
        Line::derived(format!("run={}  entries={}", run.id, run.entries)),
        Line::derived(format!("run={}  content-verified={}", run.id, verified)),
        Line::derived(format!(
            "run={}  content-not-verified={}  reason=unstable-or-unreadable",
            run.id, not_verified
        )),
        Line::derived(format!(
            "run={}  started={}  finished={}",
            run.id,
            format_unix_ns(run.started_at_ns),
            format_unix_ns(run.finished_at_ns)
        )),
        Line::derived(format!("run={}  complete={}", run.id, run.complete())),
    ];
    out
}

// ---------------------------------------------------------------------------------------
// status
// ---------------------------------------------------------------------------------------

pub fn status(ws: &Workspace, log: &dyn ObservationLog) -> Result<Vec<Line>, crate::log::LogError> {
    let mut out = Vec::new();
    // The canonical root is the filesystem's answer; the workspace identifier is a
    // fingerprint the tool computed over it, and the state path is composed from the data
    // home plus that identifier. Only the first is observed.
    let canonical = render_path(&ws.canonical);
    let root = render_path(&ws.root);
    out.push(Line::observed(canonical.field("canonical")));
    out.push(Line::derived(root.field("root")));
    out.push(Line::derived(format!("workspace-id={}", ws.id)));

    let Some(run) = log.latest_run()? else {
        out.push(Line::unknown("observations=none  reason=no-run-recorded"));
        return Ok(out);
    };

    out.push(Line::derived(format!(
        "last-run={}  started={}  entries={}",
        run.id,
        format_unix_ns(run.started_at_ns),
        run.entries
    )));
    out.push(Line::derived(format!(
        "last-run={}  complete={}",
        run.id,
        run.complete()
    )));

    let obs = log.observations_for_run(run.id)?;
    let (mut files, mut dirs, mut symlinks, mut other) = (0u64, 0u64, 0u64, 0u64);
    let (mut verified, mut not_verified, mut errored) = (0u64, 0u64, 0u64);

    for o in &obs {
        match o.kind {
            EntryKind::File => files += 1,
            EntryKind::Dir => dirs += 1,
            EntryKind::Symlink => symlinks += 1,
            EntryKind::Other => other += 1,
        }
        if o.error.is_some() {
            errored += 1;
        }
        if o.is_content_verified() {
            verified += 1;
        } else if o.kind == EntryKind::File {
            not_verified += 1;
        }
    }

    // Every value below is an aggregate computed from the stored observations. The count of
    // paths that could not be observed is a computed count too, even though what it counts
    // are things the tool does not know: the number is the tool's arithmetic, and the
    // unknowns themselves are reported per path by `show`.
    out.push(Line::derived(format!(
        "entries={}  files={}  dirs={}  symlinks={}  other={}",
        obs.len(),
        files,
        dirs,
        symlinks,
        other
    )));
    out.push(Line::derived(format!("content-verified={verified}")));
    out.push(Line::derived(format!(
        "content-not-verified={not_verified}  reason=unstable-or-unreadable"
    )));
    out.push(Line::derived(format!(
        "content-verification-not-applicable={}  reason=not-a-regular-file",
        dirs + symlinks + other
    )));
    out.push(Line::derived(format!(
        "unobservable-paths={errored}  reason=not-observed-at-observation-time"
    )));

    let (runs, observations) = log.counts()?;
    out.push(Line::derived(format!(
        "log-runs={runs}  log-observations={observations}"
    )));
    Ok(out)
}

// ---------------------------------------------------------------------------------------
// changes
// ---------------------------------------------------------------------------------------

pub fn changes(
    ws: &Workspace,
    log: &dyn ObservationLog,
) -> Result<Vec<Line>, crate::log::LogError> {
    let _ = ws;
    let runs = log.runs()?;
    if runs.is_empty() {
        return Ok(vec![Line::unknown(
            "comparison-not-possible  reason=no-run-recorded",
        )]);
    }
    if runs.len() == 1 {
        return Ok(vec![Line::unknown(format!(
            "comparison-not-possible  reason=only-one-run  run={}",
            runs[0].id
        ))]);
    }

    let prev = &runs[runs.len() - 2];
    let cur = &runs[runs.len() - 1];
    let ps = observation_set(log, prev)?;
    let cs = observation_set(log, cur)?;
    let r = reconcile(&ps, &cs);

    let mut out = vec![Line::derived(format!(
        "compared  from-run={}  to-run={}  complete={}",
        prev.id, cur.id, r.complete
    ))];

    // Every count is printed, including zeros. A count that is merely absent would make
    // "there were no deletions" readable only from the absence of a line, which is exactly
    // the kind of inference the output contract exists to prevent.
    for kind in [
        MutationKind::Unchanged,
        MutationKind::Modified,
        MutationKind::Created,
        MutationKind::Deleted,
        MutationKind::Unobserved,
        MutationKind::RenamedOrMoved,
        MutationKind::Recreated,
        MutationKind::Ambiguous,
    ] {
        out.push(Line::derived(format!(
            "count  {}={}",
            kind.as_str(),
            r.count(kind)
        )));
    }

    for m in r
        .mutations
        .iter()
        .filter(|m| m.kind != MutationKind::Unchanged)
    {
        out.extend(mutation_line(m));
    }

    Ok(out)
}

fn mutation_line(m: &Mutation) -> Vec<Line> {
    let path = render_path(&m.path);
    let old = m.old_path.as_ref().map(|p| render_path(p));
    let mut parts = vec![path.field("path")];
    // `created` is relative to the reference run: an entry absent from an incomplete
    // reference may have existed unseen. Stated explicitly, true or false (D-V01-9).
    if m.kind == MutationKind::Created {
        parts.push(format!(
            "reference-complete={}",
            m.evidence.reference_complete
        ));
    }
    if let Some(o) = &old {
        parts.push(o.field("old-path"));
    }
    if let Some(b) = m.evidence.content_changed {
        parts.push(format!("content-changed={b}"));
    }
    if let Some(s) = m.evidence.object_survives {
        parts.push(format!("object-survives={s}"));
    }
    if let Some(r) = m.evidence.reason {
        parts.push(format!("reason={}", r.as_str()));
    }
    parts.push(format!("scan-complete={}", m.evidence.complete_scan));
    let text = parts.join("  ");
    let line = match m.kind {
        MutationKind::Ambiguous => Line::ambiguous(format!("{}  {text}", m.kind.as_str())),
        _ => Line::derived(format!("{}  {text}", m.kind.as_str())),
    };
    vec![line]
}

// ---------------------------------------------------------------------------------------
// show
// ---------------------------------------------------------------------------------------

pub fn show(
    ws: &Workspace,
    log: &dyn ObservationLog,
    rel: &Path,
) -> Result<Vec<Line>, crate::log::LogError> {
    let _ = ws;
    let obs = log.observations_for_path(rel)?;
    let rel_rendered = render_path(rel);
    if obs.is_empty() {
        return Ok(vec![Line::unknown(format!(
            "{}  reason=not-observed-in-any-run",
            rel_rendered.field("path")
        ))]);
    }

    let runs = log.runs()?;
    let mut out = Vec::new();

    for o in &obs {
        // Split by what produced each value: kind, size and mtime are the filesystem's own
        // statements about the entry; the run identifier is assigned by the tool, the hash
        // is a function of bytes the tool read, and the stability verdict is the outcome of
        // comparing two readings. All are stored observations, but only the first group is
        // `observed`. Ordering pairs them: the derived line introduces the run whose observed
        // facts follow it.
        out.push(Line::derived(format!(
            "run={}  hash={}  stability={}",
            o.run_id,
            opt_hash(o),
            opt_stability(o)
        )));
        out.push(Line::observed(format!(
            "kind={}  size={}  mtime={}",
            kind_field(o),
            opt_u64(o.size),
            match o.mtime {
                Some((s, n)) => format_unix_ns(s * 1_000_000_000 + n as i64),
                None => "none".to_string(),
            }
        )));

        let mut absent = Vec::new();
        if o.size.is_none() {
            absent.push("size");
        }
        if o.mtime.is_none() {
            absent.push("mtime");
        }
        if o.dev.is_none() || o.ino.is_none() {
            absent.push("physical-identity");
        }
        if !absent.is_empty() {
            out.push(Line::unknown(format!(
                "run={}  fields={}  reason=not-obtainable-at-observation-time",
                o.run_id,
                absent.join(",")
            )));
        }
        if let Some(e) = &o.error {
            out.push(Line::unknown(format!(
                "run={}  {}",
                o.run_id,
                text_field("observation-error", e)
            )));
        }
        // Why the content was not obtained, as recorded when it was not (D-V01-10).
        if let Some(e) = &o.content_error {
            out.push(Line::unknown(format!(
                "run={}  {}",
                o.run_id,
                text_field("content-error", e)
            )));
        }
        if o.stability == Some(Stability::Unstable) && !o.deltas.is_empty() {
            out.push(Line::ambiguous(format!(
                "run={}  reason=UnstableObservation  deltas={}",
                o.run_id,
                o.deltas
                    .iter()
                    .map(|d| d.as_str())
                    .collect::<Vec<_>>()
                    .join(",")
            )));
        }
    }

    // Transitions between consecutive observations of this path, classified by the same
    // function that `changes` uses — there is no second implementation of the rules.
    for pair in obs.windows(2) {
        let (a, b) = (&pair[0], &pair[1]);
        let complete = runs
            .iter()
            .find(|r| r.id == b.run_id)
            .map(RunMeta::complete)
            .unwrap_or(false);
        // The previous side's completeness is its own run's, not assumed (D-V01-9).
        let complete_a = runs
            .iter()
            .find(|r| r.id == a.run_id)
            .map(RunMeta::complete)
            .unwrap_or(false);
        let set_a = ObservationSet::new(vec![observed_path(a)], complete_a);
        let set_b = ObservationSet::new(vec![observed_path(b)], complete);
        let rec = reconcile(&set_a, &set_b);
        for m in &rec.mutations {
            for line in mutation_line(m) {
                out.push(Line {
                    label: line.label,
                    text: format!("run={} -> {}  {}", a.run_id, b.run_id, line.text),
                });
            }
        }
    }

    Ok(out)
}

// ---------------------------------------------------------------------------------------
// check
// ---------------------------------------------------------------------------------------

/// Verify the log, and print only verifications that can fail.
///
/// What is checked: every observation belongs to a run that exists; every stored value is one
/// this build can interpret, read raw rather than through the normalising readers; and the
/// stored tables are the log and nothing else, so no derived state is persisted. Each of
/// these is shown failing by a test that corrupts a log on purpose. Three earlier lines were
/// removed because they could not fail (D-V01-11): a per-run count compared with itself, the
/// same table read twice through the same parser, and duplicates the primary key forbids.
pub fn check(ws: &Workspace, log: &dyn ObservationLog) -> Result<Vec<Line>, crate::log::LogError> {
    let mut out = Vec::new();
    // The state path is composed from the data home and the workspace identifier, and the
    // counts are aggregates over the log. Both are the tool's own output.
    let state = render_path(&ws.log_path());
    out.push(Line::derived(state.field("state")));

    let (runs_n, obs_n) = log.counts()?;
    out.push(Line::derived(format!(
        "log-runs={runs_n}  log-observations={obs_n}"
    )));

    let runs = log.runs()?;
    let all = log.all_observations()?;

    // 1. Referential integrity: every observation belongs to a run that exists.
    let known: std::collections::BTreeSet<i64> = runs.iter().map(|r| r.id).collect();
    let orphans = all.iter().filter(|o| !known.contains(&o.run_id)).count();

    // 2. Every stored value is interpretable, read raw.
    let invalid = log.invalid_values()?;

    // 3. No derived state is persisted: the stored tables are the log, and nothing else.
    let mut tables = log.tables()?;
    tables.sort();
    let only_log = tables
        == vec![
            "observation".to_string(),
            "run".to_string(),
            "schema_meta".to_string(),
        ];

    out.push(Line::derived(format!(
        "referential-integrity={}  orphan-observations={orphans}",
        orphans == 0
    )));
    out.push(Line::derived(format!(
        "row-values-valid={}  invalid-values={}",
        invalid.is_empty(),
        invalid.len()
    )));
    for v in &invalid {
        let mut text = format!("invalid-value  run={}", v.run_id);
        let rendered = v.path.as_ref().map(|p| render_path(p));
        if let Some(p) = &rendered {
            text.push_str(&format!("  {}", p.field("path")));
        }
        text.push_str(&format!("  field={}  reason={}", v.field, v.reason));
        out.push(Line::derived(text));
    }
    out.push(Line::derived(format!(
        "stored-tables={}  derived-state-persisted={}",
        tables.join(","),
        !only_log
    )));

    let ok = orphans == 0 && invalid.is_empty() && only_log;
    out.push(Line::derived(format!("consistent={ok}")));
    Ok(out)
}

fn path_key(p: &Path) -> Vec<u8> {
    #[cfg(unix)]
    {
        use std::os::unix::ffi::OsStrExt;
        p.as_os_str().as_bytes().to_vec()
    }
    #[cfg(not(unix))]
    {
        p.to_string_lossy().as_bytes().to_vec()
    }
}
