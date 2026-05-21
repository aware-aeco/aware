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

/// Build the ambient `run` context (`run.id`, `run.date`, `run.operator`) that is
/// injected into every node's template environment for a single run, so the
/// documented time-dependent expressions resolve (#127).
///
/// - `run.id`   — the run identifier (the same UUID used for the provenance log).
/// - `run.date` — the UTC calendar date as `YYYY-MM-DD`, the form the specs and
///   example apps embed in file names and titles (`monday-audit-{{ run.date }}`).
/// - `run.operator` — the human running the workflow, from `AWARE_OPERATOR`, else
///   the OS user (`USER` / `USERNAME`), else empty.
///
/// Populated identically for live, dry-run, and simulate runs: the goal is only
/// that `{{ run.* }}` renders, which a real value satisfies as well as a stub.
pub fn run_context(run_id: &str) -> serde_json::Map<String, serde_json::Value> {
    let date = chrono::Utc::now().format("%Y-%m-%d").to_string();
    let operator = std::env::var("AWARE_OPERATOR")
        .or_else(|_| std::env::var("USER"))
        .or_else(|_| std::env::var("USERNAME"))
        .unwrap_or_default();
    let mut m = serde_json::Map::new();
    m.insert("id".into(), serde_json::Value::String(run_id.to_string()));
    m.insert("date".into(), serde_json::Value::String(date));
    m.insert("operator".into(), serde_json::Value::String(operator));
    m
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

    #[test]
    fn run_context_populates_id_date_operator() {
        let m = run_context("run-123");
        assert_eq!(m["id"], serde_json::json!("run-123"));
        let date = m["date"].as_str().unwrap();
        assert_eq!(date.len(), 10, "run.date should be YYYY-MM-DD: {date:?}");
        assert_eq!(date.matches('-').count(), 2, "run.date format: {date:?}");
        // `operator` is always present (possibly empty) so `{{ run.operator }}` renders.
        assert!(m.contains_key("operator"));
        assert!(m["operator"].is_string());
    }
}
