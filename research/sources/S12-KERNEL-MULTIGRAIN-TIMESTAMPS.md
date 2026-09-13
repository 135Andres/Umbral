# S12 — Linux kernel: multigrain timestamps

Status: **REFERENCE RECORD** (2026-09-12). External evidence. Not evidence about Umbral.

This is a *reference record with citation*, **not** a verbatim archive. `research/sources/`
otherwise holds immutable verbatim archives; this entry deliberately departs from that,
for two reasons stated so a reader is not misled:

1. the source is a living document that changes between kernel releases, so a "verbatim
   archive" would freeze a snapshot that the project would then have to re-verify;
2. the page is kernel documentation under its own licence, and copying it wholesale into
   this Apache-2.0 repository is not something this project should do casually.

What is reproduced below is the minimum quotation needed to support the claims Umbral makes
about it, with the exact wording preserved so the claim can be checked.

## SOURCE

```
Linux kernel documentation — "Multigrain Timestamps"
  https://docs.kernel.org/filesystems/multigrain-ts.html
  Retrieved 2026-09-12. Documented as covering kernel 7.3.0-rc2 at retrieval time.
  Local kernel at retrieval: 7.1.13-200.fc44.x86_64
```

Authority: **external technical documentation**. It describes kernel behaviour. It says
nothing about Umbral, and nothing in it validates any Umbral design.

## THE CLAIMS UMBRAL RELIES ON, QUOTED

**1. ctime is the inode change time, and it is not settable from userland.**

> "ctime: The inode change time. This is stamped with the current time whenever the inode's
> metadata is changed. Note that this value is not settable from userland."

**2. Updating mtime implies updating ctime.**

> "Updating the mtime always implies a change to the ctime, but updating the atime due to a
> read request does not."

**3. Timestamps were historically coarse, which is exactly the risk.**

> "Historically, the kernel has always used coarse time values to stamp inodes. This value is
> updated every jiffy, so any change that happens within that jiffy will end up with the same
> timestamp."

**4. Multigrain timestamps reduce that risk, and are an opt-in per filesystem.**

> "Multigrain timestamps aim to remedy this by selectively using fine-grained timestamps when
> a file has had its timestamps queried recently, and the current coarse-grained time does not
> cause a change."
>
> "For most filesystems, it's sufficient to just set the FS_MGTIME flag in the fstype->fs_flags
> in order to opt-in…"

**5. Timestamps can appear to go backwards.**

> "Note that the above assumes that the system doesn't experience a backward jump of the
> realtime clock. If that occurs at an inopportune time, then timestamps can appear to go
> backward, even on a properly functioning system."

**6. Fine-grained timestamps are not for every filesystem.**

> "Multigrain timestamps are intended for use by local filesystems that get ctime values from
> the local clock. This is in contrast to network filesystems and the like that just mirror
> timestamp values from a server."

## WHAT THIS SUPPORTS IN THIS PROJECT

- `UD-018`: `ctime` is an optimisation heuristic and may never be used to assert
  content-verified. Claim 1 is why a writer cannot forge it; claims 3–4 and 6 are why it is
  still not a guarantee.
- `EXP-CTIME` (`experiments/exp-ctime/EXP-CTIME.md`): the local measurements are consistent
  with claims 1 and 4. The experiment's own limits section is the reason claim 3 matters —
  the local result was obtained on a kernel and filesystems that had opted in, and that is a
  condition of the measurement, not a property of the field.
- `docs/candidates/V0.2-SCOPE-PROPOSAL.md` §6, which carries the open question of whether
  `ctime` enters a skip condition.

## WHAT THIS DOES NOT SUPPORT

- It does not establish that `ctime` changes in every case that matters to Umbral. It is
  precisely the document that explains why it might not.
- It does not establish anything about ext4 specifically on this project's CI runner. The
  page states that ext4 is among the filesystems that opted in (recorded in the public patch
  series, not verified here); the project has **not** measured ext4 and records that as
  `UNKNOWN`.
- It is not a validation of any Umbral mechanism. A published practice does not demonstrate
  that Umbral implements it correctly.

## RELATED

- `experiments/exp-ctime/EXP-CTIME.md` — the local probe and its raw output.
- `docs/decisions/DECISIONS.md` — `UD-018`.
- `umbral/src/content.rs` — the guard whose blind spot this bears on.
- `docs/versions/v0.1.md` — known limitation 6.
