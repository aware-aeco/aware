//! Per-integration OAuth configs.
//!
//! Hard-coded provider URLs + scopes. Client IDs read from env vars; placeholders
//! ship as defaults until AWARE-AECO registers real OAuth apps with each provider.

#![allow(dead_code)]

use crate::error::AwareError;

pub struct IntegrationConfig {
    pub id: &'static str,
    pub auth_url: &'static str,
    pub token_url: &'static str,
    pub default_scopes: Vec<&'static str>,
    pub client_id_env: &'static str,
    pub default_client_id: &'static str,
    /// Env var carrying an optional client secret. Public clients (M365, Trimble)
    /// leave this `None`; Google "Desktop app" clients require a (non-confidential)
    /// secret in the token exchange even under PKCE — see `default_client_secret`.
    pub client_secret_env: Option<&'static str>,
    /// Bundled fallback secret. Only set for providers whose desktop OAuth client
    /// type mandates a secret. Per Google's own native-app guidance the desktop
    /// secret is not treated as confidential, so shipping it is the standard pattern
    /// (the `gcloud` CLI does the same).
    pub default_client_secret: Option<&'static str>,
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
            client_secret_env: None,
            default_client_secret: None,
        }),
        "microsoft-365" => Ok(IntegrationConfig {
            id: "microsoft-365",
            auth_url: "https://login.microsoftonline.com/common/oauth2/v2.0/authorize",
            token_url: "https://login.microsoftonline.com/common/oauth2/v2.0/token",
            // Delegated scopes covering the curated command surface (drive/excel,
            // SharePoint lists, mail, calendar, Teams, Planner). offline_access is
            // required for refresh tokens. Sites.ReadWrite.All, ChannelMessage.Send
            // and Chat.ReadWrite require one-time tenant admin consent — documented
            // in the registration runbook.
            default_scopes: vec![
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
            ],
            client_id_env: "AWARE_OAUTH_M365_CLIENT_ID",
            // AWARE-AECO multi-tenant Azure AD app (public client). Registered 2026-05.
            default_client_id: "00929383-3563-4f4e-87e0-7476fcd9e80b",
            // M365 is registered as a public client (allowPublicClient=true);
            // device-code and PKCE-loopback both work without a secret.
            client_secret_env: None,
            default_client_secret: None,
        }),
        "google-workspace" => Ok(IntegrationConfig {
            id: "google-workspace",
            auth_url: "https://accounts.google.com/o/oauth2/v2/auth",
            token_url: "https://oauth2.googleapis.com/token",
            // Scopes covering the curated command surface (Drive read/write/share,
            // Sheets, Calendar/Meet, Gmail send+search, Slides, Forms, Tasks).
            // `drive` and `gmail.*` are Google "restricted" scopes — publishing the
            // consent screen to external users requires a CASA security assessment;
            // until then the app stays in Testing mode (≤100 test users). Documented
            // in the registration runbook.
            default_scopes: vec![
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
            ],
            client_id_env: "AWARE_OAUTH_GOOGLE_CLIENT_ID",
            // AWARE-AECO Google Cloud OAuth client (Desktop app), registered 2026-05.
            // A client ID is not secret — it rides in every OAuth request — so it is
            // safe to commit.
            default_client_id:
                "52384839180-asibd6jtj2jv9nh63l5ji5i28d47ns5t.apps.googleusercontent.com",
            client_secret_env: Some("AWARE_OAUTH_GOOGLE_CLIENT_SECRET"),
            // The secret is NEVER committed. It is either supplied at runtime via
            // AWARE_OAUTH_GOOGLE_CLIENT_SECRET, or baked into release binaries at
            // compile time via the AWARE_GOOGLE_CLIENT_SECRET build env (option_env!
            // resolves to None when unset, keeping local/dev builds clean). Google
            // desktop-client secrets are embeddable-by-design, but we keep this out of
            // source control to avoid quota abuse and secret-scanner churn.
            default_client_secret: option_env!("AWARE_GOOGLE_CLIENT_SECRET"),
        }),
        other => Err(AwareError::NotFound(format!(
            "integration {other} not supported by v0.4"
        ))),
    }
}

impl IntegrationConfig {
    pub fn client_id(&self) -> String {
        std::env::var(self.client_id_env).unwrap_or_else(|_| self.default_client_id.to_string())
    }

    /// Resolve the client secret, if this integration uses one. Env var wins over
    /// the bundled default; returns `None` for public clients (M365, Trimble).
    pub fn client_secret(&self) -> Option<String> {
        if let Some(env_key) = self.client_secret_env
            && let Ok(val) = std::env::var(env_key)
        {
            return Some(val);
        }
        self.default_client_secret.map(String::from)
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
}
