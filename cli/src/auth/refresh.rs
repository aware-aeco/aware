//! Lazy token refresh — call before any access_token read; refreshes when within 60s of expiry.

#![allow(dead_code)]

use std::io::Read;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::auth::config;
use crate::auth::keychain::{self, StoredToken};
use crate::error::AwareError;

const REFRESH_BUFFER_SECS: i64 = 60;

pub fn ensure_fresh(integration: &str, alias: Option<&str>) -> Result<StoredToken, AwareError> {
    let token = keychain::load_token(integration, alias)?
        .ok_or_else(|| AwareError::AuthExpired(integration.to_string()))?;

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    if token.expires_at > now + REFRESH_BUFFER_SECS {
        return Ok(token);
    }

    let refresh_token = token.refresh_token.as_deref().ok_or_else(|| {
        AwareError::AuthExpired(format!(
            "{integration}: no refresh_token; re-run aware connect"
        ))
    })?;
    let cfg = config::for_integration(integration)?;

    let body_params = [
        ("grant_type", "refresh_token".to_string()),
        ("refresh_token", refresh_token.to_string()),
        ("client_id", cfg.client_id()),
    ];
    let body = body_params
        .iter()
        .map(|(k, v)| format!("{}={}", urlencode(k), urlencode(v)))
        .collect::<Vec<_>>()
        .join("&");

    let resp = ureq::post(cfg.token_url)
        .set("Content-Type", "application/x-www-form-urlencoded")
        .send_string(&body)
        .map_err(|e| AwareError::Network(format!("refresh: {e}")))?;

    let mut body_str = String::new();
    resp.into_reader()
        .read_to_string(&mut body_str)
        .map_err(|e| AwareError::Network(format!("refresh body: {e}")))?;
    let token_json: serde_json::Value = serde_json::from_str(&body_str)
        .map_err(|e| AwareError::Validation(format!("refresh response: {e}")))?;

    let expires_in = token_json
        .get("expires_in")
        .and_then(|v| v.as_i64())
        .unwrap_or(3600);
    let access_token = token_json
        .get("access_token")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AwareError::Validation("refresh: missing access_token".into()))?
        .to_string();
    let new_refresh = token_json
        .get("refresh_token")
        .and_then(|v| v.as_str())
        .map(String::from)
        .or_else(|| token.refresh_token.clone());
    let scope = token_json
        .get("scope")
        .and_then(|v| v.as_str())
        .unwrap_or(&token.scope)
        .to_string();

    let new_token = StoredToken {
        access_token,
        refresh_token: new_refresh,
        expires_at: now + expires_in,
        scope,
        token_type: token.token_type.clone(),
        integration: integration.to_string(),
        obtained_at: now,
    };
    keychain::store_token(&new_token, alias)?;
    Ok(new_token)
}

fn urlencode(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_credential_returns_auth_expired() {
        let err = ensure_fresh("test-never-stored-integration-12345", None).unwrap_err();
        assert!(matches!(err, AwareError::AuthExpired(_)), "got: {err:?}");
    }
}
