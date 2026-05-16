//! Install agents + apps into `~/.aware/`.
//!
//! Submodules added across Tasks 6-9.

#![allow(dead_code)]

pub mod local;
// pub mod registry;     // added in Task 7
// pub mod bundle;       // added in Task 8
// pub mod uninstall;    // added in Task 9

#[allow(unused_imports)]
pub use local::{install_agent_from_path, install_app_from_path};
