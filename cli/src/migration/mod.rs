//! Workflow migration machinery (#628): carrying an approved app forward to a
//! newer agent version without pretending a person re-approved it.
//!
//! This module holds the READ-ONLY plumbing every later step stands on:
//!
//! * [`effect`] — what each node of a workflow is *declared* to do (read or
//!   write), where that mode came from, and whether a whole workflow is
//!   *declared read-only* under both the old and the new pins.
//! * [`contract`] — the executable-contract diff between two stored packages of
//!   one agent (`aware.contract-diff/v1`): what a run would hand the executor,
//!   minus what the run never reads.
//!
//! Nothing here writes a file or decides a migration. The verbs that do
//! (`aware app migrate plan|prepare|promote …`) arrive in later PRs and call
//! these functions; until then only `aware agent describe` reaches into
//! [`effect`], so each submodule allows `dead_code` outside tests, with the PR
//! that removes the allowance named beside it.

pub mod contract;
pub mod effect;
