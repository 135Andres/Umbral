
---

## HARNESS FINDINGS (classified per mandate §21)

Each finding: symptom / cause / classification / disposition.

### F-1 (class D — harness defect) Invalid operation generated
Symptom: `Append` applied to a path that did not exist; harness aborted.
Cause: the generator emitted ops without checking preconditions.
Disposition: explicit `precondition_holds()` gate added; invalid ops are SKIPPED, never
applied (mandate §2). Not a V0 defect.

### F-2 (class D — harness defect) Atomic-replace temp path built outside the target dir
Symptom: `No such file or directory` on `.d0/c.tmp`.
Cause: temp name constructed as `.{path}.tmp`, putting it under a non-existent `.d0`.
Disposition: temp now created in the target's own directory (same-fs rename semantics).
Not a V0 defect.

### F-3 (class D — harness defect, SECOND OCCURRENCE of the same trap) Directory-rename
prefix matching
Symptom: `unwrap()` on `None` while moving children during `RenameDir`.
Cause: `Path::new("d0/")` has ONE component, so `starts_with` matched the directory
entry itself — the identical trap recorded in increment 1 for `dirlink/`.
Disposition: explicit `k.as_path() != from_p` exclusion. Not a V0 defect.
NOTE: this trap has now cost time twice in the same project. Recorded in the skill as a
standing lesson: never use a trailing-slash string as a `Path::starts_with` prefix.

### F-4 (class D — ORACLE defect) Hard-link content propagation missing
Symptom: `content mismatch for "hl"` after writing through the sibling path `a`.
Cause: the reference model stored content per PATH; writing via `a` changed the object,
so `hl` (same inode) changed too — the oracle did not model aliasing.
Disposition: `propagate_content()` added — an oracle that does not model hard links is
wrong, not conservative. fsp-check's observation was CORRECT here; the oracle was fixed,
not the implementation (mandate rule: never adapt the harness to the implementation —
this was the reverse case, adapting the harness to reality).

### Accepted limitations (class B)
- V0 has no tombstone table; deletions are reported by reconcile, not materialised in the
  projection (documented since increment 4).
- Directory-rename children are reported individually (flagged `under_dir_rename`); no
  collapsing pass (explicitly not optimised, mandate §10).
- Reconciler has no temporal semantics: `reconcile(C, A)` is a reverse set comparison,
  not time travel. Defined for all pairs; ordering is the caller's responsibility.
- Crash tests cover process-kill only (no power loss, no storage corruption).
- Benchmark observation: after recording 2040 observations the WAL (4.1 MB) exceeded the
  database file (324 KB) — expected without a checkpoint; WAL checkpointing is not
  implemented in V0 (class B, revisit if store growth matters).

### F-5 (class B — coverage gap, found during publication preparation)

Symptom: `cargo clippy --all-targets` reports `variant \`DeleteDir\` is never constructed`
and `method \`p_of\` is never used` in `tests/harness_falsification.rs`.

Cause and significance: `Op::DeleteDir` exists as a variant with an apply arm and a
precondition, but **the property-test strategy never generates it**. Directory deletion was
therefore never exercised by the generated sequences, and no other test removes a directory
end-to-end. This also means the increment-6 report's list of generated operations
("delete file / directory") was inaccurate: directory deletion was specified but not
generated.

Disposition: **recorded, not patched.** V0 is frozen; adding the generator case would
change the evidence rather than describe it. The gap is a limitation of V0's falsification
coverage, not a falsified property — nothing in the results is retracted, but SC-1's claim
of coverage must be read with this qualification. Carried to the V1 handoff.

Secondary note (methodology): the earlier "clippy clean" check during increment 6 read a
cached build and reported zero warnings. Re-checked with a forced rebuild during
publication preparation, which is how this gap surfaced. Lesson: a tool result read from a
cache is not a verification.

### Class A (V0 must fix): none outstanding
All defects found were in the harness/oracle, not in fsp-check. No falsification of the
V0 implementation was produced by this increment; the properties below hold.

---

## PROPERTIES PROVED (mandate §3)

| id | property | status |
|---|---|---|
| A | scan of a stable state matches the independent oracle | PROVED (generated sequences, 512 cases) |
| B | consecutive scans of an unchanged tree produce equal observable state | PROVED (increment 1 + harness) |
| C | reconcile(A,A) yields only Unchanged | PROVED (dedicated test + proptest) |
| D | reconcile is deterministic | PROVED (proptest, arbitrary pairs) |
| E | store rebuild reproduces the projection | PROVED (through generated-ish sequences, incl. rename/delete/recreate) |
| F | same content ≠ same physical identity | PROVED (proptest + duplicate-content test) |
| G | same PhysicalId + changed hash ⇒ Modified | PROVED (hash tests + reconcile evidence) |
| H | rename with physical evidence is never Delete+Created | PROVED (harness property) |
| I | delete+recreate never invents continuity | PROVED (no Unchanged across different inodes) |
| J | incomplete scan never yields Deleted | PROVED (dedicated test) |
| K | a valid hash corresponds to bytes actually observed | PROVED (INV-1 tests, stable-only persistence) |

Rejected/qualified: none of the candidate properties had to be rejected. Property B is
qualified: it compares OBSERVABLE STATE (not observation timestamps), which is the
determinism contract of increment 1.
