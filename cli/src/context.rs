//! Per-invocation context handed to every dispatcher.

use crate::paths::Paths;

#[derive(Debug, Clone)]
pub struct Context {
    pub paths: Paths,
    pub json: bool,
}
