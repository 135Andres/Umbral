//! The operations of `umbral-continuity/1` §6: save, resume, list, export.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use serde::Deserialize;
use serde_json::{json, Value};

use super::sources::{self, Seen};
use super::store::{self, Store, PRIVACY_WARNING};
use super::{
    Capture, Channel, Declaration, Emitter, Error, Kind, Reason, Revision, Source, EDITION,
};
use crate::contract;

const LIMITS: &str = "the evaluation compares the content of the declared sources; it does not \
observe configuration, environment, undeclared files, or whether the declarations are true";

const KIB: usize = 1024;
pub(super) const MAX_FILE: usize = 512 * KIB;

/// The save input (contract §6.1), the same object on stdin and over MCP.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SaveInput {
    pub task: String,
    /// Required, and may be `null`.
    #[serde(deserialize_with = "Option::deserialize")]
    pub expected_revision: Option<Expected>,
    #[serde(default)]
    pub emitter: DeclaredEmitter,
    pub goal: String,
    pub next_step: String,
    pub blockers: Vec<String>,
    pub declarations: Vec<Declaration>,
    pub sources: Vec<String>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeclaredEmitter {
    pub client: Option<String>,
    pub model: Option<String>,
    pub session: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Expected {
    One(String),
    Many(Vec<String>),
}

fn invalid(m: impl Into<String>) -> Error {
    Error::InvalidInput(m.into())
}

/// Contract §3.6: UTF-8 (given by the type), no C0 control but LF and TAB, nothing deceptive,
/// and never truncated.
fn check_text(field: &str, s: &str, max: usize) -> Result<(), Error> {
    if s.len() > max {
        return Err(Error::LimitExceeded {
            field: field.to_string(),
            limit: max,
            actual: s.len(),
        });
    }
    if let Some(c) = s.chars().find(|&c| {
        (c < ' ' && c != '\n' && c != '\t') || c == '\u{7f}' || contract::is_deceptive(c)
    }) {
        return Err(invalid(format!(
            "{field} contains a forbidden character U+{:04X}",
            c as u32
        )));
    }
    Ok(())
}

fn check_count(field: &str, n: usize, max: usize) -> Result<(), Error> {
    if n > max {
        return Err(Error::LimitExceeded {
            field: field.to_string(),
            limit: max,
            actual: n,
        });
    }
    Ok(())
}

/// Everything the emitter declared. `paths` are the source paths, as written.
fn check_declared(r: &Revision, paths: &[String]) -> Result<(), Error> {
    if !super::is_task_id(&r.task) {
        return Err(invalid(format!(
            "task id {:?} does not match the rule",
            r.task
        )));
    }
    check_count("parents", r.parents.len(), 8)?;
    if let Some(p) = r.parents.iter().find(|p| !super::is_revision_id(p)) {
        return Err(invalid(format!("parent {p:?} is not a revision id")));
    }
    for (name, v) in [
        ("emitter.client", &r.emitter.client),
        ("emitter.model", &r.emitter.model),
        ("emitter.session", &r.emitter.session),
    ] {
        if let Some(v) = v {
            check_text(name, v, 128)?;
        }
    }
    check_text("goal", &r.goal, 2 * KIB)?;
    check_text("next_step", &r.next_step, 2 * KIB)?;
    check_count("blockers", r.blockers.len(), 20)?;
    for b in &r.blockers {
        check_text("blockers[]", b, KIB)?;
    }

    check_count("sources", paths.len(), 200)?;
    let mut seen = BTreeSet::new();
    for p in paths {
        let bytes = contract::read_value(p)
            .map_err(|e| invalid(format!("source path {p:?} is not a written value: {e:?}")))?;
        check_count("sources[].path", bytes.len(), 4096)?;
        sources::check_path(&bytes).map_err(|m| invalid(format!("source path {p:?}: {m}")))?;
        if !seen.insert(p.as_str()) {
            return Err(invalid(format!("source path {p:?} is listed twice")));
        }
    }

    check_count("declarations", r.declarations.len(), 100)?;
    let mut ids = BTreeSet::new();
    for d in &r.declarations {
        if !super::is_slug(&d.id, 32) || !ids.insert(d.id.as_str()) {
            return Err(invalid(format!(
                "declaration id {:?} is invalid or repeated",
                d.id
            )));
        }
        check_text("declarations[].text", &d.text, 2 * KIB)?;
        let is_check = d.kind == Kind::Check;
        let has_check_fields =
            d.command.is_some() && d.claimed_result.is_some() && d.evidence.is_some();
        let has_any = d.command.is_some() || d.claimed_result.is_some() || d.evidence.is_some();
        if (is_check && !has_check_fields) || (!is_check && has_any) {
            return Err(invalid(format!(
                "declaration {:?}: command, claimed_result and evidence belong to, and are \
                 required on, a check",
                d.id
            )));
        }
        for (name, v) in [
            ("declarations[].command", &d.command),
            ("declarations[].evidence", &d.evidence),
        ] {
            if let Some(Some(v)) = v {
                check_text(name, v, KIB)?;
            }
        }
        check_count("declarations[].depends_on", d.depends_on.len(), 50)?;
        if let Some(p) = d.depends_on.iter().find(|p| !seen.contains(p.as_str())) {
            return Err(invalid(format!(
                "declaration {:?} depends on {p:?}, which is not a source",
                d.id
            )));
        }
    }
    Ok(())
}

/// The UTC millisecond form produced by `format_unix_ns`, with a real calendar date.
fn valid_timestamp(s: &str) -> bool {
    let b = s.as_bytes();
    if b.len() != 24
        || b[4] != b'-'
        || b[7] != b'-'
        || b[10] != b'T'
        || b[13] != b':'
        || b[16] != b':'
        || b[19] != b'.'
        || b[23] != b'Z'
    {
        return false;
    }
    if !(0..24)
        .filter(|i| ![4, 7, 10, 13, 16, 19, 23].contains(i))
        .all(|i| b[i].is_ascii_digit())
    {
        return false;
    }
    let number = |start: usize, end: usize| {
        b[start..end]
            .iter()
            .fold(0u32, |n, c| n * 10 + (c - b'0') as u32)
    };
    let year = number(0, 4);
    let month = number(5, 7);
    let days = match month {
        4 | 6 | 9 | 11 => 30,
        2 if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) => 29,
        2 => 28,
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        _ => return false,
    };
    (1..=days).contains(&number(8, 10))
        && number(11, 13) < 24
        && number(14, 16) < 60
        && number(17, 19) < 60
}

/// A revision read from disk: the declared part, and that each capture is well formed.
pub(super) fn check_record(r: &Revision) -> Result<(), Error> {
    if !valid_timestamp(&r.saved_at) {
        return Err(invalid("saved_at is not a UTC millisecond timestamp"));
    }
    let valid_head = match r.git.head_state {
        super::HeadState::Present => r.git.head.as_deref().is_some_and(|h| {
            matches!(h.len(), 40 | 64) && h.bytes().all(|c| matches!(c, b'0'..=b'9' | b'a'..=b'f'))
        }),
        super::HeadState::Unborn | super::HeadState::Unknown => r.git.head.is_none(),
    };
    if !valid_head {
        return Err(invalid("git.head does not match git.head_state"));
    }
    let paths: Vec<String> = r.sources.iter().map(|s| s.path.clone()).collect();
    check_declared(r, &paths)?;
    for s in &r.sources {
        if !valid_timestamp(&s.captured_at) {
            return Err(invalid(
                "sources[].captured_at is not a UTC millisecond timestamp",
            ));
        }
        let ok = match s.capture {
            Capture::Captured => {
                s.reason.is_none()
                    && s.size.is_some()
                    && s.hash.as_deref().is_some_and(|h| {
                        h.strip_prefix("blake3:").is_some_and(|x| {
                            x.len() == 64
                                && x.bytes().all(|c| matches!(c, b'0'..=b'9' | b'a'..=b'f'))
                        })
                    })
            }
            Capture::Unknown => {
                s.hash.is_none()
                    && s.size.is_none()
                    && matches!(
                        s.reason,
                        Some(Reason::PermissionDenied | Reason::UnstableRead | Reason::ReadError)
                    )
            }
        };
        if !ok {
            return Err(invalid(format!("source {:?}: malformed capture", s.path)));
        }
    }
    Ok(())
}

fn now() -> Result<String, Error> {
    crate::report::unix_ns(std::time::SystemTime::now())
        .map(crate::report::format_unix_ns)
        .ok_or_else(|| Error::Io("the system clock is out of range".into()))
}

fn decode(p: &str) -> Vec<u8> {
    // Every stored or accepted path has passed `check_declared`, so it decodes.
    contract::read_value(p).unwrap_or_default()
}

fn fork(
    revision: Option<String>,
    privacy_warning: bool,
    heads: &[String],
    revs: &[(String, Revision)],
) -> Error {
    Error::Fork {
        privacy_warning,
        revision,
        heads: heads
            .iter()
            .map(|h| {
                let at = revs
                    .iter()
                    .find(|(id, _)| id == h)
                    .map(|(_, r)| r.saved_at.clone())
                    .unwrap_or_default();
                (h.clone(), at)
            })
            .collect(),
    }
}

pub fn save(root: &Path, input: SaveInput, channel: Channel) -> Result<Value, Error> {
    let root = sources::git_root(root)?;
    let store = Store::new(root.clone());

    let mut rev = Revision {
        format: EDITION.to_string(),
        task: input.task,
        parents: Vec::new(),
        saved_at: String::new(),
        emitter: Emitter {
            channel,
            client: input.emitter.client,
            model: input.emitter.model,
            session: input.emitter.session,
        },
        goal: input.goal,
        next_step: input.next_step,
        blockers: input.blockers,
        declarations: input.declarations,
        git: super::GitState {
            head_state: super::HeadState::Unborn,
            head: None,
        },
        sources: Vec::new(),
    };
    check_declared(&rev, &input.sources)?;
    // Malformed is invalid input, not a conflict (A M-7).
    let expected: &[String] = match &input.expected_revision {
        None => &[],
        Some(Expected::One(id)) => std::slice::from_ref(id),
        Some(Expected::Many(ids)) => ids,
    };
    let distinct: BTreeSet<&String> = expected.iter().collect();
    if expected.iter().any(|id| !super::is_revision_id(id))
        || distinct.len() != expected.len()
        || matches!(&input.expected_revision, Some(Expected::Many(ids)) if ids.len() < 2)
    {
        return Err(invalid(
            "expected_revision must be null, a revision id, or a list of 2 or more distinct ids",
        ));
    }

    let existing = store.load_task(&rev.task)?.unwrap_or_default();
    let heads = store::heads(&rev.task, &existing)?;
    let conflict = || Error::RevisionConflict {
        heads: heads.clone(),
    };
    rev.parents = match input.expected_revision {
        None if heads.is_empty() => Vec::new(),
        Some(Expected::One(id)) if heads == [id.clone()] => vec![id],
        Some(Expected::Many(mut ids)) if ids.len() >= 2 => {
            // The contract's 0-8 parents, checked before writing (A-1).
            check_count("parents", ids.len(), 8)?;
            ids.sort();
            if !ids.iter().all(|id| heads.contains(id)) {
                return Err(conflict());
            }
            ids
        }
        _ => return Err(conflict()),
    };

    let mut rejected = Vec::new();
    for p in &input.sources {
        let captured_at = now()?;
        let mut s = Source {
            path: p.clone(),
            capture: Capture::Unknown,
            hash: None,
            size: None,
            reason: None,
            captured_at,
        };
        match sources::inspect(&root, &decode(p)) {
            Seen::Hash { hash, size } => {
                s.capture = Capture::Captured;
                s.hash = Some(hash);
                s.size = Some(size);
            }
            Seen::Unknown { reason, .. } => s.reason = Some(reason),
            Seen::Absent => rejected.push((p.clone(), "absent")),
            Seen::Refused(r) => rejected.push((p.clone(), r.as_str())),
        }
        rev.sources.push(s);
    }
    let readable: Vec<Vec<u8>> = input
        .sources
        .iter()
        .filter(|p| !rejected.iter().any(|(r, _)| r == *p))
        .map(|p| decode(p))
        .collect();
    let ignored = sources::git_ignored(&root, &readable)?;
    for p in &input.sources {
        if ignored.contains(&decode(p)) {
            rejected.push((p.clone(), "git-ignored"));
        }
    }
    if !rejected.is_empty() {
        return Err(Error::SourceRejected(rejected));
    }
    rev.git = sources::git_head(&root)?;
    rev.saved_at = now()?;

    let bytes = store::serialize(&rev)?;
    check_count("revision file", bytes.len(), MAX_FILE)?;
    let (id, created) = store.write(&rev.task, &bytes)?;

    let after = store.load_task(&rev.task)?.unwrap_or_default();
    let now_heads = store::heads(&rev.task, &after)?;
    if now_heads.len() > 1 {
        return Err(fork(Some(id), created, &now_heads, &after));
    }

    let mut out = json!({
        "contract": EDITION,
        "task": rev.task,
        "revision": id,
        "parents": rev.parents,
        "saved_at": rev.saved_at,
        "git": rev.git,
        "sources": rev.sources,
        "uncheckable": rev.sources.iter()
            .filter(|s| s.capture == Capture::Unknown)
            .map(|s| json!({ "path": s.path, "reason": s.reason }))
            .collect::<Vec<_>>(),
    });
    if created {
        out["privacy_warning"] = json!(PRIVACY_WARNING);
    }
    Ok(out)
}

/// A task's revisions, its heads and the one asked for (the single head by default).
/// `None` when the task has no record.
type Selected = (Vec<(String, Revision)>, Vec<String>, String);

fn select(store: &Store, task: &str, revision: Option<&str>) -> Result<Option<Selected>, Error> {
    let Some(revs) = store.load_task(task)? else {
        return Ok(None);
    };
    let heads = store::heads(task, &revs)?;
    let id = match revision {
        Some(r) => r.to_string(),
        None if heads.len() == 1 => heads[0].clone(),
        None if heads.is_empty() => return Ok(None),
        None => return Err(fork(None, false, &heads, &revs)),
    };
    if !revs.iter().any(|(r, _)| *r == id) {
        return Err(Error::NoSuchRevision(id));
    }
    Ok(Some((revs, heads, id)))
}

/// The ids `resume` and `export` are given. A malformed revision is invalid input, not a
/// missing revision.
fn check_ids(task: &str, revision: Option<&str>) -> Result<(), Error> {
    if !super::is_task_id(task) {
        return Err(invalid(format!("task id {task:?} does not match the rule")));
    }
    if let Some(r) = revision.filter(|r| !super::is_revision_id(r)) {
        return Err(invalid(format!("revision {r:?} is not a revision id")));
    }
    Ok(())
}

pub fn resume(root: &Path, task: &str, revision: Option<&str>) -> Result<Value, Error> {
    check_ids(task, revision)?;
    let root = sources::git_root(root)?;
    let Some((revs, heads, id)) = select(&Store::new(root.clone()), task, revision)? else {
        return Ok(json!({ "contract": EDITION, "state": "no-record", "task": task }));
    };
    let rev = &revs.iter().find(|(r, _)| *r == id).expect("selected").1;

    let mut states: BTreeMap<&str, &str> = BTreeMap::new();
    let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
    let mut reasons: BTreeMap<&str, usize> = BTreeMap::new();
    let mut evaluated = Vec::new();
    for s in &rev.sources {
        let mut e = json!({ "path": s.path, "recorded": s });
        let (state, reason) = match (&s.hash, s.capture) {
            (Some(recorded), Capture::Captured) => {
                match sources::inspect(&root, &decode(&s.path)) {
                    Seen::Hash { hash, size } => {
                        e["current"] = json!({ "hash": hash, "size": size });
                        (
                            if hash == *recorded {
                                "match"
                            } else {
                                "differs"
                            },
                            None,
                        )
                    }
                    Seen::Absent => ("absent", None),
                    Seen::Refused(r) => ("unknown", Some(r)),
                    Seen::Unknown {
                        reason,
                        deltas,
                        detail,
                    } => {
                        if let Some(d) = deltas {
                            e["deltas"] = json!(d);
                        }
                        if let Some(d) = detail {
                            e["detail"] = json!(d);
                        }
                        ("unknown", Some(reason))
                    }
                }
            }
            _ => ("unknown", Some(Reason::NoRecordedHash)),
        };
        e["state"] = json!(state);
        if let Some(r) = reason {
            e["reason"] = json!(r);
            *reasons.entry(r.as_str()).or_default() += 1;
        }
        *counts.entry(state).or_default() += 1;
        states.insert(s.path.as_str(), state);
        evaluated.push(e);
    }
    for k in ["match", "differs", "absent", "unknown"] {
        counts.entry(k).or_default();
    }
    let summary = if rev.sources.is_empty() {
        "no-sources"
    } else if counts["match"] == rev.sources.len() {
        "all-match"
    } else {
        "not-all-match"
    };
    let checks_needing_review: Vec<&str> = rev
        .declarations
        .iter()
        .filter(|d| {
            d.kind == Kind::Check
                && d.depends_on
                    .iter()
                    .any(|p| states.get(p.as_str()) != Some(&"match"))
        })
        .map(|d| d.id.as_str())
        .collect();
    let without_dependencies: Vec<&str> = rev
        .declarations
        .iter()
        .filter(|d| d.depends_on.is_empty())
        .map(|d| d.id.as_str())
        .collect();

    Ok(json!({
        "contract": EDITION,
        "state": "record",
        "task": task,
        "revision": id,
        "heads": heads,
        "parents": rev.parents,
        "saved_at": rev.saved_at,
        "git": rev.git,
        "declared": {
            "emitter": rev.emitter,
            "goal": rev.goal,
            "next_step": rev.next_step,
            "blockers": rev.blockers,
            "declarations": rev.declarations,
        },
        "sources": evaluated,
        "summary": summary,
        "counts": { "states": counts, "unknown_reasons": reasons },
        "checks_needing_review": checks_needing_review,
        "declarations_without_dependencies": without_dependencies,
        "limits": LIMITS,
    }))
}

/// One revision as a portable document (contract §6.1): the file's bytes as they are, with
/// where they came from. Readable by any JSON tool; the revision id is the first 32 hex
/// digits of BLAKE3 over `record`.
pub fn export(root: &Path, task: &str, revision: Option<&str>) -> Result<Value, Error> {
    check_ids(task, revision)?;
    let store = Store::new(sources::git_root(root)?);
    let Some((_, _, id)) = select(&store, task, revision)? else {
        return Err(Error::NoSuchRevision(format!("task {task} has no record")));
    };
    let bytes = store.raw(task, &id)?;
    let record = String::from_utf8(bytes).map_err(|_| Error::Record {
        token: "registro-invalido",
        task: task.to_string(),
        file: format!("{id}.json"),
        reason: "not UTF-8".into(),
    })?;
    Ok(json!({
        "contract": EDITION,
        "format": EDITION,
        "task": task,
        "revision": id,
        "integrity": "ok",
        "record": record,
    }))
}

pub fn list(root: &Path, limit: Option<usize>, cursor: Option<&str>) -> Result<Value, Error> {
    let limit = limit.unwrap_or(50);
    if !(1..=500).contains(&limit) {
        return Err(invalid("limit must be between 1 and 500"));
    }
    if let Some(c) = cursor.filter(|c| !super::is_task_id(c)) {
        return Err(invalid(format!("cursor {c:?} is not a task id")));
    }
    let root = sources::git_root(root)?;
    let store = Store::new(root);
    let names: Vec<String> = store
        .task_names()?
        .into_iter()
        .filter(|n| cursor.is_none_or(|c| n.as_str() > c))
        .collect();

    let mut tasks = Vec::new();
    for name in names.iter().take(limit) {
        let loaded = if super::is_task_id(name) {
            store.load_task(name).and_then(|revs| {
                let revs = revs.unwrap_or_default();
                let heads = store::heads(name, &revs)?;
                Ok((revs, heads))
            })
        } else {
            Err(Error::Record {
                token: "registro-invalido",
                task: name.clone(),
                file: name.clone(),
                reason: "not a task id".into(),
            })
        };
        let entry = match loaded {
            Ok((revs, heads)) => json!({
                "task": name,
                "integrity": "ok",
                "heads": heads.iter().filter_map(|h| revs.iter().find(|(id, _)| id == h)).map(|(id, r)| {
                    let truncated = r.goal.chars().count() > 120;
                    json!({
                        "revision": id,
                        "saved_at": r.saved_at,
                        "goal": r.goal.chars().take(120).collect::<String>(),
                        "goal_truncated": truncated,
                    })
                }).collect::<Vec<_>>(),
            }),
            Err(e @ (Error::Record { .. } | Error::Unsafe(_))) => json!({
                "task": name,
                "integrity": e.token(),
                "integrity_detail": e.to_string(),
                "heads": [],
            }),
            Err(e) => return Err(e),
        };
        tasks.push(entry);
    }
    let next_cursor = (names.len() > limit).then(|| names[limit - 1].clone());
    Ok(json!({ "contract": EDITION, "tasks": tasks, "next_cursor": next_cursor }))
}
