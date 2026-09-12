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

pub fn render(lines: &[Line]) -> String {
    let mut s = String::new();
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
    vec![
        Line::observed(format!(
            "run={}  root={}  canonical={}",
            run.id,
            ws.root.display(),
            ws.canonical.display()
        )),
        Line::observed(format!("run={}  entries={}", run.id, run.entries)),
        Line::observed(format!("run={}  content-verified={}", run.id, verified)),
        Line::observed(format!(
            "run={}  content-not-verified={}  reason=unstable-or-unreadable",
            run.id, not_verified
        )),
        Line::observed(format!(
            "run={}  started={}  finished={}",
            run.id,
            format_unix_ns(run.started_at_ns),
            format_unix_ns(run.finished_at_ns)
        )),
        Line::derived(format!("run={}  complete={}", run.id, run.complete())),
    ]
}

// ---------------------------------------------------------------------------------------
// status
// ---------------------------------------------------------------------------------------

pub fn status(ws: &Workspace, log: &dyn ObservationLog) -> Result<Vec<Line>, crate::log::LogError> {
    let mut out = Vec::new();
    out.push(Line::observed(format!(
        "root={}  canonical={}",
        ws.root.display(),
        ws.canonical.display()
    )));
    out.push(Line::observed(format!("workspace-id={}", ws.id)));

    let Some(run) = log.latest_run()? else {
        out.push(Line::unknown("observations=none  reason=no-run-recorded"));
        return Ok(out);
    };

    out.push(Line::observed(format!(
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

    out.push(Line::observed(format!(
        "entries={}  files={}  dirs={}  symlinks={}  other={}",
        obs.len(),
        files,
        dirs,
        symlinks,
        other
    )));
    out.push(Line::observed(format!("content-verified={verified}")));
    out.push(Line::observed(format!(
        "content-not-verified={not_verified}  reason=unstable-or-unreadable"
    )));
    out.push(Line::derived(format!(
        "content-verification-not-applicable={}  reason=not-a-regular-file",
        dirs + symlinks + other
    )));
    out.push(Line::unknown(format!(
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
        out.push(mutation_line(m));
    }

    Ok(out)
}

fn mutation_line(m: &Mutation) -> Line {
    let mut parts = vec![format!("path={}", m.path.display())];
    if let Some(old) = &m.old_path {
        parts.push(format!("old-path={}", old.display()));
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
    match m.kind {
        MutationKind::Ambiguous => Line::ambiguous(format!("{}  {text}", m.kind.as_str())),
        _ => Line::derived(format!("{}  {text}", m.kind.as_str())),
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
    if obs.is_empty() {
        return Ok(vec![Line::unknown(format!(
            "path={}  reason=not-observed-in-any-run",
            rel.display()
        ))]);
    }

    let runs = log.runs()?;
    let mut out = Vec::new();

    for o in &obs {
        out.push(Line::observed(format!(
            "run={}  kind={}  size={}  mtime={}  hash={}  stability={}",
            o.run_id,
            kind_field(o),
            opt_u64(o.size),
            match o.mtime {
                Some((s, n)) => format_unix_ns(s * 1_000_000_000 + n as i64),
                None => "none".to_string(),
            },
            opt_hash(o),
            opt_stability(o)
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
                "run={}  observation-error={e}",
                o.run_id
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
        let set_a = ObservationSet::new(vec![observed_path(a)], true);
        let set_b = ObservationSet::new(vec![observed_path(b)], complete);
        let rec = reconcile(&set_a, &set_b);
        for m in &rec.mutations {
            let line = mutation_line(m);
            out.push(Line {
                label: line.label,
                text: format!("run={} -> {}  {}", a.run_id, b.run_id, line.text),
            });
        }
    }

    Ok(out)
}

// ---------------------------------------------------------------------------------------
// check
// ---------------------------------------------------------------------------------------

/// Demonstrate the claim the persistence model rests on: the log is the truth, and the
/// derived state is recomputed from it rather than stored.
///
/// This does not add product capability. It answers exactly one question — can the
/// observable state be reconstructed from the persisted record — and reports it.
pub fn check(ws: &Workspace, log: &dyn ObservationLog) -> Result<Vec<Line>, crate::log::LogError> {
    let mut out = Vec::new();
    out.push(Line::observed(format!("state={}", ws.log_path().display())));

    let (runs_n, obs_n) = log.counts()?;
    out.push(Line::observed(format!(
        "log-runs={runs_n}  log-observations={obs_n}"
    )));

    let runs = log.runs()?;
    let all = log.all_observations()?;

    // 1. Referential integrity: every observation belongs to a run that exists.
    let known: std::collections::BTreeSet<i64> = runs.iter().map(|r| r.id).collect();
    let orphans = all.iter().filter(|o| !known.contains(&o.run_id)).count();

    // 2. Counts agree: the per-run entry count equals the rows actually stored.
    let mut count_mismatch = 0u64;
    for r in &runs {
        let rows = all.iter().filter(|o| o.run_id == r.id).count() as u64;
        if rows != r.entries {
            count_mismatch += 1;
        }
    }

    // 3. No duplicate (run, path) pairs.
    let mut seen: std::collections::BTreeSet<(i64, Vec<u8>)> = std::collections::BTreeSet::new();
    let mut duplicates = 0u64;
    for o in &all {
        let key = (o.run_id, path_key(&o.path));
        if !seen.insert(key) {
            duplicates += 1;
        }
    }

    // 4. Independent recomputation: derive the latest run's state twice, once through the
    //    per-run query and once by filtering the full log, and compare.
    let recomputed_agrees = match runs.last() {
        None => true,
        Some(last) => {
            let via_run = log.observations_for_run(last.id)?;
            let via_all: Vec<Observation> = all
                .iter()
                .filter(|o| o.run_id == last.id)
                .cloned()
                .collect();
            via_run == via_all
        }
    };

    // 5. No derived state is persisted: the stored tables are the log, and nothing else.
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
        "run-counts-agree={}  mismatched-runs={count_mismatch}",
        count_mismatch == 0
    )));
    out.push(Line::derived(format!("duplicate-entries={duplicates}")));
    out.push(Line::derived(format!(
        "derived-state-recomputed={}  agrees-with-stored={}",
        true, recomputed_agrees
    )));
    out.push(Line::derived(format!(
        "stored-tables={}  derived-state-persisted={}",
        tables.join(","),
        !only_log
    )));

    let ok =
        orphans == 0 && count_mismatch == 0 && duplicates == 0 && recomputed_agrees && only_log;
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
