//! Lazy token refresh — call before any access_token read; refreshes when within 60s of expiry.

use std::io::Read;
use std::time::{Duration, Instant};

use crate::auth::config;
use crate::auth::keychain::{self, StoredToken};
use crate::auth::token_response::TokenResponse;
use crate::auth::urlencode;
use crate::error::AwareError;

const REFRESH_BUFFER_SECS: i64 = 60;
const REFRESH_DEADLINE: Duration = Duration::from_secs(30);
const MAX_REFRESH_RESPONSE_BYTES: usize = 64 * 1024;

pub fn ensure_fresh(
    integration: &str,
    alias: Option<&str>,
    aware_home: &std::path::Path,
) -> Result<StoredToken, AwareError> {
    let loaded = keychain::load_token_for_refresh(integration, alias, aware_home)?
        .ok_or_else(|| AwareError::AuthExpired(integration.to_string()))?;
    if token_is_fresh(&loaded.token)? {
        return Ok(loaded.token);
    }
    let cfg = config::for_integration(integration)?.with_profile(aware_home, alias)?;
    refresh_loaded(integration, aware_home, loaded, &cfg)
}

/// Refresh using one already-resolved OAuth configuration snapshot.
///
/// Security-sensitive callers can validate a provider endpoint and then pass the
/// same value here, so a profile rewrite cannot swap the destination between the
/// validation and the request.
pub(crate) fn ensure_fresh_with_config(
    integration: &str,
    alias: Option<&str>,
    aware_home: &std::path::Path,
    cfg: &config::IntegrationConfig,
) -> Result<StoredToken, AwareError> {
    if cfg.id != integration {
        return Err(AwareError::Validation(format!(
            "OAuth config for {} cannot refresh {integration}",
            cfg.id
        )));
    }
    let loaded = keychain::load_token_for_refresh(integration, alias, aware_home)?
        .ok_or_else(|| AwareError::AuthExpired(integration.to_string()))?;
    if token_is_fresh(&loaded.token)? {
        return Ok(loaded.token);
    }
    refresh_loaded(integration, aware_home, loaded, cfg)
}

fn token_is_fresh(token: &StoredToken) -> Result<bool, AwareError> {
    let now = super::unix_now_secs()?;
    Ok(token.expires_at > now + REFRESH_BUFFER_SECS)
}

fn refresh_loaded(
    integration: &str,
    aware_home: &std::path::Path,
    loaded: keychain::RefreshTokenLoad,
    cfg: &config::IntegrationConfig,
) -> Result<StoredToken, AwareError> {
    let token = loaded.token;
    let now = super::unix_now_secs()?;
    let refresh_token = token.refresh_token.as_deref().ok_or_else(|| {
        AwareError::AuthExpired(format!(
            "{integration}: no refresh_token; re-run aware connect"
        ))
    })?;
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

    // OAuth refresh carries the long-lived refresh token and must not follow a
    // provider redirect to a different origin. Bound every phase as well as the
    // total request so a send preflight cannot hang indefinitely (#495).
    let agent = ureq::AgentBuilder::new()
        .redirects(0)
        .timeout_connect(Duration::from_secs(10))
        .timeout_write(Duration::from_secs(10))
        .timeout_read(Duration::from_secs(10))
        .timeout(REFRESH_DEADLINE)
        .build();
    let started = Instant::now();
    let resp = agent
        .post(cfg.token_url())
        .set("Content-Type", "application/x-www-form-urlencoded")
        .send_string(&body)
        .map_err(|e| AwareError::Network(format!("refresh: {e}")))?;

    let body = read_refresh_body(resp.into_reader(), started, REFRESH_DEADLINE)?;
    let refreshed = TokenResponse::new(
        serde_json::from_slice(&body)
            .map_err(|e| AwareError::Validation(format!("refresh response: {e}")))?,
    );

    // Not `into_new_credential`: a refresh starts from a credential that already
    // exists, and RFC 6749 §6 lets the reply omit `refresh_token` and `scope` —
    // omitted means "unchanged", so each falls back to what `token` already holds
    // rather than to the empty value a fresh grant would take. `token_type` and
    // `source` are carried over wholesale for the same reason.
    let access_token = refreshed
        .access_token()
        .ok_or_else(|| AwareError::Validation("refresh: missing access_token".into()))?;
    let old_scope = keychain::normalized_scope_string(&token.scope);
    let new_scope = keychain::normalized_scope_string(refreshed.scope().unwrap_or(&token.scope));
    let generation = if new_scope == old_scope {
        token.generation.clone()
    } else {
        Some(keychain::new_credential_generation())
    };
    let new_token = StoredToken {
        access_token,
        refresh_token: refreshed
            .refresh_token()
            .or_else(|| token.refresh_token.clone()),
        expires_at: refreshed.expires_at(now),
        scope: new_scope,
        token_type: token.token_type.clone(),
        integration: integration.to_string(),
        obtained_at: now,
        generation,
        source: token.source.clone(),
    };
    keychain::compare_and_store_token(&loaded.snapshot, &new_token, aware_home)?;
    Ok(new_token)
}

fn read_refresh_body(
    reader: Box<dyn Read + Send + Sync + 'static>,
    started: Instant,
    deadline: Duration,
) -> Result<Vec<u8>, AwareError> {
    let remaining = deadline.checked_sub(started.elapsed()).ok_or_else(|| {
        AwareError::Network("refresh deadline exceeded before reading response body".into())
    })?;
    crate::http_body::read_with_deadline(
        reader,
        remaining,
        MAX_REFRESH_RESPONSE_BYTES,
        "aware-oauth-refresh-response-reader",
    )
    .map_err(|error| AwareError::Network(format!("refresh body: {error}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::path::{Path, PathBuf};
    use std::sync::mpsc;
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::auth::keychain::TokenSource;

    /// Public client (no `client_secret_env`), so nothing in these tests depends on
    /// process env — the resolved `client_secret` is always `None`.
    const INTEGRATION: &str = "trimble-connect";

    struct SlowDribble {
        remaining: usize,
    }

    impl Read for SlowDribble {
        fn read(&mut self, buffer: &mut [u8]) -> std::io::Result<usize> {
            if self.remaining == 0 || buffer.is_empty() {
                return Ok(0);
            }
            std::thread::sleep(Duration::from_millis(10));
            buffer[0] = b'x';
            self.remaining -= 1;
            Ok(1)
        }
    }

    fn unix_now() -> i64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    }

    /// What the "provider" actually received. Recorded so a test can assert on the
    /// refresh *request* — method, framing and encoding included — and not just on
    /// how the response is parsed.
    struct RecordedRequest {
        method: String,
        content_type: Option<String>,
        body: String,
    }

    impl RecordedRequest {
        fn pairs(&self) -> Vec<&str> {
            self.body.split('&').collect()
        }
    }

    /// One-shot token endpoint on an ephemeral loopback port.
    ///
    /// Dropping it unblocks the worker, so the two tests that deliberately never
    /// reach the network don't strand a thread parked in `recv()` holding a
    /// listening socket for the rest of the test binary's run.
    struct TokenEndpoint {
        url: String,
        requests: mpsc::Receiver<RecordedRequest>,
        server: std::sync::Arc<tiny_http::Server>,
    }

    impl Drop for TokenEndpoint {
        fn drop(&mut self) {
            self.server.unblock();
        }
    }

    fn spawn_token_endpoint(response_body: &str) -> TokenEndpoint {
        spawn_token_endpoint_with_release(response_body, None)
    }

    #[test]
    fn refresh_body_honors_the_total_deadline_after_headers() {
        let started = Instant::now();
        let error = read_refresh_body(
            Box::new(SlowDribble { remaining: 20 }),
            started,
            Duration::from_millis(35),
        )
        .unwrap_err();
        assert!(error.to_string().contains("deadline exceeded"), "{error}");
        assert!(
            started.elapsed() < Duration::from_millis(150),
            "refresh followed the dribbling body instead of its total deadline"
        );
    }

    #[test]
    fn refresh_body_is_size_bounded() {
        let error = read_refresh_body(
            Box::new(std::io::Cursor::new(vec![
                b'x';
                MAX_REFRESH_RESPONSE_BYTES + 1
            ])),
            Instant::now(),
            Duration::from_secs(1),
        )
        .unwrap_err();
        assert!(error.to_string().contains("byte limit"), "{error}");
    }

    fn spawn_token_endpoint_with_release(
        response_body: &str,
        release: Option<mpsc::Receiver<()>>,
    ) -> TokenEndpoint {
        let server = std::sync::Arc::new(tiny_http::Server::http("127.0.0.1:0").unwrap());
        let port = server.server_addr().to_ip().unwrap().port();
        let (tx, requests) = mpsc::channel();
        let body = response_body.to_string();
        let worker = std::sync::Arc::clone(&server);
        std::thread::spawn(move || {
            let Ok(mut request) = worker.recv() else {
                return; // unblocked on drop
            };
            let mut received = String::new();
            let _ = request.as_reader().read_to_string(&mut received);
            let _ = tx.send(RecordedRequest {
                method: request.method().to_string(),
                content_type: request
                    .headers()
                    .iter()
                    .find(|h| h.field.equiv("Content-Type"))
                    .map(|h| h.value.as_str().to_string()),
                body: received,
            });
            if let Some(release) = release {
                let _ = release.recv();
            }
            let response = tiny_http::Response::from_string(body).with_header(
                "Content-Type: application/json"
                    .parse::<tiny_http::Header>()
                    .unwrap(),
            );
            let _ = request.respond(response);
        });
        TokenEndpoint {
            url: format!("http://127.0.0.1:{port}/token"),
            requests,
            server,
        }
    }

    /// Removes the account its test seeded, when that test goes out of scope.
    ///
    /// The temporary `AWARE_HOME` isolates the *file* store only. The OS keychain
    /// is process-global and not scoped by it, so under `AWARE_TEST_KEYRING=1`
    /// (`keychain.rs:84`) every one of these tests writes a real keychain entry —
    /// and without this guard leaves it there, one per test per run.
    /// [`keychain::delete_token`] is idempotent and clears both stores, so on the
    /// default file-backed path this is a harmless no-op.
    ///
    /// `#[must_use]` is the gate, not decoration: an unbound `seed_token(..)` drops
    /// the guard immediately and deletes the credential before the test can use it,
    /// so the same lint that makes a future test bind the guard is what keeps the
    /// cleanup honest. `cargo clippy -- -D warnings` turns it into a build failure.
    ///
    /// One measured limit, so nobody reads more into this than it delivers. At
    /// `--test-threads=1` cleanup is total: 16 accounts seeded, 0 left behind.
    /// At the default thread count on Windows, 4–9 targets per run survive a
    /// `delete_token` that returned `Ok(())` — and a `load_token` immediately after
    /// reports them absent, so the entry is gone as far as this crate can observe
    /// while `cmdkey /list` still shows the target. That is a Credential Manager
    /// artifact under concurrent writes to one service, not something the tests can
    /// reach: a verify-and-retry loop was tried here and measured to change nothing,
    /// so it was removed rather than shipped as reassurance. It does not touch
    /// `aware connect disconnect`, which deletes once from a single thread — the
    /// single-threaded result above is that path.
    #[must_use]
    struct SeededToken {
        home: PathBuf,
        alias: String,
    }

    impl Drop for SeededToken {
        fn drop(&mut self) {
            // Best-effort: `drop` must never panic, and a cleanup failure must not
            // mask the test's own verdict.
            let _ = keychain::delete_token(INTEGRATION, Some(&self.alias), &self.home);
        }
    }

    /// Seed the credential store with a token expiring `expires_in` seconds from now.
    ///
    /// Every test passes its own `alias`, which does two things: it exercises the
    /// aliased path both production callers use (`--as <name>` —
    /// `runtime/invoker.rs`, `commands/connect.rs`), and it keeps the account name
    /// unique per test, so parallel tests cannot race on one account.
    ///
    /// Every alias carries the `awaretest-` prefix because `INTEGRATION` is a real
    /// integration: under `AWARE_TEST_KEYRING=1` the account written is the
    /// process-global `trimble-connect.<alias>`, so a bare name like `personal`
    /// would seed over — and then, on cleanup, delete — a developer's own
    /// credential. The prefix is what makes that collision implausible; the
    /// returned [`SeededToken`] is what stops the entry outliving the test.
    fn seed_token(
        home: &Path,
        alias: &str,
        expires_in: i64,
        refresh_token: Option<&str>,
    ) -> SeededToken {
        let now = unix_now();
        let token = StoredToken {
            access_token: "old-access".into(),
            refresh_token: refresh_token.map(String::from),
            expires_at: now + expires_in,
            scope: "openid offline_access".into(),
            token_type: "Bearer".into(),
            integration: INTEGRATION.into(),
            obtained_at: now - 3600,
            generation: Some(keychain::new_credential_generation()),
            source: TokenSource::Oauth,
        };
        keychain::store_token(&token, Some(alias), home).unwrap();
        SeededToken {
            home: home.to_path_buf(),
            alias: alias.to_string(),
        }
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

    fn stored(home: &Path, alias: &str) -> Option<StoredToken> {
        keychain::load_token(INTEGRATION, Some(alias), home).unwrap()
    }

    fn stored_access_token(home: &Path, alias: &str) -> String {
        stored(home, alias).unwrap().access_token
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
        let endpoint = spawn_token_endpoint(r#"{"access_token":"refreshed-access"}"#);
        write_profile(tmp.path(), &endpoint.url);
        let _seeded = seed_token(
            tmp.path(),
            "awaretest-outside-buffer",
            3600,
            Some("rt-stored"),
        );

        let token =
            ensure_fresh(INTEGRATION, Some("awaretest-outside-buffer"), tmp.path()).unwrap();
        assert_eq!(token.access_token, "old-access");
        assert_eq!(
            stored_access_token(tmp.path(), "awaretest-outside-buffer"),
            "old-access"
        );
    }

    #[test]
    fn token_inside_the_refresh_buffer_is_refreshed_before_it_expires() {
        // 30s of life left — still valid, but inside REFRESH_BUFFER_SECS, which is
        // the whole point of refreshing eagerly rather than on first 401.
        let tmp = tempfile::tempdir().unwrap();
        let endpoint = spawn_token_endpoint(r#"{"access_token":"refreshed-access"}"#);
        write_profile(tmp.path(), &endpoint.url);
        let _seeded = seed_token(tmp.path(), "awaretest-inside-buffer", 30, Some("rt-stored"));

        let token = ensure_fresh(INTEGRATION, Some("awaretest-inside-buffer"), tmp.path()).unwrap();
        assert_eq!(token.access_token, "refreshed-access");
    }

    #[test]
    fn an_already_validated_config_cannot_be_replaced_before_refresh() {
        let tmp = tempfile::tempdir().unwrap();
        let validated_endpoint = spawn_token_endpoint(r#"{"access_token":"validated-endpoint"}"#);
        let replacement_endpoint =
            spawn_token_endpoint(r#"{"access_token":"replacement-endpoint"}"#);
        write_profile(tmp.path(), &validated_endpoint.url);
        let cfg = config::for_integration(INTEGRATION)
            .unwrap()
            .with_profile(tmp.path(), None)
            .unwrap();
        write_profile(tmp.path(), &replacement_endpoint.url);
        let alias = "awaretest-config-snapshot";
        let _seeded = seed_token(tmp.path(), alias, -10, Some("rt-stored"));

        let token = ensure_fresh_with_config(INTEGRATION, Some(alias), tmp.path(), &cfg).unwrap();

        assert_eq!(token.access_token, "validated-endpoint");
        validated_endpoint.requests.recv().unwrap();
        assert!(replacement_endpoint.requests.try_recv().is_err());
    }

    #[test]
    fn stale_refresh_response_cannot_overwrite_a_concurrent_rotation() {
        let tmp = tempfile::tempdir().unwrap();
        let (release_tx, release_rx) = mpsc::channel();
        let endpoint = spawn_token_endpoint_with_release(
            r#"{"access_token":"stale-refreshed-access"}"#,
            Some(release_rx),
        );
        write_profile(tmp.path(), &endpoint.url);
        let alias = "awaretest-refresh-cas";
        let _seeded = seed_token(tmp.path(), alias, -10, Some("rt-stored"));
        let original_generation = stored(tmp.path(), alias).unwrap().generation;

        let home = tmp.path().to_path_buf();
        let refresh = std::thread::spawn(move || ensure_fresh(INTEGRATION, Some(alias), &home));

        // The request proves refresh captured its old credential and is now
        // waiting on the provider, without holding the account lock.
        endpoint.requests.recv().unwrap();
        let replacement = StoredToken {
            access_token: "new-connect-token".into(),
            refresh_token: Some("new-connect-refresh".into()),
            expires_at: unix_now() + 3600,
            scope: "openid profile".into(),
            token_type: "Bearer".into(),
            integration: INTEGRATION.into(),
            obtained_at: unix_now(),
            // Deliberately retain the old generation: comparing generation alone
            // would miss this rotation and let the stale response overwrite it.
            generation: original_generation,
            source: TokenSource::Oauth,
        };
        keychain::store_token(&replacement, Some(alias), tmp.path()).unwrap();
        release_tx.send(()).unwrap();

        let err = refresh.join().unwrap().unwrap_err();
        assert!(
            matches!(&err, AwareError::Conflict(message) if message.contains("changed") && message.contains("retry")),
            "expected a non-secret retryable conflict, got {err:?}"
        );
        let final_token = stored(tmp.path(), alias).unwrap();
        assert_eq!(final_token, replacement);
        assert!(!err.to_string().contains("new-connect-token"));
        assert!(!err.to_string().contains("stale-refreshed-access"));
    }

    #[test]
    fn refresh_posts_the_stored_refresh_token_and_grant_type() {
        let tmp = tempfile::tempdir().unwrap();
        let endpoint = spawn_token_endpoint(r#"{"access_token":"refreshed-access"}"#);
        write_profile(tmp.path(), &endpoint.url);
        // Reserved characters on purpose: real refresh tokens routinely contain
        // `/` and `+`, and an unencoded one would split the form body apart.
        let _seeded = seed_token(
            tmp.path(),
            "awaretest-request-shape",
            -10,
            Some("rt/with+chars=x"),
        );

        ensure_fresh(INTEGRATION, Some("awaretest-request-shape"), tmp.path()).unwrap();

        let req = endpoint.requests.recv().unwrap();
        let body = &req.body;
        // RFC 6749 §4.1.3: a form-encoded POST. A GET, or a body sent without the
        // form Content-Type, is rejected by every real provider.
        assert_eq!(req.method, "POST");
        assert_eq!(
            req.content_type.as_deref(),
            Some("application/x-www-form-urlencoded")
        );
        let pairs = req.pairs();
        assert!(pairs.contains(&"grant_type=refresh_token"), "body: {body}");
        assert!(
            pairs.contains(&"refresh_token=rt%2Fwith%2Bchars%3Dx"),
            "body: {body}"
        );
        assert!(pairs.contains(&"client_id=test-client"), "body: {body}");
        // Public client — a secret must not be invented for it.
        assert!(!body.contains("client_secret"), "body: {body}");
    }

    #[test]
    fn refresh_writes_back_to_the_aliased_slot_only() {
        // Both production callers pass an alias (`--as <name>`). Dropping it on the
        // write-back would overwrite the *default* account with the alias's token:
        // the default credential is destroyed and the alias never updates, so it
        // re-refreshes forever.
        let tmp = tempfile::tempdir().unwrap();
        let endpoint = spawn_token_endpoint(r#"{"access_token":"refreshed-access"}"#);
        write_profile(tmp.path(), &endpoint.url);
        let _seeded = seed_token(tmp.path(), "awaretest-alias-scope", -10, Some("rt-alias"));
        let _seeded_neighbour = seed_token(
            tmp.path(),
            "awaretest-alias-scope-other",
            86_400,
            Some("rt-other"),
        );

        ensure_fresh(INTEGRATION, Some("awaretest-alias-scope"), tmp.path()).unwrap();

        assert_eq!(
            stored_access_token(tmp.path(), "awaretest-alias-scope"),
            "refreshed-access"
        );
        // A neighbouring account must be left exactly as it was.
        assert_eq!(
            stored_access_token(tmp.path(), "awaretest-alias-scope-other"),
            "old-access"
        );
    }

    #[test]
    fn refresh_carries_over_token_type_and_source_and_restamps_obtained_at() {
        // These three ride along untouched by the response body, so nothing else
        // in the suite notices if they are dropped — yet `source` drives the
        // first-party/paste labelling in `aware connect --list`, and a stale
        // `obtained_at` misreports the credential's age.
        let tmp = tempfile::tempdir().unwrap();
        let endpoint = spawn_token_endpoint(r#"{"access_token":"refreshed-access"}"#);
        write_profile(tmp.path(), &endpoint.url);
        let _seeded = seed_token(tmp.path(), "awaretest-carry-over", -10, Some("rt-stored"));
        let seeded_obtained_at = stored(tmp.path(), "awaretest-carry-over")
            .unwrap()
            .obtained_at;

        let before = unix_now();
        let token = ensure_fresh(INTEGRATION, Some("awaretest-carry-over"), tmp.path()).unwrap();

        assert_eq!(token.token_type, "Bearer");
        assert_eq!(token.source, TokenSource::Oauth);
        assert_eq!(token.integration, INTEGRATION);
        assert!(
            token.obtained_at > seeded_obtained_at,
            "obtained_at {} not restamped (seeded {seeded_obtained_at})",
            token.obtained_at
        );
        assert!(
            (before..=unix_now()).contains(&token.obtained_at),
            "obtained_at {} outside the call window",
            token.obtained_at
        );
    }

    #[test]
    fn refreshed_token_is_persisted_not_just_returned() {
        // The next process invocation reads from the store, so a refresh that is
        // only returned in-memory would re-refresh (or 401) on every call.
        let tmp = tempfile::tempdir().unwrap();
        let endpoint = spawn_token_endpoint(r#"{"access_token":"refreshed-access"}"#);
        write_profile(tmp.path(), &endpoint.url);
        let _seeded = seed_token(tmp.path(), "awaretest-persisted", -10, Some("rt-stored"));

        ensure_fresh(INTEGRATION, Some("awaretest-persisted"), tmp.path()).unwrap();
        assert_eq!(
            stored_access_token(tmp.path(), "awaretest-persisted"),
            "refreshed-access"
        );
    }

    #[test]
    fn response_without_a_refresh_token_keeps_the_existing_one() {
        // Microsoft and Trimble omit refresh_token on renewal. Dropping it here
        // would strand the account after one hour with no way back but a browser.
        let tmp = tempfile::tempdir().unwrap();
        let endpoint = spawn_token_endpoint(r#"{"access_token":"refreshed-access"}"#);
        write_profile(tmp.path(), &endpoint.url);
        let _seeded = seed_token(tmp.path(), "awaretest-keep-rt", -10, Some("rt-stored"));

        let token = ensure_fresh(INTEGRATION, Some("awaretest-keep-rt"), tmp.path()).unwrap();
        assert_eq!(token.refresh_token.as_deref(), Some("rt-stored"));
        assert_eq!(
            stored(tmp.path(), "awaretest-keep-rt")
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
        let endpoint = spawn_token_endpoint(
            r#"{"access_token":"refreshed-access","refresh_token":"rt-rotated"}"#,
        );
        write_profile(tmp.path(), &endpoint.url);
        let _seeded = seed_token(tmp.path(), "awaretest-rotate-rt", -10, Some("rt-stored"));

        let token = ensure_fresh(INTEGRATION, Some("awaretest-rotate-rt"), tmp.path()).unwrap();
        assert_eq!(token.refresh_token.as_deref(), Some("rt-rotated"));
    }

    #[test]
    fn response_without_scope_keeps_the_stored_scope() {
        let tmp = tempfile::tempdir().unwrap();
        let endpoint = spawn_token_endpoint(r#"{"access_token":"refreshed-access"}"#);
        write_profile(tmp.path(), &endpoint.url);
        let _seeded = seed_token(tmp.path(), "awaretest-keep-scope", -10, Some("rt-stored"));
        let old_generation = stored(tmp.path(), "awaretest-keep-scope")
            .unwrap()
            .generation;

        let token = ensure_fresh(INTEGRATION, Some("awaretest-keep-scope"), tmp.path()).unwrap();
        assert_eq!(token.scope, "offline_access openid");
        assert_eq!(token.generation, old_generation);
    }

    #[test]
    fn response_scope_overrides_the_stored_scope() {
        let tmp = tempfile::tempdir().unwrap();
        let endpoint =
            spawn_token_endpoint(r#"{"access_token":"refreshed-access","scope":"openid"}"#);
        write_profile(tmp.path(), &endpoint.url);
        let _seeded = seed_token(tmp.path(), "awaretest-new-scope", -10, Some("rt-stored"));

        let old_generation = stored(tmp.path(), "awaretest-new-scope")
            .unwrap()
            .generation;
        let token = ensure_fresh(INTEGRATION, Some("awaretest-new-scope"), tmp.path()).unwrap();
        assert_eq!(token.scope, "openid");
        assert_ne!(token.generation, old_generation);
    }

    #[test]
    fn semantically_identical_reordered_scope_preserves_generation() {
        let tmp = tempfile::tempdir().unwrap();
        let endpoint = spawn_token_endpoint(
            r#"{"access_token":"refreshed-access","scope":"offline_access openid openid"}"#,
        );
        write_profile(tmp.path(), &endpoint.url);
        let _seeded = seed_token(
            tmp.path(),
            "awaretest-normalized-scope",
            -10,
            Some("rt-stored"),
        );
        let old_generation = stored(tmp.path(), "awaretest-normalized-scope")
            .unwrap()
            .generation;

        let token =
            ensure_fresh(INTEGRATION, Some("awaretest-normalized-scope"), tmp.path()).unwrap();
        assert_eq!(token.scope, "offline_access openid");
        assert_eq!(token.generation, old_generation);
    }

    #[test]
    fn expires_at_follows_the_response_expires_in() {
        let tmp = tempfile::tempdir().unwrap();
        let endpoint =
            spawn_token_endpoint(r#"{"access_token":"refreshed-access","expires_in":7200}"#);
        write_profile(tmp.path(), &endpoint.url);
        let _seeded = seed_token(tmp.path(), "awaretest-expires-in", -10, Some("rt-stored"));

        let before = unix_now();
        let token = ensure_fresh(INTEGRATION, Some("awaretest-expires-in"), tmp.path()).unwrap();
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
        let endpoint = spawn_token_endpoint(r#"{"access_token":"refreshed-access"}"#);
        write_profile(tmp.path(), &endpoint.url);
        let _seeded = seed_token(
            tmp.path(),
            "awaretest-default-expiry",
            -10,
            Some("rt-stored"),
        );

        let before = unix_now();
        let token =
            ensure_fresh(INTEGRATION, Some("awaretest-default-expiry"), tmp.path()).unwrap();
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
        let endpoint = spawn_token_endpoint(r#"{"error":"invalid_grant"}"#);
        write_profile(tmp.path(), &endpoint.url);
        let _seeded = seed_token(
            tmp.path(),
            "awaretest-no-access-token",
            -10,
            Some("rt-stored"),
        );

        let err =
            ensure_fresh(INTEGRATION, Some("awaretest-no-access-token"), tmp.path()).unwrap_err();
        assert!(matches!(err, AwareError::Validation(_)), "got: {err:?}");
        assert_eq!(
            stored_access_token(tmp.path(), "awaretest-no-access-token"),
            "old-access"
        );
    }

    #[test]
    fn non_json_response_is_a_validation_error_not_a_network_error() {
        // A 200 that isn't JSON at all (a proxy's HTML error page) is a malformed
        // *response*, not a transport failure.
        //
        // NOTE: this pins the 200-with-garbage case only. An HTTP >=400 from the
        // token endpoint — including the common `invalid_grant` on a revoked
        // refresh token — is currently blanket-mapped to `AwareError::Network` at
        // line 55, because ureq turns any >=400 into `Err` and the JSON error body
        // is never read. That misdirects the operator to their network when the
        // fix is `aware connect`. Deliberately not tested here: asserting it would
        // pin behaviour that should change. Left as a production follow-up.
        let tmp = tempfile::tempdir().unwrap();
        let endpoint = spawn_token_endpoint("<html>gateway error</html>");
        write_profile(tmp.path(), &endpoint.url);
        let _seeded = seed_token(tmp.path(), "awaretest-bad-json", -10, Some("rt-stored"));

        let err = ensure_fresh(INTEGRATION, Some("awaretest-bad-json"), tmp.path()).unwrap_err();
        assert!(matches!(err, AwareError::Validation(_)), "got: {err:?}");
    }

    #[test]
    fn expired_token_without_a_refresh_token_says_to_reconnect() {
        // No refresh_token (consent granted without offline_access) — there is
        // nothing to POST, so this must fail fast with an actionable message
        // rather than sending `refresh_token=` to the provider.
        let tmp = tempfile::tempdir().unwrap();
        let endpoint = spawn_token_endpoint(r#"{"access_token":"refreshed-access"}"#);
        write_profile(tmp.path(), &endpoint.url);
        let _seeded = seed_token(tmp.path(), "awaretest-no-rt", -10, None);

        let err = ensure_fresh(INTEGRATION, Some("awaretest-no-rt"), tmp.path()).unwrap_err();
        let AwareError::AuthExpired(msg) = err else {
            panic!("expected AuthExpired, got {err:?}");
        };
        assert!(msg.contains("no refresh_token"), "msg: {msg}");
        assert!(msg.contains("aware connect"), "msg: {msg}");
    }
}
