# CONSTRAINTS

Status: CURRENT (living) — canonical knowledge (T2).

Category E: facts about the environment that bound every candidate architecture. These
are not choices the project makes; they are conditions it must operate inside. Each is
sourced. Where a figure comes from a single study, that is noted.

-------------------------------------------------------------------------------
ENVIRONMENT
-------------------------------------------------------------------------------
C1  Filesystems are mutable by external actors at any time.
    Other editors, scripts, sync clients and VCS operations modify, rename, replace and
    delete files without notifying the application.
    Source: S1 §2, S5 §Problem, S9 §3. Status: FACT.

C2  Identity cannot rest on path or filesystem node.
    Paths break on rename/move; POSIX inode/NTFS file IDs are invalidated by atomic saves
    (write-temp, fsync, rename), volume boundaries and reinstall; mtime is rewritten by
    checkout, sync and archive extraction; xattrs are stripped by Git, archives, cloud
    sync and non-native filesystems.
    Source: S1 §7, S5 §Evaluation of Metadata Persistence. Status: FACT.

C3  External mutation and a second state store create a two-writers problem.
    Every architecture that keeps files authoritative plus internal state must reconcile
    concurrent writers, or lose either user edits or index coherence.
    Source: S1 §13, S5 §Failure Modes, S9 Failure Modes, S11 §9. Status: FACT.

C4  Kernel file-watch facilities have hard ceilings.
    inotify is bounded by max_user_watches and drops events (IN_Q_OVERFLOW); Windows
    ReadDirectoryChangesW overflow is unrecoverable; FSEvents coalesces and obscures
    ordering. Network and FUSE mounts often do not propagate events at all.
    Source: S1 §14, S5 §Watcher Ceilings, S9 §3. Status: FACT.

C5  Cost of analysis grows with corpus size.
    Deterministic work is linear and cheap per file but I/O-bound at scale; dense
    embedding is 3-4 orders of magnitude more expensive than lexical indexing; full
    directory walks become syscall-bound past ~500k files.
    Source: S1 §14, S2 §10, S5 §Scalability. Status: EMPIRICAL (measured, still
    environment-specific). See H8 for the hypothesis built on top of it.

C6  Model attention and adherence degrade with input volume and instruction count.
    Instruction-following decays measurably past roughly 100-150 active instructions, and
    long uncurated contexts degrade cross-document reasoning and instruction compliance.
    Source: S11 §7, S2 §3.1. Status: EMPIRICAL (single-study figures; direction robust).

C7  Local hardware is the reference target.
    Consumer desktops and laptops, offline-capable, without a server or GPU farm.
    Full-corpus dense embedding is infeasible at scale on this class of hardware.
    Source: S2 §10, S9 §Scalability, S11 §8. Status: ASSUMPTION (see Q9) with measured
    supporting data.

C8  Identity and authorship of assertions cannot be established by cryptography alone.
    A valid signature proves origin, not truth. Cryptographic provenance is necessary and
    insufficient for epistemic validity.
    Source: S4 §3/§7. Status: FACT.

-------------------------------------------------------------------------------
HUMAN AND ORGANISATIONAL
-------------------------------------------------------------------------------
C9  Users do not maintain what they are not obliged to maintain.
    Note systems require capture with no maintenance burden; anything requiring ongoing
    curation decays. Dashboards, indexes and allowlists rot unless self-maintaining.
    Source: S2 §15, S8 §5.6/§9, S11 §7. Status: OBSERVATION (widely reported, not
    experimentally established).

C10 Interactive confirmation does not scale as a safety control.
    Under volume, users approve the large majority of prompts reflexively; per-action
    prompting is not a reliable boundary.
    Source: S7 §2/§17, S11 §7 (reports ~93% approval, one of three threats missed).
    Status: EMPIRICAL for the direction; the exact figure is from one study.

C11 Visualisation has a cognitive ceiling.
    Global force-directed graphs are consistently reported as unreadable past a few
    hundred nodes; local, filtered views remain useful.
    Source: S8 §5.7/§7, S11 §7. Status: EMPIRICAL (convergent reports; threshold varies).

C12 Provenance metadata is itself sensitive.
    Prompts, retrieved context and session traces can carry secrets or client data; a
    record of the user's work is a privacy surface, not only an audit asset.
    Source: S6 §Security, S5 Security. Status: FACT.

-------------------------------------------------------------------------------
CONSTRAINT NOT YET ESTABLISHED
-------------------------------------------------------------------------------
Scale target. The corpus treats 10^4 to 10^6 files as the design envelope, but the
figures are borrowed from benchmark suites (Linux kernel, Chromium 489,684 files) rather
than from the intended use case. Whether FSP must operate at 10^6 files is unconfirmed
and determines which of the candidate mechanisms are even relevant. See Q8. Treating
10^6 as a requirement would pre-commit the architecture (see CONSTRAINTS-NOTE in
ARCHITECTURE-HYPOTHESES.md, risk R2).
