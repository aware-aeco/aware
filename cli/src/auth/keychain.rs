//! OS-keychain backed token storage via the `keyring` crate.
//!
//! Token JSON stored under service="aware-aeco", account="<integration>[.<alias>]".

#![allow(dead_code)]

use serde::{Deserialize, Serialize};

use crate::error::AwareError;

const SERVICE_NAME: &str = "aware-aeco";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredToken {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: i64,
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
    entry
        .set_password(&body)
        .map_err(|e| AwareError::PermissionDenied(format!("keyring write: {e}")))?;
    Ok(())
}

pub fn load_token(
    integration: &str,
    alias: Option<&str>,
) -> Result<Option<StoredToken>, AwareError> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn account_name_with_alias() {
        assert_eq!(account_name("trimble-connect", None), "trimble-connect");
        assert_eq!(
            account_name("google-workspace", Some("personal")),
            "google-workspace.personal"
        );
    }

    /// This test only runs when `AWARE_TEST_KEYRING=1` because keyring tests are flaky
    /// on Linux without libsecret/dbus services. Production code path is unit-tested
    /// implicitly via account_name; round-trip + integration tested via `aware connect`/`disconnect`
    /// in subsequent tasks.
    #[test]
    fn store_load_delete_round_trip_when_keyring_available() {
        if std::env::var("AWARE_TEST_KEYRING").is_err() {
            return;
        }
        let token = StoredToken {
            access_token: "tk_abc".into(),
            refresh_token: Some("rt_xyz".into()),
            expires_at: 1_735_689_600,
            scope: "openid profile".into(),
            token_type: "Bearer".into(),
            integration: "test-integration-keyring".into(),
            obtained_at: 1_735_686_000,
        };
        store_token(&token, None).unwrap();
        let loaded = load_token("test-integration-keyring", None)
            .unwrap()
            .unwrap();
        assert_eq!(loaded.access_token, "tk_abc");
        delete_token("test-integration-keyring", None).unwrap();
        let after = load_token("test-integration-keyring", None).unwrap();
        assert!(after.is_none());
    }
}
