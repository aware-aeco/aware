//! `aware doctor` — health check.
//!
//! Phase v0.1 (limited): config + filesystem.
//! Phase v0.2+: also check installed agents/apps, registry index, host plugins.
//! Phase v0.4+: also check credential validity and refresh state.

use crate::error::AwareError;

pub fn run() -> Result<(), AwareError> {
    Err(AwareError::NotYetImplemented("doctor"))
}
