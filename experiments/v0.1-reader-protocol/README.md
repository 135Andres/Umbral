# READER PROTOCOL — v0.1

Status: EXPERIMENT RECORD. Protocol designed before implementation; dry pass executed by the
author-agent. **The formal run, by a person who did not write the code, is PENDING.**

Classification of this record: experiment. It is evidence about whether v0.1's acceptance
criterion A1 holds. It is not a decision, and it changes no status.

---

## 1. WHY THIS EXISTS

v0.1's contract is not "the code runs". It is "a person who did not write this can find out
what is known, when it was known, and what changed". That is only established by having such a
person do it. The protocol was written before the tool, so that the tool could not be shaped
to answer questions it had been shown.

The subject must not be the author. Per the owner's decision D4, if the owner runs it, the
record must declare that methodological weakness. An agent that wrote the code is weaker still,
so the run below is labelled a **dry pass** and does not satisfy the criterion.

## 2. PROTOCOL

**Subject.** A person who did not write the code. Preferred: someone other than the project
owner. If neither is available, the owner, with the weakness declared in the record.

**What the subject receives.** The binary and a directory of their own whose contents they
know. Nothing else — not this repository, not the source, not an explanation of the design.

**Steps, executed by the subject.**

1. `umbral init <their directory>`
2. `umbral observe <their directory>`
3. Make three or four changes of their own choosing — create, edit, rename, delete. The
   protocol does not say which.
4. `umbral observe <their directory>` again.
5. `umbral status`, `umbral changes`, `umbral show <a path they choose>`

**Questions, answered from the output alone.**

| | Question |
|---|---|
| Q1 | How many entries are known in that directory? |
| Q2 | Which of them are verified by content, and which are not? |
| Q3 | When was it last observed, and was that observation complete? |
| Q4 | What changed since the previous observation, and how does the tool know? |
| Q5 | Is anything ambiguous? Why? |
| Q6 | Name one thing the tool does NOT know. |
| Q7 | Name one thing the tool asserts and one thing it refuses to assert. |
| Q8 | Is there anywhere in the output where you cannot tell whether something was observed or computed? |

**Recording.** The raw answers, including any that are wrong, and the transcript of the
output. Nothing is summarised.

**Pass condition.** Q1–Q8 answered correctly from the output alone. A failure is a finding,
and has two legitimate dispositions: fix it inside v0.1, or declare it outside v0.1's contract
and narrow the contract explicitly in this record. Narrowing it silently is not a disposition.

---

## 3. DRY PASS — EXECUTED 2026-09-12 BY THE AUTHOR-AGENT

**This does not satisfy A1 and does not count as the formal run.** It was run to validate that
the protocol's questions are answerable at all from the output, and to catch output defects
before a human spends time on it. The subject wrote the code, so any question it answers is
answered with knowledge the protocol is designed to withhold.

Subject's directory: a small project tree with five files (two of them with identical content)
in two subdirectories, on tmpfs. The subject then edited one file, created one, deleted one,
and renamed the created one into a subdirectory.

### Transcript

**`umbral init`**

```
observed  root=/tmp/umbral-rp-root/proyecto
observed  canonical=/tmp/umbral-rp-root/proyecto
observed  workspace-id=8e51a5b619e6d91f
observed  state-dir=/tmp/umbral-rp-data/umbral/ws-8e51a5b619e6d91f
derived   initialised=true
```

**`umbral observe` (first)**

```
observed  run=1  root=/tmp/umbral-rp-root/proyecto  canonical=/tmp/umbral-rp-root/proyecto
observed  run=1  entries=7
observed  run=1  content-verified=5
observed  run=1  content-not-verified=0  reason=unstable-or-unreadable
observed  run=1  started=2026-09-12T22:28:07.927Z  finished=2026-09-12T22:28:07.927Z
derived   run=1  complete=true
```

**`umbral observe` (after the changes)**

```
observed  run=2  root=/tmp/umbral-rp-root/proyecto  canonical=/tmp/umbral-rp-root/proyecto
observed  run=2  entries=7
observed  run=2  content-verified=5
observed  run=2  content-not-verified=0  reason=unstable-or-unreadable
observed  run=2  started=2026-09-12T22:28:07.935Z  finished=2026-09-12T22:28:07.935Z
derived   run=2  complete=true
```

**`umbral status`**

```
observed  root=/tmp/umbral-rp-root/proyecto  canonical=/tmp/umbral-rp-root/proyecto
observed  workspace-id=8e51a5b619e6d91f
observed  last-run=2  started=2026-09-12T22:28:07.935Z  entries=7
derived   last-run=2  complete=true
observed  entries=7  files=5  dirs=2  symlinks=0  other=0
observed  content-verified=5
observed  content-not-verified=0  reason=unstable-or-unreadable
derived   content-verification-not-applicable=2  reason=not-a-regular-file
unknown   unobservable-paths=0  reason=not-observed-at-observation-time
derived   log-runs=2  log-observations=14
```

**`umbral changes`**

```
derived   compared  from-run=1  to-run=2  complete=true
derived   count  unchanged=5
derived   count  modified=1
derived   count  created=1
derived   count  deleted=1
derived   count  unobserved=0
derived   count  renamed-or-moved=0
derived   count  recreated=0
derived   count  ambiguous=0
derived   deleted  path=build.log  object-survives=false  scan-complete=true
derived   modified  path=notas/borrador.md  content-changed=true  scan-complete=true
derived   created  path=notas/temporal.txt  scan-complete=true
```

**`umbral show notas/borrador.md`**

```
observed  run=1  kind=file  size=25  mtime=2026-09-12T22:28:07.923Z  hash=95ab8be11cc0  stability=stable
observed  run=2  kind=file  size=35  mtime=2026-09-12T22:28:07.930Z  hash=6ca62478c352  stability=stable
derived   run=1 -> 2  modified  path=notas/borrador.md  content-changed=true  scan-complete=true
```

**`umbral check`**

```
observed  state=/tmp/umbral-rp-data/umbral/ws-8e51a5b619e6d91f/observations.sqlite
observed  log-runs=2  log-observations=14
derived   referential-integrity=true  orphan-observations=0
derived   run-counts-agree=true  mismatched-runs=0
derived   duplicate-entries=0
derived   derived-state-recomputed=true  agrees-with-stored=true
derived   stored-tables=observation,run,schema_meta  derived-state-persisted=false
derived   consistent=true
```

**`umbral show no-existe.txt`** (a path never observed)

```
unknown   path=no-existe.txt  reason=not-observed-in-any-run
```

### Answers, from the output alone

| | Answer | Verdict |
|---|---|---|
| Q1 | `entries=7  files=5  dirs=2  symlinks=0  other=0` | answered |
| Q2 | `content-verified=5`, `content-not-verified=0`, and 2 entries where verification does not apply (`reason=not-a-regular-file`). All five files are verified. | answered at the level of counts — see F-V01-1 |
| Q3 | `last-run=2  started=2026-09-12T22:28:07.935Z`, and `derived  last-run=2  complete=true` | answered |
| Q4 | `count modified=1`, `deleted=1`, `created=1`, and one line per path carrying the evidence: `content-changed=true` for the edit, `scan-complete=true` for the deletion | answered |
| Q5 | `count ambiguous=0` — nothing was ambiguous in this run | answered |
| Q6 | `unknown  path=no-existe.txt  reason=not-observed-in-any-run`, and `unknown  unobservable-paths=0` | answered |
| Q7 | Asserts: `content-verified=5`, each with a hash in `show`. Refuses to assert: `show` on a directory reports `hash=none  stability=none` rather than a content claim, and the deleted file is reported as `deleted` only because the scan was complete | answered |
| Q8 | No. Every line begins with one of the four labels, and the label distinguishes what was read from the filesystem from what was computed | answered |

### Findings from the dry pass

**F-V01-1 — `status` reports content verification as counts, not identities.** Q2 can be
answered for this tree because the count covers every file. On a tree where some files are
verified and some are not, the reader would learn *how many* are unverified but not *which*.
Identifying them requires one `show` per path. The question is answerable, but not efficiently.

Disposition: **accepted for v0.1, and narrowed explicitly.** Adding a listing command after the
acceptance criteria were fixed would be scope growth, and identifying entries by criterion is
the territory of a later version (deterministic inspection). The limitation is recorded in
`docs/versions/v0.1.md` (limitation 8) and in the next-version handoff.

**No output defect was found.** Nothing in the transcript was ambiguous about its own status,
and no line required knowledge from outside the output to interpret.

---

## 4. WHAT REMAINS TO BE DONE

The formal run, by a person who did not write the code. Until it happens, v0.1's criterion A1
is unverified and the version cannot be declared complete.

The protocol above is the thing to run. The questions are the ones to ask. The result belongs
in a new section of this record, appended — not substituted for the dry pass, which is part of
the evidence about how this version was checked.
