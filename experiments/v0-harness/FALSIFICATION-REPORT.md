
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

Coverage, stated precisely (three different things, not one):

  - **Modelled by the harness** (variants with an apply arm and a precondition): create
    file, write, append, truncate, rename file, rename directory, delete file, delete
    directory, recreate, atomic replace, create symlink, remove symlink, create hard link,
    remove hard link. Fourteen operations.
  - **Actually produced by the property-test strategy**: all of the above **except delete
    directory**. The strategy never emits `Op::DeleteDir`; clippy reports it as never
    constructed, which is how the gap was found.
  - **Exercised end-to-end against a real filesystem by the generated sequences**: the
    thirteen operations above, plus rename, delete-and-recreate, atomic save, hard-link
    creation and symlink creation/removal as targeted tests. Directory deletion is covered
    by no test in V0.

Disposition: **recorded, not patched.** V0 is frozen; adding the generator case would
change the evidence rather than describe it. The gap is a limitation of V0's falsification
coverage, not a falsified property — nothing in the results is retracted, but SC-1's claim
of coverage must be read with this qualification. Carried to the V1 handoff.

Secondary note (methodology): the earlier "clippy clean" check during increment 6 read a
cached build and reported zero warnings. Re-checked with a forced rebuild during
publication preparation, which is how this gap surfaced. Lesson: a tool result read from a
cache is not a verification.

### F-6 (class B — defective test assertion, found by the CI runner on ext4)

Symptom: `tests/reconcile_mutations.rs::full_cycle_scan_reconcile_store` failed
deterministically on the GitHub Actions runner (ubuntu-latest, ext4), twice in a row, at the
assertion `kinds(&r, MutationKind::Deleted).iter().any(|m| m.path == *"bye.txt")`. The same
suite passed 68/68 on the development machine (Fedora, tmpfs `/tmp`).

Cause: the fixture deletes `bye.txt` and creates `new.txt` in the same instant, then asserted
`Deleted(bye.txt)` AND `Created(new.txt)`. That pair of verdicts holds **only if the
filesystem does not hand the freed `dev+ino` to the new file**. On tmpfs (a monotonically
increasing inode counter) it never does; on ext4 the freed inode is reused immediately, and
`reconcile` then pairs the two paths into a single `RenamedOrMoved(new.txt, old=bye.txt)` —
which is the correct reading of that evidence, not a defect. The test had an implicit,
unstated assumption about the filesystem baked into it.

Three distinct things, kept apart:

  - **The implementation of `reconcile`:** not implicated. Its output followed its
    documented rules in both environments. dev+ino is defined as *evidence*, not as eternal
    identity, and reuse after delete is documented in `src/identity.rs` ("it is NOT eternal
    (inodes are reused after delete)"). `identity_physical.rs` already contains a test that
    records observed inode reuse as a documented limitation.
  - **The assertion:** defective — it required one of two equally valid readings.
  - **Filesystem-dependent behaviour:** the source of the difference between environments.
    This is a property of the substrate, not of V0.

Independent reproduction (outside the repository, no V0 code changed): a standalone crate
driving the public API with synthetic observation sets showed that a fresh inode yields
`Deleted(bye.txt) + Created(new.txt)`, while a reused inode yields
`RenamedOrMoved(new.txt, old=bye.txt)` and no `Deleted` — exactly the CI failure. On the
development machine, inode reuse could not be provoked (btrfs and tmpfs: 0 reuses in 40
immediate and 750-of-1500 churned cases), which is why the gap survived local verification
and was found by an independent environment.

Resolution: the assertion now states the property that holds in **both** environments — the
disappearance of `bye.txt` is accounted for **exactly once** (a `Deleted` XOR a
`RenamedOrMoved` whose `old_path` is `bye.txt`), and the accompanying mutation must agree
with whichever reading was taken (no identity reuse ⇒ `Created(new.txt)`; reuse ⇒ no separate
`Created`). This is stronger than the original assertion, not weaker: it fails if the
reconciler loses the mutation *or* double-reports it, and it still fails if the path is merely
`Unobserved`. No source file, no semantics and no behaviour of `reconcile` was changed; only
the test's expectation.

Corrected coverage statement (supersedes the unqualified "68/68 tests green" recorded in the
increment-6 report, `V0-CLOSEOUT.md` and `fsp-check/README.md`):

  - **68/68 on the development environment** (Fedora 44, tmpfs `/tmp`).
  - **67/68 + 1 filesystem-dependent test** on ext4 before this correction: the failing test
    was not detecting a V0 defect, it was encoding a substrate assumption.
  - **After the correction: 68/68 on both environments.** Verified locally (fresh run, no
    cache) and in CI on the runner's ext4, which is the environment that exposed the gap.

Class: B. Not a falsification of V0, and not evidence of incorrect `reconcile` behaviour.

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
