//! Install agents + apps into `~/.aware/`.

pub mod bundle;
pub mod local;
pub mod registry;
pub mod uninstall;

pub use bundle::install_bundle;
pub use local::install_agent_from_path;
#[allow(unused_imports)] // consumed by v0.2 `aware app install` (app command, Task 13+)
pub use local::install_app_from_path;
pub use registry::install_agent_from_registry;
pub use registry::update_agent_from_registry;
#[allow(unused_imports)] // consumed by v0.2 `aware agent uninstall` + `aware app uninstall`
pub use uninstall::{uninstall_agent, uninstall_app};
