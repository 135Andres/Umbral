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
derived   modified  path=a\x0Ab\xFF  path-encoding=escaped:control-character,not-valid-utf8  scan-complete=true
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
