# V0 PRE-FLIGHT — FEDORA LINUX (evidence record)

Status: OBSERVATION RECORD (environment probe, 2026-09-11). Non-destructive; probes ran
in throwaway temp dirs, removed afterwards. Input mandate: "Pre-flight de implementación
V0 en Fedora Linux". Local versions are RECORDED FOR REPRODUCIBILITY ONLY — none is a
project requirement.

================================================================================
1. ENVIRONMENT DETECTED
================================================================================
  OS          Fedora release 44 (Forty Four)
  Kernel      7.1.13-200.fc44.x86_64 · x86_64 · Intel i7-8850H
  Workspace   <repo> on btrfs (nvme0n1p3), dev_id 51 at probe time
  /tmp        tmpfs (dev_id 52) — DIFFERENT filesystem from the workspace
  gcc         16.2.1 (present — rusqlite bundled-feature C build is viable)
  libsqlite3  libsqlite3.so.0 present (system); rusqlite `bundled` remains the safer
              default and does not need the system library
  git         2.55.0
  Rust/Cargo  **NOT INSTALLED** (no rustc, cargo, rustup; rpm packages absent)
  Network     crates.io reachable (HEAD returned 403 bot-filter; normal for curl)

================================================================================
2. FILESYSTEM PROBE RESULTS (btrfs workspace and tmpfs /tmp behaved identically)
================================================================================
  sub-microsecond mtime distinctness : YES both (st_mtime_ns granularity real)
  symlinks + lstat/stat distinction  : YES both
  hard links (same dev+ino, nlink=2) : YES both
  atomic replace (os.replace)        : YES both
  directory rename (contents follow) : YES both
  case sensitivity                   : YES both (Case.txt != case.txt)
  inode reuse on delete+recreate     : NOT observed in one probe — recorded, NOT assumed
                                       absent (§3 below)
  NOTE: probes were Python-level; Rust-level semantics expected identical (same syscalls).

================================================================================
3. LINUX-SPECIFIC ASSUMPTIONS THE IMPLEMENTATION MUST NOT SILENTLY MAKE
================================================================================
  - dev+ino as stable identity: reuse after delete is possible on Linux even though the
    probe did not catch it; plan §7-A already classifies this as must-test.
  - mtime resolution: high-resolution here; must not be trusted for ordering beyond the
    observation sequence (plan §7-B already).
  - case sensitivity: this environment is case-sensitive; ext4 default is, but the
    implementation must not hardcode the assumption (plan §7-A).
  - tempfile targets /tmp (tmpfs), the real target is btrfs: harness results on tmpfs
    are valid for semantics but any benchmark numbers are fs-dependent — labelled
    EVIDENCE with the fs named (user rule: no numeric thresholds anyway).
  - path encoding: Linux paths are bytes; Rust OsStr/PathBuf must be used end-to-end,
    no premature UTF-8 lossy conversion (added to plan §13).
  - atomic rename is same-filesystem only; cross-device moves are copy+delete, never
    atomic (added to plan §13).

================================================================================
4. PORTABILITY CLASSIFICATION OF THE V0 DESIGN (mandate §2)
================================================================================
  A portable (by design): reconciler logic, classification rules, invariant checks,
      store schema (SQLite), hash semantics (INV-1), proptest harness logic.
  B Linux-specific (must stay behind an observation boundary, plan §13): dev/ino
      gathering, stat fields used, path/OsStr handling, rename semantics reliance.
  C filesystem-dependent: mtime resolution, inode reuse behaviour, btrfs CoW effects on
      timestamps, tmpfs-vs-btrfs benchmark differences.
  D still unknown: crash-window behaviour on btrfs under real crash injection (INV-7
      testing will tell); whether scan-based detection suffices (plan §7-C/Q25-adjacent).

================================================================================
5. DEPENDENCY VERDICT
================================================================================
  All seven V0 dependencies are sufficient; none requires a companion crate on Linux.
  walkdir covers traversal; std::fs covers rename/symlink/hardlink/permissions; no need
  for notify (out of scope), no extra error-management crate needed at this size (std
  Error + thiserror-free is fine for V0; if the implementer later proposes thiserror or
  similar, register as PROPOSAL per DOCUMENTATION-ARCHITECTURE §8b, do not adopt
  silently). The ONLY missing piece is the Rust toolchain itself (§1).

================================================================================
6. IDENTITY / OBSERVATION RULES — VERIFIED IN PLAN
================================================================================
  - Plan §4 carries the user principle verbatim: ruta → identidad física → contenido is
    an efficiency strategy, NOT semantic identity; INV-3/INV-8 (new) force evidence-
    backed classification and explicit ambiguity.
  - INV-1 carries the corrected hash formulation; no invariant implies FSP controls or
    freezes the filesystem.
