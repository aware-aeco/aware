# AWARE CLI v0.4 (Auth + Host Plugins) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development to implement this plan task-by-task.

**Goal:** Credentials flow works end-to-end. `aware connect <integration>` runs PKCE OAuth with localhost callback, stores the token encrypted in the OS keychain, transparently refreshes on use. Host plugins auto-regenerate so claude-code (and later codex/opencode) users discover the AWARE agents from their familiar terminals without manual setup.

**Architecture:** A new `cli::auth` module ships the OAuth machinery — provider configs (Trimble Connect, Microsoft 365, Google Workspace), PKCE flow with a tiny localhost HTTP server for the callback, encrypted token storage via the `keyring` crate, lazy refresh on read. The runtime's existing `context::load_secret` switches from plain JSON files to the keyring backend. A new `cli::plugins` module generates per-host plugin folders from installed agents — claude-code gets a first-class implementation, codex/opencode get a minimal scaffold with TODO markers (those host plugin formats are still in motion).

**Tech Stack additions:**
- `oauth2` 4.x — PKCE flow, token exchange (well-maintained crate by ramosbugs)
- `keyring` 3.x — cross-platform OS keychain (Windows DPAPI / macOS Keychain / Linux libsecret)
- `tiny_http` 0.12.x — minimal blocking HTTP server for the localhost OAuth callback (no tokio adapter needed; runs on a spawned thread)
- `webbrowser` 1.x — opens the user's default browser to the auth URL
- `url` 2.x — already pulled in transitively by `ureq` / `oauth2`; we use it directly for callback URL parsing
- (Already present from v0.1-v0.3: tokio, ureq, serde, etc.)

---

## Open design decisions (resolved here)

| Decision | Choice | Why | Alternative |
|---|---|---|---|
| **OAuth library** | `oauth2` 4.x | De-facto Rust OAuth; well-maintained; built-in PKCE | Hand-roll PKCE (~150 LOC; not worth it); `oxide-auth` (server-focused) |
| **OAuth client-config source** | Env vars (`AWARE_OAUTH_TRIMBLE_CLIENT_ID`, etc.) with **placeholder defaults** in CLI source; user supplies real client IDs by setting env or `~/.aware/oauth/<integration>.json` | Decalog #4 (no vendor lock); shipping shared public client IDs requires AWARE-AECO to register OAuth apps with Trimble/Microsoft/Google — an operational task out of CLI scope | Hard-code shared client IDs (requires registration); per-user manual config only (worse UX) |
| **OAuth provider URLs** | Hard-coded per integration (Trimble's auth/token endpoints don't change weekly) | Stable; matches gh CLI / az CLI pattern | Env override (overkill for endpoints) |
| **Keychain library** | `keyring` 3.x | Cross-platform; battle-tested; flagged in v0.3 validation as the keychain dep "single point of decay" — accepted | OS-specific FFI (more code, less portable) |
| **Callback server port** | Try 7421 first (matches spec example), auto-increment on `EADDRINUSE` up to 7430 | Predictable; documented; tolerates re-runs while a prior server is winding down | Random ephemeral port (forces dynamic redirect_uri registration which most providers reject) |
| **Token shape on disk** | Single JSON blob per integration: `{access_token, refresh_token, expires_at: unix-epoch, scope, token_type, integration, obtained_at}`. Stored in keychain under service `aware-aeco`, account `<integration-id>` (or `<integration>.<alias>` for multi-account) | Self-contained; matches OAuth conventions; easy to swap to a different store later | Multiple secrets per integration (complicates the multi-account case) |
| **Refresh strategy** | Lazy: on read, if `expires_at < now + 60s`, refresh first. Cache the new token. Returns `AwareError::AuthExpired` only if refresh itself fails | Standard; transparent to the runtime | Eager (background refresh thread): more complex, marginal benefit |
| **Multi-account** | `--as <alias>` flag stores under account name `<integration>.<alias>`; default is account `<integration>` | Matches spec (cli-spec.md) | Single-account only (would conflict with the spec) |
| **Localhost callback HTML** | Minimal "✓ Authenticated. You can close this tab." page; closes the tab via JS if the browser allows it | Friendly; matches gh/az CLI feel | Plain text response (looks broken) |
| **Tests** | Mock OAuth provider in a tokio task that serves authorize + token endpoints; integration tests exercise the full PKCE round-trip against the mock | Real-OAuth tests against Trimble/Google require registered apps + manual approval — impossible in CI | Skip end-to-end tests (then we can't claim "it works") |
| **Host plugin format — claude-code** | First-class: writes `~/.claude/plugins/aware-aeco/plugin.json` + per-agent `commands/<agent>/<command>.md` files that wrap the runtime invocation. Follows the official Claude Code plugin schema | Claude Code is the most established host; the AWARE team builds *on* it; we know the format | Generic format (vague; users have to translate) |
| **Host plugin format — codex / opencode** | Scaffolded: writes `~/.<host>/plugins/aware-aeco/README.md` with a TODO ("AWARE plugin format for <host> pending — open an issue at github.com/aware-aeco/aware to contribute"). The `plugins regenerate` command tracks these as "stubbed" | Codex / opencode plugin formats aren't settled; better to scaffold the directory + signal "WIP" than guess wrong | Skip them entirely (worse — invisible) |
| **`plugins regenerate` idempotency** | Re-running produces an identical filesystem state (byte-for-byte). Old plugin contents are wiped before write | Matches v0.4 DoD; predictable diff under version control if user commits `~/.<host>/plugins/` | Incremental (more complex; harder to reason about) |
| **`runtime::context::load_secret` migration** | Phase 1 (Tasks 3-5): keychain becomes the primary backend; the existing plain-JSON file path is retained as a fallback for tests + the migration window | Doesn't break the v0.3 tests that write plain-JSON credentials | Hard-break (would invalidate the v0.3 integration tests) |

---

## Scope discipline

**v0.4 user-facing surfaces** (4 new):
- `aware connect <integration> [--as <alias>] [--refresh] [--scopes <s1,s2>]`
- `aware disconnect <integration> [--as <alias>]`
- `aware plugins regenerate` — new top-level subcommand (or `aware plugins` group with `regenerate` action)
- `aware doctor` — extended with a `Credentials:` block showing validity per integration

**v0.4 internal surfaces:**
- `cli::auth::config` — `IntegrationConfig` (auth URL, token URL, default scopes, client_id env-var name)
- `cli::auth::keychain` — `store_token`, `load_token`, `delete_token` against the OS keyring
- `cli::auth::pkce` — `run_pkce_flow(integration, scopes) -> Token`
- `cli::auth::refresh` — `ensure_fresh(integration, alias) -> Token` (lazy refresh)
- `cli::plugins::claude_code` — `generate(installed_agents, target_dir)`
- `cli::plugins::codex` — `generate(...)` (scaffold)
- `cli::plugins::opencode` — `generate(...)` (scaffold)

**Out of scope** — stay `NotYetImplemented`:
- `aware build agent`, `aware skill ...` — v0.5
- `aware agent publish` — v0.2+ (needs registry write)
- Per-machine real OAuth-app registration with Trimble/Microsoft/Google — operational task, not CLI code

---

## File Structure

### New files

| Path | Responsibility |
|---|---|
| `cli/src/auth/mod.rs` | Re-exports. |
| `cli/src/auth/config.rs` | `IntegrationConfig` per supported integration. Loads from hard-coded defaults + env overrides. |
| `cli/src/auth/keychain.rs` | `store_token`, `load_token`, `delete_token` via `keyring` crate. |
| `cli/src/auth/pkce.rs` | `run_pkce_flow` — generate code verifier/challenge, open browser, run localhost callback server, exchange code for token. |
| `cli/src/auth/refresh.rs` | `ensure_fresh` — lazy refresh logic. |
| `cli/src/plugins/mod.rs` | Re-exports. |
| `cli/src/plugins/claude_code.rs` | First-class Claude Code plugin generator. |
| `cli/src/plugins/codex.rs` | Scaffold + TODO. |
| `cli/src/plugins/opencode.rs` | Scaffold + TODO. |
| `cli/tests/connect.rs` | Integration tests against a mock OAuth server. |
| `cli/tests/disconnect.rs` | Integration tests for credential deletion. |
| `cli/tests/plugins.rs` | Integration tests for plugin generation (verify file shape + idempotency). |

### Modified files

| Path | Change |
|---|---|
| `cli/Cargo.toml` | Add `oauth2 = "4"`, `keyring = "3"`, `tiny_http = "0.12"`, `webbrowser = "1"`, `url = "2"`. |
| `cli/src/main.rs` | Add `mod auth; mod plugins;`. Add `Plugins` subcommand variant. |
| `cli/src/commands/connect.rs` | Implement `run_connect` and `run_disconnect`. |
| `cli/src/commands/mod.rs` | Add `pub mod plugins;`. |
| `cli/src/commands/plugins.rs` (new) | `PluginsCommand` enum + dispatch. |
| `cli/src/commands/doctor.rs` | Add `Credentials:` block showing validity per integration. |
| `cli/src/runtime/context.rs` | `load_secret` reads keychain first, falls back to plain JSON for tests. |
| `cli/README.md` | Bump quickstart with `aware connect`, `aware plugins regenerate`. |

---

## Task 0: Branch + baseline

- [ ] Already done — on `feat/cli-v04-auth-plugins` off main at `58ebb88` (v0.3 release).
- [ ] Run `cargo test` — 135 v0.3 tests still pass. No commit.

---

## Task 1: Add OAuth + keyring + HTTP + browser dependencies

Append to `cli/Cargo.toml` `[dependencies]`:

```toml
oauth2     = "4"
keyring    = "3"
tiny_http  = "0.12"
webbrowser = "1"
url        = "2"
```

Build (slow first time — oauth2 + keyring pull in lots), regression-check tests, commit:

```
chore(cli): add oauth2 + keyring + tiny_http + webbrowser + url for v0.4
```

---

## Task 2: `cli::auth::config` — IntegrationConfig

Hard-coded provider configs + env-var overrides for client IDs.

```rust
//! Per-integration OAuth configs.

use crate::error::AwareError;

pub struct IntegrationConfig {
    pub id: &'static str,
    pub auth_url: &'static str,
    pub token_url: &'static str,
    pub default_scopes: Vec<&'static str>,
    /// Env var name holding the client_id. Falls back to `default_client_id` if unset.
    pub client_id_env: &'static str,
    /// Placeholder default. Real OAuth-app registration with the provider replaces this.
    pub default_client_id: &'static str,
}

pub fn for_integration(id: &str) -> Result<IntegrationConfig, AwareError> {
    match id {
        "trimble-connect" => Ok(IntegrationConfig {
            id: "trimble-connect",
            auth_url: "https://id.trimble.com/oauth/authorize",
            token_url: "https://id.trimble.com/oauth/token",
            default_scopes: vec!["openid", "profile", "TrimbleConnect"],
            client_id_env: "AWARE_OAUTH_TRIMBLE_CLIENT_ID",
            default_client_id: "AWARE_AECO_PLACEHOLDER_TRIMBLE_CLIENT_ID",
        }),
        "microsoft-365" => Ok(IntegrationConfig {
            id: "microsoft-365",
            auth_url: "https://login.microsoftonline.com/common/oauth2/v2.0/authorize",
            token_url: "https://login.microsoftonline.com/common/oauth2/v2.0/token",
            default_scopes: vec!["offline_access", "Files.Read", "Sites.Read.All"],
            client_id_env: "AWARE_OAUTH_M365_CLIENT_ID",
            default_client_id: "AWARE_AECO_PLACEHOLDER_M365_CLIENT_ID",
        }),
        "google-workspace" => Ok(IntegrationConfig {
            id: "google-workspace",
            auth_url: "https://accounts.google.com/o/oauth2/v2/auth",
            token_url: "https://oauth2.googleapis.com/token",
            default_scopes: vec!["https://www.googleapis.com/auth/drive.readonly"],
            client_id_env: "AWARE_OAUTH_GOOGLE_CLIENT_ID",
            default_client_id: "AWARE_AECO_PLACEHOLDER_GOOGLE_CLIENT_ID",
        }),
        other => Err(AwareError::NotFound(format!("integration {other} not supported by v0.4"))),
    }
}

impl IntegrationConfig {
    pub fn client_id(&self) -> String {
        std::env::var(self.client_id_env).unwrap_or_else(|_| self.default_client_id.to_string())
    }
}
```

Tests:
- `for_integration("trimble-connect")` returns Ok
- `for_integration("nope")` returns NotFound
- `client_id()` reads env var when set
- `client_id()` falls back to default

Commit: `feat(cli): auth integration configs`

---

## Task 3: `cli::auth::keychain` — encrypted token storage

```rust
//! OS-keychain backed token storage.

use serde::{Deserialize, Serialize};

use crate::error::AwareError;

const SERVICE_NAME: &str = "aware-aeco";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredToken {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: i64,   // unix epoch seconds
    pub scope: String,
    pub token_type: String,
    pub integration: String,
    pub obtained_at: i64,
}

fn account_name(integration: &str, alias: Option<&str>) -> String {
    match alias {
        Some(a) => format!("{integration}.{a}"),
        None => integration.to_string(),
    }
}

pub fn store_token(token: &StoredToken, alias: Option<&str>) -> Result<(), AwareError> {
    let entry = keyring::Entry::new(SERVICE_NAME, &account_name(&token.integration, alias))
        .map_err(|e| AwareError::Internal(format!("keyring entry: {e}")))?;
    let body = serde_json::to_string(token)
        .map_err(|e| AwareError::Internal(format!("serialize token: {e}")))?;
    entry.set_password(&body)
        .map_err(|e| AwareError::PermissionDenied(format!("keyring write: {e}")))?;
    Ok(())
}

pub fn load_token(integration: &str, alias: Option<&str>) -> Result<Option<StoredToken>, AwareError> {
    let entry = keyring::Entry::new(SERVICE_NAME, &account_name(integration, alias))
        .map_err(|e| AwareError::Internal(format!("keyring entry: {e}")))?;
    match entry.get_password() {
        Ok(body) => {
            let token: StoredToken = serde_json::from_str(&body)
                .map_err(|e| AwareError::Validation(format!("token JSON: {e}")))?;
            Ok(Some(token))
        }
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(AwareError::PermissionDenied(format!("keyring read: {e}"))),
    }
}

pub fn delete_token(integration: &str, alias: Option<&str>) -> Result<(), AwareError> {
    let entry = keyring::Entry::new(SERVICE_NAME, &account_name(integration, alias))
        .map_err(|e| AwareError::Internal(format!("keyring entry: {e}")))?;
    match entry.delete_credential() {
        Ok(()) => Ok(()),
        Err(keyring::Error::NoEntry) => Ok(()), // idempotent
        Err(e) => Err(AwareError::PermissionDenied(format!("keyring delete: {e}"))),
    }
}
```

**Tests** — keyring tests are flaky in CI (Linux needs `libsecret` running). Add a feature flag `keyring-mock` that swaps in an in-memory map for tests. Or simpler: only run keyring tests when `AWARE_TEST_KEYRING=1` env var is set; otherwise skip. Document in CONTRIBUTING.

For Task 3 tests:
- Unit: account name format
- Unit (gated by `AWARE_TEST_KEYRING=1`): store → load → delete round-trip

Commit: `feat(cli): auth keychain storage (encrypted at rest via OS keyring)`

---

## Task 4: `cli::auth::pkce` — PKCE flow with localhost callback

This is the substantive task. The flow:

1. Generate PKCE code verifier (random 64-byte URL-safe base64) and challenge (SHA-256 of verifier, URL-safe base64-encoded).
2. Pick a localhost callback port (try 7421, increment on conflict).
3. Build the auth URL: `<auth_url>?response_type=code&client_id=<id>&redirect_uri=http://localhost:<port>/callback&scope=<scopes>&code_challenge=<challenge>&code_challenge_method=S256&state=<state>`.
4. Open the user's browser to the auth URL (via `webbrowser::open`).
5. Run `tiny_http` server on the chosen port. Wait for one request to `/callback`.
6. Parse the response: `?code=<code>&state=<state>` (verify state matches).
7. Respond to the browser with a friendly HTML "✓ Authenticated; close this tab".
8. Exchange the code for a token: POST to `<token_url>` with `grant_type=authorization_code`, `code`, `redirect_uri`, `client_id`, `code_verifier`.
9. Parse the JSON response, build a `StoredToken`, return it.

Implementation skeleton:

```rust
//! PKCE OAuth flow with localhost callback.

use std::time::{SystemTime, UNIX_EPOCH};

use base64::Engine;
use sha2::Digest;

use crate::auth::config::IntegrationConfig;
use crate::auth::keychain::StoredToken;
use crate::error::AwareError;

pub fn run_pkce_flow(config: &IntegrationConfig, extra_scopes: &[String]) -> Result<StoredToken, AwareError> {
    // 1. Generate PKCE pair
    let (verifier, challenge) = make_pkce_pair();

    // 2. Find a free port in 7421..7430
    let (server, port) = bind_callback_server()?;
    let redirect_uri = format!("http://localhost:{port}/callback");

    // 3. State for CSRF protection
    let state = random_token(16);

    // 4. Build auth URL
    let mut all_scopes: Vec<&str> = config.default_scopes.clone();
    for s in extra_scopes { all_scopes.push(s); }
    let scope_param = all_scopes.join(" ");
    let auth_url = format!(
        "{}?response_type=code&client_id={}&redirect_uri={}&scope={}&code_challenge={}&code_challenge_method=S256&state={}",
        config.auth_url,
        urlencode(&config.client_id()),
        urlencode(&redirect_uri),
        urlencode(&scope_param),
        challenge,
        state,
    );

    // 5. Open browser
    println!("Opening {} in your default browser...", config.id);
    println!("If it doesn't open automatically, visit:\n  {auth_url}");
    let _ = webbrowser::open(&auth_url);

    // 6. Wait for callback
    println!("✓ Listening on http://localhost:{port}/callback");
    let request = server.recv()
        .map_err(|e| AwareError::Network(format!("callback recv: {e}")))?;

    // Parse query string from request URL
    let url_str = format!("http://localhost{}", request.url());
    let parsed = url::Url::parse(&url_str)
        .map_err(|e| AwareError::Network(format!("callback URL parse: {e}")))?;
    let mut code = None;
    let mut returned_state = None;
    for (k, v) in parsed.query_pairs() {
        if k == "code" { code = Some(v.to_string()); }
        if k == "state" { returned_state = Some(v.to_string()); }
    }
    let code = code.ok_or_else(|| AwareError::Network("callback missing 'code'".into()))?;
    let returned_state = returned_state.ok_or_else(|| AwareError::Network("callback missing 'state'".into()))?;
    if returned_state != state {
        return Err(AwareError::PermissionDenied("OAuth state mismatch (possible CSRF)".into()));
    }

    // 7. Respond
    let response_html = r#"<html><body><h1>✓ Authenticated</h1><p>You can close this tab.</p><script>setTimeout(()=>window.close(),500)</script></body></html>"#;
    let response = tiny_http::Response::from_string(response_html)
        .with_header("Content-Type: text/html".parse::<tiny_http::Header>().unwrap());
    let _ = request.respond(response);

    // 8. Exchange code for token
    println!("✓ Exchanging code for token...");
    let body_params = vec![
        ("grant_type", "authorization_code".to_string()),
        ("code", code),
        ("redirect_uri", redirect_uri.clone()),
        ("client_id", config.client_id()),
        ("code_verifier", verifier),
    ];
    let body = body_params.iter()
        .map(|(k, v)| format!("{}={}", urlencode(k), urlencode(v)))
        .collect::<Vec<_>>()
        .join("&");
    let resp = ureq::post(config.token_url)
        .set("Content-Type", "application/x-www-form-urlencoded")
        .send_string(&body)
        .map_err(|e| AwareError::Network(format!("token exchange: {e}")))?;
    let token_json: serde_json::Value = resp.into_json()
        .map_err(|e| AwareError::Validation(format!("token response: {e}")))?;

    // 9. Build StoredToken
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
    let expires_in = token_json.get("expires_in").and_then(|v| v.as_i64()).unwrap_or(3600);
    let token = StoredToken {
        access_token: token_json.get("access_token").and_then(|v| v.as_str()).ok_or_else(|| AwareError::Validation("missing access_token".into()))?.to_string(),
        refresh_token: token_json.get("refresh_token").and_then(|v| v.as_str()).map(String::from),
        expires_at: now + expires_in,
        scope: token_json.get("scope").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        token_type: token_json.get("token_type").and_then(|v| v.as_str()).unwrap_or("Bearer").to_string(),
        integration: config.id.to_string(),
        obtained_at: now,
    };

    println!("✓ Done.");
    Ok(token)
}

fn make_pkce_pair() -> (String, String) {
    use rand::RngCore;
    let mut rng_bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut rng_bytes);
    let verifier = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(rng_bytes);
    let mut hasher = sha2::Sha256::new();
    hasher.update(verifier.as_bytes());
    let challenge = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(hasher.finalize());
    (verifier, challenge)
}

fn random_token(n: usize) -> String {
    use rand::Rng;
    let chars: Vec<u8> = (0..n).map(|_| rand::thread_rng().gen_range(b'a'..=b'z')).collect();
    String::from_utf8(chars).unwrap()
}

fn urlencode(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
}

fn bind_callback_server() -> Result<(tiny_http::Server, u16), AwareError> {
    for port in 7421..=7430 {
        let addr = format!("127.0.0.1:{port}");
        match tiny_http::Server::http(&addr) {
            Ok(server) => return Ok((server, port)),
            Err(_) => continue,
        }
    }
    Err(AwareError::Network("could not bind callback port in 7421-7430".into()))
}
```

**New deps needed:** add to Cargo.toml in Task 1: `rand = "0.8"`, `base64 = "0.22"`, `sha2 = "0.10"`. Adjust Task 1 to include these.

**Tests** — full PKCE flow is hard to test without a real or mocked OAuth provider. Strategy: build a small mock provider as a tokio task that:
1. Receives the redirect with `code` (we POST it as if we were the browser)
2. Verifies the PKCE challenge matches
3. Responds with a fake token

Actually simpler: extract `make_pkce_pair`, `bind_callback_server`, the URL-building, and the response-parsing as small testable units. End-to-end test deferred to manual smoke against a real provider.

Commit: `feat(cli): auth PKCE flow with localhost callback`

---

## Task 5: `cli::auth::refresh` — lazy token refresh

```rust
//! Lazy token refresh.

use std::time::{SystemTime, UNIX_EPOCH};

use crate::auth::config;
use crate::auth::keychain::{self, StoredToken};
use crate::error::AwareError;

const REFRESH_BUFFER_SECS: i64 = 60;

pub fn ensure_fresh(integration: &str, alias: Option<&str>) -> Result<StoredToken, AwareError> {
    let token = keychain::load_token(integration, alias)?
        .ok_or_else(|| AwareError::AuthExpired(integration.to_string()))?;

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
    if token.expires_at > now + REFRESH_BUFFER_SECS {
        return Ok(token); // still fresh
    }

    // Refresh
    let refresh_token = token.refresh_token.as_deref()
        .ok_or_else(|| AwareError::AuthExpired(format!("{integration}: no refresh_token available")))?;
    let cfg = config::for_integration(integration)?;

    let body_params = vec![
        ("grant_type", "refresh_token".to_string()),
        ("refresh_token", refresh_token.to_string()),
        ("client_id", cfg.client_id()),
    ];
    let body = body_params.iter()
        .map(|(k, v)| format!("{}={}", urlencode(k), urlencode(v)))
        .collect::<Vec<_>>()
        .join("&");
    let resp = ureq::post(cfg.token_url)
        .set("Content-Type", "application/x-www-form-urlencoded")
        .send_string(&body)
        .map_err(|e| AwareError::Network(format!("refresh: {e}")))?;
    let token_json: serde_json::Value = resp.into_json()
        .map_err(|e| AwareError::Validation(format!("refresh response: {e}")))?;

    let new_expires_in = token_json.get("expires_in").and_then(|v| v.as_i64()).unwrap_or(3600);
    let new_token = StoredToken {
        access_token: token_json.get("access_token").and_then(|v| v.as_str())
            .ok_or_else(|| AwareError::Validation("refresh: missing access_token".into()))?.to_string(),
        refresh_token: token_json.get("refresh_token").and_then(|v| v.as_str()).map(String::from)
            .or_else(|| token.refresh_token.clone()),
        expires_at: now + new_expires_in,
        scope: token_json.get("scope").and_then(|v| v.as_str()).unwrap_or(&token.scope).to_string(),
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
```

Tests: unit-test the buffer logic (mock-ish — set token's expires_at to now+30s, ensure_fresh should attempt refresh; expires_at to now+3600s should return as-is). Hard to test the actual refresh HTTP call without a mock.

Commit: `feat(cli): auth lazy refresh`

---

## Task 6: Wire `aware connect <integration>` (and `--refresh`)

`cli/src/commands/connect.rs` — replace `run_connect` body:

```rust
pub fn run_connect(args: ConnectArgs, _ctx: &Context) -> Result<(), AwareError> {
    let cfg = crate::auth::config::for_integration(&args.integration)?;
    let extra_scopes = args.scopes.unwrap_or_default()
        .split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect::<Vec<_>>();

    if args.refresh {
        let _ = crate::auth::refresh::ensure_fresh(&args.integration, args.alias.as_deref())?;
        println!("✓ refreshed {} credential", args.integration);
        return Ok(());
    }

    let token = crate::auth::pkce::run_pkce_flow(&cfg, &extra_scopes)?;
    crate::auth::keychain::store_token(&token, args.alias.as_deref())?;
    println!("✓ stored encrypted credential for {} (expires {})", args.integration, token.expires_at);
    Ok(())
}
```

The existing `ConnectArgs` already has `--as <alias>`, `--refresh`, `--scopes <s1,s2>` per the v0.1 scaffold — no changes needed.

Test: hard to write end-to-end without a mock provider; defer to manual smoke. Add a unit test that `--refresh` on a missing credential returns `AuthExpired`.

Commit: `feat(cli): wire aware connect (PKCE + refresh)`

---

## Task 7: Wire `aware disconnect`

```rust
pub fn run_disconnect(args: DisconnectArgs, _ctx: &Context) -> Result<(), AwareError> {
    crate::auth::keychain::delete_token(&args.integration, args.alias.as_deref())?;
    println!("✓ removed credential for {}", args.integration);
    Ok(())
}
```

Test: gated by `AWARE_TEST_KEYRING=1` — store + disconnect + verify deleted.

Commit: `feat(cli): wire aware disconnect`

---

## Task 8: Update `runtime::context::load_secret` to use keychain

Currently `load_secret` reads `<creds_dir>/<id>.json`. Change to:
1. Try keychain via `keychain::load_token(id, None)`. If found, use that.
2. Else fall back to the existing plain-JSON read (for tests + migration).

```rust
pub fn load_secret(ctx: &mut RenderContext, creds_dir: &Path, id: &str) -> Result<(), AwareError> {
    // Try keychain first
    if let Ok(Some(token)) = crate::auth::keychain::load_token(id, None) {
        let value = serde_json::to_value(&token)
            .map_err(|e| AwareError::Internal(format!("token to value: {e}")))?;
        ctx.secrets.insert(id.to_string(), value);
        return Ok(());
    }
    // Fallback: plain JSON file
    let path = creds_dir.join(format!("{id}.json"));
    if !path.is_file() { return Ok(()); }
    let body = std::fs::read_to_string(&path)?;
    let v: serde_json::Value = serde_json::from_str(&body)
        .map_err(|e| AwareError::Validation(format!("secret {id}: {e}")))?;
    ctx.secrets.insert(id.to_string(), v);
    Ok(())
}
```

Existing v0.3 tests (using plain-JSON fallback) keep working. Add a new test under `AWARE_TEST_KEYRING=1` that exercises the keychain path.

Commit: `feat(cli): runtime context reads keychain first, falls back to JSON`

---

## Task 9: Doctor `Credentials:` block

After the `Running:` block, add:

```rust
println!();
println!("Credentials:");
for integration in &["trimble-connect", "microsoft-365", "google-workspace"] {
    match crate::auth::keychain::load_token(integration, None) {
        Ok(Some(token)) => {
            let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64;
            let remaining = token.expires_at - now;
            if remaining > 0 {
                let mins = remaining / 60;
                println!("  ✓ {integration:<22} valid    expires in {mins}m");
            } else {
                println!("  · {integration:<22} expired  run: aware connect {integration} --refresh");
            }
        }
        Ok(None) => {
            println!("  ✗ {integration:<22} missing  run: aware connect {integration}");
        }
        Err(_) => {
            println!("  ? {integration:<22} (keyring unavailable)");
        }
    }
}
```

Test: when no credentials stored, all three show "missing".

Commit: `feat(cli): doctor Credentials block`

---

## Task 10: `cli::plugins::claude_code` — first-class Claude Code plugin

Generate `~/.claude/plugins/aware-aeco/` with:
- `plugin.json` — top-level plugin manifest declaring all bundled commands
- `commands/<agent>-<command>.md` — one per agent command, with frontmatter pointing at `aware app run` or `aware app install` etc.

Minimal v0.4: just ensure the directory + plugin.json get written; per-command files contain a frontmatter block + body that delegates to the `aware` CLI.

```rust
//! Claude Code plugin generator.

use std::path::{Path, PathBuf};

use crate::error::AwareError;
use crate::manifest::loader::DiscoveredAgent;

pub fn generate(agents: &[DiscoveredAgent], plugin_root: &Path) -> Result<usize, AwareError> {
    let aware_aeco_dir = plugin_root.join("aware-aeco");
    // Wipe + re-create for idempotency
    if aware_aeco_dir.exists() {
        std::fs::remove_dir_all(&aware_aeco_dir)?;
    }
    std::fs::create_dir_all(aware_aeco_dir.join("commands"))?;

    // plugin.json
    let mut commands_index = Vec::new();
    for agent in agents {
        for (cmd_name, cmd_spec) in &agent.manifest.commands {
            let file_name = format!("{}-{}.md", agent.manifest.agent, cmd_name);
            commands_index.push(format!("commands/{file_name}"));

            let body = format!(
                "---\nname: {agent_id}-{cmd_name}\ndescription: {description}\n---\n\n\
                Invokes `{agent_id}/{cmd_name}` via the AWARE CLI.\n\n\
                Run:\n```bash\naware app run --instance default --config <key>=<value> <app-name>\n```\n",
                agent_id = agent.manifest.agent,
                cmd_name = cmd_name,
                description = cmd_spec.description.lines().next().unwrap_or(""),
            );
            std::fs::write(aware_aeco_dir.join("commands").join(&file_name), body)?;
        }
    }

    let plugin_json = serde_json::json!({
        "name": "aware-aeco",
        "version": env!("CARGO_PKG_VERSION"),
        "description": "AWARE agents and apps bundle. Composes AECO software into AI-runnable scripts.",
        "homepage": "https://github.com/aware-aeco/aware",
        "commands": commands_index,
    });
    std::fs::write(aware_aeco_dir.join("plugin.json"), serde_json::to_string_pretty(&plugin_json)?)?;

    Ok(agents.iter().map(|a| a.manifest.commands.len()).sum())
}
```

Tests: generate against fixture, verify `plugin.json` exists, command files exist, idempotency (running twice produces identical bytes).

Commit: `feat(cli): plugins claude-code generator`

---

## Task 11: `cli::plugins::codex` + `opencode` — scaffold

```rust
//! Codex plugin generator — scaffold only.

pub fn generate(_agents: &[crate::manifest::loader::DiscoveredAgent], plugin_root: &std::path::Path) -> Result<usize, crate::error::AwareError> {
    let dir = plugin_root.join("aware-aeco");
    if dir.exists() {
        std::fs::remove_dir_all(&dir)?;
    }
    std::fs::create_dir_all(&dir)?;
    let readme = "# AWARE Plugin for Codex\n\nThe Codex plugin format isn't settled yet.\n\
        File an issue at https://github.com/aware-aeco/aware if you'd like to contribute the format.\n";
    std::fs::write(dir.join("README.md"), readme)?;
    Ok(0)
}
```

Identical pattern for `opencode.rs`. Same commit:

`feat(cli): plugins codex + opencode scaffolds`

---

## Task 12: Wire `aware plugins regenerate`

Add `Plugins` subcommand to `cli/src/main.rs`:

```rust
/// Manage host-side plugin folders for claude-code / codex / opencode.
Plugins {
    #[command(subcommand)]
    action: commands::plugins::PluginsCommand,
},
```

Create `cli/src/commands/plugins.rs`:

```rust
use clap::Subcommand;

use crate::context::Context;
use crate::error::AwareError;

#[derive(Subcommand, Debug)]
pub enum PluginsCommand {
    /// Rebuild host plugin folders from installed agents.
    Regenerate,
}

pub fn dispatch(cmd: PluginsCommand, ctx: &Context) -> Result<(), AwareError> {
    match cmd {
        PluginsCommand::Regenerate => regenerate(ctx),
    }
}

fn regenerate(ctx: &Context) -> Result<(), AwareError> {
    let agents = crate::manifest::loader::discover_agents(&ctx.paths)?;
    println!("Regenerating host plugins from {} installed agents...", agents.len());

    let home = dirs::home_dir().ok_or_else(|| AwareError::Internal("home dir".into()))?;

    let claude_root = home.join(".claude/plugins");
    if claude_root.exists() || std::env::var_os("AWARE_PLUGINS_CLAUDE").is_some() {
        let target = std::env::var_os("AWARE_PLUGINS_CLAUDE")
            .map(|p| std::path::PathBuf::from(p))
            .unwrap_or(claude_root);
        let count = crate::plugins::claude_code::generate(&agents, &target)?;
        println!("  ✓ claude-code: {count} commands");
    }

    // Same pattern for codex + opencode...

    Ok(())
}
```

Add `pub mod plugins;` to `cli/src/commands/mod.rs`.

Integration test: set `AWARE_PLUGINS_CLAUDE` to a temp dir, run `aware plugins regenerate`, verify the dir structure.

Commit: `feat(cli): wire aware plugins regenerate`

---

## Task 13: Auto-regenerate on `aware agent install`

After a successful agent install, automatically call `plugins::claude_code::generate` if `~/.claude/plugins/` exists. This makes new installs visible to the host without manual intervention.

```rust
// In commands/agent.rs install() function, after the existing success print:
let agents = crate::manifest::loader::discover_agents(&ctx.paths)?;
if let Some(home) = dirs::home_dir() {
    let claude_root = home.join(".claude/plugins");
    if claude_root.exists() {
        let _ = crate::plugins::claude_code::generate(&agents, &claude_root);
    }
}
```

(Best-effort — failures don't kill the install.) Same for `agent uninstall` (regenerate to remove stale entries).

Test: install an agent, verify `~/.claude/plugins/aware-aeco/plugin.json` updates. Run with `AWARE_PLUGINS_CLAUDE` to redirect.

Commit: `feat(cli): auto-regenerate host plugins on install/uninstall`

---

## Task 14: README + final verification

Update `cli/README.md`:
- Status section bumps to v0.4 (23 surfaces)
- Run section adds `aware connect`, `aware disconnect`, `aware plugins regenerate`
- New "Credentials" subsection explaining encryption, refresh, multi-account
- New "Host plugins" subsection explaining the auto-regeneration on install

Final checks:
- `cargo fmt --all -- --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test --all-targets` (including the `AWARE_TEST_KEYRING=1` gated tests if locally available)
- `cargo build --release`
- Manual smoke: `aware doctor` shows the new Credentials block

Commit: `docs(cli): README quickstart for v0.4`

---

## Self-review

**Spec coverage** — v0.4 DoD:
- `aware connect trimble-connect` survives a real OAuth flow → Tasks 2/4 build the full machinery; **the actual "real flow against app.connect.trimble.com" requires OAuth-app registration with Trimble — an operational task outside CLI scope.** The CLI is ready to do it once that registration happens.
- Tokens encrypted at rest, never leaked to logs → Task 3 (OS keyring) + no token bodies printed anywhere in the codebase
- `aware app run` with real `aware connect`-ed integration works → Task 8 (runtime context reads keychain)
- Host plugin regeneration idempotent + diff-stable → Task 10 (claude-code) + Task 12 (regenerate command). Wipe-and-rewrite ensures byte-identical output across runs

**Operational note (cannot complete in code):** The "default_client_id" placeholders in `cli::auth::config` need to be replaced with real client IDs from registered OAuth apps at Trimble / Microsoft / Google. This is a one-time operational task — until done, end-users have to supply their own client IDs via env vars.

**Out of scope** (correctly stubbed): `agent publish` (v0.2+), `build agent` (v0.5), `skill ...` (v0.5).
