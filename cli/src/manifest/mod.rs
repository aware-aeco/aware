//! Typed deserialization of AWARE agent + app manifests.
//!
//! Shapes verified against the 7 reference agents under `20-agents/` and
//! the 2 reference apps under `30-apps/_examples/`.
//!
//! `pub use` re-exports below let callers write `manifest::Agent` /
//! `manifest::App` without chasing the inner module paths.

pub mod agent;
pub mod app;
pub mod expose;
pub mod loader;

pub use agent::Agent;
pub use app::App;
