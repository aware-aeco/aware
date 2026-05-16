//! `aware skill eval <agent> <skill-name>` — run trigger-corpus eval.
//!
//! Placeholder — full implementation in Task 12.

use crate::context::Context;
use crate::error::AwareError;

pub fn run(_ctx: &Context, _agent_id: &str, _skill_name: &str) -> Result<(), AwareError> {
    Err(AwareError::NotYetImplemented("skill eval"))
}
