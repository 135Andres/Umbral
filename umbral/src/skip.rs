//! The skip decision (`V0.2-TECHNICAL-DESIGN.md` §C.1, `UD-035`, `UD-036`).
//!
//! A pure function of two values — the previous run's observation of the same path and the
//! current entry — so it can be tested against hand-built observations and never depends on
//! what a filesystem happens to do (F-6).
//!
//! A skip is never content verification (`UD-017`): it carries a reading forward, and the
//! reading keeps naming the run that actually read the bytes.

use crate::log::Observation;
use crate::scan::{Entry, EntryKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decision {
    /// Read and hash the bytes now.
    Read,
    /// Carry the previous reading forward; the bytes are not read.
    Skip,
}

/// Skip only when every term of the condition holds and is present on both sides:
///
/// 1. both are regular files;
/// 2. the previous observation holds a valid reading, attributed to the run that read it;
/// 3. same `dev` and `ino`;
/// 4. same size;
/// 5. same mtime;
/// 6. same `ctime` — the one timestamp a writer cannot restore (`UD-018`, E-TD-2/E-TD-3).
///
/// Anything absent is a reason to read: absence of evidence is not evidence.
pub fn decide(previous: Option<&Observation>, current: &Entry) -> Decision {
    let Some(p) = previous else {
        return Decision::Read;
    };
    fn same<T: PartialEq>(a: Option<T>, b: Option<T>) -> bool {
        matches!((a, b), (Some(a), Some(b)) if a == b)
    }
    let holds = p.kind == EntryKind::File
        && current.kind == EntryKind::File
        && p.valid_hash().is_some()
        && p.hash_read_run.is_some()
        && same(p.dev, current.dev)
        && same(p.ino, current.ino)
        && same(p.size, current.size)
        && same(p.mtime, current.mtime)
        && same(p.ctime, current.ctime);
    if holds {
        Decision::Skip
    } else {
        Decision::Read
    }
}
