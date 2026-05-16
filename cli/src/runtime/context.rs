//! Runtime context (re-exports template::RenderContext + secret loader).

#![allow(dead_code)]

use std::path::Path;

use crate::error::AwareError;
use crate::runtime::template::RenderContext;

#[allow(unused_imports)]
pub use crate::runtime::template::RenderContext as RuntimeContext;

/// Load a secret credential into the context.
///
/// Lookup order:
/// 1. OS keychain (`aware-aeco` service, account = `id`) — used in production.
/// 2. Plain JSON file `<creds_dir>/<id>.json` — fallback for tests + v0.3 migration.
///
/// Missing credentials are not an error (they are surfaced at template render time).
pub fn load_secret(ctx: &mut RenderContext, creds_dir: &Path, id: &str) -> Result<(), AwareError> {
    // Try OS keychain first
    if let Ok(Some(token)) = crate::auth::keychain::load_token(id, None) {
        let value = serde_json::to_value(&token)
            .map_err(|e| AwareError::Internal(format!("token to value: {e}")))?;
        ctx.secrets.insert(id.to_string(), value);
        return Ok(());
    }
    // Fallback: plain JSON file (for tests + v0.3 migration)
    let path = creds_dir.join(format!("{id}.json"));
    if !path.is_file() {
        return Ok(()); // soft: missing secrets aren't fatal at load time
    }
    let body = std::fs::read_to_string(&path)?;
    let v: serde_json::Value = serde_json::from_str(&body)
        .map_err(|e| AwareError::Validation(format!("secret {id}: {e}")))?;
    ctx.secrets.insert(id.to_string(), v);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_existing_secret() {
        let tmp = tempfile::tempdir().unwrap();
        let creds = tmp.path();
        std::fs::write(creds.join("trimble-connect.json"), r#"{"token":"tk_abc"}"#).unwrap();
        let mut ctx = RuntimeContext::default();
        load_secret(&mut ctx, creds, "trimble-connect").unwrap();
        assert_eq!(ctx.secrets["trimble-connect"]["token"], "tk_abc");
    }

    #[test]
    fn missing_secret_is_not_an_error() {
        let tmp = tempfile::tempdir().unwrap();
        let mut ctx = RuntimeContext::default();
        load_secret(&mut ctx, tmp.path(), "nope").unwrap();
        assert!(ctx.secrets.is_empty());
    }
}
