//! Install agents + apps into `~/.aware/`.

#![allow(dead_code)]

pub mod bundle;
pub mod local;
pub mod registry;
// pub mod uninstall;    // added in Task 9

#[allow(unused_imports)]
pub use bundle::install_bundle;
#[allow(unused_imports)]
pub use local::{install_agent_from_path, install_app_from_path};
#[allow(unused_imports)]
pub use registry::install_agent_from_registry;
