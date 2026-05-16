//! `aware skill port <source> <target-agent>` — copy a skill into an agent, adapting frontmatter.
//!
//! Placeholder — full implementation in Task 10.

use crate::context::Context;
use crate::error::AwareError;

pub fn run(_ctx: &Context, _source: &str, _target_agent: &str) -> Result<(), AwareError> {
    Err(AwareError::NotYetImplemented("skill port"))
}
