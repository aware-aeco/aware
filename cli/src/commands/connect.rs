//! `aware connect ...` / `aware disconnect ...` — credential management.
//!
//! Phase v0.4. Browser-paste flow (default) + OAuth/PKCE (--oauth) + non-interactive paths.

use clap::Args;

use crate::context::Context;
use crate::error::AwareError;

#[derive(Args, Debug)]
pub struct ConnectArgs {
    /// Integration id (e.g. trimble-connect).
    pub integration: String,

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

    // Mutually exclusive: only one input source allowed
    let modes_set = [args.from_file.is_some(), args.from_env, args.oauth]
        .iter()
        .filter(|b| **b)
        .count();
    if modes_set > 1 {
        return Err(AwareError::Validation(
            "at most one of --from-file, --from-env, --oauth may be set".into(),
        ));
    }

    let token = if let Some(path) = &args.from_file {
        load_token_from_file(path, &args.integration)?
    } else if args.from_env {
        load_token_from_env(&args.integration)?
    } else if args.oauth {
        let extra_scopes = parse_scopes(args.scopes.as_deref());
        crate::auth::pkce::run_pkce_flow(&cfg, &extra_scopes)?
    } else {
        // Default: browser-paste flow
        crate::auth::paste::run_paste_flow(&args.integration)?
    };

    crate::auth::keychain::store_token(&token, args.r#as.as_deref())?;
    let kind = match token.source {
        crate::auth::keychain::TokenSource::Paste => "paste token (user-managed)",
        crate::auth::keychain::TokenSource::Oauth => "OAuth token",
    };
    println!(
        "\u{2713} stored {} {} encrypted to OS keychain",
        args.integration, kind
    );
    Ok(())
}

pub fn run_disconnect(args: DisconnectArgs, _ctx: &Context) -> Result<(), AwareError> {
    crate::auth::keychain::delete_token(&args.integration, args.r#as.as_deref())?;
    println!("\u{2713} Removed credential for {}", args.integration);
    Ok(())
}

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
    use std::time::{SystemTime, UNIX_EPOCH};

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
        .unwrap()
        .as_secs() as i64;

    if body.starts_with('{') {
        // JSON: try to parse fields
        let v: serde_json::Value = serde_json::from_str(&body)
            .map_err(|e| AwareError::Validation(format!("token JSON: {e}")))?;
        let access_token = v
            .get("access_token")
            .and_then(|x| x.as_str())
            .or_else(|| v.get("token").and_then(|x| x.as_str()))
            .ok_or_else(|| {
                AwareError::Validation("token JSON has neither access_token nor token field".into())
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
        // Plain bearer token
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
    use std::time::{SystemTime, UNIX_EPOCH};

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
        .unwrap()
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
}
