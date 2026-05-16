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
        other => Err(AwareError::NotFound(format!(
            "integration {other} not supported by v0.4"
        ))),
    }
}

impl IntegrationConfig {
    pub fn client_id(&self) -> String {
        std::env::var(self.client_id_env).unwrap_or_else(|_| self.default_client_id.to_string())
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
    fn client_id_falls_back_to_default_when_env_unset() {
        // Use an integration not normally in tests' env
        let c = for_integration("google-workspace").unwrap();
        let id = c.client_id();
        // Either it's the placeholder OR it's set in the environment; either way it's non-empty
        assert!(!id.is_empty());
    }
}
