//! Async orchestrator + provenance + agent invocation.
//!
//! v0.3 phase. Submodules added across Tasks 2-9; orchestrator + command
//! wiring come in Tasks 10-18.

pub mod artifact_retention;
pub mod artifact_stream;
pub mod context;
pub mod google_mail;
pub mod inline;
pub mod invoker;
pub mod lifecycle;
pub mod orchestrator;
pub mod pidfile;
pub mod progress;
pub mod provenance;
pub mod report_reservation;
pub mod template;
pub mod trimble_files;
// other submodules added later in v0.3
