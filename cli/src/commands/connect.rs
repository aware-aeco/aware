//! `aware connect ...` / `aware disconnect ...` — credential management.
//!
//! Phase v0.4. PKCE OAuth flow + OS keyring encryption.

use clap::Args;

use crate::error::AwareError;

#[derive(Args, Debug)]
pub struct ConnectArgs {
    /// Integration id (`trimble-connect`, `microsoft-365`, `google-workspace`, …).
    pub integration: String,

    /// Named credential alias for multi-account setups.
    #[arg(long)]
    pub r#as: Option<String>,

    /// Force-refresh an existing credential.
    #[arg(long)]
    pub refresh: bool,

    /// Additional scopes to request (comma-separated).
    #[arg(long)]
    pub scopes: Option<String>,
}

#[derive(Args, Debug)]
pub struct DisconnectArgs {
    pub integration: String,

    #[arg(long)]
    pub r#as: Option<String>,
}

pub fn run_connect(_args: ConnectArgs) -> Result<(), AwareError> {
    Err(AwareError::NotYetImplemented("connect"))
}

pub fn run_disconnect(_args: DisconnectArgs) -> Result<(), AwareError> {
    Err(AwareError::NotYetImplemented("disconnect"))
}
