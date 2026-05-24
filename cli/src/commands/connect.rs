//! `aware connect ...` / `aware disconnect ...` — credential management.
//!
//! Phase v0.4. Browser-paste flow (default) + OAuth/PKCE (--oauth) + non-interactive paths.
//! Phase v0.13 additions: `--device-code` (RFC 8628) + `--tenant` (M365).
//! Phase v0.41 additions: `--list` (machine-readable credential status, closes #140).

use std::time::{SystemTime, UNIX_EPOCH};

use clap::Args;

use crate::auth::keychain::TokenSource;
use crate::context::Context;
use crate::error::AwareError;

#[derive(Args, Debug)]
pub struct ConnectArgs {
    /// Integration id (e.g. trimble-connect). Required unless --list is set.
    #[arg(required_unless_present = "list")]
    pub integration: Option<String>,

    /// Account alias for multi-account setups.
    #[arg(long = "as")]
    pub r#as: Option<String>,

    /// Force refresh of an existing OAuth credential.
    #[arg(long)]
    pub refresh: bool,

    /// Additional OAuth scopes (comma-separated).
    #[arg(long)]
    pub scopes: Option<String>,

    /// Read token from a file (plain text or JSON with `access_token` field).
    #[arg(long = "from-file")]
    pub from_file: Option<std::path::PathBuf>,

    /// Read token from env var `AWARE_TOKEN_<INTEGRATION>` (uppercase, kebab → underscore).
    #[arg(long = "from-env")]
    pub from_env: bool,

    /// Use the OAuth/PKCE flow instead of paste-form (requires AWARE_OAUTH_*_CLIENT_ID).
    #[arg(long)]
    pub oauth: bool,

    /// Use the RFC 8628 device-code flow — for headless / IT-managed
    /// workstations where browser-localhost loopback isn't available. (v0.13)
    #[arg(long = "device-code")]
    pub device_code: bool,

    /// Microsoft tenant id (or onmicrosoft.com domain) — overrides the
    /// `common` endpoint with a tenant-specific one. Required by IT
    /// departments that enforce tenant restrictions. M365 only. (v0.13)
    #[arg(long)]
    pub tenant: Option<String>,

    /// List stored credentials for all integrations and exit.
    /// Machine-readable when combined with global `--json`. (#140)
    #[arg(long)]
    pub list: bool,

    /// Store a bring-your-own (BYO) OAuth client secret for this integration in
    /// the OS keychain, then exit. The secret is read from stdin (never argv, to
    /// keep it out of shell history):
    /// `echo $SECRET | aware connect google-workspace --set-app-secret`. (#146)
    #[arg(long = "set-app-secret")]
    pub set_app_secret: bool,
}

#[derive(Args, Debug)]
pub struct DisconnectArgs {
    pub integration: String,

    #[arg(long)]
    pub r#as: Option<String>,
}

// ── Known integrations for --list ────────────────────────────────────────────

const KNOWN_INTEGRATIONS: &[&str] = &["trimble-connect", "microsoft-365", "google-workspace"];

// ── run_connect ───────────────────────────────────────────────────────────────

pub fn run_connect(args: ConnectArgs, ctx: &Context) -> Result<(), AwareError> {
    // --list: show all credential statuses and exit.
    if args.list {
        return run_list(ctx);
    }

    // `integration` is guaranteed Some by clap's `required_unless_present = "list"`.
    let integration = args.integration.as_deref().unwrap();

    // --set-app-secret: stash a BYO OAuth client secret (read from stdin) in the
    // keychain and exit. (#146)
    if args.set_app_secret {
        return run_set_app_secret(integration, args.r#as.as_deref(), ctx);
    }

    // Validate the integration up front. The BYO OAuth profile is loaded only on
    // the OAuth paths (--oauth / --device-code) below — token-import paths
    // (--from-file, --from-env, paste) must not be blocked by a malformed or
    // unreadable OAuth profile, since they don't use OAuth config at all.
    crate::auth::config::for_integration(integration)?;

    if args.refresh {
        let token = crate::auth::refresh::ensure_fresh(
            integration,
            args.r#as.as_deref(),
            &ctx.paths.aware_home,
        )?;
        if ctx.json {
            println!(
                "{}",
                serde_json::json!({
                    "status": "refreshed",
                    "integration": integration,
                    "expires_at": token.expires_at,
                })
            );
        } else {
            println!(
                "\u{2713} refreshed {} (expires at unix-{})",
                integration, token.expires_at
            );
        }
        return Ok(());
    }

    // Mutually exclusive: only one input source allowed.
    let modes_set = [
        args.from_file.is_some(),
        args.from_env,
        args.oauth,
        args.device_code,
    ]
    .iter()
    .filter(|b| **b)
    .count();
    if modes_set > 1 {
        return Err(AwareError::Validation(
            "at most one of --from-file, --from-env, --oauth, --device-code may be set".into(),
        ));
    }
    if args.tenant.is_some() && integration != "microsoft-365" {
        return Err(AwareError::Validation(
            "--tenant is only supported for microsoft-365 today".into(),
        ));
    }

    // Device-code needs special handling: it emits NDJSON mid-flow when --json
    // and errors need to be caught to emit a structured failure rather than
    // propagating through main's eprintln path.
    if args.device_code {
        let cfg = crate::auth::config::for_integration(integration)?
            .with_profile(&ctx.paths.aware_home, args.r#as.as_deref())?;
        let extra_scopes = parse_scopes(args.scopes.as_deref());
        let result = crate::auth::device::run_device_code_flow(
            &cfg,
            args.tenant.as_deref(),
            &extra_scopes,
            ctx.json,
        );
        match result {
            Ok(token) => {
                crate::auth::keychain::store_token(
                    &token,
                    args.r#as.as_deref(),
                    &ctx.paths.aware_home,
                )?;
                if ctx.json {
                    // Final NDJSON line: completion signal (#143).
                    println!(
                        "{}",
                        serde_json::json!({
                            "phase": "result",
                            "status": "connected",
                            "integration": integration,
                            "expires_at": token.expires_at,
                        })
                    );
                } else {
                    println!(
                        "\u{2713} stored {} OAuth token (OS keychain or ~/.aware/credentials fallback)",
                        integration
                    );
                }
                return Ok(());
            }
            Err(e) => {
                if ctx.json {
                    // Emit structured failure; don't propagate to main's stderr path.
                    let status = match &e {
                        AwareError::Validation(msg)
                            if msg.contains("access_denied")
                                || msg.contains("expired")
                                || msg.contains("denied") =>
                        {
                            "denied"
                        }
                        _ => "error",
                    };
                    println!(
                        "{}",
                        serde_json::json!({
                            "phase": "result",
                            "status": status,
                            "integration": integration,
                            "error": e.to_string(),
                        })
                    );
                    return Ok(());
                }
                return Err(e);
            }
        }
    }

    let token = if let Some(path) = &args.from_file {
        load_token_from_file(path, integration)?
    } else if args.from_env {
        load_token_from_env(integration)?
    } else if args.oauth {
        let cfg = crate::auth::config::for_integration(integration)?
            .with_profile(&ctx.paths.aware_home, args.r#as.as_deref())?;
        let extra_scopes = parse_scopes(args.scopes.as_deref());
        crate::auth::pkce::run_pkce_flow(&cfg, &extra_scopes)?
    } else {
        // Default: browser-paste flow.
        crate::auth::paste::run_paste_flow(integration)?
    };

    crate::auth::keychain::store_token(&token, args.r#as.as_deref(), &ctx.paths.aware_home)?;
    let kind = match token.source {
        TokenSource::Paste => "paste token (user-managed)",
        TokenSource::Oauth => "OAuth token",
    };
    // store_token may fall back to a credentials file on Windows (keyring blob limit);
    // print a generic success that's accurate either way.
    println!(
        "\u{2713} stored {} {} (OS keychain or ~/.aware/credentials fallback)",
        integration, kind
    );
    Ok(())
}

// ── run_set_app_secret (BYO, #146) ──────────────────────────────────────────

/// Read a BYO OAuth client secret from stdin and store it in the OS keychain
/// under the `oauth-app.<integration>[.<alias>]` slot. Never echoed; never
/// written to the profile YAML.
fn run_set_app_secret(
    integration: &str,
    alias: Option<&str>,
    ctx: &Context,
) -> Result<(), AwareError> {
    // Validate the integration is one we support before storing anything.
    crate::auth::config::for_integration(integration)?;

    let mut secret = String::new();
    std::io::Read::read_to_string(&mut std::io::stdin(), &mut secret)
        .map_err(|e| AwareError::Validation(format!("reading secret from stdin: {e}")))?;
    let secret = secret.trim();
    if secret.is_empty() {
        return Err(AwareError::Validation(
            "no secret on stdin (pipe it: echo $SECRET | aware connect <integration> --set-app-secret)".into(),
        ));
    }

    crate::auth::keychain::store_app_secret(integration, alias, secret, &ctx.paths.aware_home)?;

    if ctx.json {
        println!(
            "{}",
            serde_json::json!({
                "status": "stored",
                "integration": integration,
                "secret": "app",
            })
        );
    } else {
        println!(
            "\u{2713} stored BYO client secret for {integration} (OS keychain or ~/.aware/credentials fallback)"
        );
    }
    Ok(())
}

// ── run_disconnect ────────────────────────────────────────────────────────────

pub fn run_disconnect(args: DisconnectArgs, ctx: &Context) -> Result<(), AwareError> {
    crate::auth::keychain::delete_token(
        &args.integration,
        args.r#as.as_deref(),
        &ctx.paths.aware_home,
    )?;
    println!("\u{2713} Removed credential for {}", args.integration);
    Ok(())
}

// ── run_list ──────────────────────────────────────────────────────────────────

/// List credential status for all known integrations.
/// Used by `aware connect --list [--json]`.
fn run_list(ctx: &Context) -> Result<(), AwareError> {
    let aware_home = &ctx.paths.aware_home;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;

    if ctx.json {
        let items: Vec<serde_json::Value> = KNOWN_INTEGRATIONS
            .iter()
            .map(|integration| credential_status_json(integration, None, aware_home, now))
            .collect();
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::Value::Array(items)).unwrap()
        );
    } else {
        println!("Credentials:");
        for integration in KNOWN_INTEGRATIONS {
            print_credential_status_text(integration, None, aware_home, now);
        }
    }
    Ok(())
}

// ── Shared credential-status helpers (used by doctor.rs too) ─────────────────

/// Resolve which OAuth app is active for an integration: `first-party`,
/// `byo-profile`, or `byo-env`. Best-effort — falls back to `first-party`. (#146)
pub fn active_app_label(
    integration: &str,
    alias: Option<&str>,
    aware_home: &std::path::Path,
) -> &'static str {
    crate::auth::config::for_integration(integration)
        .ok()
        .and_then(|c| c.with_profile(aware_home, alias).ok())
        .map(|c| c.app_source_label())
        .unwrap_or("first-party")
}

/// Build the JSON object for one integration's credential status.
pub fn credential_status_json(
    integration: &str,
    alias: Option<&str>,
    aware_home: &std::path::Path,
    now: i64,
) -> serde_json::Value {
    let app = active_app_label(integration, alias, aware_home);
    match crate::auth::keychain::load_token(integration, alias, aware_home) {
        Ok(Some(token)) => {
            let (status, source, expires_in) = match token.source {
                TokenSource::Paste => ("valid".to_string(), "paste".to_string(), None),
                TokenSource::Oauth => {
                    let remaining = token.expires_at - now;
                    if remaining > 0 {
                        ("valid".to_string(), "oauth".to_string(), Some(remaining))
                    } else {
                        ("expired".to_string(), "oauth".to_string(), Some(0i64))
                    }
                }
            };
            serde_json::json!({
                "integration": integration,
                "status": status,
                "source": source,
                "expires_in_secs": expires_in,
                "app": app,
            })
        }
        Ok(None) => serde_json::json!({
            "integration": integration,
            "status": "missing",
            "source": null,
            "expires_in_secs": null,
            "app": app,
        }),
        Err(_) => serde_json::json!({
            "integration": integration,
            "status": "keyring_unavailable",
            "source": null,
            "expires_in_secs": null,
            "app": app,
        }),
    }
}

/// Print human-readable credential status for one integration.
pub fn print_credential_status_text(
    integration: &str,
    alias: Option<&str>,
    aware_home: &std::path::Path,
    now: i64,
) {
    let app = active_app_label(integration, alias, aware_home);
    match crate::auth::keychain::load_token(integration, alias, aware_home) {
        Ok(Some(token)) => match token.source {
            TokenSource::Paste => {
                println!(
                    "  \u{2713} {integration:<22} valid    paste token (user-managed) [app: {app}]"
                );
            }
            TokenSource::Oauth => {
                let remaining = token.expires_at - now;
                if remaining > 0 {
                    let mins = remaining / 60;
                    println!(
                        "  \u{2713} {integration:<22} valid    OAuth, expires in {mins}m [app: {app}]"
                    );
                } else {
                    println!(
                        "  \u{00b7} {integration:<22} expired  run: aware connect {integration} --refresh [app: {app}]"
                    );
                }
            }
        },
        Ok(None) => {
            println!(
                "  \u{2717} {integration:<22} missing  run: aware connect {integration} [app: {app}]"
            );
        }
        Err(_) => {
            println!("  ? {integration:<22} (keyring unavailable) [app: {app}]");
        }
    }
}

// ── helpers ───────────────────────────────────────────────────────────────────

fn parse_scopes(s: Option<&str>) -> Vec<String> {
    s.map(|s| {
        s.split(',')
            .map(|p| p.trim().to_string())
            .filter(|p| !p.is_empty())
            .collect()
    })
    .unwrap_or_default()
}

fn load_token_from_file(
    path: &std::path::Path,
    integration: &str,
) -> Result<crate::auth::keychain::StoredToken, AwareError> {
    use crate::auth::keychain::{StoredToken, TokenSource};

    let raw = std::fs::read_to_string(path)
        .map_err(|e| AwareError::NotFound(format!("{}: {e}", path.display())))?;
    let body = raw.trim().to_string();
    if body.is_empty() {
        return Err(AwareError::Validation(format!(
            "token file {} is empty",
            path.display()
        )));
    }
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;

    if body.starts_with('{') {
        let v: serde_json::Value = serde_json::from_str(&body)
            .map_err(|e| AwareError::Validation(format!("token JSON: {e}")))?;
        let access_token = v
            .get("access_token")
            .and_then(|x| x.as_str())
            .or_else(|| v.get("token").and_then(|x| x.as_str()))
            .ok_or_else(|| {
                AwareError::Validation(
                    "token JSON has neither access_token nor token field".into(),
                )
            })?
            .to_string();
        Ok(StoredToken {
            access_token,
            refresh_token: v
                .get("refresh_token")
                .and_then(|x| x.as_str())
                .map(String::from),
            expires_at: v.get("expires_at").and_then(|x| x.as_i64()).unwrap_or(0),
            scope: v
                .get("scope")
                .and_then(|x| x.as_str())
                .unwrap_or("")
                .to_string(),
            token_type: v
                .get("token_type")
                .and_then(|x| x.as_str())
                .unwrap_or("Bearer")
                .to_string(),
            integration: integration.to_string(),
            obtained_at: now,
            source: TokenSource::Paste,
        })
    } else {
        // Plain bearer token.
        Ok(StoredToken {
            access_token: body,
            refresh_token: None,
            expires_at: 0,
            scope: String::new(),
            token_type: "Bearer".into(),
            integration: integration.to_string(),
            obtained_at: now,
            source: TokenSource::Paste,
        })
    }
}

fn load_token_from_env(
    integration: &str,
) -> Result<crate::auth::keychain::StoredToken, AwareError> {
    use crate::auth::keychain::{StoredToken, TokenSource};

    let env_var = format!(
        "AWARE_TOKEN_{}",
        integration.to_uppercase().replace('-', "_")
    );
    let token = std::env::var(&env_var).map_err(|_| {
        AwareError::Validation(format!(
            "env var {env_var} not set; either set it or use --from-file"
        ))
    })?;
    if token.trim().is_empty() {
        return Err(AwareError::Validation(format!(
            "env var {env_var} is empty"
        )));
    }
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;
    Ok(StoredToken {
        access_token: token.trim().to_string(),
        refresh_token: None,
        expires_at: 0,
        scope: String::new(),
        token_type: "Bearer".into(),
        integration: integration.to_string(),
        obtained_at: now,
        source: TokenSource::Paste,
    })
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_file_reads_plain_bearer_token() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("token.txt");
        std::fs::write(&path, "tk_abc123\n").unwrap();
        let token = load_token_from_file(&path, "trimble-connect").unwrap();
        assert_eq!(token.access_token, "tk_abc123");
        assert_eq!(token.source, crate::auth::keychain::TokenSource::Paste);
        assert_eq!(token.expires_at, 0);
    }

    #[test]
    fn from_file_reads_json_token() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("token.json");
        std::fs::write(
            &path,
            r#"{"access_token":"tk_xyz","refresh_token":"rt_abc","expires_at":99999}"#,
        )
        .unwrap();
        let token = load_token_from_file(&path, "google-workspace").unwrap();
        assert_eq!(token.access_token, "tk_xyz");
        assert_eq!(token.refresh_token.as_deref(), Some("rt_abc"));
        assert_eq!(token.expires_at, 99999);
    }

    #[test]
    fn from_file_json_must_have_access_token() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("token.json");
        std::fs::write(&path, r#"{"other":"x"}"#).unwrap();
        let err = load_token_from_file(&path, "x").unwrap_err();
        assert!(matches!(err, AwareError::Validation(_)));
    }

    #[test]
    fn empty_file_is_validation_error() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("empty.txt");
        std::fs::write(&path, "   \n").unwrap();
        let err = load_token_from_file(&path, "x").unwrap_err();
        assert!(matches!(err, AwareError::Validation(_)));
    }

    #[test]
    fn credential_status_json_missing() {
        let tmp = tempfile::tempdir().unwrap();
        let v = credential_status_json("trimble-connect", None, tmp.path(), 0);
        assert_eq!(v["status"], "missing");
        assert_eq!(v["source"], serde_json::Value::Null);
    }

    #[test]
    fn credential_status_json_includes_app_source() {
        let tmp = tempfile::tempdir().unwrap();
        let v = credential_status_json("microsoft-365", None, tmp.path(), 0);
        assert_eq!(v["app"], "first-party");
    }

    #[test]
    fn credential_status_json_reports_byo_profile() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("oauth");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("microsoft-365.yaml"), "client_id: org-app\n").unwrap();
        let v = credential_status_json("microsoft-365", None, tmp.path(), 0);
        assert_eq!(v["app"], "byo-profile");
    }

    #[test]
    fn credential_status_json_paste_token() {
        use crate::auth::keychain::{StoredToken, TokenSource};
        let tmp = tempfile::tempdir().unwrap();
        let creds = tmp.path().join("credentials");
        std::fs::create_dir_all(&creds).unwrap();
        let token = StoredToken {
            access_token: "tk".into(),
            refresh_token: None,
            expires_at: 9_999_999_999,
            scope: "openid".into(),
            token_type: "Bearer".into(),
            integration: "trimble-connect".into(),
            obtained_at: 0,
            source: TokenSource::Paste,
        };
        std::fs::write(
            creds.join("trimble-connect.json"),
            serde_json::to_string(&token).unwrap(),
        )
        .unwrap();
        let v = credential_status_json("trimble-connect", None, tmp.path(), 0);
        assert_eq!(v["status"], "valid");
        assert_eq!(v["source"], "paste");
    }
}
