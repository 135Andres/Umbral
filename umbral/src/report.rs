//! The output contract, and the read-only views that satisfy it.
//!
//! # The four labels
//!
//! Every line of user-facing output begins with exactly one of:
//!
//! - `observed` — a fact read from the filesystem during a named run.
//! - `derived` — a result computed from observations.
//! - `ambiguous` — a classification the evidence leaves open between more than one outcome,
//!   with a named reason. None of the outcomes is chosen.
//! - `unknown` — a value that is not determinable from the available evidence: not
//!   observed, not obtainable, or not comparable. An error is a reason for `unknown`, not a
//!   label of its own (`UD-031`).
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

use crate::acquisition::{
    content_diagnostic, content_source, content_state, metadata_state, AcquisitionState,
    FAILED_DIAGNOSTICS,
};
use crate::content::{hex_short, Stability};
use crate::log::{Observation, ObservationLog, RunId, RunMeta};
use crate::reconcile::{
    reconcile, Mutation, MutationKind, ObservationSet, ObservedPath, Side, SideBasis,
};
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
    "reference-run=",
    "compared-run=",
    // What a verdict rests on (`UD-034`)
    "reference=",
    "compared=",
    "counterpart=",
    "reference-fields=",
    "compared-fields=",
    "counterpart-fields=",
    "reference-absent=",
    "compared-absent=",
    "reference-complete=",
    "compared-complete=",
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
    "kind-unknown=",
    "metadata-fresh=",
    "metadata-failed=",
    "content-fresh=",
    "content-reused=",
    "content-failed=",
    "content-not-attempted=",
    "content-not-recorded=",
    "unstable-observation=",
    "not-found=",
    "permission-denied=",
    "not-a-regular-file=",
    "read-error=",
    "traversal-complete=",
    "traversal-not-descended=",
    "traversal-metadata-failed=",
    "traversal-not-recorded=",
    "root-not-descended=",
    "scope=",
    "traversal=",
    "log-runs=",
    "log-observations=",
    // Values computed from the bytes read
    "hash=",
    "stability=",
    // The tool's account of how each component was obtained (`UD-031`)
    "metadata=",
    "content=",
    "content-source=",
    // The content work of one run (A2-V1)
    "content-read-entries=",
    "content-read-bytes=",
    // Configuration echoed back
    "root=",
    "tool-version=",
];

/// The complete set of fields a line labelled `observed` may carry. Everything else is the
/// tool's output and belongs on a `derived` line.
///
/// Deliberately exhaustive rather than a denylist: adding an `observed` field is then a
/// deliberate act that requires changing this list, instead of something that slips in.
pub const OBSERVED_FIELDS: &[&str] = &["canonical=", "kind=", "size=", "mtime=", "ctime="];

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

/// Fields that say which observation a line is about and state nothing about the filesystem,
/// so they may stand on a line of any label (`UD-033`, `CONTRACT.md` §6a).
pub const IDENTIFICATION_FIELDS: &[&str] = &["observation="];

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
            if is_encoding_annotation(&field) || IDENTIFICATION_FIELDS.contains(&field.as_str()) {
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
/// Nanoseconds since the Unix epoch, negative before it (D-V01-15). `None` when the time does
/// not fit the log's signed 64-bit field — it is then refused, never replaced by a stand-in.
pub fn unix_ns(t: std::time::SystemTime) -> Option<i64> {
    match t.duration_since(std::time::UNIX_EPOCH) {
        Ok(d) => i64::try_from(d.as_nanos()).ok(),
        Err(e) => i64::try_from(e.duration().as_nanos()).ok().map(|n| -n),
    }
}

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

fn opt_time(t: Option<(i64, u32)>) -> String {
    match t {
        Some((s, n)) => format_unix_ns(s * 1_000_000_000 + n as i64),
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
    obs: &[Observation],
    counters: &crate::observe::Counters,
) -> Vec<Line> {
    // Everything here describes the run the tool just performed, so all of it is the tool's
    // own output. The one exception is the canonical root: that is the filesystem's answer
    // about where the observed path really is.
    let canonical = render_path(&ws.canonical);
    let root = render_path(&ws.root);
    let mut out = vec![
        Line::observed(canonical.field("canonical")),
        Line::derived(format!("run={}  {}", run.id, root.field("root"))),
    ];
    // Counted from what the log stored for this run, by the same function as `status`, so the
    // two cannot disagree (A2-T2a-5).
    out.extend(state_counts(run.id, obs));
    out.extend(run_facts(run, obs));
    // What this run actually read: the measure of O(changes), counted, never timed (A2-V1).
    out.push(Line::derived(format!(
        "run={}  content-read-entries={}  content-read-bytes={}",
        run.id, counters.read_entries, counters.read_bytes
    )));
    out.extend([
        Line::derived(format!(
            "run={}  started={}  finished={}",
            run.id,
            format_unix_ns(run.started_at_ns),
            format_unix_ns(run.finished_at_ns)
        )),
        Line::derived(format!("run={}  complete={}", run.id, run.complete())),
    ]);
    out
}

/// The traversal facts of one run and the rules it applied (`UD-025`, `UD-027`, `UD-037`): which
/// part of the scope was not observed, by class, and the build and scope that produced it.
pub fn run_facts(run: &RunMeta, obs: &[Observation]) -> Vec<Line> {
    let count = |class: &str| {
        obs.iter()
            .filter(|o| o.traversal.as_deref() == Some(class))
            .count()
    };
    let mut out = vec![Line::derived(format!(
        "run={}  traversal-complete={}  traversal-not-descended={}  traversal-metadata-failed={}  traversal-not-recorded={}  root-not-descended={}",
        run.id,
        run.complete(),
        count("not-descended"),
        count("metadata-failed"),
        count("not-recorded"),
        run.root_error.is_some()
    ))];
    if let Some(e) = &run.root_error {
        out.push(Line::unknown(format!(
            "run={}  {}",
            run.id,
            text_field("root-error", e)
        )));
    }
    match (&run.tool_version, &run.scope) {
        (Some(v), Some(s)) => out.push(Line::derived(format!(
            "run={}  {}  scope={s}",
            run.id,
            text_field("tool-version", v)
        ))),
        _ => out.push(Line::unknown(format!(
            "run={}  fields=tool-version,scope  reason=not-recorded",
            run.id
        ))),
    }
    out
}

/// The counts of one run by kind and by acquisition state, each on a line naming the run
/// (`UD-033`). Every count is printed, zeros included: an absent count would make "none"
/// readable only from the absence of a line.
pub fn state_counts(run: crate::log::RunId, obs: &[Observation]) -> Vec<Line> {
    let (mut files, mut dirs, mut symlinks, mut other, mut kind_unknown) = (0, 0, 0, 0, 0);
    let mut metadata = [0u64; 5];
    let mut content = [0u64; 5];
    let mut diagnostics = [0u64; FAILED_DIAGNOSTICS.len()];
    let index = |s: AcquisitionState| AcquisitionState::ALL.iter().position(|x| *x == s).unwrap();

    for o in obs {
        match o.kind {
            // Stored as `other`, but no kind was observed (D-V01-16).
            _ if o.metadata_failed() => kind_unknown += 1,
            EntryKind::File => files += 1,
            EntryKind::Dir => dirs += 1,
            EntryKind::Symlink => symlinks += 1,
            EntryKind::Other => other += 1,
        }
        metadata[index(metadata_state(o))] += 1;
        if let Some(c) = content_state(o) {
            content[index(c)] += 1;
        }
        if let Some(d) = content_diagnostic(o) {
            diagnostics[FAILED_DIAGNOSTICS.iter().position(|x| *x == d).unwrap()] += 1;
        }
    }

    let by_state = |component: &str, counts: &[u64; 5], states: &[AcquisitionState]| {
        let mut text = format!("run={run}");
        for s in states {
            text.push_str(&format!(
                "  {component}-{}={}",
                s.as_str(),
                counts[index(*s)]
            ));
        }
        Line::derived(text)
    };
    // The failed content readings by diagnostic; they sum to `content-failed`, which is stated
    // once, on the line above.
    let mut failed = format!("run={run}  content-failed-diagnostics");
    for (d, n) in FAILED_DIAGNOSTICS.iter().zip(diagnostics) {
        failed.push_str(&format!("  {d}={n}"));
    }
    vec![
        Line::derived(format!(
            "run={run}  entries={}  files={files}  dirs={dirs}  symlinks={symlinks}  other={other}  kind-unknown={kind_unknown}",
            obs.len()
        )),
        // Metadata is obtained fresh or not at all in v0.2: the scan reads it for every entry.
        by_state(
            "metadata",
            &metadata,
            &[AcquisitionState::Fresh, AcquisitionState::Failed],
        ),
        by_state("content", &content, &AcquisitionState::ALL),
        Line::derived(failed),
    ]
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
        "last-run={}  started={}",
        run.id,
        format_unix_ns(run.started_at_ns)
    )));
    out.push(Line::derived(format!(
        "last-run={}  complete={}",
        run.id,
        run.complete()
    )));

    let obs = log.observations_for_run(run.id)?;
    out.extend(state_counts(run.id, &obs));

    out.extend(run_facts(&run, &obs));

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
        "compared  reference-run={}  compared-run={}  reference-complete={}  compared-complete={}",
        prev.id,
        cur.id,
        prev.complete(),
        cur.complete()
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
        out.extend(mutation_line(m, prev.id, cur.id));
    }

    Ok(out)
}

/// One verdict, with what it rests on (`UD-033`, `UD-034`): the subject (`path=`), the
/// observation it relates on each side — or that side's absence — with the fields the rules
/// consulted, every other entry it rests on, and both sides' completeness.
fn mutation_line(m: &Mutation, reference_run: RunId, compared_run: RunId) -> Vec<Line> {
    let path = render_path(&m.path);
    let basis = &m.evidence.basis;
    let mut parts = vec![path.field("path")];
    for (key, side, run) in [
        ("reference", &basis.reference, reference_run),
        ("compared", &basis.compared, compared_run),
    ] {
        match side {
            SideBasis::Related { path, fields } => {
                parts.push(reference_field(key, run, path));
                parts.push(format!("{key}-fields={}", field_list(fields)));
            }
            SideBasis::Absent => parts.push(format!("{key}-absent={run}")),
            SideBasis::NotRelated => {}
        }
    }
    for (side, p) in &basis.counterparts {
        let run = match side {
            Side::Reference => reference_run,
            Side::Compared => compared_run,
        };
        parts.push(reference_field("counterpart", run, p));
    }
    if !basis.counterparts.is_empty() {
        parts.push(format!(
            "counterpart-fields={}",
            field_list(&basis.counterpart_fields)
        ));
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
    // A verdict is relative to both runs it compares, and to how complete each was (A2-V7).
    parts.push(format!(
        "reference-complete={}  compared-complete={}",
        m.evidence.reference_complete, m.evidence.complete_scan
    ));
    let text = parts.join("  ");
    let line = match m.kind {
        MutationKind::Ambiguous => Line::ambiguous(format!("{}  {text}", m.kind.as_str())),
        _ => Line::derived(format!("{}  {text}", m.kind.as_str())),
    };
    vec![line]
}

/// `<key>=<run>:<path>`, written by the contract (`CONTRACT.md` §6a).
fn reference_field(key: &str, run: RunId, path: &Path) -> String {
    crate::contract::write_reference(run as u64, &path_key(path)).field(key)
}

/// A list of consulted field names; `none` when the rules consulted none.
fn field_list(fields: &[&str]) -> String {
    if fields.is_empty() {
        "none".to_string()
    } else {
        fields.join(",")
    }
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
        // statements about the entry; the hash is a function of bytes the tool read, the
        // stability verdict the outcome of comparing two readings, and the acquisition states
        // the tool's account of how each component was obtained. All are stored observations,
        // but only the first group is `observed`. Every line names the observation it reports,
        // so no line depends on its neighbour for its subject (`UD-019`, `UD-020`, `UD-033`).
        let subject = observation_field(o);
        let mut basis = format!("metadata={}", metadata_state(o).as_str());
        if let Some(c) = content_state(o) {
            basis.push_str(&format!("  content={}", c.as_str()));
        }
        // A reused reading names the observation that actually read its bytes (`UD-036`).
        if let Some(source) = content_source(o) {
            basis.push_str("  ");
            basis.push_str(&reference_field("content-source", source, &o.path));
        }
        out.push(Line::derived(format!(
            "{subject}  hash={}  stability={}  {basis}",
            opt_hash(o),
            opt_stability(o)
        )));
        // A path whose metadata could not be obtained has nothing observed to show; its stored
        // kind is a placeholder, so it is listed as absent instead (D-V01-16).
        if !o.metadata_failed() {
            out.push(Line::observed(format!(
                "{subject}  kind={}  size={}  mtime={}  ctime={}",
                kind_field(o),
                opt_u64(o.size),
                opt_time(o.mtime),
                opt_time(o.ctime)
            )));
        }

        let mut absent = Vec::new();
        if o.metadata_failed() {
            absent.push("kind");
        }
        if o.size.is_none() {
            absent.push("size");
        }
        if o.mtime.is_none() {
            absent.push("mtime");
        }
        if o.dev.is_none() || o.ino.is_none() {
            absent.push("physical-identity");
        }
        // `ctime` absent from a run that recorded it was not obtainable; from an earlier run,
        // it was never recorded — a different statement (`UD-036`, A2-V8).
        if o.ctime.is_none() && o.ctime_recorded {
            absent.push("ctime");
        }
        if !absent.is_empty() {
            out.push(Line::unknown(format!(
                "{subject}  fields={}  reason=not-obtainable-at-observation-time",
                absent.join(",")
            )));
        }
        if o.ctime.is_none() && !o.ctime_recorded {
            out.push(Line::unknown(format!(
                "{subject}  fields=ctime  reason=not-recorded"
            )));
        }
        if let Some(e) = &o.error {
            // The class says which part of the scope the failure left unobserved (`UD-037`).
            let class = o.traversal.as_deref().unwrap_or("not-recorded");
            out.push(Line::unknown(format!(
                "{subject}  traversal={class}  {}",
                text_field("observation-error", e)
            )));
        }
        // Why the content was not obtained, as recorded when it was not (D-V01-10).
        if let Some(e) = &o.content_error {
            out.push(Line::unknown(format!(
                "{subject}  {}",
                text_field("content-error", e)
            )));
        }
        if o.stability == Some(Stability::Unstable) && !o.deltas.is_empty() {
            // No value was obtained, so this is `unknown`; `ambiguous` is for a classification
            // the evidence leaves open between several outcomes (`UD-031`).
            out.push(Line::unknown(format!(
                "{subject}  reason=unstable-observation  deltas={}",
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
        // The same verdict line as `changes`: its references name both runs.
        for m in &rec.mutations {
            out.extend(mutation_line(m, a.run_id, b.run_id));
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

    // 4. Every reading is attributed to the run that read it (`UD-036`): never a later run, and
    //    a carried reading equals the fresh reading of the observation it names.
    let broken = broken_readings(&all);
    out.push(Line::derived(format!(
        "readings-consistent={}  invalid-readings={}",
        broken.is_empty(),
        broken.len()
    )));
    for (o, reason) in &broken {
        out.push(Line::derived(format!(
            "invalid-reading  run={}  {}  reason={reason}",
            o.run_id,
            render_path(&o.path).field("path")
        )));
    }

    let ok = orphans == 0 && invalid.is_empty() && only_log && broken.is_empty();
    out.push(Line::derived(format!("consistent={ok}")));
    Ok(out)
}

/// Observations whose reading is not attributed as `UD-036` requires, with the reason.
fn broken_readings(all: &[Observation]) -> Vec<(&Observation, &'static str)> {
    let by_key: std::collections::BTreeMap<(RunId, &Path), &Observation> = all
        .iter()
        .map(|o| ((o.run_id, o.path.as_path()), o))
        .collect();
    let mut out = Vec::new();
    for o in all {
        let Some(hash) = o.hash else { continue };
        let reason = match o.hash_read_run {
            None => Some("reading-without-read-run"),
            Some(r) if r > o.run_id => Some("read-run-after-own-run"),
            Some(r) if r == o.run_id => None,
            Some(r) => match by_key.get(&(r, o.path.as_path())) {
                None => Some("source-missing"),
                Some(src) if src.hash_read_run != Some(r) => Some("source-not-fresh"),
                Some(src) if src.hash != Some(hash) || src.stability != o.stability => {
                    Some("differs-from-source")
                }
                Some(_) => None,
            },
        };
        if let Some(reason) = reason {
            out.push((o, reason));
        }
    }
    out
}

/// The identification field naming one observation: `observation=<run>:<path>` (`UD-033`).
fn observation_field(o: &Observation) -> String {
    crate::contract::write_reference(o.run_id as u64, &path_key(&o.path)).field("observation")
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
