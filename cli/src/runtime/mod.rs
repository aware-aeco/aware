//! Async orchestrator + provenance + agent invocation.
//!
//! v0.3 phase. Submodules added across Tasks 2-9; orchestrator + command
//! wiring come in Tasks 10-18.

#![allow(dead_code)]

pub mod context;
pub mod inline;
pub mod invoker;
pub mod lifecycle;
pub mod orchestrator;
pub mod pidfile;
pub mod provenance;
pub mod template;
// other submodules added later in v0.3
