//! Lazy token refresh — call before any access_token read; refreshes when within 60s of expiry.

#![allow(dead_code)]

use std::io::Read;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::auth::config;
use crate::auth::keychain::{self, StoredToken};
use crate::error::AwareError;

const REFRESH_BUFFER_SECS: i64 = 60;

pub fn ensure_fresh(
    integration: &str,
    alias: Option<&str>,
    aware_home: &std::path::Path,
) -> Result<StoredToken, AwareError> {
    let token = keychain::load_token(integration, alias, aware_home)?
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
    let cfg = config::for_integration(integration)?.with_profile(aware_home, alias)?;

    let mut body_params = vec![
        ("grant_type", "refresh_token".to_string()),
        ("refresh_token", refresh_token.to_string()),
        ("client_id", cfg.client_id()),
    ];
    // Google requires the desktop client secret on refresh too; public clients add nothing.
    if let Some(secret) = cfg.client_secret() {
        body_params.push(("client_secret", secret));
    }
    let body = body_params
        .iter()
        .map(|(k, v)| format!("{}={}", urlencode(k), urlencode(v)))
        .collect::<Vec<_>>()
        .join("&");

    let resp = ureq::post(cfg.token_url())
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
        source: token.source.clone(),
    };
    keychain::store_token(&new_token, alias, aware_home)?;
    Ok(new_token)
}

fn urlencode(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::Path;
    use std::sync::mpsc;

    use crate::auth::keychain::TokenSource;

    /// Public client (no `client_secret_env`), so nothing in these tests depends on
    /// process env — the resolved `client_secret` is always `None`.
    const INTEGRATION: &str = "trimble-connect";

    fn unix_now() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    }

    /// One-shot token endpoint on an ephemeral loopback port.
    ///
    /// Returns the URL to point a BYO profile at, plus a receiver that yields the
    /// form body the "provider" actually received — so a test can assert on the
    /// refresh *request*, not just the response handling.
    fn spawn_token_endpoint(response_body: &str) -> (String, mpsc::Receiver<String>) {
        let server = tiny_http::Server::http("127.0.0.1:0").unwrap();
        let port = server.server_addr().to_ip().unwrap().port();
        let (tx, rx) = mpsc::channel();
        let body = response_body.to_string();
        std::thread::spawn(move || {
            let Ok(mut request) = server.recv() else {
                return;
            };
            let mut received = String::new();
            let _ = request.as_reader().read_to_string(&mut received);
            let _ = tx.send(received);
            let response = tiny_http::Response::from_string(body).with_header(
                "Content-Type: application/json"
                    .parse::<tiny_http::Header>()
                    .unwrap(),
            );
            let _ = request.respond(response);
        });
        (format!("http://127.0.0.1:{port}/token"), rx)
    }

    /// Seed the credential store with a token expiring `expires_in` seconds from
    /// now. Under `cfg(test)` the keychain is file-only under `aware_home`, so
    /// each tempdir is fully isolated.
    fn seed_token(home: &Path, expires_in: i64, refresh_token: Option<&str>) {
        let now = unix_now();
        let token = StoredToken {
            access_token: "old-access".into(),
            refresh_token: refresh_token.map(String::from),
            expires_at: now + expires_in,
            scope: "openid offline_access".into(),
            token_type: "Bearer".into(),
            integration: INTEGRATION.into(),
            obtained_at: now - 3600,
            source: TokenSource::Oauth,
        };
        keychain::store_token(&token, None, home).unwrap();
    }

    fn write_profile(home: &Path, token_url: &str) {
        let dir = home.join("oauth");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join(format!("{INTEGRATION}.yaml")),
            format!("client_id: test-client\ntoken_url: {token_url}\n"),
        )
        .unwrap();
    }

    fn stored_access_token(home: &Path) -> String {
        keychain::load_token(INTEGRATION, None, home)
            .unwrap()
            .unwrap()
            .access_token
    }

    #[test]
    fn missing_credential_returns_auth_expired() {
        let tmp = tempfile::tempdir().unwrap();
        let err =
            ensure_fresh("test-never-stored-integration-12345", None, tmp.path()).unwrap_err();
        assert!(matches!(err, AwareError::AuthExpired(_)), "got: {err:?}");
    }

    #[test]
    fn token_outside_the_refresh_buffer_is_returned_untouched() {
        // The endpoint is live and would hand back a *different* token, so if the
        // early return is lost this fails with the refreshed value rather than
        // depending on a connection error.
        let tmp = tempfile::tempdir().unwrap();
        let (url, _rx) = spawn_token_endpoint(r#"{"access_token":"refreshed-access"}"#);
        write_profile(tmp.path(), &url);
        seed_token(tmp.path(), 3600, Some("rt-stored"));

        let token = ensure_fresh(INTEGRATION, None, tmp.path()).unwrap();
        assert_eq!(token.access_token, "old-access");
        assert_eq!(stored_access_token(tmp.path()), "old-access");
    }

    #[test]
    fn token_inside_the_refresh_buffer_is_refreshed_before_it_expires() {
        // 30s of life left — still valid, but inside REFRESH_BUFFER_SECS, which is
        // the whole point of refreshing eagerly rather than on first 401.
        let tmp = tempfile::tempdir().unwrap();
        let (url, _rx) = spawn_token_endpoint(r#"{"access_token":"refreshed-access"}"#);
        write_profile(tmp.path(), &url);
        seed_token(tmp.path(), 30, Some("rt-stored"));

        let token = ensure_fresh(INTEGRATION, None, tmp.path()).unwrap();
        assert_eq!(token.access_token, "refreshed-access");
    }

    #[test]
    fn refresh_posts_the_stored_refresh_token_and_grant_type() {
        let tmp = tempfile::tempdir().unwrap();
        let (url, rx) = spawn_token_endpoint(r#"{"access_token":"refreshed-access"}"#);
        write_profile(tmp.path(), &url);
        seed_token(tmp.path(), -10, Some("rt-stored"));

        ensure_fresh(INTEGRATION, None, tmp.path()).unwrap();

        let body = rx.recv().unwrap();
        let pairs: Vec<&str> = body.split('&').collect();
        assert!(pairs.contains(&"grant_type=refresh_token"), "body: {body}");
        assert!(pairs.contains(&"refresh_token=rt-stored"), "body: {body}");
        assert!(pairs.contains(&"client_id=test-client"), "body: {body}");
        // Public client — a secret must not be invented for it.
        assert!(!body.contains("client_secret"), "body: {body}");
    }

    #[test]
    fn refreshed_token_is_persisted_not_just_returned() {
        // The next process invocation reads from the store, so a refresh that is
        // only returned in-memory would re-refresh (or 401) on every call.
        let tmp = tempfile::tempdir().unwrap();
        let (url, _rx) = spawn_token_endpoint(r#"{"access_token":"refreshed-access"}"#);
        write_profile(tmp.path(), &url);
        seed_token(tmp.path(), -10, Some("rt-stored"));

        ensure_fresh(INTEGRATION, None, tmp.path()).unwrap();
        assert_eq!(stored_access_token(tmp.path()), "refreshed-access");
    }

    #[test]
    fn response_without_a_refresh_token_keeps_the_existing_one() {
        // Microsoft and Trimble omit refresh_token on renewal. Dropping it here
        // would strand the account after one hour with no way back but a browser.
        let tmp = tempfile::tempdir().unwrap();
        let (url, _rx) = spawn_token_endpoint(r#"{"access_token":"refreshed-access"}"#);
        write_profile(tmp.path(), &url);
        seed_token(tmp.path(), -10, Some("rt-stored"));

        let token = ensure_fresh(INTEGRATION, None, tmp.path()).unwrap();
        assert_eq!(token.refresh_token.as_deref(), Some("rt-stored"));
        assert_eq!(
            keychain::load_token(INTEGRATION, None, tmp.path())
                .unwrap()
                .unwrap()
                .refresh_token
                .as_deref(),
            Some("rt-stored")
        );
    }

    #[test]
    fn rotated_refresh_token_in_the_response_replaces_the_stored_one() {
        // Google rotates. Keeping the old one would fail the *next* refresh.
        let tmp = tempfile::tempdir().unwrap();
        let (url, _rx) = spawn_token_endpoint(
            r#"{"access_token":"refreshed-access","refresh_token":"rt-rotated"}"#,
        );
        write_profile(tmp.path(), &url);
        seed_token(tmp.path(), -10, Some("rt-stored"));

        let token = ensure_fresh(INTEGRATION, None, tmp.path()).unwrap();
        assert_eq!(token.refresh_token.as_deref(), Some("rt-rotated"));
    }

    #[test]
    fn response_without_scope_keeps_the_stored_scope() {
        let tmp = tempfile::tempdir().unwrap();
        let (url, _rx) = spawn_token_endpoint(r#"{"access_token":"refreshed-access"}"#);
        write_profile(tmp.path(), &url);
        seed_token(tmp.path(), -10, Some("rt-stored"));

        let token = ensure_fresh(INTEGRATION, None, tmp.path()).unwrap();
        assert_eq!(token.scope, "openid offline_access");
    }

    #[test]
    fn response_scope_overrides_the_stored_scope() {
        let tmp = tempfile::tempdir().unwrap();
        let (url, _rx) =
            spawn_token_endpoint(r#"{"access_token":"refreshed-access","scope":"openid"}"#);
        write_profile(tmp.path(), &url);
        seed_token(tmp.path(), -10, Some("rt-stored"));

        let token = ensure_fresh(INTEGRATION, None, tmp.path()).unwrap();
        assert_eq!(token.scope, "openid");
    }

    #[test]
    fn expires_at_follows_the_response_expires_in() {
        let tmp = tempfile::tempdir().unwrap();
        let (url, _rx) =
            spawn_token_endpoint(r#"{"access_token":"refreshed-access","expires_in":7200}"#);
        write_profile(tmp.path(), &url);
        seed_token(tmp.path(), -10, Some("rt-stored"));

        let before = unix_now();
        let token = ensure_fresh(INTEGRATION, None, tmp.path()).unwrap();
        assert!(
            (before + 7200..=unix_now() + 7200).contains(&token.expires_at),
            "expires_at {} not ~{}s ahead",
            token.expires_at,
            7200
        );
    }

    #[test]
    fn missing_expires_in_defaults_to_one_hour() {
        // Trimble omits expires_in. A 0 default would make every token look
        // permanently stale and refresh on every single invocation.
        let tmp = tempfile::tempdir().unwrap();
        let (url, _rx) = spawn_token_endpoint(r#"{"access_token":"refreshed-access"}"#);
        write_profile(tmp.path(), &url);
        seed_token(tmp.path(), -10, Some("rt-stored"));

        let before = unix_now();
        let token = ensure_fresh(INTEGRATION, None, tmp.path()).unwrap();
        assert!(
            (before + 3600..=unix_now() + 3600).contains(&token.expires_at),
            "expires_at {} not ~3600s ahead",
            token.expires_at
        );
    }

    #[test]
    fn response_without_an_access_token_errors_and_leaves_the_store_intact() {
        // A 200 body carrying only an OAuth error must not overwrite the stored
        // credential with an empty access token.
        let tmp = tempfile::tempdir().unwrap();
        let (url, _rx) = spawn_token_endpoint(r#"{"error":"invalid_grant"}"#);
        write_profile(tmp.path(), &url);
        seed_token(tmp.path(), -10, Some("rt-stored"));

        let err = ensure_fresh(INTEGRATION, None, tmp.path()).unwrap_err();
        assert!(matches!(err, AwareError::Validation(_)), "got: {err:?}");
        assert_eq!(stored_access_token(tmp.path()), "old-access");
    }

    #[test]
    fn non_json_response_is_a_validation_error_not_a_network_error() {
        // An HTML error page from a proxy is a malformed *response*, not a
        // transport failure — the distinction drives the operator-facing message.
        let tmp = tempfile::tempdir().unwrap();
        let (url, _rx) = spawn_token_endpoint("<html>gateway error</html>");
        write_profile(tmp.path(), &url);
        seed_token(tmp.path(), -10, Some("rt-stored"));

        let err = ensure_fresh(INTEGRATION, None, tmp.path()).unwrap_err();
        assert!(matches!(err, AwareError::Validation(_)), "got: {err:?}");
    }

    #[test]
    fn expired_token_without_a_refresh_token_says_to_reconnect() {
        // No refresh_token (consent granted without offline_access) — there is
        // nothing to POST, so this must fail fast with an actionable message
        // rather than sending `refresh_token=` to the provider.
        let tmp = tempfile::tempdir().unwrap();
        let (url, _rx) = spawn_token_endpoint(r#"{"access_token":"refreshed-access"}"#);
        write_profile(tmp.path(), &url);
        seed_token(tmp.path(), -10, None);

        let err = ensure_fresh(INTEGRATION, None, tmp.path()).unwrap_err();
        let AwareError::AuthExpired(msg) = err else {
            panic!("expected AuthExpired, got {err:?}");
        };
        assert!(msg.contains("no refresh_token"), "msg: {msg}");
        assert!(msg.contains("aware connect"), "msg: {msg}");
    }
}
