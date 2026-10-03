//! Umbral v0.2 — a local, read-only workspace observation instrument.
//!
//! # What this crate is
//!
//! A person points Umbral at a directory they own, records what is there, and later reads
//! back what is known, when it was known, and what changed. That is the whole contract.
//!
//! # What this crate is NOT
//!
//! This is **development code for one version**, not the architecture of Umbral. Nothing
//! here is a selected design. It does not depend on, import, or modify `fsp-check/` — the
//! frozen V0 experiment — which is cited as evidence only.
//!
//! Deliberately absent, and each absence is a decision, not an oversight:
//! watcher, daemon, FTS, embeddings, API, UI, MCP, AI of any kind, sync, multi-device,
//! multi-writer, permissions, semantic layer, Project Reality.
//!
//! # The epistemic contract
//!
//! Every line of user-facing output carries exactly one of four labels, and the
//! distinction is the point of the tool:
//!
//! - [`report::Label::Observed`] — a fact read from the filesystem during a named run
//! - [`report::Label::Derived`] — a result computed from observations
//! - [`report::Label::Ambiguous`] — a classification the evidence leaves open between more
//!   than one outcome, with a named reason
//! - [`report::Label::Unknown`] — a value not determinable from the available evidence
//!
//! A missing value is never filled with an invented one.

pub mod acquisition;
pub mod content;
pub mod contract;
pub mod identity;
pub mod log;
pub mod observe;
pub mod reconcile;
pub mod report;
pub mod scan;
pub mod skip;
pub mod workspace;

pub use content::{ContentObservation, Stability};
pub use identity::PhysicalId;
pub use log::{Observation, ObservationLog, RunId, RunMeta};
pub use reconcile::{Mutation, MutationKind, ObservationSet, ObservedPath, Reconciliation};
pub use scan::{Entry, EntryKind, PathError, Scan};
