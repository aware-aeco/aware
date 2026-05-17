//! Subcommand modules.
//!
//! Each group (`agent`, `app`, `connect`, `skill`, `build`, `doctor`) lives
//! in its own module. The fresh-session implementer fills these in per
//! the phases in `10-core/cli-roadmap.md`.

pub mod agent;
pub mod app;
pub mod build;
pub mod connect;
pub mod diagram;
pub mod doctor;
pub mod plugins;
pub mod search;
pub mod skill;
pub mod tree;
