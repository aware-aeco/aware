# Tier 2 — BYO OAuth App Profiles Implementation Plan

> **For Claude:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Let an org point `aware connect` at its own registered Google / M365 / Trimble OAuth app (its client_id, secret, tenant, scopes) via first-class config — no env vars — with the secret stored in the OS keychain, overriding the bundled first-party (Tier 1) app.

**Architecture:** Add an `OAuthAppProfile` loaded from `~/.aware/oauth/<integration>[.<alias>].yaml` (non-secret fields only). The BYO client secret lives in the OS keychain under a dedicated `oauth-app.<integration>[.<alias>]` account. `IntegrationConfig` gains an overlay step `with_profile(aware_home, alias)` that resolves every value by precedence **profile > env var > bundled default**. All three flows (PKCE, refresh, device-code) consume the resolved config. `aware connect --set-app-secret` stashes the secret from stdin; `--list` / `doctor` report which app is active.

**Tech Stack:** Rust 2024, `serde` + `serde_yaml`, `keyring` crate (already used), `clap` derive. Tests via `#[cfg(test)]` modules; run with `cargo test --bin aware`.

**Resolution precedence (the crux):**
| Value | 1st | 2nd | 3rd |
|---|---|---|---|
| client_id | profile.client_id | env `AWARE_OAUTH_<INT>_CLIENT_ID` | bundled `default_client_id` |
| client_secret | keychain app-secret | env `client_secret_env` | bundled `default_client_secret` |
| scopes | profile.scopes | — | bundled `default_scopes` |
| auth_url / token_url | profile override (or m365 tenant substitution) | — | bundled static |
| tenant (m365) | CLI `--tenant` | profile.tenant | `common` |

**App-source label** (for `--list` / `doctor`): `byo-profile` if a profile file exists; else `byo-env` if the client_id env var is set; else `first-party`.

---

## Chunk 1: Profile model + keychain app-secret + config overlay

### Task 1: `OAuthAppProfile` model + YAML loader

**Files:**
- Create: `cli/src/auth/profile.rs`
- Modify: `cli/src/auth/mod.rs` (add `pub mod profile;`)
- Modify: `cli/src/paths.rs` (add `oauth_dir()` + `oauth_profile_path()`)

- [ ] **Step 1: Write failing tests** in `cli/src/auth/profile.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn absent_profile_is_none() {
        let tmp = tempfile::tempdir().unwrap();
        assert!(load_profile(tmp.path(), "microsoft-365", None).unwrap().is_none());
    }

    #[test]
    fn loads_default_profile_fields() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("oauth");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("microsoft-365.yaml"),
            "client_id: org-app-123\ntenant: contoso.onmicrosoft.com\nscopes:\n  - User.Read\n  - Mail.Send\n",
        ).unwrap();
        let p = load_profile(tmp.path(), "microsoft-365", None).unwrap().unwrap();
        assert_eq!(p.client_id.as_deref(), Some("org-app-123"));
        assert_eq!(p.tenant.as_deref(), Some("contoso.onmicrosoft.com"));
        assert_eq!(p.scopes.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn alias_profile_shadows_default() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("oauth");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("google-workspace.yaml"), "client_id: base\n").unwrap();
        std::fs::write(dir.join("google-workspace.staging.yaml"), "client_id: staging\n").unwrap();
        let p = load_profile(tmp.path(), "google-workspace", Some("staging")).unwrap().unwrap();
        assert_eq!(p.client_id.as_deref(), Some("staging"));
    }

    #[test]
    fn alias_falls_back_to_default_when_no_alias_file() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("oauth");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("google-workspace.yaml"), "client_id: base\n").unwrap();
        let p = load_profile(tmp.path(), "google-workspace", Some("nope")).unwrap().unwrap();
        assert_eq!(p.client_id.as_deref(), Some("base"));
    }

    #[test]
    fn malformed_yaml_is_validation_error() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("oauth");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("microsoft-365.yaml"), "client_id: [unterminated\n").unwrap();
        assert!(matches!(
            load_profile(tmp.path(), "microsoft-365", None),
            Err(crate::error::AwareError::Validation(_))
        ));
    }
}
```

- [ ] **Step 2:** Run `cargo test --bin aware auth::profile` → FAIL (no `load_profile`).

- [ ] **Step 3: Implement** `cli/src/auth/profile.rs`:

```rust
//! Bring-your-own (BYO) OAuth app profiles — Tier 2 of the hybrid auth model (#146).
//!
//! Non-secret app config (client_id, tenant, scopes, optional endpoint URLs) lives
//! in `~/.aware/oauth/<integration>[.<alias>].yaml`. The BYO client *secret* is NOT
//! stored here — it goes to the OS keychain (see `keychain::store_app_secret`).
//! Resolution precedence is applied in `config::IntegrationConfig::with_profile`.

use std::path::Path;

use serde::Deserialize;

use crate::error::AwareError;

/// Non-secret BYO OAuth app configuration. Every field is optional: a profile
/// overrides only the keys it sets, leaving the rest to env / bundled defaults.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct OAuthAppProfile {
    pub client_id: Option<String>,
    /// Microsoft tenant id or `*.onmicrosoft.com` domain (M365 only).
    pub tenant: Option<String>,
    /// Full scope set. When present it REPLACES the bundled default scopes.
    pub scopes: Option<Vec<String>>,
    /// Explicit authorization endpoint override (rare; tenant substitution is automatic for M365).
    pub auth_url: Option<String>,
    /// Explicit token endpoint override.
    pub token_url: Option<String>,
}

/// Load the BYO profile for `integration`, preferring an alias-specific file
/// (`<integration>.<alias>.yaml`) over the default (`<integration>.yaml`).
/// Returns `Ok(None)` when no profile file exists.
pub fn load_profile(
    aware_home: &Path,
    integration: &str,
    alias: Option<&str>,
) -> Result<Option<OAuthAppProfile>, AwareError> {
    let dir = aware_home.join("oauth");
    let candidates: Vec<std::path::PathBuf> = match alias {
        Some(a) => vec![
            dir.join(format!("{integration}.{a}.yaml")),
            dir.join(format!("{integration}.yaml")),
        ],
        None => vec![dir.join(format!("{integration}.yaml"))],
    };
    let Some(path) = candidates.into_iter().find(|p| p.is_file()) else {
        return Ok(None);
    };
    let raw = std::fs::read_to_string(&path)
        .map_err(|e| AwareError::Internal(format!("read profile {}: {e}", path.display())))?;
    let profile: OAuthAppProfile = serde_yaml::from_str(&raw)
        .map_err(|e| AwareError::Validation(format!("profile {}: {e}", path.display())))?;
    Ok(Some(profile))
}
```

Add to `cli/src/auth/mod.rs`: `pub mod profile; // Tier 2 BYO (#146)`.
Add to `cli/src/paths.rs` impl: `pub fn oauth_dir(&self) -> PathBuf { self.aware_home.join("oauth") }` (with a `config_path`-style unit test).

- [ ] **Step 4:** Run `cargo test --bin aware auth::profile` + `cargo test --bin aware paths` → PASS.

- [ ] **Step 5: Commit** `git add cli/src/auth/profile.rs cli/src/auth/mod.rs cli/src/paths.rs && git commit -m "feat(auth): OAuth app profile model + loader (#146)"`

---

### Task 2: Keychain app-secret slot

**Files:**
- Modify: `cli/src/auth/keychain.rs` (new `store_app_secret` / `load_app_secret` / `delete_app_secret`)

App secrets use a separate account namespace so they never collide with tokens:
`app_account_name(integration, alias)` → `oauth-app.<integration>[.<alias>]`. Same
service (`aware-aeco`) and the same Windows file-fallback path (`<aware_home>/credentials/oauth-app.<integration>[.alias].json`) the token store already uses.

- [ ] **Step 1: Write failing tests** (append to keychain.rs `mod tests`):

```rust
#[test]
fn app_account_name_with_alias() {
    assert_eq!(app_account_name("microsoft-365", None), "oauth-app.microsoft-365");
    assert_eq!(app_account_name("google-workspace", Some("staging")), "oauth-app.google-workspace.staging");
}

#[test]
fn app_secret_file_fallback_round_trip() {
    let tmp = tempfile::tempdir().unwrap();
    write_app_secret_file(tmp.path(), &app_account_name("google-workspace", None), "s3cr3t").unwrap();
    let got = read_app_secret_file(tmp.path(), &app_account_name("google-workspace", None)).unwrap();
    assert_eq!(got.as_deref(), Some("s3cr3t"));
}

#[test]
fn app_secret_absent_is_none() {
    let tmp = tempfile::tempdir().unwrap();
    assert!(read_app_secret_file(tmp.path(), "oauth-app.nope").unwrap().is_none());
}
```

- [ ] **Step 2:** `cargo test --bin aware auth::keychain` → FAIL.

- [ ] **Step 3: Implement** in keychain.rs (mirror the token store/load/delete + file-fallback pattern; the secret is a plain string, not JSON):

```rust
fn app_account_name(integration: &str, alias: Option<&str>) -> String {
    match alias {
        Some(a) => format!("oauth-app.{integration}.{a}"),
        None => format!("oauth-app.{integration}"),
    }
}

/// Store a BYO OAuth client secret in the OS keychain (file-fallback on failure).
pub fn store_app_secret(integration: &str, alias: Option<&str>, secret: &str, aware_home: &Path) -> Result<(), AwareError> {
    let account = app_account_name(integration, alias);
    let entry = keyring::Entry::new(SERVICE_NAME, &account)
        .map_err(|e| AwareError::Internal(format!("keyring entry: {e}")))?;
    match entry.set_password(secret) {
        Ok(()) => Ok(()),
        Err(e) => {
            eprintln!("aware: keyring write failed ({e}); falling back to ~/.aware/credentials file");
            write_app_secret_file(aware_home, &account, secret)
        }
    }
}

/// Load a BYO client secret: keychain first, then file fallback. `Ok(None)` if unset.
pub fn load_app_secret(integration: &str, alias: Option<&str>, aware_home: &Path) -> Result<Option<String>, AwareError> {
    let account = app_account_name(integration, alias);
    let entry = keyring::Entry::new(SERVICE_NAME, &account)
        .map_err(|e| AwareError::Internal(format!("keyring entry: {e}")))?;
    match entry.get_password() {
        Ok(s) => Ok(Some(s)),
        Err(keyring::Error::NoEntry) => read_app_secret_file(aware_home, &account),
        Err(e) => Err(AwareError::PermissionDenied(format!("keyring read: {e}"))),
    }
}

/// Remove a stored BYO client secret from keychain and file fallback (idempotent).
pub fn delete_app_secret(integration: &str, alias: Option<&str>, aware_home: &Path) -> Result<(), AwareError> {
    let account = app_account_name(integration, alias);
    let entry = keyring::Entry::new(SERVICE_NAME, &account)
        .map_err(|e| AwareError::Internal(format!("keyring entry: {e}")))?;
    let kr = match entry.delete_credential() {
        Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(AwareError::PermissionDenied(format!("keyring delete: {e}"))),
    };
    let file = cred_file_path(aware_home, &account);
    if file.is_file() { let _ = std::fs::remove_file(&file); }
    kr
}

fn write_app_secret_file(aware_home: &Path, account: &str, secret: &str) -> Result<(), AwareError> {
    let path = cred_file_path(aware_home, account);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| AwareError::Internal(format!("create credentials dir: {e}")))?;
    }
    write_restricted(&path, secret.as_bytes())
        .map_err(|e| AwareError::PermissionDenied(format!("app-secret file write: {e}")))
}

fn read_app_secret_file(aware_home: &Path, account: &str) -> Result<Option<String>, AwareError> {
    let path = cred_file_path(aware_home, account);
    if !path.is_file() { return Ok(None); }
    let s = std::fs::read_to_string(&path)
        .map_err(|e| AwareError::Internal(format!("app-secret file read: {e}")))?;
    Ok(Some(s.trim().to_string()))
}
```

- [ ] **Step 4:** `cargo test --bin aware auth::keychain` → PASS.
- [ ] **Step 5: Commit** `git add cli/src/auth/keychain.rs && git commit -m "feat(auth): keychain slot for BYO OAuth client secret (#146)"`

---

### Task 3: `IntegrationConfig::with_profile` overlay + resolution

**Files:**
- Modify: `cli/src/auth/config.rs`

Convert the directly-read string fields to owned and make `auth_url`/`token_url`/`default_scopes` resolve through accessor methods. Add an `overlay: Option<Overlay>` populated by `with_profile`. Keep the existing `for_integration` returning bundled defaults with `overlay: None` (so its current tests stay valid).

Design:
```rust
pub struct IntegrationConfig {
    pub id: String,
    auth_url: String,                 // private base; read via auth_url()
    token_url: String,                // private base; read via token_url()
    default_scopes: Vec<String>,
    client_id_env: &'static str,
    default_client_id: String,
    client_secret_env: Option<&'static str>,
    default_client_secret: Option<String>,
    overlay: Overlay,                 // empty by default
}

#[derive(Default)]
struct Overlay {
    profile: Option<crate::auth::profile::OAuthAppProfile>,
    app_secret: Option<String>,       // resolved from keychain
    source: AppSource,                // FirstParty | ByoProfile | ByoEnv
}
```
- Change all struct literals in `for_integration` to use `.to_string()` / `vec![...].into_iter().map(String::from).collect()` and `overlay: Overlay::default()`. `default_client_secret: option_env!("AWARE_GOOGLE_CLIENT_SECRET").map(String::from)`.
- Accessor methods (each consulted by the flows):
  - `pub fn id(&self) -> &str` — keep `id` as a public field is fine too; flows use `cfg.id`. Keep `id` public field; only `auth_url`/`token_url` become private+method.
  - `pub fn auth_url(&self) -> &str` — `overlay.profile.auth_url` ▸ `self.auth_url`
  - `pub fn token_url(&self) -> &str` — `overlay.profile.token_url` ▸ `self.token_url`
  - `pub fn scopes(&self) -> Vec<String>` — `overlay.profile.scopes.clone()` ▸ `self.default_scopes.clone()`
  - `pub fn client_id(&self)` — `overlay.profile.client_id` ▸ env ▸ default (extend existing)
  - `pub fn client_secret(&self)` — `overlay.app_secret` ▸ env ▸ default (extend existing)
  - `pub fn tenant(&self) -> Option<&str>` — `overlay.profile.tenant`
  - `pub fn app_source(&self) -> AppSource` (Copy enum) + `pub fn app_source_label(&self) -> &'static str` → `"first-party" | "byo-profile" | "byo-env"`
- `with_profile`:
```rust
pub fn with_profile(mut self, aware_home: &std::path::Path, alias: Option<&str>) -> Result<Self, AwareError> {
    let profile = crate::auth::profile::load_profile(aware_home, &self.id, alias)?;
    let app_secret = crate::auth::keychain::load_app_secret(&self.id, alias, aware_home)?;
    // M365 single-tenant substitution: if profile sets a tenant and no explicit
    // endpoint override, swap `common` for the tenant in auth/token URLs.
    if self.id == "microsoft-365"
        && let Some(p) = &profile
        && let Some(t) = &p.tenant
    {
        if p.auth_url.is_none() { self.auth_url = self.auth_url.replace("/common/", &format!("/{t}/")); }
        if p.token_url.is_none() { self.token_url = self.token_url.replace("/common/", &format!("/{t}/")); }
    }
    let source = if profile.is_some() {
        AppSource::ByoProfile
    } else if std::env::var(self.client_id_env).is_ok() {
        AppSource::ByoEnv
    } else {
        AppSource::FirstParty
    };
    self.overlay = Overlay { profile, app_secret, source };
    Ok(self)
}
```

- [ ] **Step 1: Write failing tests** (in config.rs `mod tests`; use `AWARE_HOME`-style tmp + `serial`-safe env handling — set env only inside the test and remove after):

```rust
#[test]
fn profile_client_id_overrides_env_and_default() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path().join("oauth");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("microsoft-365.yaml"), "client_id: org-app-999\n").unwrap();
    let cfg = for_integration("microsoft-365").unwrap().with_profile(tmp.path(), None).unwrap();
    assert_eq!(cfg.client_id(), "org-app-999");
    assert_eq!(cfg.app_source_label(), "byo-profile");
}

#[test]
fn no_profile_falls_back_to_bundled_first_party() {
    let tmp = tempfile::tempdir().unwrap();
    let cfg = for_integration("microsoft-365").unwrap().with_profile(tmp.path(), None).unwrap();
    assert_eq!(cfg.client_id(), "00929383-3563-4f4e-87e0-7476fcd9e80b");
    assert_eq!(cfg.app_source_label(), "first-party");
}

#[test]
fn profile_scopes_replace_bundled() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path().join("oauth");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("google-workspace.yaml"), "client_id: x\nscopes:\n  - openid\n").unwrap();
    let cfg = for_integration("google-workspace").unwrap().with_profile(tmp.path(), None).unwrap();
    assert_eq!(cfg.scopes(), vec!["openid".to_string()]);
}

#[test]
fn m365_tenant_substitutes_common_in_endpoints() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path().join("oauth");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("microsoft-365.yaml"), "client_id: x\ntenant: contoso.onmicrosoft.com\n").unwrap();
    let cfg = for_integration("microsoft-365").unwrap().with_profile(tmp.path(), None).unwrap();
    assert!(cfg.auth_url().contains("/contoso.onmicrosoft.com/"));
    assert!(!cfg.token_url().contains("/common/"));
    assert_eq!(cfg.tenant(), Some("contoso.onmicrosoft.com"));
}
```

- [ ] **Step 2:** `cargo test --bin aware auth::config` → FAIL.
- [ ] **Step 3: Implement** the struct/field/accessor/`with_profile` changes above. Update the existing `client_id`/`client_secret` methods to consult `overlay` first.
- [ ] **Step 4:** `cargo test --bin aware auth::config` → PASS (incl. the 4 pre-existing config tests).
- [ ] **Step 5: Commit** `git add cli/src/auth/config.rs && git commit -m "feat(auth): with_profile overlay + BYO resolution precedence (#146)"`

---

## Chunk 2: Wire the flows + CLI surface + docs

### Task 4: Make all three flows honor the resolved config

**Files:**
- Modify: `cli/src/auth/pkce.rs` (use `config.scopes()`, `&config.token_url()`, `config.auth_url()`)
- Modify: `cli/src/auth/refresh.rs` (call `.with_profile(aware_home, alias)`; `&cfg.token_url()`)
- Modify: `cli/src/auth/device.rs` (`match cfg.id.as_str()`, `cfg.scopes()`, tenant fallback to `cfg.tenant()`)

- [ ] **Step 1:** Update `pkce.rs`:
  - scopes: `let mut all_scopes = config.scopes();` then push extras.
  - auth URL base: `base = config.auth_url(),`
  - token post: `ureq::post(config.token_url())` (now `&str` from method).
- [ ] **Step 2:** Update `refresh.rs`:
  - signature already has `aware_home` + `alias`; change `let cfg = config::for_integration(integration)?;` → `let cfg = config::for_integration(integration)?.with_profile(aware_home, alias)?;`
  - `ureq::post(cfg.token_url())`.
- [ ] **Step 3:** Update `device.rs`:
  - `device_endpoints_for`: `match cfg.id.as_str()`; for m365 `let t = tenant.or(cfg.tenant()).unwrap_or("common");` so a profile tenant is honored when `--tenant` is absent; `token_url: cfg.token_url().to_string()` in fallback arms.
  - scopes: `let scopes: Vec<String> = cfg.scopes().into_iter().chain(extra_scopes.iter().cloned()).collect();`
  - Fix the existing device tests that call `for_integration(...)` — they still compile (overlay default), but assertions on endpoints stay valid since no profile is loaded.
- [ ] **Step 4:** Run `cargo test --bin aware auth::` → PASS (all flows + existing tests).
- [ ] **Step 5: Commit** `git add cli/src/auth/pkce.rs cli/src/auth/refresh.rs cli/src/auth/device.rs && git commit -m "feat(auth): PKCE/refresh/device-code honor BYO profile (#146)"`

---

### Task 5: `aware connect` wiring — `--set-app-secret`, profile overlay, active-app display

**Files:**
- Modify: `cli/src/commands/connect.rs`

- [ ] **Step 1: Add the flag** to `ConnectArgs`:
```rust
/// Store a BYO OAuth client secret (read from stdin) in the OS keychain for this
/// integration, then exit. Use with your own registered app: `echo $SECRET | aware connect google-workspace --set-app-secret`.
#[arg(long = "set-app-secret")]
pub set_app_secret: bool,
```

- [ ] **Step 2: Handle it early** in `run_connect` (after `--list`, before the for_integration token flow):
```rust
if args.set_app_secret {
    let integration = args.integration.as_deref().unwrap();
    // validate it's a known integration
    crate::auth::config::for_integration(integration)?;
    let mut secret = String::new();
    std::io::Read::read_to_string(&mut std::io::stdin(), &mut secret)
        .map_err(|e| AwareError::Validation(format!("reading secret from stdin: {e}")))?;
    let secret = secret.trim();
    if secret.is_empty() {
        return Err(AwareError::Validation("no secret on stdin (pipe it: echo $SECRET | aware connect <int> --set-app-secret)".into()));
    }
    crate::auth::keychain::store_app_secret(integration, args.r#as.as_deref(), secret, &ctx.paths.aware_home)?;
    println!("\u{2713} stored BYO client secret for {integration} (OS keychain or ~/.aware/credentials fallback)");
    return Ok(());
}
```

- [ ] **Step 3: Overlay the profile** wherever `for_integration` builds the flow config:
  - `let cfg = crate::auth::config::for_integration(integration)?.with_profile(&ctx.paths.aware_home, args.r#as.as_deref())?;`
  - Resolve tenant from profile when `--tenant` absent before the device-code call: `let tenant = args.tenant.clone().or_else(|| cfg.tenant().map(String::from));` and pass `tenant.as_deref()`.

- [ ] **Step 4: Show active app** in the status helpers. Extend `credential_status_json` and `print_credential_status_text` to take `aware_home` (already have it) and append the app source. Compute via:
```rust
let app = crate::auth::config::for_integration(integration)
    .ok()
    .and_then(|c| c.with_profile(aware_home, alias).ok())
    .map(|c| c.app_source_label())
    .unwrap_or("first-party");
```
  - JSON: add `"app": app`.
  - Text: append ` [app: {app}]` to the valid/expired lines.

- [ ] **Step 5: Add tests** (connect.rs `mod tests`):
```rust
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
```

- [ ] **Step 6:** `cargo test --bin aware` (full bin) → PASS. Manually sanity-check: `cargo run -- connect --list` and `echo test | cargo run -- connect google-workspace --set-app-secret`.
- [ ] **Step 7: Commit** `git add cli/src/commands/connect.rs && git commit -m "feat(connect): --set-app-secret + BYO profile overlay + active-app in --list (#146)"`

---

### Task 6: `doctor` shows active OAuth app + docs

**Files:**
- Modify: `cli/src/commands/doctor.rs` (credentials section already calls the shared connect helpers — confirm it picks up the new `app` field; if doctor builds its own credential lines, add the app label there too)
- Modify: `10-core/oauth-registration.md` (org self-registration section)

- [ ] **Step 1:** Confirm doctor's credentials block uses `connect::credential_status_json` / `print_credential_status_text`; if so the app label flows through for free. If doctor renders its own, mirror the `[app: ...]` addition. Add/adjust a doctor test only if doctor has its own credential rendering.
- [ ] **Step 2: Docs.** Append to `10-core/oauth-registration.md` a "## Bring your own app (Tier 2)" section:
  - When to use it (internal-use app → no Google CASA / no M365 publisher verification).
  - Per-provider: register a single-tenant (M365) / internal (Google) / your-own (Trimble) OAuth app; copy the client_id.
  - Write `~/.aware/oauth/<integration>.yaml` (show the M365 + Google examples with `client_id`, `tenant`, optional `scopes`).
  - Store the secret: `echo $SECRET | aware connect google-workspace --set-app-secret` (never put it in the YAML).
  - Per-alias profiles: `<integration>.<alias>.yaml` + `--as`.
  - Precedence note: profile > env var > bundled default; verify with `aware connect --list`.
- [ ] **Step 3:** `cargo test --bin aware` → PASS.
- [ ] **Step 4: Commit** `git add cli/src/commands/doctor.rs 10-core/oauth-registration.md && git commit -m "docs(oauth): Tier 2 BYO self-registration runbook (#146)"`

---

## Final verification (before PR)

- [ ] `cargo fmt` on **changed files only** (NOT `--all` — see CLAUDE.md rustfmt-drift gotcha). E.g. `cargo fmt -- cli/src/auth/profile.rs cli/src/auth/config.rs ...`.
- [ ] `cargo clippy --all-targets -- -D warnings` clean.
- [ ] `cargo test --bin aware` green.
- [ ] Re-read #146 acceptance criteria; tick each.
- [ ] PR review gate: **Codex first** (`codex exec review --base main`), fall back to `pr-review-toolkit:code-reviewer` only if Codex genuinely errors.
- [ ] Do NOT commit any secret. Bundled source carries only public client IDs.

## Notes / gotchas (carried from handoff)
- `cargo` not on bash PATH → use PowerShell with `$env:Path = "$env:USERPROFILE\.cargo\bin;$env:Path"`.
- `aware` is a binary crate → `cargo test --bin aware <filter>`, never `--lib`.
- Env-var-reading tests (`client_id_env`) can race under parallel test threads. The new tests avoid setting process env where possible (profile files in a tmp dir are race-free); if an env-precedence test is added, gate it behind a single serialized test or use a unique env key.
- Never auto-commit; commit only when the user asks.
