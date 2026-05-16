//! `aware skill modify <agent> <skill-name>` — open the skill in $EDITOR.
//!
//! Placeholder — full implementation in Task 11.

use crate::context::Context;
use crate::error::AwareError;

pub fn run(_ctx: &Context, _agent_id: &str, _skill_name: &str) -> Result<(), AwareError> {
    Err(AwareError::NotYetImplemented("skill modify"))
}
