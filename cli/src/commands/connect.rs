//! `aware connect ...` / `aware disconnect ...` — credential management.
//!
//! Phase v0.4. PKCE OAuth flow + OS keyring encryption.

use clap::Args;

use crate::context::Context;
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

pub fn run_connect(args: ConnectArgs, _ctx: &Context) -> Result<(), AwareError> {
    let cfg = crate::auth::config::for_integration(&args.integration)?;

    if args.refresh {
        let token = crate::auth::refresh::ensure_fresh(&args.integration, args.r#as.as_deref())?;
        println!(
            "\u{2713} refreshed {} (expires at unix-{})",
            args.integration, token.expires_at
        );
        return Ok(());
    }

    let extra_scopes: Vec<String> = args
        .scopes
        .as_deref()
        .map(|s| {
            s.split(',')
                .map(|p| p.trim().to_string())
                .filter(|p| !p.is_empty())
                .collect()
        })
        .unwrap_or_default();

    let token = crate::auth::pkce::run_pkce_flow(&cfg, &extra_scopes)?;
    crate::auth::keychain::store_token(&token, args.r#as.as_deref())?;
    println!("\u{2713} Encrypted to OS keychain.");
    println!("  service: aware-aeco");
    println!(
        "  account: {}{}",
        args.integration,
        args.r#as
            .as_deref()
            .map(|a| format!(".{a}"))
            .unwrap_or_default()
    );
    Ok(())
}

pub fn run_disconnect(args: DisconnectArgs, _ctx: &Context) -> Result<(), AwareError> {
    crate::auth::keychain::delete_token(&args.integration, args.r#as.as_deref())?;
    println!("\u{2713} Removed credential for {}", args.integration);
    Ok(())
}
