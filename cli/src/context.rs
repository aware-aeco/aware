//! Per-invocation context handed to every dispatcher.

use crate::paths::Paths;

#[derive(Debug, Clone)]
pub struct Context {
    #[allow(dead_code)] // consumed by Task 9+ (agent list reads paths)
    pub paths: Paths,
    #[allow(dead_code)] // consumed by Task 9+ (JSON output flag)
    pub json: bool,
}
