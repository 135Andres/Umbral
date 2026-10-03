# EXP-CTIME — does ctime move when a change is otherwise hidden?

Status: **RESULT RECORDED** (2026-09-12). Method, raw output and limits are recorded here.

## 0. PROVENANCE — READ THIS FIRST

This experiment was **not pre-specified**. The probes ran during v0.2 planning, while
analysing whether `ctime` could serve as a skip heuristic, and the specification below was
written afterwards, from the code that was actually run. `DOCUMENTATION-ARCHITECTURE.md`
§10 rule 3b asks for a spec *before* the run; that rule was not followed here, and the
deviation is recorded rather than smoothed over.

What that costs, stated plainly:

- The probe design was **not** reviewed before running, so it can carry the author's
  assumptions about which cases matter.
- The case list was chosen by the same reasoning that wanted `ctime` to work. A
  confirmation-friendly selection is possible and cannot be ruled out from inside.
- The mitigations are: the script is deterministic and public, the raw output is archived
  below, and the conclusion drawn from it (`UD-018`) is deliberately **negative** about what
  `ctime` may be used to claim. The probe is not being used to justify a guarantee.

An independent re-run, or a replication on ext4, would raise confidence. Neither is
scheduled.

## 1. QUESTION

v0.1 documents a blind spot in its content guard: a rewrite that restores both size and
exact mtime inside the read window is invisible to a `stat`-based guard
(`umbral/src/content.rs`, and limitation 6 in `docs/versions/v0.1.md`).

If re-observation is to skip reading content, the same class of question moves from *inside*
one read to *between* two runs: **can a file change while size, mtime and physical identity
all stay equal?** If such a change is possible and undetectable, a metadata-based skip would
silently miss it.

`ctime` is a candidate additional field. The question is narrow and empirical:

> Does `ctime` change in the cases where size and mtime are deliberately made to match?

This experiment does **not** ask whether `ctime` is a guarantee of content stability. It
cannot be: it is inode metadata, not content. `UD-018` already fixes that it may never be
used to assert content-verified.

## 2. METHOD

Script: [`ctime_probe.py`](ctime_probe.py) — deterministic, public, re-runnable.

Environment (`observed`):

```
Kernel     7.1.13-200.fc44.x86_64  (Fedora 44)
Filesystems probed   /tmp                    type=tmpfs
                     <btrfs mount>          type=btrfs   (the workspace filesystem)
ext4        NOT AVAILABLE on this machine — not probed
```

REDACTION: the absolute path of the btrfs mount is replaced above and in §3 by
`<btrfs mount>`. It was present in the original probe output; it is redacted here so the
repository stays free of personal filesystem paths. The redaction removes a path and
nothing else — every measured value is as captured.

Each probe runs in a throwaway temp directory under the filesystem under test and removes it
afterwards. Nothing is written into the repository, and nothing is written into any user
directory that is not a temp directory.

Cases:

| # | Case | Why it is here |
|---|---|---|
| 1 | same size, mtime restored exactly via `utime` | the v0.1 blind spot, moved between runs |
| 2 | `chmod` only | metadata change with content untouched — should NOT force a read |
| 3 | hard link created to the file | inode-level change with content untouched |
| 4 | atomic replacement (`rename` over the target) | the standard safe-write pattern; new inode |
| 5 | 300 consecutive writes, counting ctime collisions | granularity: can two changes share a ctime? |
| 6 | `utime` then compare ctime | is ctime settable from userland? |
| 7 | `touch -d 2001-01-01` | can a backdated mtime coexist with a current ctime? |

## 3. RAW RESULT

Archived verbatim from the run above.

```json
[
  {
    "fs": "/tmp",
    "same_size_mtime_restored": {
      "size_equal": true, "mtime_equal": true, "ino_equal": true,
      "ctime_equal": false, "ctime_delta_ns": 50419383
    },
    "chmod_only":          { "mtime_equal": true, "ctime_equal": false },
    "hardlink_created":    { "mtime_equal": true, "ctime_equal": false, "nlink": 2 },
    "atomic_replace":      { "ino_equal": false, "size_equal": true,
                             "mtime_equal": true, "ctime_equal": false },
    "granularity":         { "writes": 300, "ctime_unchanged_after_write": 0,
                             "min_delta_ns": 86039, "max_delta_ns": 177858 },
    "utime_can_forge_ctime": false,
    "touch_backdated": {
      "before": "mtime=2026-09-12 19:29:06.723520125 -0700 ctime=2026-09-12 19:29:06.804472515 -0700",
      "after":  "mtime=2001-01-01 00:00:00.000000000 -0800 ctime=2026-09-12 19:29:06.809617661 -0700"
    }
  },
  {
    "fs": "<btrfs mount>",
    "same_size_mtime_restored": {
      "size_equal": true, "mtime_equal": true, "ino_equal": true,
      "ctime_equal": false, "ctime_delta_ns": 50561672
    },
    "chmod_only":          { "mtime_equal": true, "ctime_equal": false },
    "hardlink_created":    { "mtime_equal": true, "ctime_equal": false, "nlink": 2 },
    "atomic_replace":      { "ino_equal": false, "size_equal": true,
                             "mtime_equal": true, "ctime_equal": false },
    "granularity":         { "writes": 300, "ctime_unchanged_after_write": 0,
                             "min_delta_ns": 230119, "max_delta_ns": 548727 },
    "utime_can_forge_ctime": false,
    "touch_backdated": {
      "before": "mtime=2026-09-12 19:29:06.816496721 -0700 ctime=2026-09-12 19:29:06.961757723 -0700",
      "after":  "mtime=2001-01-01 00:00:00.000000000 -0800 ctime=2026-09-12 19:29:06.965618669 -0700"
    }
  }
]
```

## 4. WHAT THE RESULT SHOWS (`observed`)

1. **Case 1 — the blind spot moves, and ctime sees it.** On both filesystems, restoring the
   exact mtime and keeping the same size left `ctime` changed (by ~50 ms, the `sleep` in the
   script). A metadata-based skip that includes `ctime` would **not** have skipped this file.
2. **Cases 2 and 3 — ctime is conservative.** `chmod` and creating a hard link change `ctime`
   with content untouched. A skip using `ctime` would re-read a file that did not change. That
   is the safe direction of error: extra work, not a wrong verdict.
3. **Case 4 — atomic replacement is visible twice over.** `ino` changed *and* `ctime` changed,
   even though size and mtime were made to match. This matters: the replacement pattern is
   exactly the case where trusting `size + mtime` alone would be wrong.
4. **Case 5 — no collision in 600 writes.** Zero `ctime` collisions across 300 writes on each
   filesystem; the smallest observed delta was ~86 µs, far above the clock's resolution.
5. **Case 6 — ctime is not forgeable from userland.** `utime` changed `ctime`; it could not
   restore it. This is the property that makes case 1 work.
6. **Case 7 — a backdated mtime coexists with a current ctime.** `mtime=2001-01-01` while
   `ctime` stayed at the moment of the `touch`. A writer can set mtime to anything; it cannot
   set ctime.

## 5. WHAT THE RESULT DOES NOT SHOW (`derived`, and explicitly limited)

- **Not** that `ctime` always changes when content changes. The Linux kernel's own
  documentation (`filesystems/multigrain-ts`, cited in
  [`research/sources/S12-KERNEL-MULTIGRAIN-TIMESTAMPS.md`](../../research/sources/S12-KERNEL-MULTIGRAIN-TIMESTAMPS.md))
  records that timestamps were historically coarse — "any change that happens within that
  jiffy will end up with the same timestamp" — and that multigrain timestamps, which reduce
  this, are a **per-filesystem opt-in** (`FS_MGTIME`). On a filesystem that has not opted in,
  or that mirrors timestamps from a server, a change inside one tick may leave `ctime` equal.
- **Not** measured on ext4. The CI runner is the only ext4 environment available to this
  project, and this probe does not run there. ext4 behaviour is `UNKNOWN`.
- **Not** a statement about network filesystems, FUSE, or any filesystem that does not
  generate its own timestamps.
- **Not** about `ctime` under concurrent modification, clock adjustment, or a system clock
  moving backwards (the kernel documentation notes timestamps can appear to go backward).
- **Not** about hard-link aliasing at the content level: case 3 shows the link creation is
  visible, but this experiment says nothing about content shared through two paths.
- The probe is a Python script using `os.lstat`; it observes what userland observes, which is
  the same surface the tool has.

## 6. CONCLUSION CARRIED FORWARD

`observed`: including `ctime` in a skip condition would have prevented the skip in every case
probed where content or inode state changed while size and mtime were made to match, and
would not have produced a wrong skip.

`derived`: therefore `ctime` **narrows** the known false negative. It does not close it, and
it does not make metadata into content evidence.

**No guarantee of integrity follows from this experiment, and none may be claimed from it.**
This is the constraint recorded as `UD-018`: `ctime` is an optimisation heuristic, it may not
be used to assert content-verified, and a result reached by avoiding the read must keep
saying that the bytes were not read.

`OPEN`: whether `ctime` enters the skip condition at all (D-PEND-2). This experiment informs
that decision; it does not make it. See `docs/candidates/V0.2-SCOPE-PROPOSAL.md` §6.

## 7. REPRODUCTION

```
python3 experiments/exp-ctime/ctime_probe.py /tmp "$HOME"
```

Deterministic given the filesystem and kernel; case 1 and case 5 depend on clock behaviour,
so a replication on another kernel is genuinely informative rather than a formality.
