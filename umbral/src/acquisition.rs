//! How each component of an observation's evidence was obtained (`UD-031`, `UD-033`).
//!
//! The state is derived from the stored observation on read, never stored itself: the log
//! already holds everything it is computed from. Two components exist in v0.2 — the metadata
//! the scan reads for every entry, and the content of a regular file. An entry that is not a
//! regular file has no content component; that is not a state.

use crate::content::Stability;
use crate::log::Observation;
use crate::scan::EntryKind;

/// The closed vocabulary of `UD-031`. A new value needs an owner decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcquisitionState {
    /// Acquired in this act; a value exists.
    Fresh,
    /// A value carried from an earlier observation. Never content verification. No v0.2 build
    /// emits it before the skip exists (slice 3).
    Reused,
    /// Attempted; no value was obtained — including a reading that kept changing.
    Failed,
    /// It is recorded that no attempt was made.
    NotAttempted,
    /// It is not recorded whether an attempt was made.
    NotRecorded,
}

impl AcquisitionState {
    pub const ALL: [AcquisitionState; 5] = [
        AcquisitionState::Fresh,
        AcquisitionState::Reused,
        AcquisitionState::Failed,
        AcquisitionState::NotAttempted,
        AcquisitionState::NotRecorded,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            AcquisitionState::Fresh => "fresh",
            AcquisitionState::Reused => "reused",
            AcquisitionState::Failed => "failed",
            AcquisitionState::NotAttempted => "not-attempted",
            AcquisitionState::NotRecorded => "not-recorded",
        }
    }
}

/// The diagnostics a failed content acquisition is counted under, in output order. Each one
/// is kept apart from the state (`UD-031`); the full message stays in the log.
pub const FAILED_DIAGNOSTICS: [&str; 5] = [
    "unstable-observation",
    "not-found",
    "permission-denied",
    "not-a-regular-file",
    "read-error",
];

/// The stored `content_error` of a row written before the reason was persisted.
const NOT_RECORDED: &str = "not-recorded";

pub fn metadata_state(o: &Observation) -> AcquisitionState {
    if o.metadata_failed() {
        AcquisitionState::Failed
    } else {
        AcquisitionState::Fresh
    }
}

/// `None` when the entry has no content component.
pub fn content_state(o: &Observation) -> Option<AcquisitionState> {
    if o.metadata_failed() {
        // No kind was obtained, so no reading was attempted.
        return Some(AcquisitionState::NotAttempted);
    }
    if o.kind != EntryKind::File {
        return None;
    }
    if o.content_error.as_deref() == Some(NOT_RECORDED) {
        return Some(AcquisitionState::NotRecorded);
    }
    if o.valid_hash().is_some() {
        return Some(AcquisitionState::Fresh);
    }
    if o.stability == Some(Stability::Unstable) || o.content_error.is_some() {
        return Some(AcquisitionState::Failed);
    }
    // A file row with neither a reading nor a recorded error: nothing says whether a reading
    // was attempted.
    Some(AcquisitionState::NotRecorded)
}

/// The diagnostic of a failed content acquisition, as one of [`FAILED_DIAGNOSTICS`].
pub fn content_diagnostic(o: &Observation) -> Option<&'static str> {
    if content_state(o) != Some(AcquisitionState::Failed) {
        return None;
    }
    if o.stability == Some(Stability::Unstable) {
        return Some(FAILED_DIAGNOSTICS[0]);
    }
    let recorded = o.content_error.as_deref()?;
    let class = recorded.split(':').next().unwrap_or(recorded);
    FAILED_DIAGNOSTICS.iter().copied().find(|d| *d == class)
}
