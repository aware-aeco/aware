//! Typed deserialization of AWARE agent + app manifests.
//!
//! Shapes verified against the 7 reference agents under `20-agents/` and
//! the 2 reference apps under `30-apps/_examples/`.
//!
//! `pub use` re-exports below are forward-declared so that callers added in
//! Tasks 5+ can use `manifest::Agent` / `manifest::App` without chasing the
//! inner module paths. The `unused_imports` lint is suppressed until those
//! callers exist.
#![allow(unused_imports)]

pub mod agent;

pub use agent::Agent;
