//! Bring-your-own (BYO) OAuth app profiles — Tier 2 of the hybrid auth model (#146).
//!
//! Non-secret app config (client_id, tenant, scopes, optional endpoint URLs) lives
//! in `~/.aware/oauth/<integration>[.<alias>].yaml`. The BYO client *secret* is NOT
//! stored here — it goes to the OS keychain (see `keychain::store_app_secret`).
//! Resolution precedence is applied in `config::IntegrationConfig::with_profile`.

#![allow(dead_code)]

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
    /// Explicit authorization endpoint override (rare; tenant substitution is
    /// automatic for M365 when only `tenant` is set).
    pub auth_url: Option<String>,
    /// Explicit token endpoint override.
    pub token_url: Option<String>,
    /// Explicit RFC 8628 device-authorization endpoint override (sovereign cloud
    /// / proxy). Only consulted by the device-code flow.
    pub device_authorization_url: Option<String>,
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
    // Strip a leading UTF-8 BOM — editors on Windows (e.g. Notepad) add one, and
    // the YAML parser would otherwise fold it into the first key and silently drop it.
    let raw = raw.strip_prefix('\u{feff}').unwrap_or(&raw);
    let profile: OAuthAppProfile = serde_yaml::from_str(raw)
        .map_err(|e| AwareError::Validation(format!("profile {}: {e}", path.display())))?;
    Ok(Some(profile))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn absent_profile_is_none() {
        let tmp = tempfile::tempdir().unwrap();
        assert!(
            load_profile(tmp.path(), "microsoft-365", None)
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn loads_default_profile_fields() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("oauth");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("microsoft-365.yaml"),
            "client_id: org-app-123\ntenant: contoso.onmicrosoft.com\nscopes:\n  - User.Read\n  - Mail.Send\n",
        )
        .unwrap();
        let p = load_profile(tmp.path(), "microsoft-365", None)
            .unwrap()
            .unwrap();
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
        std::fs::write(
            dir.join("google-workspace.staging.yaml"),
            "client_id: staging\n",
        )
        .unwrap();
        let p = load_profile(tmp.path(), "google-workspace", Some("staging"))
            .unwrap()
            .unwrap();
        assert_eq!(p.client_id.as_deref(), Some("staging"));
    }

    #[test]
    fn alias_falls_back_to_default_when_no_alias_file() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("oauth");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("google-workspace.yaml"), "client_id: base\n").unwrap();
        let p = load_profile(tmp.path(), "google-workspace", Some("nope"))
            .unwrap()
            .unwrap();
        assert_eq!(p.client_id.as_deref(), Some("base"));
    }

    #[test]
    fn leading_utf8_bom_is_tolerated() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("oauth");
        std::fs::create_dir_all(&dir).unwrap();
        // Editors on Windows prepend a UTF-8 BOM (EF BB BF).
        std::fs::write(
            dir.join("microsoft-365.yaml"),
            "\u{feff}client_id: org-app-bom\n",
        )
        .unwrap();
        let p = load_profile(tmp.path(), "microsoft-365", None)
            .unwrap()
            .unwrap();
        assert_eq!(p.client_id.as_deref(), Some("org-app-bom"));
    }

    #[test]
    fn malformed_yaml_is_validation_error() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("oauth");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("microsoft-365.yaml"), "client_id: [unterminated\n").unwrap();
        assert!(matches!(
            load_profile(tmp.path(), "microsoft-365", None),
            Err(AwareError::Validation(_))
        ));
    }
}
