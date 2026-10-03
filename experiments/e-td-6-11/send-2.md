Thank you. Here is the specification the output follows, and six mechanically
transformed versions of the same session. Please keep your earlier answers as they are; answer
these as a new step.

**TASK**

A. Using the session from before and the specification below: for **each result line** of
   `changes` and of every `show`, state (i) which entry or object it applies to, and (ii) which
   observation(s) — run and path — and which fields of each its evidence consists of. If a result
   rests on something that is not a field of an observation (an absence, a fact about a whole
   run), say so.
B. For each transformed version below: does any result's answer to A change, become ambiguous, or
   become impossible to determine? Name the result and the transformation. A transformation that
   changes nothing is a valid finding.
C. Does any value — in particular any name — fail to come back exactly when you read the output
   under the specification?

**SPECIFICATION**

# umbral-output/1 — the output contract

Status: **IMPLEMENTED** (accepted by the owner 2026-10-02, `UD-030`; implemented the same day as
v0.2's first slice, `docs/candidates/V0.2-SCOPE-PROPOSAL.md` §9.3, criteria `A2-T1-*`). The
grammar family, the error semantics and the versioning mechanism are `UD-022`; the exact choices
below — the escape table, the space rule, the deceptive-character set, the header and the
version value — are `UD-030`. Writer and reader: `src/contract.rs`. Tests: `tests/output_grammar.rs`.

Two clarifications made while implementing, neither changing what is accepted: in §6, a `\x`
followed by hexadecimal digits in lowercase is *non-canonical* (the first draft listed it under
both rows), and an output whose last line has no line feed is refused as *truncated*, which is
what `UD-022`'s "reject truncated representation" requires of a line-terminated format.

This document defines how `umbral` **writes** its standard output and how that output is
**read back**. It does not define what any field *means*; that is in [`README.md`](README.md)
and the version records.

## 1. Scope

- Covered: everything `umbral` writes to **standard output**, for every command.
- Not covered: standard error (`error: …`, usage text). It is free text for humans, carries no
  label and no header, and must never be parsed.
- Not covered: exit codes. They are documented in [`README.md`](README.md) and unchanged.

## 2. Structure

An output is a sequence of **results**, one per line, each terminated by a line feed (`0x0A`).
No value can contain a raw line feed (§4), so a line feed is always a result boundary and
nothing else. Wrapping a line for display does not change it.

The **first line of every output** is the header:

```
derived   contract=umbral-output/1
```

A line is:

```
<label padded to 9 characters> <item>(  <item>)*
```

- `<label>` is one of `observed`, `derived`, `ambiguous`, `unknown`, left-aligned in a
  nine-character column and followed by one space.
- Items are separated by **exactly two spaces**.
- An item is either a bare token (`count`, `modified`, `compared`, …) or a field
  `key=value`. A key never contains `=` or a space; the value is everything after the first
  `=` up to the next item separator, and is written in the escaped form of §4.
- A key may appear more than once on a line (`counterpart=…  counterpart=…`). The order of a
  line's items is part of the result: a reader keeps every occurrence, in order.

## 3. Values

A value is a byte string. Paths are bytes as the filesystem holds them; other values are text
produced by the tool or reported by the operating system (error messages). Every value is written
through the same function, so the rules below hold for all of them.

## 4. Escaping

Exactly two escape forms exist:

| Form | Meaning |
|---|---|
| `\\` | one backslash byte (`0x5C`) |
| `\xNN` | one byte, `NN` = two **uppercase** hexadecimal digits |

A byte is written as `\xNN` when, and only when, it is one of:

| Class | Bytes / characters | Reason token (§5) |
|---|---|---|
| a backslash | `0x5C` — written `\\`, not `\x5C` | `backslash` |
| not valid UTF-8 | any byte that is not part of a well-formed UTF-8 sequence | `not-valid-utf8` |
| a control character | `0x00`–`0x1F`, `0x7F` | `control-character` |
| an ambiguous space | `0x20` that is the first or last byte of the value, or is adjacent to another `0x20` | `ambiguous-space` |
| a deceptive character | each UTF-8 byte of: C1 controls U+0080–U+009F; bidirectional controls U+061C, U+200E, U+200F, U+202A–U+202E, U+2066–U+2069; line/paragraph separators U+2028, U+2029; zero-width characters U+200B–U+200D, U+FEFF | `deceptive-unicode` |

Every other byte is written as itself — letters with diacritics, non-Latin scripts and emoji are
readable as they are, and a single space between two other characters stays a space.

Consequences, stated because they are the point:

- **One value, one rendering.** The rules are a function of the value alone; there is no
  choice the writer can make (G-5's canonical form).
- **No value can produce a separator or a boundary.** A raw line feed is a control character;
  two adjacent spaces are ambiguous spaces; so neither can occur inside a written value.
- **A literal backslash is always `\\`**, so `\x41` in the output is always an escape, never a
  name that happens to contain those characters.

Examples:

```
path=Mis documentos/año.txt            a single interior space and UTF-8: literal
path=a\x20\x20b.txt                     two adjacent spaces
path=\x20leading.txt                    leading space
path=line1\x0Aline2                     a line feed
path=C:\\temp                           a backslash
path=\xFF\xFE                           bytes that are not UTF-8
path=invoice\xE2\x80\xAEfdp.exe         U+202E, which would otherwise reverse the text shown
```

## 5. The encoding annotation

When a written value contains at least one escape, the field is immediately followed, on the
same line, by an annotation naming the classes of §4 that occurred:

```
derived   modified  path=a\x0Ab\xFF  path-encoding=escaped:control-character,not-valid-utf8  compared-complete=true
observed  canonical=/tmp/a\x0Ab  canonical-encoding=escaped:control-character
```

- Its key is the annotated field's key followed by `-encoding`; it follows that field directly,
  so its subject is never in doubt (the defect EXP-AI-01 recorded for a separate note line).
- The reasons are listed once each, in the order of the table in §4.
- **It is part of the grammar, not a claim.** It says how a value is written, not anything about
  the filesystem or the tool's conclusions, so it may appear on a line of any label and does not
  count as a field of an `observed` line.
- It is redundant by construction: the escapes in the value already show that it is escaped. Its
  absence therefore carries no meaning that the value itself does not carry. It exists so that a
  human reading the output without this document knows the text is not the literal name on disk.

## 6. Reading

The library function that reads this format (it is not a command) must:

| Input | Behaviour |
|---|---|
| a first line that is not a header | refuse: `missing contract header` |
| a header naming any edition other than `umbral-output/1` | refuse to interpret, naming the edition |
| a malformed escape (`\` followed by anything but `\` or `x`; `\x` not followed by two hexadecimal digits; a `\` at the end of a value) | error naming line and column |
| a well-formed escape the writer would never produce (`\x41` for `A`, `\x5C` for a backslash, lowercase hex) | error: non-canonical encoding |
| a raw control character inside a line | error naming line and column |
| an output whose last line does not end with a line feed | refuse: truncated |
| after the label column: an empty item (three or more spaces in a row, a leading space, or a separator at the end of a line) | error naming line and column |
| a label this edition does not define | error |
| a well-formed `key=value` whose key, or a bare token, this edition does not define | **preserve it** as opaque data — never drop it, never rename it |

There is no recovery: no skipping, no resynchronising at a later delimiter, no best-effort
reading (`UD-022`). Unknown fields are the only thing that is tolerated, because that is what
lets a later edition add fields without breaking earlier readers.

## 6a. Observation references and the identification field

Added 2026-10-02 by v0.2 slice 2a (`UD-033`); additive, so the edition is unchanged (§7).

A **reference** names one stored observation. Its value is

```
<run>:<path>
```

- `<run>` is the run identifier in decimal, without leading zeros (`0` is written `0`);
- `<path>` is the path's bytes, relative to the observed root, as stored;
- the whole value — run, `:` and path — is written by §4 like any other value, so a path with a
  line feed or bytes that are not UTF-8 is escaped, and annotated (§5), as usual;
- it is read by splitting the decoded value at the **first** `:`. A run never contains one, so a
  path may: `12:a:b` is run 12, path `a:b`.

`umbral::contract::read_reference` refuses, with a reason: a value with no `:`; nothing before it;
a run that is not made of decimal digits only (a sign is not); a leading zero; a run larger than
the log's signed 64-bit identifier.

`observation=<reference>` is an **identification field**: it says which observation a line is
about and states nothing about the filesystem. Like the encoding annotation, it may stand on a
line of any label and is not counted as a field of an `observed` line.

```
derived   observation=2:docs/api.md  hash=6d333f581dde  stability=stable  metadata=fresh  content=fresh
observed  observation=2:docs/api.md  kind=file  size=36  mtime=2026-10-02T23:38:58.639Z
```

## 7. Versioning

- The edition is `umbral-output/1`. It is its own number. It is not the log schema version
  (`umbral-v0.1.1`), the crate version (`0.1.0`) or a build identifier, and it does not move
  with any of them.
- It moves to `umbral-output/2` only for a **breaking** change: a change to this grammar, or to
  the meaning of an existing field or token.
- Adding a field, a token or a line is **not** breaking and keeps the edition: earlier readers
  preserve what they do not know (§6).

## 8. Properties this contract must satisfy

From [`V0.2-TECHNICAL-DESIGN.md`](../docs/candidates/V0.2-TECHNICAL-DESIGN.md) §G.2d–G.2f, each
with a test that fails when the property is removed:

| Property | Statement |
|---|---|
| P34 / PC-1 | for every valid value `V`: `parse(serialize(V)) == V`, including every class of §4 and their combinations |
| P35 / PC-2 | for every valid result, its associations after reading equal the writer's |
| P36 / P42 | invalid input is rejected with a position, never repaired |
| P37 | the edition is determinable from the output alone; its absence is refused |
| P38 | no association depends on how a line is wrapped for display |
| P39 | an unknown field is preserved, not dropped or reinterpreted |
| P40 / P41 | the escape character, every delimiter and the line terminator round-trip in every position |


---

## What this is

A local, read-only tool. You point it at a directory you own, it records what is there, and
later it tells you what is known, when it was known, and what changed — labelling every
statement as `observed`, `derived`, `ambiguous` or `unknown`.

The distinction is drawn at **who produced the value**, so a reader can tell from the output
alone what came from their filesystem and what the tool produced:

- **`observed`** — the filesystem reported it for an entry during this run: `canonical`, `kind`,
  `size`, `mtime`. Those four, and no others.
- **`derived`** — the tool produced it: content fingerprints, stability verdicts, run
  identifiers, the run's own timestamps, workspace identifiers, composed paths, counts, and
  configuration echoed back.
- **`ambiguous`** — a classification the evidence leaves open between more than one outcome
  (renamed, or recreated?), with the reason named. None is chosen.
- **`unknown`** — a value that is not determinable from the evidence available: not observed,
  not obtainable, not comparable. An error is a reason given on an `unknown` line, not a label
  of its own (`UD-031`).

The set of fields allowed on an `observed` line is a constant (`OBSERVED_FIELDS` in
`src/report.rs`) and is enforced by a test, so the distinction cannot decay by inattention.

### The output format: `umbral-output/1`

Every output begins with a header naming the contract edition it follows, and every value is
written by one escaping rule, specified in [`CONTRACT.md`](CONTRACT.md) (`UD-022`, `UD-030`):

```
derived   contract=umbral-output/1
observed  canonical=/tmp/ws-\xFF\xFE  canonical-encoding=escaped:not-valid-utf8
```

A path is a byte string; the output is text. What cannot stand in a line — a backslash (written
`\\`), bytes that are not UTF-8, control characters such as a line feed, spaces that would be
ambiguous, and characters that could disguise a name on a terminal — is written as `\xNN`. The
writing is reversible and has exactly one form per value; names in any script, with single
spaces, are written as themselves. When anything was escaped, the field is followed **on the same
line** by `<field>-encoding=escaped:<reasons>`, so a reader is never left taking the escaped form
for the name on disk, and never has to guess which value the note is about.

The library reads it back (`umbral::contract::parse`), refusing anything it cannot reconstruct
exactly — an unknown edition, a malformed or non-canonical escape, a truncated output — and
keeping fields it does not know. Standard error is not part of the contract.

### How each value was obtained

Every observation has two components: its **metadata** (kind, size, mtime, physical identity),
read for every entry, and — for a regular file only — its **content**, read and hashed. For each
component the output states how it was obtained, in a closed vocabulary (`UD-031`, `UD-033`):

| State | Meaning |
|---|---|
| `fresh` | obtained in this run; a value exists |
| `reused` | carried from an earlier observation by the skip; never content verification. `show` names the observation that read the bytes: `content-source=<run>:<path>` |
| `failed` | attempted, and no value was obtained — including a reading that kept changing |
| `not-attempted` | it is recorded that no attempt was made (a path whose metadata failed: its kind is unknown) |
| `not-recorded` | it is not recorded whether an attempt was made (a row written by an older build) |

`show` writes `metadata=` and `content=` on each observation's line; a directory, symlink or
special file has no `content=` because it has no content component. Every line of `show` names
the observation it reports, `observation=<run>:<path>` ([`CONTRACT.md`](CONTRACT.md) §6a), so no
line depends on the one above it.

`observe` and `status` count by state, on lines that name their run: `metadata-fresh`,
`metadata-failed`; `content-fresh`, `content-reused`, `content-failed`, `content-not-attempted`,
`content-not-recorded` (they sum to `files + kind-unknown`); and the failed content readings by
diagnostic (`unstable-observation`, `not-found`, `permission-denied`, `not-a-regular-file`,
`read-error`).

### Reading only what changed

`observe` re-reads a regular file only when it may have changed (`UD-035`, `UD-036`). It compares
the file with the previous run's observation **of the same path**, and carries that reading
forward without reading the bytes when all of these are present and equal: the physical identity
(`dev`, `ino`), the size, the mtime and the **ctime** — and the previous reading was valid.
Anything else, anything absent, a path new to that run (a rename included): the file is read.

`ctime` is in the condition because a writer can restore size and mtime but cannot set `ctime`.
Measured before the decision (`experiments/e-td-2-3/`): on ext4, deleting and recreating a file
with different bytes, the same length and the original mtime kept its inode in 1000 of 1000
attempts; without `ctime` every one would have been skipped, with it none was.

What each run read is counted, never timed:

```
derived   run=3  content-read-entries=1  content-read-bytes=36
```

An unchanged tree reads `0` entries and `0` bytes; one modified file reads exactly that file.

Known limits, stated rather than hidden:

- **It is a heuristic, not a guarantee** (`UD-018`). A change that leaves identity, size, mtime
  and `ctime` all equal is not seen until something else moves: on a filesystem that does not
  generate its own timestamps, or within one tick of a coarse clock. None was provoked in the
  experiments; that does not make it impossible. A reused reading is therefore never presented
  as verified in that run.
- **A metadata change costs a read.** `chmod`, `chown` or a new hard link moves `ctime`, so the
  file is read again although its bytes did not change.
- `ctime` is recorded with every observation and shown on `show`'s `observed` line. Runs written
  before this version did not record it, and say so (`fields=ctime  reason=not-recorded`).

### What the traversal saw, and the rules of each run

When part of the tree cannot be observed, the failure is recorded with its class, decided when it
happens (`UD-037`): `not-descended` — a directory whose contents could not be listed, so
everything below it is unobserved — or `metadata-failed` — an entry that could not be `lstat`ed,
so only that entry is. A root that cannot be listed is a fact of the run. Any failure makes the
run incomplete, and an incomplete run never reports `deleted` (v0.2 does not narrow that to the
unobserved subtree).

```
derived   run=2  traversal-complete=false  traversal-not-descended=1  traversal-metadata-failed=0  traversal-not-recorded=0  root-not-descended=false
unknown   observation=2:fotos  traversal=not-descended  observation-error=Permission denied (os error 13)
derived   run=2  tool-version=0.1.0  scope=recursive,symlinks-not-followed,no-exclusions
```

Every run records the version of the build that wrote it and the scope it applied; runs written
before this was recorded say so (`fields=tool-version,scope  reason=not-recorded`).

### What a verdict rests on

Each line of `changes` (and the comparison lines of `show`) states, besides the verdict and its
subject `path=` (`UD-034`):

- `reference=<run>:<path>` and `compared=<run>:<path>` — the observation the verdict relates on
  each side, each followed by `reference-fields=` / `compared-fields=`: the fields the rules
  consulted on it (`dev,ino,kind,size,mtime,hash`, a subset of them, or `none`);
- `reference-absent=<run>` / `compared-absent=<run>` instead, where the verdict rests on the
  path having no observation in that run (`created`, `deleted`, `unobserved`) — written only when
  that is so;
- `counterpart=<run>:<path>`, once per entry, with `counterpart-fields=` — other entries the
  verdict rests on: every surviving hard link of a `deleted` entry, every other member of a group
  of conflicting candidates;
- `reference-complete=` and `compared-complete=` — how complete each run was. A verdict is
  relative to both.

The header line names the two runs: `compared  reference-run=…  compared-run=…`.

## What this is NOT

- **Not the product.** This is development code for one version. Nothing here is a selected
  design, and no interface decision is made by it.
- **Not V1.** V1 is reserved for a version that has passed an external audit.
- **Not a foundation to build on.** Each version is re-evaluated after it closes.
- **Not `fsp-check`.** That is the frozen V0 experiment, a separate crate in the same
  repository. This crate does not import, depend on, or modify it. V0 is cited here as
  *evidence*, never as a dependency.


**TRANSFORMED VERSIONS**

### Transformation 1-reordering: every output's lines permuted

```
$ umbral init /tmp/umbral-check-02/subject
derived   state-dir=/tmp/umbral-check-02/data/umbral/ws-242c37d665addf11
observed  canonical=/tmp/umbral-check-02/subject
derived   workspace-id=242c37d665addf11
derived   initialised=true
derived   root=/tmp/umbral-check-02/subject
derived   contract=umbral-output/1
exit=0

$ umbral observe /tmp/umbral-check-02/subject
derived   contract=umbral-output/1
derived   run=1  content-failed-diagnostics  unstable-observation=0  not-found=0  permission-denied=0  not-a-regular-file=0  read-error=0
derived   run=1  root=/tmp/umbral-check-02/subject
derived   run=1  entries=14  files=13  dirs=1  symlinks=0  other=0  kind-unknown=0
derived   run=1  started=2026-10-03T07:25:19.335Z  finished=2026-10-03T07:25:19.335Z
derived   run=1  metadata-fresh=14  metadata-failed=0
derived   run=1  traversal-complete=true  traversal-not-descended=0  traversal-metadata-failed=0  traversal-not-recorded=0  root-not-descended=false
derived   run=1  complete=true
derived   run=1  content-read-entries=13  content-read-bytes=90
derived   run=1  tool-version=0.1.0  scope=recursive,symlinks-not-followed,no-exclusions
derived   run=1  content-fresh=13  content-reused=0  content-failed=0  content-not-attempted=0  content-not-recorded=0
observed  canonical=/tmp/umbral-check-02/subject
exit=0

# a.txt edited; locked/ made unreadable (mode 000)

$ umbral observe /tmp/umbral-check-02/subject
derived   contract=umbral-output/1
derived   run=2  content-failed-diagnostics  unstable-observation=0  not-found=0  permission-denied=0  not-a-regular-file=0  read-error=0
derived   run=2  root=/tmp/umbral-check-02/subject
derived   run=2  entries=13  files=12  dirs=1  symlinks=0  other=0  kind-unknown=0
derived   run=2  started=2026-10-03T07:25:20.537Z  finished=2026-10-03T07:25:20.537Z
derived   run=2  metadata-fresh=13  metadata-failed=0
derived   run=2  traversal-complete=false  traversal-not-descended=1  traversal-metadata-failed=0  traversal-not-recorded=0  root-not-descended=false
derived   run=2  complete=false
derived   run=2  content-read-entries=1  content-read-bytes=14
derived   run=2  tool-version=0.1.0  scope=recursive,symlinks-not-followed,no-exclusions
derived   run=2  content-fresh=1  content-reused=11  content-failed=0  content-not-attempted=0  content-not-recorded=0
observed  canonical=/tmp/umbral-check-02/subject
exit=3

# locked/ readable again; c.txt renamed to d.txt; link-two deleted

$ umbral observe /tmp/umbral-check-02/subject
derived   contract=umbral-output/1
derived   run=3  content-failed-diagnostics  unstable-observation=0  not-found=0  permission-denied=0  not-a-regular-file=0  read-error=0
derived   run=3  root=/tmp/umbral-check-02/subject
derived   run=3  entries=13  files=12  dirs=1  symlinks=0  other=0  kind-unknown=0
derived   run=3  started=2026-10-03T07:25:21.739Z  finished=2026-10-03T07:25:21.739Z
derived   run=3  metadata-fresh=13  metadata-failed=0
derived   run=3  traversal-complete=true  traversal-not-descended=0  traversal-metadata-failed=0  traversal-not-recorded=0  root-not-descended=false
derived   run=3  complete=true
derived   run=3  content-read-entries=3  content-read-bytes=22
derived   run=3  tool-version=0.1.0  scope=recursive,symlinks-not-followed,no-exclusions
derived   run=3  content-fresh=3  content-reused=9  content-failed=0  content-not-attempted=0  content-not-recorded=0
observed  canonical=/tmp/umbral-check-02/subject
exit=0

$ umbral status /tmp/umbral-check-02/subject
derived   contract=umbral-output/1
derived   run=3  content-failed-diagnostics  unstable-observation=0  not-found=0  permission-denied=0  not-a-regular-file=0  read-error=0
derived   root=/tmp/umbral-check-02/subject
derived   run=3  metadata-fresh=13  metadata-failed=0
derived   run=3  tool-version=0.1.0  scope=recursive,symlinks-not-followed,no-exclusions
derived   run=3  entries=13  files=12  dirs=1  symlinks=0  other=0  kind-unknown=0
derived   last-run=3  started=2026-10-03T07:25:21.739Z
derived   workspace-id=242c37d665addf11
derived   log-runs=3  log-observations=40
derived   run=3  traversal-complete=true  traversal-not-descended=0  traversal-metadata-failed=0  traversal-not-recorded=0  root-not-descended=false
derived   run=3  content-fresh=3  content-reused=9  content-failed=0  content-not-attempted=0  content-not-recorded=0
derived   last-run=3  complete=true
observed  canonical=/tmp/umbral-check-02/subject
exit=0

$ umbral changes /tmp/umbral-check-02/subject
derived   contract=umbral-output/1
derived   count  ambiguous=0
derived   count  unchanged=11
derived   count  renamed-or-moved=1
derived   deleted  path=link-two  reference=2:link-two  reference-fields=dev,ino  compared-absent=3  counterpart=3:link-one  counterpart-fields=dev,ino  object-survives=true  reference-complete=false  compared-complete=true
derived   count  unobserved=0
derived   count  created=1
derived   count  modified=0
derived   created  path=locked/inside.txt  reference-absent=2  compared=3:locked/inside.txt  compared-fields=dev,ino  reference-complete=false  compared-complete=true
derived   renamed-or-moved  path=d.txt  reference=2:c.txt  reference-fields=dev,ino  compared=3:d.txt  compared-fields=dev,ino  reference-complete=false  compared-complete=true
derived   count  recreated=0
derived   count  deleted=1
derived   compared  reference-run=2  compared-run=3  reference-complete=false  compared-complete=true
exit=0

$ umbral show /tmp/umbral-check-02/subject a.txt
derived   modified  path=a.txt  reference=1:a.txt  reference-fields=dev,ino,kind,size,mtime,hash  compared=2:a.txt  compared-fields=dev,ino,kind,size,mtime,hash  content-changed=true  reference-complete=true  compared-complete=false
derived   observation=2:a.txt  hash=29a077efa2d8  stability=stable  metadata=fresh  content=fresh
observed  observation=1:a.txt  kind=file  size=6  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:19.332Z
derived   unchanged  path=a.txt  reference=2:a.txt  reference-fields=dev,ino,kind,size,mtime,hash  compared=3:a.txt  compared-fields=dev,ino,kind,size,mtime,hash  content-changed=false  reference-complete=false  compared-complete=true
derived   contract=umbral-output/1
observed  observation=3:a.txt  kind=file  size=14  mtime=2026-10-03T07:25:20.535Z  ctime=2026-10-03T07:25:20.535Z
observed  observation=2:a.txt  kind=file  size=14  mtime=2026-10-03T07:25:20.535Z  ctime=2026-10-03T07:25:20.535Z
derived   observation=3:a.txt  hash=29a077efa2d8  stability=stable  metadata=fresh  content=reused  content-source=2:a.txt
derived   observation=1:a.txt  hash=ac678d92b3d7  stability=stable  metadata=fresh  content=fresh
exit=0

$ umbral show /tmp/umbral-check-02/subject b.txt
derived   unchanged  path=b.txt  reference=1:b.txt  reference-fields=dev,ino,kind,size,mtime,hash  compared=2:b.txt  compared-fields=dev,ino,kind,size,mtime,hash  content-changed=false  reference-complete=true  compared-complete=false
derived   observation=2:b.txt  hash=2001794aa22d  stability=stable  metadata=fresh  content=reused  content-source=1:b.txt
observed  observation=1:b.txt  kind=file  size=6  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:19.332Z
derived   unchanged  path=b.txt  reference=2:b.txt  reference-fields=dev,ino,kind,size,mtime,hash  compared=3:b.txt  compared-fields=dev,ino,kind,size,mtime,hash  content-changed=false  reference-complete=false  compared-complete=true
derived   contract=umbral-output/1
observed  observation=3:b.txt  kind=file  size=6  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:19.332Z
observed  observation=2:b.txt  kind=file  size=6  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:19.332Z
derived   observation=3:b.txt  hash=2001794aa22d  stability=stable  metadata=fresh  content=reused  content-source=1:b.txt
derived   observation=1:b.txt  hash=2001794aa22d  stability=stable  metadata=fresh  content=fresh
exit=0

$ umbral show /tmp/umbral-check-02/subject d.txt
observed  observation=3:d.txt  kind=file  size=8  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:21.737Z
derived   observation=3:d.txt  hash=6fefa7c34afd  stability=stable  metadata=fresh  content=fresh
derived   contract=umbral-output/1
exit=0

$ umbral show /tmp/umbral-check-02/subject link-one
derived   unchanged  path=link-one  reference=1:link-one  reference-fields=dev,ino,kind,size,mtime,hash  compared=2:link-one  compared-fields=dev,ino,kind,size,mtime,hash  content-changed=false  reference-complete=true  compared-complete=false
derived   observation=2:link-one  hash=7fde9f117e69  stability=stable  metadata=fresh  content=reused  content-source=1:link-one
observed  observation=1:link-one  kind=file  size=7  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:19.332Z
derived   unchanged  path=link-one  reference=2:link-one  reference-fields=dev,ino,kind,size,mtime,hash  compared=3:link-one  compared-fields=dev,ino,kind,size,mtime,hash  content-changed=false  reference-complete=false  compared-complete=true
derived   contract=umbral-output/1
observed  observation=3:link-one  kind=file  size=7  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:21.737Z
observed  observation=2:link-one  kind=file  size=7  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:19.332Z
derived   observation=3:link-one  hash=7fde9f117e69  stability=stable  metadata=fresh  content=fresh
derived   observation=1:link-one  hash=7fde9f117e69  stability=stable  metadata=fresh  content=fresh
exit=0

$ umbral show /tmp/umbral-check-02/subject locked
derived   contract=umbral-output/1
derived   observation=3:locked  hash=none  stability=none  metadata=fresh
observed  observation=1:locked  kind=dir  size=60  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:19.332Z
derived   observation=2:locked  hash=none  stability=none  metadata=fresh
derived   unchanged  path=locked  reference=1:locked  reference-fields=dev,ino,kind  compared=2:locked  compared-fields=dev,ino,kind  reference-complete=true  compared-complete=false
observed  observation=2:locked  kind=dir  size=60  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:20.535Z
observed  observation=3:locked  kind=dir  size=60  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:21.737Z
derived   unchanged  path=locked  reference=2:locked  reference-fields=dev,ino,kind  compared=3:locked  compared-fields=dev,ino,kind  reference-complete=false  compared-complete=true
unknown   observation=2:locked  traversal=not-descended  observation-error=IO error for operation on /tmp/umbral-check-02/subject/locked: Permission denied (os error 13)
derived   observation=1:locked  hash=none  stability=none  metadata=fresh
exit=0

$ umbral show /tmp/umbral-check-02/subject line\x0Abreak.txt
derived   unchanged  path=line\x0Abreak.txt  path-encoding=escaped:control-character  reference=1:line\x0Abreak.txt  reference-encoding=escaped:control-character  reference-fields=dev,ino,kind,size,mtime,hash  compared=2:line\x0Abreak.txt  compared-encoding=escaped:control-character  compared-fields=dev,ino,kind,size,mtime,hash  content-changed=false  reference-complete=true  compared-complete=false
derived   observation=2:line\x0Abreak.txt  observation-encoding=escaped:control-character  hash=0a61c6a450a2  stability=stable  metadata=fresh  content=reused  content-source=1:line\x0Abreak.txt  content-source-encoding=escaped:control-character
observed  observation=1:line\x0Abreak.txt  observation-encoding=escaped:control-character  kind=file  size=7  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:19.332Z
derived   unchanged  path=line\x0Abreak.txt  path-encoding=escaped:control-character  reference=2:line\x0Abreak.txt  reference-encoding=escaped:control-character  reference-fields=dev,ino,kind,size,mtime,hash  compared=3:line\x0Abreak.txt  compared-encoding=escaped:control-character  compared-fields=dev,ino,kind,size,mtime,hash  content-changed=false  reference-complete=false  compared-complete=true
derived   contract=umbral-output/1
observed  observation=3:line\x0Abreak.txt  observation-encoding=escaped:control-character  kind=file  size=7  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:19.332Z
observed  observation=2:line\x0Abreak.txt  observation-encoding=escaped:control-character  kind=file  size=7  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:19.332Z
derived   observation=3:line\x0Abreak.txt  observation-encoding=escaped:control-character  hash=0a61c6a450a2  stability=stable  metadata=fresh  content=reused  content-source=1:line\x0Abreak.txt  content-source-encoding=escaped:control-character
derived   observation=1:line\x0Abreak.txt  observation-encoding=escaped:control-character  hash=0a61c6a450a2  stability=stable  metadata=fresh  content=fresh
exit=0

$ umbral show /tmp/umbral-check-02/subject bytes-\xFF.txt
derived   unchanged  path=bytes-\xFF.txt  path-encoding=escaped:not-valid-utf8  reference=1:bytes-\xFF.txt  reference-encoding=escaped:not-valid-utf8  reference-fields=dev,ino,kind,size,mtime,hash  compared=2:bytes-\xFF.txt  compared-encoding=escaped:not-valid-utf8  compared-fields=dev,ino,kind,size,mtime,hash  content-changed=false  reference-complete=true  compared-complete=false
derived   observation=2:bytes-\xFF.txt  observation-encoding=escaped:not-valid-utf8  hash=bac25e1466ab  stability=stable  metadata=fresh  content=reused  content-source=1:bytes-\xFF.txt  content-source-encoding=escaped:not-valid-utf8
observed  observation=1:bytes-\xFF.txt  observation-encoding=escaped:not-valid-utf8  kind=file  size=7  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:19.332Z
derived   unchanged  path=bytes-\xFF.txt  path-encoding=escaped:not-valid-utf8  reference=2:bytes-\xFF.txt  reference-encoding=escaped:not-valid-utf8  reference-fields=dev,ino,kind,size,mtime,hash  compared=3:bytes-\xFF.txt  compared-encoding=escaped:not-valid-utf8  compared-fields=dev,ino,kind,size,mtime,hash  content-changed=false  reference-complete=false  compared-complete=true
derived   contract=umbral-output/1
observed  observation=3:bytes-\xFF.txt  observation-encoding=escaped:not-valid-utf8  kind=file  size=7  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:19.332Z
observed  observation=2:bytes-\xFF.txt  observation-encoding=escaped:not-valid-utf8  kind=file  size=7  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:19.332Z
derived   observation=3:bytes-\xFF.txt  observation-encoding=escaped:not-valid-utf8  hash=bac25e1466ab  stability=stable  metadata=fresh  content=reused  content-source=1:bytes-\xFF.txt  content-source-encoding=escaped:not-valid-utf8
derived   observation=1:bytes-\xFF.txt  observation-encoding=escaped:not-valid-utf8  hash=bac25e1466ab  stability=stable  metadata=fresh  content=fresh
exit=0
```

### Transformation 2-subject-removal: `path=` and `observation=` deleted from every line

```
$ umbral init /tmp/umbral-check-02/subject
derived   contract=umbral-output/1
observed  canonical=/tmp/umbral-check-02/subject
derived   root=/tmp/umbral-check-02/subject
derived   workspace-id=242c37d665addf11
derived   state-dir=/tmp/umbral-check-02/data/umbral/ws-242c37d665addf11
derived   initialised=true
exit=0

$ umbral observe /tmp/umbral-check-02/subject
derived   contract=umbral-output/1
observed  canonical=/tmp/umbral-check-02/subject
derived   run=1  root=/tmp/umbral-check-02/subject
derived   run=1  entries=14  files=13  dirs=1  symlinks=0  other=0  kind-unknown=0
derived   run=1  metadata-fresh=14  metadata-failed=0
derived   run=1  content-fresh=13  content-reused=0  content-failed=0  content-not-attempted=0  content-not-recorded=0
derived   run=1  content-failed-diagnostics  unstable-observation=0  not-found=0  permission-denied=0  not-a-regular-file=0  read-error=0
derived   run=1  traversal-complete=true  traversal-not-descended=0  traversal-metadata-failed=0  traversal-not-recorded=0  root-not-descended=false
derived   run=1  tool-version=0.1.0  scope=recursive,symlinks-not-followed,no-exclusions
derived   run=1  content-read-entries=13  content-read-bytes=90
derived   run=1  started=2026-10-03T07:25:19.335Z  finished=2026-10-03T07:25:19.335Z
derived   run=1  complete=true
exit=0

# a.txt edited; locked/ made unreadable (mode 000)

$ umbral observe /tmp/umbral-check-02/subject
derived   contract=umbral-output/1
observed  canonical=/tmp/umbral-check-02/subject
derived   run=2  root=/tmp/umbral-check-02/subject
derived   run=2  entries=13  files=12  dirs=1  symlinks=0  other=0  kind-unknown=0
derived   run=2  metadata-fresh=13  metadata-failed=0
derived   run=2  content-fresh=1  content-reused=11  content-failed=0  content-not-attempted=0  content-not-recorded=0
derived   run=2  content-failed-diagnostics  unstable-observation=0  not-found=0  permission-denied=0  not-a-regular-file=0  read-error=0
derived   run=2  traversal-complete=false  traversal-not-descended=1  traversal-metadata-failed=0  traversal-not-recorded=0  root-not-descended=false
derived   run=2  tool-version=0.1.0  scope=recursive,symlinks-not-followed,no-exclusions
derived   run=2  content-read-entries=1  content-read-bytes=14
derived   run=2  started=2026-10-03T07:25:20.537Z  finished=2026-10-03T07:25:20.537Z
derived   run=2  complete=false
exit=3

# locked/ readable again; c.txt renamed to d.txt; link-two deleted

$ umbral observe /tmp/umbral-check-02/subject
derived   contract=umbral-output/1
observed  canonical=/tmp/umbral-check-02/subject
derived   run=3  root=/tmp/umbral-check-02/subject
derived   run=3  entries=13  files=12  dirs=1  symlinks=0  other=0  kind-unknown=0
derived   run=3  metadata-fresh=13  metadata-failed=0
derived   run=3  content-fresh=3  content-reused=9  content-failed=0  content-not-attempted=0  content-not-recorded=0
derived   run=3  content-failed-diagnostics  unstable-observation=0  not-found=0  permission-denied=0  not-a-regular-file=0  read-error=0
derived   run=3  traversal-complete=true  traversal-not-descended=0  traversal-metadata-failed=0  traversal-not-recorded=0  root-not-descended=false
derived   run=3  tool-version=0.1.0  scope=recursive,symlinks-not-followed,no-exclusions
derived   run=3  content-read-entries=3  content-read-bytes=22
derived   run=3  started=2026-10-03T07:25:21.739Z  finished=2026-10-03T07:25:21.739Z
derived   run=3  complete=true
exit=0

$ umbral status /tmp/umbral-check-02/subject
derived   contract=umbral-output/1
observed  canonical=/tmp/umbral-check-02/subject
derived   root=/tmp/umbral-check-02/subject
derived   workspace-id=242c37d665addf11
derived   last-run=3  started=2026-10-03T07:25:21.739Z
derived   last-run=3  complete=true
derived   run=3  entries=13  files=12  dirs=1  symlinks=0  other=0  kind-unknown=0
derived   run=3  metadata-fresh=13  metadata-failed=0
derived   run=3  content-fresh=3  content-reused=9  content-failed=0  content-not-attempted=0  content-not-recorded=0
derived   run=3  content-failed-diagnostics  unstable-observation=0  not-found=0  permission-denied=0  not-a-regular-file=0  read-error=0
derived   run=3  traversal-complete=true  traversal-not-descended=0  traversal-metadata-failed=0  traversal-not-recorded=0  root-not-descended=false
derived   run=3  tool-version=0.1.0  scope=recursive,symlinks-not-followed,no-exclusions
derived   log-runs=3  log-observations=40
exit=0

$ umbral changes /tmp/umbral-check-02/subject
derived   contract=umbral-output/1
derived   compared  reference-run=2  compared-run=3  reference-complete=false  compared-complete=true
derived   count  unchanged=11
derived   count  modified=0
derived   count  created=1
derived   count  deleted=1
derived   count  unobserved=0
derived   count  renamed-or-moved=1
derived   count  recreated=0
derived   count  ambiguous=0
derived   renamed-or-moved  reference=2:c.txt  reference-fields=dev,ino  compared=3:d.txt  compared-fields=dev,ino  reference-complete=false  compared-complete=true
derived   deleted  reference=2:link-two  reference-fields=dev,ino  compared-absent=3  counterpart=3:link-one  counterpart-fields=dev,ino  object-survives=true  reference-complete=false  compared-complete=true
derived   created  reference-absent=2  compared=3:locked/inside.txt  compared-fields=dev,ino  reference-complete=false  compared-complete=true
exit=0

$ umbral show /tmp/umbral-check-02/subject a.txt
derived   contract=umbral-output/1
derived   hash=ac678d92b3d7  stability=stable  metadata=fresh  content=fresh
observed  kind=file  size=6  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:19.332Z
derived   hash=29a077efa2d8  stability=stable  metadata=fresh  content=fresh
observed  kind=file  size=14  mtime=2026-10-03T07:25:20.535Z  ctime=2026-10-03T07:25:20.535Z
derived   hash=29a077efa2d8  stability=stable  metadata=fresh  content=reused  content-source=2:a.txt
observed  kind=file  size=14  mtime=2026-10-03T07:25:20.535Z  ctime=2026-10-03T07:25:20.535Z
derived   modified  reference=1:a.txt  reference-fields=dev,ino,kind,size,mtime,hash  compared=2:a.txt  compared-fields=dev,ino,kind,size,mtime,hash  content-changed=true  reference-complete=true  compared-complete=false
derived   unchanged  reference=2:a.txt  reference-fields=dev,ino,kind,size,mtime,hash  compared=3:a.txt  compared-fields=dev,ino,kind,size,mtime,hash  content-changed=false  reference-complete=false  compared-complete=true
exit=0

$ umbral show /tmp/umbral-check-02/subject b.txt
derived   contract=umbral-output/1
derived   hash=2001794aa22d  stability=stable  metadata=fresh  content=fresh
observed  kind=file  size=6  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:19.332Z
derived   hash=2001794aa22d  stability=stable  metadata=fresh  content=reused  content-source=1:b.txt
observed  kind=file  size=6  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:19.332Z
derived   hash=2001794aa22d  stability=stable  metadata=fresh  content=reused  content-source=1:b.txt
observed  kind=file  size=6  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:19.332Z
derived   unchanged  reference=1:b.txt  reference-fields=dev,ino,kind,size,mtime,hash  compared=2:b.txt  compared-fields=dev,ino,kind,size,mtime,hash  content-changed=false  reference-complete=true  compared-complete=false
derived   unchanged  reference=2:b.txt  reference-fields=dev,ino,kind,size,mtime,hash  compared=3:b.txt  compared-fields=dev,ino,kind,size,mtime,hash  content-changed=false  reference-complete=false  compared-complete=true
exit=0

$ umbral show /tmp/umbral-check-02/subject d.txt
derived   contract=umbral-output/1
derived   hash=6fefa7c34afd  stability=stable  metadata=fresh  content=fresh
observed  kind=file  size=8  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:21.737Z
exit=0

$ umbral show /tmp/umbral-check-02/subject link-one
derived   contract=umbral-output/1
derived   hash=7fde9f117e69  stability=stable  metadata=fresh  content=fresh
observed  kind=file  size=7  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:19.332Z
derived   hash=7fde9f117e69  stability=stable  metadata=fresh  content=reused  content-source=1:link-one
observed  kind=file  size=7  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:19.332Z
derived   hash=7fde9f117e69  stability=stable  metadata=fresh  content=fresh
observed  kind=file  size=7  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:21.737Z
derived   unchanged  reference=1:link-one  reference-fields=dev,ino,kind,size,mtime,hash  compared=2:link-one  compared-fields=dev,ino,kind,size,mtime,hash  content-changed=false  reference-complete=true  compared-complete=false
derived   unchanged  reference=2:link-one  reference-fields=dev,ino,kind,size,mtime,hash  compared=3:link-one  compared-fields=dev,ino,kind,size,mtime,hash  content-changed=false  reference-complete=false  compared-complete=true
exit=0

$ umbral show /tmp/umbral-check-02/subject locked
derived   contract=umbral-output/1
derived   hash=none  stability=none  metadata=fresh
observed  kind=dir  size=60  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:19.332Z
derived   hash=none  stability=none  metadata=fresh
observed  kind=dir  size=60  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:20.535Z
unknown   traversal=not-descended  observation-error=IO error for operation on /tmp/umbral-check-02/subject/locked: Permission denied (os error 13)
derived   hash=none  stability=none  metadata=fresh
observed  kind=dir  size=60  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:21.737Z
derived   unchanged  reference=1:locked  reference-fields=dev,ino,kind  compared=2:locked  compared-fields=dev,ino,kind  reference-complete=true  compared-complete=false
derived   unchanged  reference=2:locked  reference-fields=dev,ino,kind  compared=3:locked  compared-fields=dev,ino,kind  reference-complete=false  compared-complete=true
exit=0

$ umbral show /tmp/umbral-check-02/subject line\x0Abreak.txt
derived   contract=umbral-output/1
derived   hash=0a61c6a450a2  stability=stable  metadata=fresh  content=fresh
observed  kind=file  size=7  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:19.332Z
derived   hash=0a61c6a450a2  stability=stable  metadata=fresh  content=reused  content-source=1:line\x0Abreak.txt  content-source-encoding=escaped:control-character
observed  kind=file  size=7  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:19.332Z
derived   hash=0a61c6a450a2  stability=stable  metadata=fresh  content=reused  content-source=1:line\x0Abreak.txt  content-source-encoding=escaped:control-character
observed  kind=file  size=7  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:19.332Z
derived   unchanged  reference=1:line\x0Abreak.txt  reference-encoding=escaped:control-character  reference-fields=dev,ino,kind,size,mtime,hash  compared=2:line\x0Abreak.txt  compared-encoding=escaped:control-character  compared-fields=dev,ino,kind,size,mtime,hash  content-changed=false  reference-complete=true  compared-complete=false
derived   unchanged  reference=2:line\x0Abreak.txt  reference-encoding=escaped:control-character  reference-fields=dev,ino,kind,size,mtime,hash  compared=3:line\x0Abreak.txt  compared-encoding=escaped:control-character  compared-fields=dev,ino,kind,size,mtime,hash  content-changed=false  reference-complete=false  compared-complete=true
exit=0

$ umbral show /tmp/umbral-check-02/subject bytes-\xFF.txt
derived   contract=umbral-output/1
derived   hash=bac25e1466ab  stability=stable  metadata=fresh  content=fresh
observed  kind=file  size=7  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:19.332Z
derived   hash=bac25e1466ab  stability=stable  metadata=fresh  content=reused  content-source=1:bytes-\xFF.txt  content-source-encoding=escaped:not-valid-utf8
observed  kind=file  size=7  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:19.332Z
derived   hash=bac25e1466ab  stability=stable  metadata=fresh  content=reused  content-source=1:bytes-\xFF.txt  content-source-encoding=escaped:not-valid-utf8
observed  kind=file  size=7  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:19.332Z
derived   unchanged  reference=1:bytes-\xFF.txt  reference-encoding=escaped:not-valid-utf8  reference-fields=dev,ino,kind,size,mtime,hash  compared=2:bytes-\xFF.txt  compared-encoding=escaped:not-valid-utf8  compared-fields=dev,ino,kind,size,mtime,hash  content-changed=false  reference-complete=true  compared-complete=false
derived   unchanged  reference=2:bytes-\xFF.txt  reference-encoding=escaped:not-valid-utf8  reference-fields=dev,ino,kind,size,mtime,hash  compared=3:bytes-\xFF.txt  compared-encoding=escaped:not-valid-utf8  compared-fields=dev,ino,kind,size,mtime,hash  content-changed=false  reference-complete=false  compared-complete=true
exit=0
```

### Transformation 3-reflow: every line wrapped at 60 columns

```
$ umbral init /tmp/umbral-check-02/subject
derived   contract=umbral-output/1
observed  canonical=/tmp/umbral-check-02/subject
derived   root=/tmp/umbral-check-02/subject
derived   workspace-id=242c37d665addf11
derived   state-dir=/tmp/umbral-check-02/data/umbral/ws-242c
37d665addf11
derived   initialised=true
exit=0

$ umbral observe /tmp/umbral-check-02/subject
derived   contract=umbral-output/1
observed  canonical=/tmp/umbral-check-02/subject
derived   run=1  root=/tmp/umbral-check-02/subject
derived   run=1  entries=14  files=13  dirs=1  symlinks=0  
other=0  kind-unknown=0
derived   run=1  metadata-fresh=14  metadata-failed=0
derived   run=1  content-fresh=13  content-reused=0  
content-failed=0  content-not-attempted=0  
content-not-recorded=0
derived   run=1  content-failed-diagnostics  
unstable-observation=0  not-found=0  permission-denied=0  
not-a-regular-file=0  read-error=0
derived   run=1  traversal-complete=true  
traversal-not-descended=0  traversal-metadata-failed=0  
traversal-not-recorded=0  root-not-descended=false
derived   run=1  tool-version=0.1.0  
scope=recursive,symlinks-not-followed,no-exclusions
derived   run=1  content-read-entries=13  
content-read-bytes=90
derived   run=1  started=2026-10-03T07:25:19.335Z  
finished=2026-10-03T07:25:19.335Z
derived   run=1  complete=true
exit=0

# a.txt edited; locked/ made unreadable (mode 000)

$ umbral observe /tmp/umbral-check-02/subject
derived   contract=umbral-output/1
observed  canonical=/tmp/umbral-check-02/subject
derived   run=2  root=/tmp/umbral-check-02/subject
derived   run=2  entries=13  files=12  dirs=1  symlinks=0  
other=0  kind-unknown=0
derived   run=2  metadata-fresh=13  metadata-failed=0
derived   run=2  content-fresh=1  content-reused=11  
content-failed=0  content-not-attempted=0  
content-not-recorded=0
derived   run=2  content-failed-diagnostics  
unstable-observation=0  not-found=0  permission-denied=0  
not-a-regular-file=0  read-error=0
derived   run=2  traversal-complete=false  
traversal-not-descended=1  traversal-metadata-failed=0  
traversal-not-recorded=0  root-not-descended=false
derived   run=2  tool-version=0.1.0  
scope=recursive,symlinks-not-followed,no-exclusions
derived   run=2  content-read-entries=1  
content-read-bytes=14
derived   run=2  started=2026-10-03T07:25:20.537Z  
finished=2026-10-03T07:25:20.537Z
derived   run=2  complete=false
exit=3

# locked/ readable again; c.txt renamed to d.txt; link-two deleted

$ umbral observe /tmp/umbral-check-02/subject
derived   contract=umbral-output/1
observed  canonical=/tmp/umbral-check-02/subject
derived   run=3  root=/tmp/umbral-check-02/subject
derived   run=3  entries=13  files=12  dirs=1  symlinks=0  
other=0  kind-unknown=0
derived   run=3  metadata-fresh=13  metadata-failed=0
derived   run=3  content-fresh=3  content-reused=9  
content-failed=0  content-not-attempted=0  
content-not-recorded=0
derived   run=3  content-failed-diagnostics  
unstable-observation=0  not-found=0  permission-denied=0  
not-a-regular-file=0  read-error=0
derived   run=3  traversal-complete=true  
traversal-not-descended=0  traversal-metadata-failed=0  
traversal-not-recorded=0  root-not-descended=false
derived   run=3  tool-version=0.1.0  
scope=recursive,symlinks-not-followed,no-exclusions
derived   run=3  content-read-entries=3  
content-read-bytes=22
derived   run=3  started=2026-10-03T07:25:21.739Z  
finished=2026-10-03T07:25:21.739Z
derived   run=3  complete=true
exit=0

$ umbral status /tmp/umbral-check-02/subject
derived   contract=umbral-output/1
observed  canonical=/tmp/umbral-check-02/subject
derived   root=/tmp/umbral-check-02/subject
derived   workspace-id=242c37d665addf11
derived   last-run=3  started=2026-10-03T07:25:21.739Z
derived   last-run=3  complete=true
derived   run=3  entries=13  files=12  dirs=1  symlinks=0  
other=0  kind-unknown=0
derived   run=3  metadata-fresh=13  metadata-failed=0
derived   run=3  content-fresh=3  content-reused=9  
content-failed=0  content-not-attempted=0  
content-not-recorded=0
derived   run=3  content-failed-diagnostics  
unstable-observation=0  not-found=0  permission-denied=0  
not-a-regular-file=0  read-error=0
derived   run=3  traversal-complete=true  
traversal-not-descended=0  traversal-metadata-failed=0  
traversal-not-recorded=0  root-not-descended=false
derived   run=3  tool-version=0.1.0  
scope=recursive,symlinks-not-followed,no-exclusions
derived   log-runs=3  log-observations=40
exit=0

$ umbral changes /tmp/umbral-check-02/subject
derived   contract=umbral-output/1
derived   compared  reference-run=2  compared-run=3  
reference-complete=false  compared-complete=true
derived   count  unchanged=11
derived   count  modified=0
derived   count  created=1
derived   count  deleted=1
derived   count  unobserved=0
derived   count  renamed-or-moved=1
derived   count  recreated=0
derived   count  ambiguous=0
derived   renamed-or-moved  path=d.txt  reference=2:c.txt  
reference-fields=dev,ino  compared=3:d.txt  
compared-fields=dev,ino  reference-complete=false  
compared-complete=true
derived   deleted  path=link-two  reference=2:link-two  
reference-fields=dev,ino  compared-absent=3  
counterpart=3:link-one  counterpart-fields=dev,ino  
object-survives=true  reference-complete=false  
compared-complete=true
derived   created  path=locked/inside.txt  
reference-absent=2  compared=3:locked/inside.txt  
compared-fields=dev,ino  reference-complete=false  
compared-complete=true
exit=0

$ umbral show /tmp/umbral-check-02/subject a.txt
derived   contract=umbral-output/1
derived   observation=1:a.txt  hash=ac678d92b3d7  
stability=stable  metadata=fresh  content=fresh
observed  observation=1:a.txt  kind=file  size=6  
mtime=2026-10-03T07:25:19.332Z  
ctime=2026-10-03T07:25:19.332Z
derived   observation=2:a.txt  hash=29a077efa2d8  
stability=stable  metadata=fresh  content=fresh
observed  observation=2:a.txt  kind=file  size=14  
mtime=2026-10-03T07:25:20.535Z  
ctime=2026-10-03T07:25:20.535Z
derived   observation=3:a.txt  hash=29a077efa2d8  
stability=stable  metadata=fresh  content=reused  
content-source=2:a.txt
observed  observation=3:a.txt  kind=file  size=14  
mtime=2026-10-03T07:25:20.535Z  
ctime=2026-10-03T07:25:20.535Z
derived   modified  path=a.txt  reference=1:a.txt  
reference-fields=dev,ino,kind,size,mtime,hash  
compared=2:a.txt  
compared-fields=dev,ino,kind,size,mtime,hash  
content-changed=true  reference-complete=true  
compared-complete=false
derived   unchanged  path=a.txt  reference=2:a.txt  
reference-fields=dev,ino,kind,size,mtime,hash  
compared=3:a.txt  
compared-fields=dev,ino,kind,size,mtime,hash  
content-changed=false  reference-complete=false  
compared-complete=true
exit=0

$ umbral show /tmp/umbral-check-02/subject b.txt
derived   contract=umbral-output/1
derived   observation=1:b.txt  hash=2001794aa22d  
stability=stable  metadata=fresh  content=fresh
observed  observation=1:b.txt  kind=file  size=6  
mtime=2026-10-03T07:25:19.332Z  
ctime=2026-10-03T07:25:19.332Z
derived   observation=2:b.txt  hash=2001794aa22d  
stability=stable  metadata=fresh  content=reused  
content-source=1:b.txt
observed  observation=2:b.txt  kind=file  size=6  
mtime=2026-10-03T07:25:19.332Z  
ctime=2026-10-03T07:25:19.332Z
derived   observation=3:b.txt  hash=2001794aa22d  
stability=stable  metadata=fresh  content=reused  
content-source=1:b.txt
observed  observation=3:b.txt  kind=file  size=6  
mtime=2026-10-03T07:25:19.332Z  
ctime=2026-10-03T07:25:19.332Z
derived   unchanged  path=b.txt  reference=1:b.txt  
reference-fields=dev,ino,kind,size,mtime,hash  
compared=2:b.txt  
compared-fields=dev,ino,kind,size,mtime,hash  
content-changed=false  reference-complete=true  
compared-complete=false
derived   unchanged  path=b.txt  reference=2:b.txt  
reference-fields=dev,ino,kind,size,mtime,hash  
compared=3:b.txt  
compared-fields=dev,ino,kind,size,mtime,hash  
content-changed=false  reference-complete=false  
compared-complete=true
exit=0

$ umbral show /tmp/umbral-check-02/subject d.txt
derived   contract=umbral-output/1
derived   observation=3:d.txt  hash=6fefa7c34afd  
stability=stable  metadata=fresh  content=fresh
observed  observation=3:d.txt  kind=file  size=8  
mtime=2026-10-03T07:25:19.332Z  
ctime=2026-10-03T07:25:21.737Z
exit=0

$ umbral show /tmp/umbral-check-02/subject link-one
derived   contract=umbral-output/1
derived   observation=1:link-one  hash=7fde9f117e69  
stability=stable  metadata=fresh  content=fresh
observed  observation=1:link-one  kind=file  size=7  
mtime=2026-10-03T07:25:19.332Z  
ctime=2026-10-03T07:25:19.332Z
derived   observation=2:link-one  hash=7fde9f117e69  
stability=stable  metadata=fresh  content=reused  
content-source=1:link-one
observed  observation=2:link-one  kind=file  size=7  
mtime=2026-10-03T07:25:19.332Z  
ctime=2026-10-03T07:25:19.332Z
derived   observation=3:link-one  hash=7fde9f117e69  
stability=stable  metadata=fresh  content=fresh
observed  observation=3:link-one  kind=file  size=7  
mtime=2026-10-03T07:25:19.332Z  
ctime=2026-10-03T07:25:21.737Z
derived   unchanged  path=link-one  reference=1:link-one  
reference-fields=dev,ino,kind,size,mtime,hash  
compared=2:link-one  
compared-fields=dev,ino,kind,size,mtime,hash  
content-changed=false  reference-complete=true  
compared-complete=false
derived   unchanged  path=link-one  reference=2:link-one  
reference-fields=dev,ino,kind,size,mtime,hash  
compared=3:link-one  
compared-fields=dev,ino,kind,size,mtime,hash  
content-changed=false  reference-complete=false  
compared-complete=true
exit=0

$ umbral show /tmp/umbral-check-02/subject locked
derived   contract=umbral-output/1
derived   observation=1:locked  hash=none  stability=none  
metadata=fresh
observed  observation=1:locked  kind=dir  size=60  
mtime=2026-10-03T07:25:19.332Z  
ctime=2026-10-03T07:25:19.332Z
derived   observation=2:locked  hash=none  stability=none  
metadata=fresh
observed  observation=2:locked  kind=dir  size=60  
mtime=2026-10-03T07:25:19.332Z  
ctime=2026-10-03T07:25:20.535Z
unknown   observation=2:locked  traversal=not-descended  
observation-error=IO error for operation on 
/tmp/umbral-check-02/subject/locked: Permission denied (os 
error 13)
derived   observation=3:locked  hash=none  stability=none  
metadata=fresh
observed  observation=3:locked  kind=dir  size=60  
mtime=2026-10-03T07:25:19.332Z  
ctime=2026-10-03T07:25:21.737Z
derived   unchanged  path=locked  reference=1:locked  
reference-fields=dev,ino,kind  compared=2:locked  
compared-fields=dev,ino,kind  reference-complete=true  
compared-complete=false
derived   unchanged  path=locked  reference=2:locked  
reference-fields=dev,ino,kind  compared=3:locked  
compared-fields=dev,ino,kind  reference-complete=false  
compared-complete=true
exit=0

$ umbral show /tmp/umbral-check-02/subject line\x0Abreak.txt
derived   contract=umbral-output/1
derived   observation=1:line\x0Abreak.txt  
observation-encoding=escaped:control-character  
hash=0a61c6a450a2  stability=stable  metadata=fresh  
content=fresh
observed  observation=1:line\x0Abreak.txt  
observation-encoding=escaped:control-character  kind=file  
size=7  mtime=2026-10-03T07:25:19.332Z  
ctime=2026-10-03T07:25:19.332Z
derived   observation=2:line\x0Abreak.txt  
observation-encoding=escaped:control-character  
hash=0a61c6a450a2  stability=stable  metadata=fresh  
content=reused  content-source=1:line\x0Abreak.txt  
content-source-encoding=escaped:control-character
observed  observation=2:line\x0Abreak.txt  
observation-encoding=escaped:control-character  kind=file  
size=7  mtime=2026-10-03T07:25:19.332Z  
ctime=2026-10-03T07:25:19.332Z
derived   observation=3:line\x0Abreak.txt  
observation-encoding=escaped:control-character  
hash=0a61c6a450a2  stability=stable  metadata=fresh  
content=reused  content-source=1:line\x0Abreak.txt  
content-source-encoding=escaped:control-character
observed  observation=3:line\x0Abreak.txt  
observation-encoding=escaped:control-character  kind=file  
size=7  mtime=2026-10-03T07:25:19.332Z  
ctime=2026-10-03T07:25:19.332Z
derived   unchanged  path=line\x0Abreak.txt  
path-encoding=escaped:control-character  
reference=1:line\x0Abreak.txt  
reference-encoding=escaped:control-character  
reference-fields=dev,ino,kind,size,mtime,hash  
compared=2:line\x0Abreak.txt  
compared-encoding=escaped:control-character  
compared-fields=dev,ino,kind,size,mtime,hash  
content-changed=false  reference-complete=true  
compared-complete=false
derived   unchanged  path=line\x0Abreak.txt  
path-encoding=escaped:control-character  
reference=2:line\x0Abreak.txt  
reference-encoding=escaped:control-character  
reference-fields=dev,ino,kind,size,mtime,hash  
compared=3:line\x0Abreak.txt  
compared-encoding=escaped:control-character  
compared-fields=dev,ino,kind,size,mtime,hash  
content-changed=false  reference-complete=false  
compared-complete=true
exit=0

$ umbral show /tmp/umbral-check-02/subject bytes-\xFF.txt
derived   contract=umbral-output/1
derived   observation=1:bytes-\xFF.txt  
observation-encoding=escaped:not-valid-utf8  
hash=bac25e1466ab  stability=stable  metadata=fresh  
content=fresh
observed  observation=1:bytes-\xFF.txt  
observation-encoding=escaped:not-valid-utf8  kind=file  
size=7  mtime=2026-10-03T07:25:19.332Z  
ctime=2026-10-03T07:25:19.332Z
derived   observation=2:bytes-\xFF.txt  
observation-encoding=escaped:not-valid-utf8  
hash=bac25e1466ab  stability=stable  metadata=fresh  
content=reused  content-source=1:bytes-\xFF.txt  
content-source-encoding=escaped:not-valid-utf8
observed  observation=2:bytes-\xFF.txt  
observation-encoding=escaped:not-valid-utf8  kind=file  
size=7  mtime=2026-10-03T07:25:19.332Z  
ctime=2026-10-03T07:25:19.332Z
derived   observation=3:bytes-\xFF.txt  
observation-encoding=escaped:not-valid-utf8  
hash=bac25e1466ab  stability=stable  metadata=fresh  
content=reused  content-source=1:bytes-\xFF.txt  
content-source-encoding=escaped:not-valid-utf8
observed  observation=3:bytes-\xFF.txt  
observation-encoding=escaped:not-valid-utf8  kind=file  
size=7  mtime=2026-10-03T07:25:19.332Z  
ctime=2026-10-03T07:25:19.332Z
derived   unchanged  path=bytes-\xFF.txt  
path-encoding=escaped:not-valid-utf8  
reference=1:bytes-\xFF.txt  
reference-encoding=escaped:not-valid-utf8  
reference-fields=dev,ino,kind,size,mtime,hash  
compared=2:bytes-\xFF.txt  
compared-encoding=escaped:not-valid-utf8  
compared-fields=dev,ino,kind,size,mtime,hash  
content-changed=false  reference-complete=true  
compared-complete=false
derived   unchanged  path=bytes-\xFF.txt  
path-encoding=escaped:not-valid-utf8  
reference=2:bytes-\xFF.txt  
reference-encoding=escaped:not-valid-utf8  
reference-fields=dev,ino,kind,size,mtime,hash  
compared=3:bytes-\xFF.txt  
compared-encoding=escaped:not-valid-utf8  
compared-fields=dev,ino,kind,size,mtime,hash  
content-changed=false  reference-complete=false  
compared-complete=true
exit=0
```

### Transformation 4-delimiter-split: every item on its own line, under its label

```
$ umbral init /tmp/umbral-check-02/subject
derived
    contract=umbral-output/1
observed
    canonical=/tmp/umbral-check-02/subject
derived
    root=/tmp/umbral-check-02/subject
derived
    workspace-id=242c37d665addf11
derived
    state-dir=/tmp/umbral-check-02/data/umbral/ws-242c37d665addf11
derived
    initialised=true
exit=0

$ umbral observe /tmp/umbral-check-02/subject
derived
    contract=umbral-output/1
observed
    canonical=/tmp/umbral-check-02/subject
derived
    run=1
    root=/tmp/umbral-check-02/subject
derived
    run=1
    entries=14
    files=13
    dirs=1
    symlinks=0
    other=0
    kind-unknown=0
derived
    run=1
    metadata-fresh=14
    metadata-failed=0
derived
    run=1
    content-fresh=13
    content-reused=0
    content-failed=0
    content-not-attempted=0
    content-not-recorded=0
derived
    run=1
    content-failed-diagnostics
    unstable-observation=0
    not-found=0
    permission-denied=0
    not-a-regular-file=0
    read-error=0
derived
    run=1
    traversal-complete=true
    traversal-not-descended=0
    traversal-metadata-failed=0
    traversal-not-recorded=0
    root-not-descended=false
derived
    run=1
    tool-version=0.1.0
    scope=recursive,symlinks-not-followed,no-exclusions
derived
    run=1
    content-read-entries=13
    content-read-bytes=90
derived
    run=1
    started=2026-10-03T07:25:19.335Z
    finished=2026-10-03T07:25:19.335Z
derived
    run=1
    complete=true
exit=0

# a.txt edited; locked/ made unreadable (mode 000)

$ umbral observe /tmp/umbral-check-02/subject
derived
    contract=umbral-output/1
observed
    canonical=/tmp/umbral-check-02/subject
derived
    run=2
    root=/tmp/umbral-check-02/subject
derived
    run=2
    entries=13
    files=12
    dirs=1
    symlinks=0
    other=0
    kind-unknown=0
derived
    run=2
    metadata-fresh=13
    metadata-failed=0
derived
    run=2
    content-fresh=1
    content-reused=11
    content-failed=0
    content-not-attempted=0
    content-not-recorded=0
derived
    run=2
    content-failed-diagnostics
    unstable-observation=0
    not-found=0
    permission-denied=0
    not-a-regular-file=0
    read-error=0
derived
    run=2
    traversal-complete=false
    traversal-not-descended=1
    traversal-metadata-failed=0
    traversal-not-recorded=0
    root-not-descended=false
derived
    run=2
    tool-version=0.1.0
    scope=recursive,symlinks-not-followed,no-exclusions
derived
    run=2
    content-read-entries=1
    content-read-bytes=14
derived
    run=2
    started=2026-10-03T07:25:20.537Z
    finished=2026-10-03T07:25:20.537Z
derived
    run=2
    complete=false
exit=3

# locked/ readable again; c.txt renamed to d.txt; link-two deleted

$ umbral observe /tmp/umbral-check-02/subject
derived
    contract=umbral-output/1
observed
    canonical=/tmp/umbral-check-02/subject
derived
    run=3
    root=/tmp/umbral-check-02/subject
derived
    run=3
    entries=13
    files=12
    dirs=1
    symlinks=0
    other=0
    kind-unknown=0
derived
    run=3
    metadata-fresh=13
    metadata-failed=0
derived
    run=3
    content-fresh=3
    content-reused=9
    content-failed=0
    content-not-attempted=0
    content-not-recorded=0
derived
    run=3
    content-failed-diagnostics
    unstable-observation=0
    not-found=0
    permission-denied=0
    not-a-regular-file=0
    read-error=0
derived
    run=3
    traversal-complete=true
    traversal-not-descended=0
    traversal-metadata-failed=0
    traversal-not-recorded=0
    root-not-descended=false
derived
    run=3
    tool-version=0.1.0
    scope=recursive,symlinks-not-followed,no-exclusions
derived
    run=3
    content-read-entries=3
    content-read-bytes=22
derived
    run=3
    started=2026-10-03T07:25:21.739Z
    finished=2026-10-03T07:25:21.739Z
derived
    run=3
    complete=true
exit=0

$ umbral status /tmp/umbral-check-02/subject
derived
    contract=umbral-output/1
observed
    canonical=/tmp/umbral-check-02/subject
derived
    root=/tmp/umbral-check-02/subject
derived
    workspace-id=242c37d665addf11
derived
    last-run=3
    started=2026-10-03T07:25:21.739Z
derived
    last-run=3
    complete=true
derived
    run=3
    entries=13
    files=12
    dirs=1
    symlinks=0
    other=0
    kind-unknown=0
derived
    run=3
    metadata-fresh=13
    metadata-failed=0
derived
    run=3
    content-fresh=3
    content-reused=9
    content-failed=0
    content-not-attempted=0
    content-not-recorded=0
derived
    run=3
    content-failed-diagnostics
    unstable-observation=0
    not-found=0
    permission-denied=0
    not-a-regular-file=0
    read-error=0
derived
    run=3
    traversal-complete=true
    traversal-not-descended=0
    traversal-metadata-failed=0
    traversal-not-recorded=0
    root-not-descended=false
derived
    run=3
    tool-version=0.1.0
    scope=recursive,symlinks-not-followed,no-exclusions
derived
    log-runs=3
    log-observations=40
exit=0

$ umbral changes /tmp/umbral-check-02/subject
derived
    contract=umbral-output/1
derived
    compared
    reference-run=2
    compared-run=3
    reference-complete=false
    compared-complete=true
derived
    count
    unchanged=11
derived
    count
    modified=0
derived
    count
    created=1
derived
    count
    deleted=1
derived
    count
    unobserved=0
derived
    count
    renamed-or-moved=1
derived
    count
    recreated=0
derived
    count
    ambiguous=0
derived
    renamed-or-moved
    path=d.txt
    reference=2:c.txt
    reference-fields=dev,ino
    compared=3:d.txt
    compared-fields=dev,ino
    reference-complete=false
    compared-complete=true
derived
    deleted
    path=link-two
    reference=2:link-two
    reference-fields=dev,ino
    compared-absent=3
    counterpart=3:link-one
    counterpart-fields=dev,ino
    object-survives=true
    reference-complete=false
    compared-complete=true
derived
    created
    path=locked/inside.txt
    reference-absent=2
    compared=3:locked/inside.txt
    compared-fields=dev,ino
    reference-complete=false
    compared-complete=true
exit=0

$ umbral show /tmp/umbral-check-02/subject a.txt
derived
    contract=umbral-output/1
derived
    observation=1:a.txt
    hash=ac678d92b3d7
    stability=stable
    metadata=fresh
    content=fresh
observed
    observation=1:a.txt
    kind=file
    size=6
    mtime=2026-10-03T07:25:19.332Z
    ctime=2026-10-03T07:25:19.332Z
derived
    observation=2:a.txt
    hash=29a077efa2d8
    stability=stable
    metadata=fresh
    content=fresh
observed
    observation=2:a.txt
    kind=file
    size=14
    mtime=2026-10-03T07:25:20.535Z
    ctime=2026-10-03T07:25:20.535Z
derived
    observation=3:a.txt
    hash=29a077efa2d8
    stability=stable
    metadata=fresh
    content=reused
    content-source=2:a.txt
observed
    observation=3:a.txt
    kind=file
    size=14
    mtime=2026-10-03T07:25:20.535Z
    ctime=2026-10-03T07:25:20.535Z
derived
    modified
    path=a.txt
    reference=1:a.txt
    reference-fields=dev,ino,kind,size,mtime,hash
    compared=2:a.txt
    compared-fields=dev,ino,kind,size,mtime,hash
    content-changed=true
    reference-complete=true
    compared-complete=false
derived
    unchanged
    path=a.txt
    reference=2:a.txt
    reference-fields=dev,ino,kind,size,mtime,hash
    compared=3:a.txt
    compared-fields=dev,ino,kind,size,mtime,hash
    content-changed=false
    reference-complete=false
    compared-complete=true
exit=0

$ umbral show /tmp/umbral-check-02/subject b.txt
derived
    contract=umbral-output/1
derived
    observation=1:b.txt
    hash=2001794aa22d
    stability=stable
    metadata=fresh
    content=fresh
observed
    observation=1:b.txt
    kind=file
    size=6
    mtime=2026-10-03T07:25:19.332Z
    ctime=2026-10-03T07:25:19.332Z
derived
    observation=2:b.txt
    hash=2001794aa22d
    stability=stable
    metadata=fresh
    content=reused
    content-source=1:b.txt
observed
    observation=2:b.txt
    kind=file
    size=6
    mtime=2026-10-03T07:25:19.332Z
    ctime=2026-10-03T07:25:19.332Z
derived
    observation=3:b.txt
    hash=2001794aa22d
    stability=stable
    metadata=fresh
    content=reused
    content-source=1:b.txt
observed
    observation=3:b.txt
    kind=file
    size=6
    mtime=2026-10-03T07:25:19.332Z
    ctime=2026-10-03T07:25:19.332Z
derived
    unchanged
    path=b.txt
    reference=1:b.txt
    reference-fields=dev,ino,kind,size,mtime,hash
    compared=2:b.txt
    compared-fields=dev,ino,kind,size,mtime,hash
    content-changed=false
    reference-complete=true
    compared-complete=false
derived
    unchanged
    path=b.txt
    reference=2:b.txt
    reference-fields=dev,ino,kind,size,mtime,hash
    compared=3:b.txt
    compared-fields=dev,ino,kind,size,mtime,hash
    content-changed=false
    reference-complete=false
    compared-complete=true
exit=0

$ umbral show /tmp/umbral-check-02/subject d.txt
derived
    contract=umbral-output/1
derived
    observation=3:d.txt
    hash=6fefa7c34afd
    stability=stable
    metadata=fresh
    content=fresh
observed
    observation=3:d.txt
    kind=file
    size=8
    mtime=2026-10-03T07:25:19.332Z
    ctime=2026-10-03T07:25:21.737Z
exit=0

$ umbral show /tmp/umbral-check-02/subject link-one
derived
    contract=umbral-output/1
derived
    observation=1:link-one
    hash=7fde9f117e69
    stability=stable
    metadata=fresh
    content=fresh
observed
    observation=1:link-one
    kind=file
    size=7
    mtime=2026-10-03T07:25:19.332Z
    ctime=2026-10-03T07:25:19.332Z
derived
    observation=2:link-one
    hash=7fde9f117e69
    stability=stable
    metadata=fresh
    content=reused
    content-source=1:link-one
observed
    observation=2:link-one
    kind=file
    size=7
    mtime=2026-10-03T07:25:19.332Z
    ctime=2026-10-03T07:25:19.332Z
derived
    observation=3:link-one
    hash=7fde9f117e69
    stability=stable
    metadata=fresh
    content=fresh
observed
    observation=3:link-one
    kind=file
    size=7
    mtime=2026-10-03T07:25:19.332Z
    ctime=2026-10-03T07:25:21.737Z
derived
    unchanged
    path=link-one
    reference=1:link-one
    reference-fields=dev,ino,kind,size,mtime,hash
    compared=2:link-one
    compared-fields=dev,ino,kind,size,mtime,hash
    content-changed=false
    reference-complete=true
    compared-complete=false
derived
    unchanged
    path=link-one
    reference=2:link-one
    reference-fields=dev,ino,kind,size,mtime,hash
    compared=3:link-one
    compared-fields=dev,ino,kind,size,mtime,hash
    content-changed=false
    reference-complete=false
    compared-complete=true
exit=0

$ umbral show /tmp/umbral-check-02/subject locked
derived
    contract=umbral-output/1
derived
    observation=1:locked
    hash=none
    stability=none
    metadata=fresh
observed
    observation=1:locked
    kind=dir
    size=60
    mtime=2026-10-03T07:25:19.332Z
    ctime=2026-10-03T07:25:19.332Z
derived
    observation=2:locked
    hash=none
    stability=none
    metadata=fresh
observed
    observation=2:locked
    kind=dir
    size=60
    mtime=2026-10-03T07:25:19.332Z
    ctime=2026-10-03T07:25:20.535Z
unknown
    observation=2:locked
    traversal=not-descended
    observation-error=IO error for operation on /tmp/umbral-check-02/subject/locked: Permission denied (os error 13)
derived
    observation=3:locked
    hash=none
    stability=none
    metadata=fresh
observed
    observation=3:locked
    kind=dir
    size=60
    mtime=2026-10-03T07:25:19.332Z
    ctime=2026-10-03T07:25:21.737Z
derived
    unchanged
    path=locked
    reference=1:locked
    reference-fields=dev,ino,kind
    compared=2:locked
    compared-fields=dev,ino,kind
    reference-complete=true
    compared-complete=false
derived
    unchanged
    path=locked
    reference=2:locked
    reference-fields=dev,ino,kind
    compared=3:locked
    compared-fields=dev,ino,kind
    reference-complete=false
    compared-complete=true
exit=0

$ umbral show /tmp/umbral-check-02/subject line\x0Abreak.txt
derived
    contract=umbral-output/1
derived
    observation=1:line\x0Abreak.txt
    observation-encoding=escaped:control-character
    hash=0a61c6a450a2
    stability=stable
    metadata=fresh
    content=fresh
observed
    observation=1:line\x0Abreak.txt
    observation-encoding=escaped:control-character
    kind=file
    size=7
    mtime=2026-10-03T07:25:19.332Z
    ctime=2026-10-03T07:25:19.332Z
derived
    observation=2:line\x0Abreak.txt
    observation-encoding=escaped:control-character
    hash=0a61c6a450a2
    stability=stable
    metadata=fresh
    content=reused
    content-source=1:line\x0Abreak.txt
    content-source-encoding=escaped:control-character
observed
    observation=2:line\x0Abreak.txt
    observation-encoding=escaped:control-character
    kind=file
    size=7
    mtime=2026-10-03T07:25:19.332Z
    ctime=2026-10-03T07:25:19.332Z
derived
    observation=3:line\x0Abreak.txt
    observation-encoding=escaped:control-character
    hash=0a61c6a450a2
    stability=stable
    metadata=fresh
    content=reused
    content-source=1:line\x0Abreak.txt
    content-source-encoding=escaped:control-character
observed
    observation=3:line\x0Abreak.txt
    observation-encoding=escaped:control-character
    kind=file
    size=7
    mtime=2026-10-03T07:25:19.332Z
    ctime=2026-10-03T07:25:19.332Z
derived
    unchanged
    path=line\x0Abreak.txt
    path-encoding=escaped:control-character
    reference=1:line\x0Abreak.txt
    reference-encoding=escaped:control-character
    reference-fields=dev,ino,kind,size,mtime,hash
    compared=2:line\x0Abreak.txt
    compared-encoding=escaped:control-character
    compared-fields=dev,ino,kind,size,mtime,hash
    content-changed=false
    reference-complete=true
    compared-complete=false
derived
    unchanged
    path=line\x0Abreak.txt
    path-encoding=escaped:control-character
    reference=2:line\x0Abreak.txt
    reference-encoding=escaped:control-character
    reference-fields=dev,ino,kind,size,mtime,hash
    compared=3:line\x0Abreak.txt
    compared-encoding=escaped:control-character
    compared-fields=dev,ino,kind,size,mtime,hash
    content-changed=false
    reference-complete=false
    compared-complete=true
exit=0

$ umbral show /tmp/umbral-check-02/subject bytes-\xFF.txt
derived
    contract=umbral-output/1
derived
    observation=1:bytes-\xFF.txt
    observation-encoding=escaped:not-valid-utf8
    hash=bac25e1466ab
    stability=stable
    metadata=fresh
    content=fresh
observed
    observation=1:bytes-\xFF.txt
    observation-encoding=escaped:not-valid-utf8
    kind=file
    size=7
    mtime=2026-10-03T07:25:19.332Z
    ctime=2026-10-03T07:25:19.332Z
derived
    observation=2:bytes-\xFF.txt
    observation-encoding=escaped:not-valid-utf8
    hash=bac25e1466ab
    stability=stable
    metadata=fresh
    content=reused
    content-source=1:bytes-\xFF.txt
    content-source-encoding=escaped:not-valid-utf8
observed
    observation=2:bytes-\xFF.txt
    observation-encoding=escaped:not-valid-utf8
    kind=file
    size=7
    mtime=2026-10-03T07:25:19.332Z
    ctime=2026-10-03T07:25:19.332Z
derived
    observation=3:bytes-\xFF.txt
    observation-encoding=escaped:not-valid-utf8
    hash=bac25e1466ab
    stability=stable
    metadata=fresh
    content=reused
    content-source=1:bytes-\xFF.txt
    content-source-encoding=escaped:not-valid-utf8
observed
    observation=3:bytes-\xFF.txt
    observation-encoding=escaped:not-valid-utf8
    kind=file
    size=7
    mtime=2026-10-03T07:25:19.332Z
    ctime=2026-10-03T07:25:19.332Z
derived
    unchanged
    path=bytes-\xFF.txt
    path-encoding=escaped:not-valid-utf8
    reference=1:bytes-\xFF.txt
    reference-encoding=escaped:not-valid-utf8
    reference-fields=dev,ino,kind,size,mtime,hash
    compared=2:bytes-\xFF.txt
    compared-encoding=escaped:not-valid-utf8
    compared-fields=dev,ino,kind,size,mtime,hash
    content-changed=false
    reference-complete=true
    compared-complete=false
derived
    unchanged
    path=bytes-\xFF.txt
    path-encoding=escaped:not-valid-utf8
    reference=2:bytes-\xFF.txt
    reference-encoding=escaped:not-valid-utf8
    reference-fields=dev,ino,kind,size,mtime,hash
    compared=3:bytes-\xFF.txt
    compared-encoding=escaped:not-valid-utf8
    compared-fields=dev,ino,kind,size,mtime,hash
    content-changed=false
    reference-complete=false
    compared-complete=true
exit=0
```

### Transformation 5-serialization: converted to JSON per the contract (the round trip back to text was checked to be byte-identical)

```
$ umbral init /tmp/umbral-check-02/subject
[
 {
  "label": "derived",
  "items": [
   "contract=umbral-output/1"
  ]
 },
 {
  "label": "observed",
  "items": [
   "canonical=/tmp/umbral-check-02/subject"
  ]
 },
 {
  "label": "derived",
  "items": [
   "root=/tmp/umbral-check-02/subject"
  ]
 },
 {
  "label": "derived",
  "items": [
   "workspace-id=242c37d665addf11"
  ]
 },
 {
  "label": "derived",
  "items": [
   "state-dir=/tmp/umbral-check-02/data/umbral/ws-242c37d665addf11"
  ]
 },
 {
  "label": "derived",
  "items": [
   "initialised=true"
  ]
 }
]
exit=0

$ umbral observe /tmp/umbral-check-02/subject
[
 {
  "label": "derived",
  "items": [
   "contract=umbral-output/1"
  ]
 },
 {
  "label": "observed",
  "items": [
   "canonical=/tmp/umbral-check-02/subject"
  ]
 },
 {
  "label": "derived",
  "items": [
   "run=1",
   "root=/tmp/umbral-check-02/subject"
  ]
 },
 {
  "label": "derived",
  "items": [
   "run=1",
   "entries=14",
   "files=13",
   "dirs=1",
   "symlinks=0",
   "other=0",
   "kind-unknown=0"
  ]
 },
 {
  "label": "derived",
  "items": [
   "run=1",
   "metadata-fresh=14",
   "metadata-failed=0"
  ]
 },
 {
  "label": "derived",
  "items": [
   "run=1",
   "content-fresh=13",
   "content-reused=0",
   "content-failed=0",
   "content-not-attempted=0",
   "content-not-recorded=0"
  ]
 },
 {
  "label": "derived",
  "items": [
   "run=1",
   "content-failed-diagnostics",
   "unstable-observation=0",
   "not-found=0",
   "permission-denied=0",
   "not-a-regular-file=0",
   "read-error=0"
  ]
 },
 {
  "label": "derived",
  "items": [
   "run=1",
   "traversal-complete=true",
   "traversal-not-descended=0",
   "traversal-metadata-failed=0",
   "traversal-not-recorded=0",
   "root-not-descended=false"
  ]
 },
 {
  "label": "derived",
  "items": [
   "run=1",
   "tool-version=0.1.0",
   "scope=recursive,symlinks-not-followed,no-exclusions"
  ]
 },
 {
  "label": "derived",
  "items": [
   "run=1",
   "content-read-entries=13",
   "content-read-bytes=90"
  ]
 },
 {
  "label": "derived",
  "items": [
   "run=1",
   "started=2026-10-03T07:25:19.335Z",
   "finished=2026-10-03T07:25:19.335Z"
  ]
 },
 {
  "label": "derived",
  "items": [
   "run=1",
   "complete=true"
  ]
 }
]
exit=0

# a.txt edited; locked/ made unreadable (mode 000)

$ umbral observe /tmp/umbral-check-02/subject
[
 {
  "label": "derived",
  "items": [
   "contract=umbral-output/1"
  ]
 },
 {
  "label": "observed",
  "items": [
   "canonical=/tmp/umbral-check-02/subject"
  ]
 },
 {
  "label": "derived",
  "items": [
   "run=2",
   "root=/tmp/umbral-check-02/subject"
  ]
 },
 {
  "label": "derived",
  "items": [
   "run=2",
   "entries=13",
   "files=12",
   "dirs=1",
   "symlinks=0",
   "other=0",
   "kind-unknown=0"
  ]
 },
 {
  "label": "derived",
  "items": [
   "run=2",
   "metadata-fresh=13",
   "metadata-failed=0"
  ]
 },
 {
  "label": "derived",
  "items": [
   "run=2",
   "content-fresh=1",
   "content-reused=11",
   "content-failed=0",
   "content-not-attempted=0",
   "content-not-recorded=0"
  ]
 },
 {
  "label": "derived",
  "items": [
   "run=2",
   "content-failed-diagnostics",
   "unstable-observation=0",
   "not-found=0",
   "permission-denied=0",
   "not-a-regular-file=0",
   "read-error=0"
  ]
 },
 {
  "label": "derived",
  "items": [
   "run=2",
   "traversal-complete=false",
   "traversal-not-descended=1",
   "traversal-metadata-failed=0",
   "traversal-not-recorded=0",
   "root-not-descended=false"
  ]
 },
 {
  "label": "derived",
  "items": [
   "run=2",
   "tool-version=0.1.0",
   "scope=recursive,symlinks-not-followed,no-exclusions"
  ]
 },
 {
  "label": "derived",
  "items": [
   "run=2",
   "content-read-entries=1",
   "content-read-bytes=14"
  ]
 },
 {
  "label": "derived",
  "items": [
   "run=2",
   "started=2026-10-03T07:25:20.537Z",
   "finished=2026-10-03T07:25:20.537Z"
  ]
 },
 {
  "label": "derived",
  "items": [
   "run=2",
   "complete=false"
  ]
 }
]
exit=3

# locked/ readable again; c.txt renamed to d.txt; link-two deleted

$ umbral observe /tmp/umbral-check-02/subject
[
 {
  "label": "derived",
  "items": [
   "contract=umbral-output/1"
  ]
 },
 {
  "label": "observed",
  "items": [
   "canonical=/tmp/umbral-check-02/subject"
  ]
 },
 {
  "label": "derived",
  "items": [
   "run=3",
   "root=/tmp/umbral-check-02/subject"
  ]
 },
 {
  "label": "derived",
  "items": [
   "run=3",
   "entries=13",
   "files=12",
   "dirs=1",
   "symlinks=0",
   "other=0",
   "kind-unknown=0"
  ]
 },
 {
  "label": "derived",
  "items": [
   "run=3",
   "metadata-fresh=13",
   "metadata-failed=0"
  ]
 },
 {
  "label": "derived",
  "items": [
   "run=3",
   "content-fresh=3",
   "content-reused=9",
   "content-failed=0",
   "content-not-attempted=0",
   "content-not-recorded=0"
  ]
 },
 {
  "label": "derived",
  "items": [
   "run=3",
   "content-failed-diagnostics",
   "unstable-observation=0",
   "not-found=0",
   "permission-denied=0",
   "not-a-regular-file=0",
   "read-error=0"
  ]
 },
 {
  "label": "derived",
  "items": [
   "run=3",
   "traversal-complete=true",
   "traversal-not-descended=0",
   "traversal-metadata-failed=0",
   "traversal-not-recorded=0",
   "root-not-descended=false"
  ]
 },
 {
  "label": "derived",
  "items": [
   "run=3",
   "tool-version=0.1.0",
   "scope=recursive,symlinks-not-followed,no-exclusions"
  ]
 },
 {
  "label": "derived",
  "items": [
   "run=3",
   "content-read-entries=3",
   "content-read-bytes=22"
  ]
 },
 {
  "label": "derived",
  "items": [
   "run=3",
   "started=2026-10-03T07:25:21.739Z",
   "finished=2026-10-03T07:25:21.739Z"
  ]
 },
 {
  "label": "derived",
  "items": [
   "run=3",
   "complete=true"
  ]
 }
]
exit=0

$ umbral status /tmp/umbral-check-02/subject
[
 {
  "label": "derived",
  "items": [
   "contract=umbral-output/1"
  ]
 },
 {
  "label": "observed",
  "items": [
   "canonical=/tmp/umbral-check-02/subject"
  ]
 },
 {
  "label": "derived",
  "items": [
   "root=/tmp/umbral-check-02/subject"
  ]
 },
 {
  "label": "derived",
  "items": [
   "workspace-id=242c37d665addf11"
  ]
 },
 {
  "label": "derived",
  "items": [
   "last-run=3",
   "started=2026-10-03T07:25:21.739Z"
  ]
 },
 {
  "label": "derived",
  "items": [
   "last-run=3",
   "complete=true"
  ]
 },
 {
  "label": "derived",
  "items": [
   "run=3",
   "entries=13",
   "files=12",
   "dirs=1",
   "symlinks=0",
   "other=0",
   "kind-unknown=0"
  ]
 },
 {
  "label": "derived",
  "items": [
   "run=3",
   "metadata-fresh=13",
   "metadata-failed=0"
  ]
 },
 {
  "label": "derived",
  "items": [
   "run=3",
   "content-fresh=3",
   "content-reused=9",
   "content-failed=0",
   "content-not-attempted=0",
   "content-not-recorded=0"
  ]
 },
 {
  "label": "derived",
  "items": [
   "run=3",
   "content-failed-diagnostics",
   "unstable-observation=0",
   "not-found=0",
   "permission-denied=0",
   "not-a-regular-file=0",
   "read-error=0"
  ]
 },
 {
  "label": "derived",
  "items": [
   "run=3",
   "traversal-complete=true",
   "traversal-not-descended=0",
   "traversal-metadata-failed=0",
   "traversal-not-recorded=0",
   "root-not-descended=false"
  ]
 },
 {
  "label": "derived",
  "items": [
   "run=3",
   "tool-version=0.1.0",
   "scope=recursive,symlinks-not-followed,no-exclusions"
  ]
 },
 {
  "label": "derived",
  "items": [
   "log-runs=3",
   "log-observations=40"
  ]
 }
]
exit=0

$ umbral changes /tmp/umbral-check-02/subject
[
 {
  "label": "derived",
  "items": [
   "contract=umbral-output/1"
  ]
 },
 {
  "label": "derived",
  "items": [
   "compared",
   "reference-run=2",
   "compared-run=3",
   "reference-complete=false",
   "compared-complete=true"
  ]
 },
 {
  "label": "derived",
  "items": [
   "count",
   "unchanged=11"
  ]
 },
 {
  "label": "derived",
  "items": [
   "count",
   "modified=0"
  ]
 },
 {
  "label": "derived",
  "items": [
   "count",
   "created=1"
  ]
 },
 {
  "label": "derived",
  "items": [
   "count",
   "deleted=1"
  ]
 },
 {
  "label": "derived",
  "items": [
   "count",
   "unobserved=0"
  ]
 },
 {
  "label": "derived",
  "items": [
   "count",
   "renamed-or-moved=1"
  ]
 },
 {
  "label": "derived",
  "items": [
   "count",
   "recreated=0"
  ]
 },
 {
  "label": "derived",
  "items": [
   "count",
   "ambiguous=0"
  ]
 },
 {
  "label": "derived",
  "items": [
   "renamed-or-moved",
   "path=d.txt",
   "reference=2:c.txt",
   "reference-fields=dev,ino",
   "compared=3:d.txt",
   "compared-fields=dev,ino",
   "reference-complete=false",
   "compared-complete=true"
  ]
 },
 {
  "label": "derived",
  "items": [
   "deleted",
   "path=link-two",
   "reference=2:link-two",
   "reference-fields=dev,ino",
   "compared-absent=3",
   "counterpart=3:link-one",
   "counterpart-fields=dev,ino",
   "object-survives=true",
   "reference-complete=false",
   "compared-complete=true"
  ]
 },
 {
  "label": "derived",
  "items": [
   "created",
   "path=locked/inside.txt",
   "reference-absent=2",
   "compared=3:locked/inside.txt",
   "compared-fields=dev,ino",
   "reference-complete=false",
   "compared-complete=true"
  ]
 }
]
exit=0

$ umbral show /tmp/umbral-check-02/subject a.txt
[
 {
  "label": "derived",
  "items": [
   "contract=umbral-output/1"
  ]
 },
 {
  "label": "derived",
  "items": [
   "observation=1:a.txt",
   "hash=ac678d92b3d7",
   "stability=stable",
   "metadata=fresh",
   "content=fresh"
  ]
 },
 {
  "label": "observed",
  "items": [
   "observation=1:a.txt",
   "kind=file",
   "size=6",
   "mtime=2026-10-03T07:25:19.332Z",
   "ctime=2026-10-03T07:25:19.332Z"
  ]
 },
 {
  "label": "derived",
  "items": [
   "observation=2:a.txt",
   "hash=29a077efa2d8",
   "stability=stable",
   "metadata=fresh",
   "content=fresh"
  ]
 },
 {
  "label": "observed",
  "items": [
   "observation=2:a.txt",
   "kind=file",
   "size=14",
   "mtime=2026-10-03T07:25:20.535Z",
   "ctime=2026-10-03T07:25:20.535Z"
  ]
 },
 {
  "label": "derived",
  "items": [
   "observation=3:a.txt",
   "hash=29a077efa2d8",
   "stability=stable",
   "metadata=fresh",
   "content=reused",
   "content-source=2:a.txt"
  ]
 },
 {
  "label": "observed",
  "items": [
   "observation=3:a.txt",
   "kind=file",
   "size=14",
   "mtime=2026-10-03T07:25:20.535Z",
   "ctime=2026-10-03T07:25:20.535Z"
  ]
 },
 {
  "label": "derived",
  "items": [
   "modified",
   "path=a.txt",
   "reference=1:a.txt",
   "reference-fields=dev,ino,kind,size,mtime,hash",
   "compared=2:a.txt",
   "compared-fields=dev,ino,kind,size,mtime,hash",
   "content-changed=true",
   "reference-complete=true",
   "compared-complete=false"
  ]
 },
 {
  "label": "derived",
  "items": [
   "unchanged",
   "path=a.txt",
   "reference=2:a.txt",
   "reference-fields=dev,ino,kind,size,mtime,hash",
   "compared=3:a.txt",
   "compared-fields=dev,ino,kind,size,mtime,hash",
   "content-changed=false",
   "reference-complete=false",
   "compared-complete=true"
  ]
 }
]
exit=0

$ umbral show /tmp/umbral-check-02/subject b.txt
[
 {
  "label": "derived",
  "items": [
   "contract=umbral-output/1"
  ]
 },
 {
  "label": "derived",
  "items": [
   "observation=1:b.txt",
   "hash=2001794aa22d",
   "stability=stable",
   "metadata=fresh",
   "content=fresh"
  ]
 },
 {
  "label": "observed",
  "items": [
   "observation=1:b.txt",
   "kind=file",
   "size=6",
   "mtime=2026-10-03T07:25:19.332Z",
   "ctime=2026-10-03T07:25:19.332Z"
  ]
 },
 {
  "label": "derived",
  "items": [
   "observation=2:b.txt",
   "hash=2001794aa22d",
   "stability=stable",
   "metadata=fresh",
   "content=reused",
   "content-source=1:b.txt"
  ]
 },
 {
  "label": "observed",
  "items": [
   "observation=2:b.txt",
   "kind=file",
   "size=6",
   "mtime=2026-10-03T07:25:19.332Z",
   "ctime=2026-10-03T07:25:19.332Z"
  ]
 },
 {
  "label": "derived",
  "items": [
   "observation=3:b.txt",
   "hash=2001794aa22d",
   "stability=stable",
   "metadata=fresh",
   "content=reused",
   "content-source=1:b.txt"
  ]
 },
 {
  "label": "observed",
  "items": [
   "observation=3:b.txt",
   "kind=file",
   "size=6",
   "mtime=2026-10-03T07:25:19.332Z",
   "ctime=2026-10-03T07:25:19.332Z"
  ]
 },
 {
  "label": "derived",
  "items": [
   "unchanged",
   "path=b.txt",
   "reference=1:b.txt",
   "reference-fields=dev,ino,kind,size,mtime,hash",
   "compared=2:b.txt",
   "compared-fields=dev,ino,kind,size,mtime,hash",
   "content-changed=false",
   "reference-complete=true",
   "compared-complete=false"
  ]
 },
 {
  "label": "derived",
  "items": [
   "unchanged",
   "path=b.txt",
   "reference=2:b.txt",
   "reference-fields=dev,ino,kind,size,mtime,hash",
   "compared=3:b.txt",
   "compared-fields=dev,ino,kind,size,mtime,hash",
   "content-changed=false",
   "reference-complete=false",
   "compared-complete=true"
  ]
 }
]
exit=0

$ umbral show /tmp/umbral-check-02/subject d.txt
[
 {
  "label": "derived",
  "items": [
   "contract=umbral-output/1"
  ]
 },
 {
  "label": "derived",
  "items": [
   "observation=3:d.txt",
   "hash=6fefa7c34afd",
   "stability=stable",
   "metadata=fresh",
   "content=fresh"
  ]
 },
 {
  "label": "observed",
  "items": [
   "observation=3:d.txt",
   "kind=file",
   "size=8",
   "mtime=2026-10-03T07:25:19.332Z",
   "ctime=2026-10-03T07:25:21.737Z"
  ]
 }
]
exit=0

$ umbral show /tmp/umbral-check-02/subject link-one
[
 {
  "label": "derived",
  "items": [
   "contract=umbral-output/1"
  ]
 },
 {
  "label": "derived",
  "items": [
   "observation=1:link-one",
   "hash=7fde9f117e69",
   "stability=stable",
   "metadata=fresh",
   "content=fresh"
  ]
 },
 {
  "label": "observed",
  "items": [
   "observation=1:link-one",
   "kind=file",
   "size=7",
   "mtime=2026-10-03T07:25:19.332Z",
   "ctime=2026-10-03T07:25:19.332Z"
  ]
 },
 {
  "label": "derived",
  "items": [
   "observation=2:link-one",
   "hash=7fde9f117e69",
   "stability=stable",
   "metadata=fresh",
   "content=reused",
   "content-source=1:link-one"
  ]
 },
 {
  "label": "observed",
  "items": [
   "observation=2:link-one",
   "kind=file",
   "size=7",
   "mtime=2026-10-03T07:25:19.332Z",
   "ctime=2026-10-03T07:25:19.332Z"
  ]
 },
 {
  "label": "derived",
  "items": [
   "observation=3:link-one",
   "hash=7fde9f117e69",
   "stability=stable",
   "metadata=fresh",
   "content=fresh"
  ]
 },
 {
  "label": "observed",
  "items": [
   "observation=3:link-one",
   "kind=file",
   "size=7",
   "mtime=2026-10-03T07:25:19.332Z",
   "ctime=2026-10-03T07:25:21.737Z"
  ]
 },
 {
  "label": "derived",
  "items": [
   "unchanged",
   "path=link-one",
   "reference=1:link-one",
   "reference-fields=dev,ino,kind,size,mtime,hash",
   "compared=2:link-one",
   "compared-fields=dev,ino,kind,size,mtime,hash",
   "content-changed=false",
   "reference-complete=true",
   "compared-complete=false"
  ]
 },
 {
  "label": "derived",
  "items": [
   "unchanged",
   "path=link-one",
   "reference=2:link-one",
   "reference-fields=dev,ino,kind,size,mtime,hash",
   "compared=3:link-one",
   "compared-fields=dev,ino,kind,size,mtime,hash",
   "content-changed=false",
   "reference-complete=false",
   "compared-complete=true"
  ]
 }
]
exit=0

$ umbral show /tmp/umbral-check-02/subject locked
[
 {
  "label": "derived",
  "items": [
   "contract=umbral-output/1"
  ]
 },
 {
  "label": "derived",
  "items": [
   "observation=1:locked",
   "hash=none",
   "stability=none",
   "metadata=fresh"
  ]
 },
 {
  "label": "observed",
  "items": [
   "observation=1:locked",
   "kind=dir",
   "size=60",
   "mtime=2026-10-03T07:25:19.332Z",
   "ctime=2026-10-03T07:25:19.332Z"
  ]
 },
 {
  "label": "derived",
  "items": [
   "observation=2:locked",
   "hash=none",
   "stability=none",
   "metadata=fresh"
  ]
 },
 {
  "label": "observed",
  "items": [
   "observation=2:locked",
   "kind=dir",
   "size=60",
   "mtime=2026-10-03T07:25:19.332Z",
   "ctime=2026-10-03T07:25:20.535Z"
  ]
 },
 {
  "label": "unknown",
  "items": [
   "observation=2:locked",
   "traversal=not-descended",
   "observation-error=IO error for operation on /tmp/umbral-check-02/subject/locked: Permission denied (os error 13)"
  ]
 },
 {
  "label": "derived",
  "items": [
   "observation=3:locked",
   "hash=none",
   "stability=none",
   "metadata=fresh"
  ]
 },
 {
  "label": "observed",
  "items": [
   "observation=3:locked",
   "kind=dir",
   "size=60",
   "mtime=2026-10-03T07:25:19.332Z",
   "ctime=2026-10-03T07:25:21.737Z"
  ]
 },
 {
  "label": "derived",
  "items": [
   "unchanged",
   "path=locked",
   "reference=1:locked",
   "reference-fields=dev,ino,kind",
   "compared=2:locked",
   "compared-fields=dev,ino,kind",
   "reference-complete=true",
   "compared-complete=false"
  ]
 },
 {
  "label": "derived",
  "items": [
   "unchanged",
   "path=locked",
   "reference=2:locked",
   "reference-fields=dev,ino,kind",
   "compared=3:locked",
   "compared-fields=dev,ino,kind",
   "reference-complete=false",
   "compared-complete=true"
  ]
 }
]
exit=0

$ umbral show /tmp/umbral-check-02/subject line\x0Abreak.txt
[
 {
  "label": "derived",
  "items": [
   "contract=umbral-output/1"
  ]
 },
 {
  "label": "derived",
  "items": [
   "observation=1:line\\x0Abreak.txt",
   "observation-encoding=escaped:control-character",
   "hash=0a61c6a450a2",
   "stability=stable",
   "metadata=fresh",
   "content=fresh"
  ]
 },
 {
  "label": "observed",
  "items": [
   "observation=1:line\\x0Abreak.txt",
   "observation-encoding=escaped:control-character",
   "kind=file",
   "size=7",
   "mtime=2026-10-03T07:25:19.332Z",
   "ctime=2026-10-03T07:25:19.332Z"
  ]
 },
 {
  "label": "derived",
  "items": [
   "observation=2:line\\x0Abreak.txt",
   "observation-encoding=escaped:control-character",
   "hash=0a61c6a450a2",
   "stability=stable",
   "metadata=fresh",
   "content=reused",
   "content-source=1:line\\x0Abreak.txt",
   "content-source-encoding=escaped:control-character"
  ]
 },
 {
  "label": "observed",
  "items": [
   "observation=2:line\\x0Abreak.txt",
   "observation-encoding=escaped:control-character",
   "kind=file",
   "size=7",
   "mtime=2026-10-03T07:25:19.332Z",
   "ctime=2026-10-03T07:25:19.332Z"
  ]
 },
 {
  "label": "derived",
  "items": [
   "observation=3:line\\x0Abreak.txt",
   "observation-encoding=escaped:control-character",
   "hash=0a61c6a450a2",
   "stability=stable",
   "metadata=fresh",
   "content=reused",
   "content-source=1:line\\x0Abreak.txt",
   "content-source-encoding=escaped:control-character"
  ]
 },
 {
  "label": "observed",
  "items": [
   "observation=3:line\\x0Abreak.txt",
   "observation-encoding=escaped:control-character",
   "kind=file",
   "size=7",
   "mtime=2026-10-03T07:25:19.332Z",
   "ctime=2026-10-03T07:25:19.332Z"
  ]
 },
 {
  "label": "derived",
  "items": [
   "unchanged",
   "path=line\\x0Abreak.txt",
   "path-encoding=escaped:control-character",
   "reference=1:line\\x0Abreak.txt",
   "reference-encoding=escaped:control-character",
   "reference-fields=dev,ino,kind,size,mtime,hash",
   "compared=2:line\\x0Abreak.txt",
   "compared-encoding=escaped:control-character",
   "compared-fields=dev,ino,kind,size,mtime,hash",
   "content-changed=false",
   "reference-complete=true",
   "compared-complete=false"
  ]
 },
 {
  "label": "derived",
  "items": [
   "unchanged",
   "path=line\\x0Abreak.txt",
   "path-encoding=escaped:control-character",
   "reference=2:line\\x0Abreak.txt",
   "reference-encoding=escaped:control-character",
   "reference-fields=dev,ino,kind,size,mtime,hash",
   "compared=3:line\\x0Abreak.txt",
   "compared-encoding=escaped:control-character",
   "compared-fields=dev,ino,kind,size,mtime,hash",
   "content-changed=false",
   "reference-complete=false",
   "compared-complete=true"
  ]
 }
]
exit=0

$ umbral show /tmp/umbral-check-02/subject bytes-\xFF.txt
[
 {
  "label": "derived",
  "items": [
   "contract=umbral-output/1"
  ]
 },
 {
  "label": "derived",
  "items": [
   "observation=1:bytes-\\xFF.txt",
   "observation-encoding=escaped:not-valid-utf8",
   "hash=bac25e1466ab",
   "stability=stable",
   "metadata=fresh",
   "content=fresh"
  ]
 },
 {
  "label": "observed",
  "items": [
   "observation=1:bytes-\\xFF.txt",
   "observation-encoding=escaped:not-valid-utf8",
   "kind=file",
   "size=7",
   "mtime=2026-10-03T07:25:19.332Z",
   "ctime=2026-10-03T07:25:19.332Z"
  ]
 },
 {
  "label": "derived",
  "items": [
   "observation=2:bytes-\\xFF.txt",
   "observation-encoding=escaped:not-valid-utf8",
   "hash=bac25e1466ab",
   "stability=stable",
   "metadata=fresh",
   "content=reused",
   "content-source=1:bytes-\\xFF.txt",
   "content-source-encoding=escaped:not-valid-utf8"
  ]
 },
 {
  "label": "observed",
  "items": [
   "observation=2:bytes-\\xFF.txt",
   "observation-encoding=escaped:not-valid-utf8",
   "kind=file",
   "size=7",
   "mtime=2026-10-03T07:25:19.332Z",
   "ctime=2026-10-03T07:25:19.332Z"
  ]
 },
 {
  "label": "derived",
  "items": [
   "observation=3:bytes-\\xFF.txt",
   "observation-encoding=escaped:not-valid-utf8",
   "hash=bac25e1466ab",
   "stability=stable",
   "metadata=fresh",
   "content=reused",
   "content-source=1:bytes-\\xFF.txt",
   "content-source-encoding=escaped:not-valid-utf8"
  ]
 },
 {
  "label": "observed",
  "items": [
   "observation=3:bytes-\\xFF.txt",
   "observation-encoding=escaped:not-valid-utf8",
   "kind=file",
   "size=7",
   "mtime=2026-10-03T07:25:19.332Z",
   "ctime=2026-10-03T07:25:19.332Z"
  ]
 },
 {
  "label": "derived",
  "items": [
   "unchanged",
   "path=bytes-\\xFF.txt",
   "path-encoding=escaped:not-valid-utf8",
   "reference=1:bytes-\\xFF.txt",
   "reference-encoding=escaped:not-valid-utf8",
   "reference-fields=dev,ino,kind,size,mtime,hash",
   "compared=2:bytes-\\xFF.txt",
   "compared-encoding=escaped:not-valid-utf8",
   "compared-fields=dev,ino,kind,size,mtime,hash",
   "content-changed=false",
   "reference-complete=true",
   "compared-complete=false"
  ]
 },
 {
  "label": "derived",
  "items": [
   "unchanged",
   "path=bytes-\\xFF.txt",
   "path-encoding=escaped:not-valid-utf8",
   "reference=2:bytes-\\xFF.txt",
   "reference-encoding=escaped:not-valid-utf8",
   "reference-fields=dev,ino,kind,size,mtime,hash",
   "compared=3:bytes-\\xFF.txt",
   "compared-encoding=escaped:not-valid-utf8",
   "compared-fields=dev,ino,kind,size,mtime,hash",
   "content-changed=false",
   "reference-complete=false",
   "compared-complete=true"
  ]
 }
]
exit=0
```

### Transformation 6-position-change: every reference field moved to the end of its line

```
$ umbral init /tmp/umbral-check-02/subject
derived   contract=umbral-output/1
observed  canonical=/tmp/umbral-check-02/subject
derived   root=/tmp/umbral-check-02/subject
derived   workspace-id=242c37d665addf11
derived   state-dir=/tmp/umbral-check-02/data/umbral/ws-242c37d665addf11
derived   initialised=true
exit=0

$ umbral observe /tmp/umbral-check-02/subject
derived   contract=umbral-output/1
observed  canonical=/tmp/umbral-check-02/subject
derived   run=1  root=/tmp/umbral-check-02/subject
derived   run=1  entries=14  files=13  dirs=1  symlinks=0  other=0  kind-unknown=0
derived   run=1  metadata-fresh=14  metadata-failed=0
derived   run=1  content-fresh=13  content-reused=0  content-failed=0  content-not-attempted=0  content-not-recorded=0
derived   run=1  content-failed-diagnostics  unstable-observation=0  not-found=0  permission-denied=0  not-a-regular-file=0  read-error=0
derived   run=1  traversal-complete=true  traversal-not-descended=0  traversal-metadata-failed=0  traversal-not-recorded=0  root-not-descended=false
derived   run=1  tool-version=0.1.0  scope=recursive,symlinks-not-followed,no-exclusions
derived   run=1  content-read-entries=13  content-read-bytes=90
derived   run=1  started=2026-10-03T07:25:19.335Z  finished=2026-10-03T07:25:19.335Z
derived   run=1  complete=true
exit=0

# a.txt edited; locked/ made unreadable (mode 000)

$ umbral observe /tmp/umbral-check-02/subject
derived   contract=umbral-output/1
observed  canonical=/tmp/umbral-check-02/subject
derived   run=2  root=/tmp/umbral-check-02/subject
derived   run=2  entries=13  files=12  dirs=1  symlinks=0  other=0  kind-unknown=0
derived   run=2  metadata-fresh=13  metadata-failed=0
derived   run=2  content-fresh=1  content-reused=11  content-failed=0  content-not-attempted=0  content-not-recorded=0
derived   run=2  content-failed-diagnostics  unstable-observation=0  not-found=0  permission-denied=0  not-a-regular-file=0  read-error=0
derived   run=2  traversal-complete=false  traversal-not-descended=1  traversal-metadata-failed=0  traversal-not-recorded=0  root-not-descended=false
derived   run=2  tool-version=0.1.0  scope=recursive,symlinks-not-followed,no-exclusions
derived   run=2  content-read-entries=1  content-read-bytes=14
derived   run=2  started=2026-10-03T07:25:20.537Z  finished=2026-10-03T07:25:20.537Z
derived   run=2  complete=false
exit=3

# locked/ readable again; c.txt renamed to d.txt; link-two deleted

$ umbral observe /tmp/umbral-check-02/subject
derived   contract=umbral-output/1
observed  canonical=/tmp/umbral-check-02/subject
derived   run=3  root=/tmp/umbral-check-02/subject
derived   run=3  entries=13  files=12  dirs=1  symlinks=0  other=0  kind-unknown=0
derived   run=3  metadata-fresh=13  metadata-failed=0
derived   run=3  content-fresh=3  content-reused=9  content-failed=0  content-not-attempted=0  content-not-recorded=0
derived   run=3  content-failed-diagnostics  unstable-observation=0  not-found=0  permission-denied=0  not-a-regular-file=0  read-error=0
derived   run=3  traversal-complete=true  traversal-not-descended=0  traversal-metadata-failed=0  traversal-not-recorded=0  root-not-descended=false
derived   run=3  tool-version=0.1.0  scope=recursive,symlinks-not-followed,no-exclusions
derived   run=3  content-read-entries=3  content-read-bytes=22
derived   run=3  started=2026-10-03T07:25:21.739Z  finished=2026-10-03T07:25:21.739Z
derived   run=3  complete=true
exit=0

$ umbral status /tmp/umbral-check-02/subject
derived   contract=umbral-output/1
observed  canonical=/tmp/umbral-check-02/subject
derived   root=/tmp/umbral-check-02/subject
derived   workspace-id=242c37d665addf11
derived   last-run=3  started=2026-10-03T07:25:21.739Z
derived   last-run=3  complete=true
derived   run=3  entries=13  files=12  dirs=1  symlinks=0  other=0  kind-unknown=0
derived   run=3  metadata-fresh=13  metadata-failed=0
derived   run=3  content-fresh=3  content-reused=9  content-failed=0  content-not-attempted=0  content-not-recorded=0
derived   run=3  content-failed-diagnostics  unstable-observation=0  not-found=0  permission-denied=0  not-a-regular-file=0  read-error=0
derived   run=3  traversal-complete=true  traversal-not-descended=0  traversal-metadata-failed=0  traversal-not-recorded=0  root-not-descended=false
derived   run=3  tool-version=0.1.0  scope=recursive,symlinks-not-followed,no-exclusions
derived   log-runs=3  log-observations=40
exit=0

$ umbral changes /tmp/umbral-check-02/subject
derived   contract=umbral-output/1
derived   compared  reference-run=2  compared-run=3  reference-complete=false  compared-complete=true
derived   count  unchanged=11
derived   count  modified=0
derived   count  created=1
derived   count  deleted=1
derived   count  unobserved=0
derived   count  renamed-or-moved=1
derived   count  recreated=0
derived   count  ambiguous=0
derived   renamed-or-moved  path=d.txt  reference-fields=dev,ino  compared-fields=dev,ino  reference-complete=false  compared-complete=true  reference=2:c.txt  compared=3:d.txt
derived   deleted  path=link-two  reference-fields=dev,ino  compared-absent=3  counterpart-fields=dev,ino  object-survives=true  reference-complete=false  compared-complete=true  reference=2:link-two  counterpart=3:link-one
derived   created  path=locked/inside.txt  reference-absent=2  compared-fields=dev,ino  reference-complete=false  compared-complete=true  compared=3:locked/inside.txt
exit=0

$ umbral show /tmp/umbral-check-02/subject a.txt
derived   contract=umbral-output/1
derived   hash=ac678d92b3d7  stability=stable  metadata=fresh  content=fresh  observation=1:a.txt
observed  kind=file  size=6  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:19.332Z  observation=1:a.txt
derived   hash=29a077efa2d8  stability=stable  metadata=fresh  content=fresh  observation=2:a.txt
observed  kind=file  size=14  mtime=2026-10-03T07:25:20.535Z  ctime=2026-10-03T07:25:20.535Z  observation=2:a.txt
derived   hash=29a077efa2d8  stability=stable  metadata=fresh  content=reused  observation=3:a.txt  content-source=2:a.txt
observed  kind=file  size=14  mtime=2026-10-03T07:25:20.535Z  ctime=2026-10-03T07:25:20.535Z  observation=3:a.txt
derived   modified  path=a.txt  reference-fields=dev,ino,kind,size,mtime,hash  compared-fields=dev,ino,kind,size,mtime,hash  content-changed=true  reference-complete=true  compared-complete=false  reference=1:a.txt  compared=2:a.txt
derived   unchanged  path=a.txt  reference-fields=dev,ino,kind,size,mtime,hash  compared-fields=dev,ino,kind,size,mtime,hash  content-changed=false  reference-complete=false  compared-complete=true  reference=2:a.txt  compared=3:a.txt
exit=0

$ umbral show /tmp/umbral-check-02/subject b.txt
derived   contract=umbral-output/1
derived   hash=2001794aa22d  stability=stable  metadata=fresh  content=fresh  observation=1:b.txt
observed  kind=file  size=6  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:19.332Z  observation=1:b.txt
derived   hash=2001794aa22d  stability=stable  metadata=fresh  content=reused  observation=2:b.txt  content-source=1:b.txt
observed  kind=file  size=6  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:19.332Z  observation=2:b.txt
derived   hash=2001794aa22d  stability=stable  metadata=fresh  content=reused  observation=3:b.txt  content-source=1:b.txt
observed  kind=file  size=6  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:19.332Z  observation=3:b.txt
derived   unchanged  path=b.txt  reference-fields=dev,ino,kind,size,mtime,hash  compared-fields=dev,ino,kind,size,mtime,hash  content-changed=false  reference-complete=true  compared-complete=false  reference=1:b.txt  compared=2:b.txt
derived   unchanged  path=b.txt  reference-fields=dev,ino,kind,size,mtime,hash  compared-fields=dev,ino,kind,size,mtime,hash  content-changed=false  reference-complete=false  compared-complete=true  reference=2:b.txt  compared=3:b.txt
exit=0

$ umbral show /tmp/umbral-check-02/subject d.txt
derived   contract=umbral-output/1
derived   hash=6fefa7c34afd  stability=stable  metadata=fresh  content=fresh  observation=3:d.txt
observed  kind=file  size=8  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:21.737Z  observation=3:d.txt
exit=0

$ umbral show /tmp/umbral-check-02/subject link-one
derived   contract=umbral-output/1
derived   hash=7fde9f117e69  stability=stable  metadata=fresh  content=fresh  observation=1:link-one
observed  kind=file  size=7  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:19.332Z  observation=1:link-one
derived   hash=7fde9f117e69  stability=stable  metadata=fresh  content=reused  observation=2:link-one  content-source=1:link-one
observed  kind=file  size=7  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:19.332Z  observation=2:link-one
derived   hash=7fde9f117e69  stability=stable  metadata=fresh  content=fresh  observation=3:link-one
observed  kind=file  size=7  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:21.737Z  observation=3:link-one
derived   unchanged  path=link-one  reference-fields=dev,ino,kind,size,mtime,hash  compared-fields=dev,ino,kind,size,mtime,hash  content-changed=false  reference-complete=true  compared-complete=false  reference=1:link-one  compared=2:link-one
derived   unchanged  path=link-one  reference-fields=dev,ino,kind,size,mtime,hash  compared-fields=dev,ino,kind,size,mtime,hash  content-changed=false  reference-complete=false  compared-complete=true  reference=2:link-one  compared=3:link-one
exit=0

$ umbral show /tmp/umbral-check-02/subject locked
derived   contract=umbral-output/1
derived   hash=none  stability=none  metadata=fresh  observation=1:locked
observed  kind=dir  size=60  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:19.332Z  observation=1:locked
derived   hash=none  stability=none  metadata=fresh  observation=2:locked
observed  kind=dir  size=60  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:20.535Z  observation=2:locked
unknown   traversal=not-descended  observation-error=IO error for operation on /tmp/umbral-check-02/subject/locked: Permission denied (os error 13)  observation=2:locked
derived   hash=none  stability=none  metadata=fresh  observation=3:locked
observed  kind=dir  size=60  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:21.737Z  observation=3:locked
derived   unchanged  path=locked  reference-fields=dev,ino,kind  compared-fields=dev,ino,kind  reference-complete=true  compared-complete=false  reference=1:locked  compared=2:locked
derived   unchanged  path=locked  reference-fields=dev,ino,kind  compared-fields=dev,ino,kind  reference-complete=false  compared-complete=true  reference=2:locked  compared=3:locked
exit=0

$ umbral show /tmp/umbral-check-02/subject line\x0Abreak.txt
derived   contract=umbral-output/1
derived   hash=0a61c6a450a2  stability=stable  metadata=fresh  content=fresh  observation=1:line\x0Abreak.txt  observation-encoding=escaped:control-character
observed  kind=file  size=7  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:19.332Z  observation=1:line\x0Abreak.txt  observation-encoding=escaped:control-character
derived   hash=0a61c6a450a2  stability=stable  metadata=fresh  content=reused  observation=2:line\x0Abreak.txt  observation-encoding=escaped:control-character  content-source=1:line\x0Abreak.txt  content-source-encoding=escaped:control-character
observed  kind=file  size=7  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:19.332Z  observation=2:line\x0Abreak.txt  observation-encoding=escaped:control-character
derived   hash=0a61c6a450a2  stability=stable  metadata=fresh  content=reused  observation=3:line\x0Abreak.txt  observation-encoding=escaped:control-character  content-source=1:line\x0Abreak.txt  content-source-encoding=escaped:control-character
observed  kind=file  size=7  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:19.332Z  observation=3:line\x0Abreak.txt  observation-encoding=escaped:control-character
derived   unchanged  path=line\x0Abreak.txt  path-encoding=escaped:control-character  reference-fields=dev,ino,kind,size,mtime,hash  compared-fields=dev,ino,kind,size,mtime,hash  content-changed=false  reference-complete=true  compared-complete=false  reference=1:line\x0Abreak.txt  reference-encoding=escaped:control-character  compared=2:line\x0Abreak.txt  compared-encoding=escaped:control-character
derived   unchanged  path=line\x0Abreak.txt  path-encoding=escaped:control-character  reference-fields=dev,ino,kind,size,mtime,hash  compared-fields=dev,ino,kind,size,mtime,hash  content-changed=false  reference-complete=false  compared-complete=true  reference=2:line\x0Abreak.txt  reference-encoding=escaped:control-character  compared=3:line\x0Abreak.txt  compared-encoding=escaped:control-character
exit=0

$ umbral show /tmp/umbral-check-02/subject bytes-\xFF.txt
derived   contract=umbral-output/1
derived   hash=bac25e1466ab  stability=stable  metadata=fresh  content=fresh  observation=1:bytes-\xFF.txt  observation-encoding=escaped:not-valid-utf8
observed  kind=file  size=7  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:19.332Z  observation=1:bytes-\xFF.txt  observation-encoding=escaped:not-valid-utf8
derived   hash=bac25e1466ab  stability=stable  metadata=fresh  content=reused  observation=2:bytes-\xFF.txt  observation-encoding=escaped:not-valid-utf8  content-source=1:bytes-\xFF.txt  content-source-encoding=escaped:not-valid-utf8
observed  kind=file  size=7  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:19.332Z  observation=2:bytes-\xFF.txt  observation-encoding=escaped:not-valid-utf8
derived   hash=bac25e1466ab  stability=stable  metadata=fresh  content=reused  observation=3:bytes-\xFF.txt  observation-encoding=escaped:not-valid-utf8  content-source=1:bytes-\xFF.txt  content-source-encoding=escaped:not-valid-utf8
observed  kind=file  size=7  mtime=2026-10-03T07:25:19.332Z  ctime=2026-10-03T07:25:19.332Z  observation=3:bytes-\xFF.txt  observation-encoding=escaped:not-valid-utf8
derived   unchanged  path=bytes-\xFF.txt  path-encoding=escaped:not-valid-utf8  reference-fields=dev,ino,kind,size,mtime,hash  compared-fields=dev,ino,kind,size,mtime,hash  content-changed=false  reference-complete=true  compared-complete=false  reference=1:bytes-\xFF.txt  reference-encoding=escaped:not-valid-utf8  compared=2:bytes-\xFF.txt  compared-encoding=escaped:not-valid-utf8
derived   unchanged  path=bytes-\xFF.txt  path-encoding=escaped:not-valid-utf8  reference-fields=dev,ino,kind,size,mtime,hash  compared-fields=dev,ino,kind,size,mtime,hash  content-changed=false  reference-complete=false  compared-complete=true  reference=2:bytes-\xFF.txt  reference-encoding=escaped:not-valid-utf8  compared=3:bytes-\xFF.txt  compared-encoding=escaped:not-valid-utf8
exit=0
```

