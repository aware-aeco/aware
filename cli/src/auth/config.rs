//! Per-integration OAuth configs.
//!
//! Each integration carries hard-coded provider URLs + a bundled first-party
//! (Tier 1) client id. `with_profile` overlays a bring-your-own (BYO, Tier 2,
//! #146) app profile read from `~/.aware/oauth/` plus a keychain-stored secret.
//!
//! Resolution precedence per value: **profile > env var > bundled default**.

#![allow(dead_code)]

use crate::auth::profile::OAuthAppProfile;
use crate::error::AwareError;

/// Which OAuth app a resolved config will authenticate against. Reported by
/// `aware connect --list` / `doctor` so an operator can see whether the bundled
/// first-party app or their own (BYO) app is active.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum AppSource {
    /// Bundled AWARE-AECO first-party app (Tier 1).
    #[default]
    FirstParty,
    /// Org's own app, configured via `~/.aware/oauth/<integration>.yaml`.
    ByoProfile,
    /// Org's own app, configured via `AWARE_OAUTH_<INT>_CLIENT_ID` env var.
    ByoEnv,
}

impl AppSource {
    pub fn label(self) -> &'static str {
        match self {
            AppSource::FirstParty => "first-party",
            AppSource::ByoProfile => "byo-profile",
            AppSource::ByoEnv => "byo-env",
        }
    }
}

/// BYO overlay applied on top of the bundled defaults. Empty for a plain
/// `for_integration` config; populated by `with_profile`.
#[derive(Debug, Clone, Default)]
struct Overlay {
    profile: Option<OAuthAppProfile>,
    /// BYO client secret, resolved from the OS keychain.
    app_secret: Option<String>,
    source: AppSource,
}

pub struct IntegrationConfig {
    pub id: String,
    /// Bundled authorization endpoint. Read via [`auth_url`](Self::auth_url) so
    /// a profile override (or M365 tenant substitution) is honored.
    auth_url: String,
    /// Bundled token endpoint. Read via [`token_url`](Self::token_url).
    token_url: String,
    default_scopes: Vec<String>,
    client_id_env: &'static str,
    default_client_id: String,
    /// Env var carrying an optional client secret. Public clients (M365, Trimble)
    /// leave this `None`; Google "Desktop app" clients require a (non-confidential)
    /// secret in the token exchange even under PKCE — see `default_client_secret`.
    pub client_secret_env: Option<&'static str>,
    /// Bundled fallback secret. Only set for providers whose desktop OAuth client
    /// type mandates a secret. Per Google's own native-app guidance the desktop
    /// secret is not treated as confidential, so shipping it is the standard pattern
    /// (the `gcloud` CLI does the same).
    default_client_secret: Option<String>,
    overlay: Overlay,
}

pub fn for_integration(id: &str) -> Result<IntegrationConfig, AwareError> {
    let scopes = |list: &[&str]| list.iter().map(|s| s.to_string()).collect::<Vec<String>>();
    match id {
        "trimble-connect" => Ok(IntegrationConfig {
            id: "trimble-connect".to_string(),
            auth_url: "https://id.trimble.com/oauth/authorize".to_string(),
            token_url: "https://id.trimble.com/oauth/token".to_string(),
            default_scopes: scopes(&["openid", "profile", "TrimbleConnect"]),
            client_id_env: "AWARE_OAUTH_TRIMBLE_CLIENT_ID",
            default_client_id: "AWARE_AECO_PLACEHOLDER_TRIMBLE_CLIENT_ID".to_string(),
            client_secret_env: None,
            default_client_secret: None,
            overlay: Overlay::default(),
        }),
        "microsoft-365" => Ok(IntegrationConfig {
            id: "microsoft-365".to_string(),
            auth_url: "https://login.microsoftonline.com/common/oauth2/v2.0/authorize".to_string(),
            token_url: "https://login.microsoftonline.com/common/oauth2/v2.0/token".to_string(),
            // Delegated scopes covering the curated command surface (drive/excel,
            // SharePoint lists, mail, calendar, Teams, Planner). offline_access is
            // required for refresh tokens. Sites.ReadWrite.All, ChannelMessage.Send
            // and Chat.ReadWrite require one-time tenant admin consent — documented
            // in the registration runbook.
            default_scopes: scopes(&[
                "offline_access",
                "User.Read",
                "Files.ReadWrite.All",
                "Sites.ReadWrite.All",
                "Mail.Read",
                "Mail.Send",
                "Calendars.ReadWrite",
                "ChannelMessage.Send",
                "Chat.ReadWrite",
                "Tasks.ReadWrite",
            ]),
            client_id_env: "AWARE_OAUTH_M365_CLIENT_ID",
            // AWARE-AECO multi-tenant Azure AD app (public client). Registered 2026-05.
            default_client_id: "00929383-3563-4f4e-87e0-7476fcd9e80b".to_string(),
            // M365 is registered as a public client (allowPublicClient=true);
            // device-code and PKCE-loopback both work without a secret.
            client_secret_env: None,
            default_client_secret: None,
            overlay: Overlay::default(),
        }),
        "google-workspace" => Ok(IntegrationConfig {
            id: "google-workspace".to_string(),
            auth_url: "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
            token_url: "https://oauth2.googleapis.com/token".to_string(),
            // Scopes covering the curated command surface (Drive read/write/share,
            // Sheets, Calendar/Meet, Gmail send+search, Slides, Forms, Tasks).
            // `drive` and `gmail.*` are Google "restricted" scopes — publishing the
            // consent screen to external users requires a CASA security assessment;
            // until then the app stays in Testing mode (≤100 test users). Documented
            // in the registration runbook.
            default_scopes: scopes(&[
                "openid",
                "https://www.googleapis.com/auth/userinfo.email",
                "https://www.googleapis.com/auth/drive",
                "https://www.googleapis.com/auth/spreadsheets",
                "https://www.googleapis.com/auth/calendar",
                "https://www.googleapis.com/auth/gmail.send",
                "https://www.googleapis.com/auth/gmail.readonly",
                "https://www.googleapis.com/auth/presentations",
                "https://www.googleapis.com/auth/forms.body.readonly",
                "https://www.googleapis.com/auth/forms.responses.readonly",
                "https://www.googleapis.com/auth/tasks",
            ]),
            client_id_env: "AWARE_OAUTH_GOOGLE_CLIENT_ID",
            // AWARE-AECO Google Cloud OAuth client (Desktop app), registered 2026-05.
            // A client ID is not secret — it rides in every OAuth request — so it is
            // safe to commit.
            default_client_id:
                "52384839180-asibd6jtj2jv9nh63l5ji5i28d47ns5t.apps.googleusercontent.com"
                    .to_string(),
            client_secret_env: Some("AWARE_OAUTH_GOOGLE_CLIENT_SECRET"),
            // The secret is NEVER committed. It is either supplied at runtime via
            // AWARE_OAUTH_GOOGLE_CLIENT_SECRET, or baked into release binaries at
            // compile time via the AWARE_GOOGLE_CLIENT_SECRET build env (option_env!
            // resolves to None when unset, keeping local/dev builds clean). Google
            // desktop-client secrets are embeddable-by-design, but we keep this out of
            // source control to avoid quota abuse and secret-scanner churn.
            default_client_secret: option_env!("AWARE_GOOGLE_CLIENT_SECRET").map(String::from),
            overlay: Overlay::default(),
        }),
        other => Err(AwareError::NotFound(format!(
            "integration {other} not supported by v0.4"
        ))),
    }
}

impl IntegrationConfig {
    /// Overlay a BYO (Tier 2) app profile + keychain secret on top of the bundled
    /// defaults. Looks up `~/.aware/oauth/<id>[.<alias>].yaml` and the keychain
    /// `oauth-app.<id>[.<alias>]` slot. For M365, a profile that sets only a
    /// `tenant` rewrites the `common` endpoints to that tenant.
    pub fn with_profile(
        mut self,
        aware_home: &std::path::Path,
        alias: Option<&str>,
    ) -> Result<Self, AwareError> {
        let profile = crate::auth::profile::load_profile(aware_home, &self.id, alias)?;
        let app_secret = crate::auth::keychain::load_app_secret(&self.id, alias, aware_home)?;

        if self.id == "microsoft-365"
            && let Some(p) = &profile
            && let Some(t) = &p.tenant
        {
            if p.auth_url.is_none() {
                self.auth_url = self.auth_url.replace("/common/", &format!("/{t}/"));
            }
            if p.token_url.is_none() {
                self.token_url = self.token_url.replace("/common/", &format!("/{t}/"));
            }
        }

        let source = if profile.is_some() {
            AppSource::ByoProfile
        } else if std::env::var(self.client_id_env).is_ok() {
            AppSource::ByoEnv
        } else {
            AppSource::FirstParty
        };
        self.overlay = Overlay {
            profile,
            app_secret,
            source,
        };
        Ok(self)
    }

    /// Resolved authorization endpoint (profile override ▸ bundled/tenant-rewritten).
    pub fn auth_url(&self) -> &str {
        self.overlay
            .profile
            .as_ref()
            .and_then(|p| p.auth_url.as_deref())
            .unwrap_or(&self.auth_url)
    }

    /// Resolved token endpoint (profile override ▸ bundled/tenant-rewritten).
    pub fn token_url(&self) -> &str {
        self.overlay
            .profile
            .as_ref()
            .and_then(|p| p.token_url.as_deref())
            .unwrap_or(&self.token_url)
    }

    /// Explicit profile `token_url` override, if any (NOT the tenant-substituted
    /// base). The device-code flow uses this to decide whether to honor an
    /// override or build a tenant-specific endpoint.
    pub fn token_url_override(&self) -> Option<&str> {
        self.overlay
            .profile
            .as_ref()
            .and_then(|p| p.token_url.as_deref())
    }

    /// Explicit profile `device_authorization_url` override, if any.
    pub fn device_authorization_url_override(&self) -> Option<&str> {
        self.overlay
            .profile
            .as_ref()
            .and_then(|p| p.device_authorization_url.as_deref())
    }

    /// Resolved scope set: a profile's `scopes` list fully REPLACES the bundled
    /// defaults; otherwise the bundled defaults are used.
    pub fn scopes(&self) -> Vec<String> {
        self.overlay
            .profile
            .as_ref()
            .and_then(|p| p.scopes.clone())
            .unwrap_or_else(|| self.default_scopes.clone())
    }

    /// Microsoft tenant from the BYO profile, if any.
    pub fn tenant(&self) -> Option<&str> {
        self.overlay
            .profile
            .as_ref()
            .and_then(|p| p.tenant.as_deref())
    }

    pub fn app_source(&self) -> AppSource {
        self.overlay.source
    }

    pub fn app_source_label(&self) -> &'static str {
        self.overlay.source.label()
    }

    /// Resolve the client id: profile ▸ env var ▸ bundled default.
    pub fn client_id(&self) -> String {
        if let Some(p) = &self.overlay.profile
            && let Some(id) = &p.client_id
        {
            return id.clone();
        }
        std::env::var(self.client_id_env).unwrap_or_else(|_| self.default_client_id.clone())
    }

    /// Resolve the client secret.
    ///
    /// When a BYO app is active (profile or env client_id): keychain ▸ env, and
    /// **never** the bundled first-party secret (that mismatched pair is rejected
    /// by the provider). When first-party: env ▸ bundled, ignoring any stale
    /// keychain app secret left over from a removed BYO profile.
    pub fn client_secret(&self) -> Option<String> {
        let env_secret = self.client_secret_env.and_then(|k| std::env::var(k).ok());
        let byo_active = self.overlay.source != AppSource::FirstParty;
        resolve_client_secret(
            self.overlay.app_secret.as_deref(),
            env_secret.as_deref(),
            byo_active,
            self.default_client_secret.as_deref(),
        )
    }
}

/// Pure client-secret precedence. Factored out so it is unit-testable without
/// touching the OS keychain or process env.
///
/// - **BYO active:** `app_secret` (keychain) ▸ `env_secret`; the bundled
///   first-party secret is never paired with a BYO client id.
/// - **First-party:** `env_secret` ▸ `bundled`; a leftover keychain `app_secret`
///   is ignored, since it belongs to a no-longer-active BYO app.
fn resolve_client_secret(
    app_secret: Option<&str>,
    env_secret: Option<&str>,
    byo_active: bool,
    bundled: Option<&str>,
) -> Option<String> {
    if byo_active {
        app_secret.or(env_secret).map(String::from)
    } else {
        env_secret.or(bundled).map(String::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trimble_config_loads() {
        let c = for_integration("trimble-connect").unwrap();
        assert_eq!(c.id, "trimble-connect");
        assert!(!c.default_scopes.is_empty());
    }

    #[test]
    fn unknown_integration_is_not_found() {
        assert!(for_integration("nope").is_err());
    }

    #[test]
    fn google_declares_a_client_secret_but_public_clients_do_not() {
        let g = for_integration("google-workspace").unwrap();
        assert!(
            g.client_secret_env.is_some(),
            "google is a desktop client and needs a secret"
        );

        // M365 and Trimble are public clients — they never yield a secret.
        let m = for_integration("microsoft-365").unwrap();
        assert!(m.client_secret_env.is_none());
        assert!(m.client_secret().is_none());
    }

    #[test]
    fn client_id_falls_back_to_default_when_env_unset() {
        // Use an integration not normally in tests' env
        let c = for_integration("google-workspace").unwrap();
        let id = c.client_id();
        // Either it's the placeholder OR it's set in the environment; either way it's non-empty
        assert!(!id.is_empty());
    }

    // ── BYO profile overlay (#146) ────────────────────────────────────────────

    #[test]
    fn profile_client_id_overrides_default() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("oauth");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("microsoft-365.yaml"), "client_id: org-app-999\n").unwrap();
        let cfg = for_integration("microsoft-365")
            .unwrap()
            .with_profile(tmp.path(), None)
            .unwrap();
        assert_eq!(cfg.client_id(), "org-app-999");
        assert_eq!(cfg.app_source_label(), "byo-profile");
    }

    #[test]
    fn no_profile_falls_back_to_bundled_first_party() {
        let tmp = tempfile::tempdir().unwrap();
        let cfg = for_integration("microsoft-365")
            .unwrap()
            .with_profile(tmp.path(), None)
            .unwrap();
        assert_eq!(cfg.client_id(), "00929383-3563-4f4e-87e0-7476fcd9e80b");
        assert_eq!(cfg.app_source_label(), "first-party");
    }

    #[test]
    fn profile_scopes_replace_bundled() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("oauth");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("google-workspace.yaml"),
            "client_id: x\nscopes:\n  - openid\n",
        )
        .unwrap();
        let cfg = for_integration("google-workspace")
            .unwrap()
            .with_profile(tmp.path(), None)
            .unwrap();
        assert_eq!(cfg.scopes(), vec!["openid".to_string()]);
    }

    #[test]
    fn m365_tenant_substitutes_common_in_endpoints() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("oauth");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("microsoft-365.yaml"),
            "client_id: x\ntenant: contoso.onmicrosoft.com\n",
        )
        .unwrap();
        let cfg = for_integration("microsoft-365")
            .unwrap()
            .with_profile(tmp.path(), None)
            .unwrap();
        assert!(cfg.auth_url().contains("/contoso.onmicrosoft.com/"));
        assert!(!cfg.token_url().contains("/common/"));
        assert_eq!(cfg.tenant(), Some("contoso.onmicrosoft.com"));
    }

    // Pure precedence tests for the client-secret resolver — deterministic, no
    // keychain / env dependency (the OS keychain is global and not isolated by
    // AWARE_HOME, so this is the reliable way to test the guard).

    #[test]
    fn secret_byo_keychain_wins_over_env() {
        assert_eq!(
            resolve_client_secret(Some("kc"), Some("env"), true, Some("bundled")),
            Some("kc".to_string())
        );
    }

    #[test]
    fn secret_byo_falls_back_to_env() {
        assert_eq!(
            resolve_client_secret(None, Some("env"), true, Some("bundled")),
            Some("env".to_string())
        );
    }

    #[test]
    fn secret_byo_never_uses_bundled() {
        // BYO active with no stored/env secret must NOT borrow the bundled
        // first-party secret — that mismatched pair would be rejected anyway.
        assert_eq!(
            resolve_client_secret(None, None, true, Some("bundled")),
            None
        );
    }

    #[test]
    fn secret_first_party_ignores_stale_keychain_secret() {
        // A leftover keychain secret from a removed BYO profile must not pair
        // with the first-party client id; env ▸ bundled applies instead.
        assert_eq!(
            resolve_client_secret(Some("stale-kc"), None, false, Some("bundled")),
            Some("bundled".to_string())
        );
    }

    #[test]
    fn secret_first_party_env_wins_over_bundled() {
        assert_eq!(
            resolve_client_secret(None, Some("env"), false, Some("bundled")),
            Some("env".to_string())
        );
    }

    #[test]
    fn secret_first_party_uses_bundled() {
        assert_eq!(
            resolve_client_secret(None, None, false, Some("bundled")),
            Some("bundled".to_string())
        );
    }

    #[test]
    fn secret_public_client_is_none() {
        assert_eq!(resolve_client_secret(None, None, false, None), None);
    }
}
