//! Runtime context (re-exports template::RenderContext + secret loader).

use std::path::Path;

use crate::error::AwareError;
use crate::runtime::template::RenderContext;

pub use crate::runtime::template::RenderContext as RuntimeContext;

/// Load a secret credential into the context.
///
/// Lookup order:
/// 1. OS keychain (`aware-aeco` service, account = `id`) — used in production.
/// 2. Plain JSON file `<creds_dir>/<id>.json` — fallback for tests + v0.3 migration.
///
/// Missing credentials are not an error (they are surfaced at template render time).
pub fn load_secret(ctx: &mut RenderContext, creds_dir: &Path, id: &str) -> Result<(), AwareError> {
    // `creds_dir` is `<aware_home>/credentials` at real call sites; derive the
    // parent so we can pass `aware_home` to the new keychain API (which handles
    // the file fallback internally). If `creds_dir` has no parent we fall back
    // to using it as-is.
    let aware_home = creds_dir.parent().unwrap_or(creds_dir);
    // Try OS keychain (with built-in file fallback at <aware_home>/credentials/)
    if let Ok(Some(token)) = crate::auth::keychain::load_token(id, None, aware_home) {
        let value = serde_json::to_value(&token)
            .map_err(|e| AwareError::Internal(format!("token to value: {e}")))?;
        ctx.secrets.insert(id.to_string(), value);
        return Ok(());
    }
    // Secondary file fallback: raw JSON directly under `creds_dir` (supports
    // legacy `{"token":"..."}` format in tests and v0.3 migration files).
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

/// Load the app's `config.yaml` into the `config` namespace, so documented
/// `{{ config.<key> }}` references resolve at run time.
///
/// app-spec § Templating: *"App config: `{{ config.<key> }}` (resolved against
/// `~/.aware/apps/<name>/config.yaml`)"*. The renderer already exposes a `config`
/// namespace; nothing populated it from disk, so every `{{ config.x }}` silently
/// rendered empty (#230). This closes that gap.
///
/// Mirrors [`load_secret`] on the soft path — a **missing** (or empty / comments-
/// only) `config.yaml` is not an error, since an app may declare no config. But a
/// present-but-**malformed** file, or one whose top level is not a mapping, IS an
/// error: a silently-empty `config` namespace is exactly the footgun #230 reports,
/// so we fail loudly rather than render nothing. Each top-level key of the mapping
/// becomes a `config.<key>` entry.
pub fn load_app_config(ctx: &mut RenderContext, app_dir: &Path) -> Result<(), AwareError> {
    let path = app_dir.join("config.yaml");
    if !path.is_file() {
        return Ok(()); // soft: an app may legitimately ship no config.yaml
    }
    let text = std::fs::read_to_string(&path)?;
    let value: serde_json::Value = serde_yaml::from_str(&text)
        .map_err(|e| AwareError::Validation(format!("config.yaml: {e}")))?;
    match value {
        // Empty file or comments-only → no config keys (not an error).
        serde_json::Value::Null => Ok(()),
        serde_json::Value::Object(map) => {
            for (k, v) in map {
                ctx.config.insert(k, v);
            }
            Ok(())
        }
        _ => Err(AwareError::Validation(
            "config.yaml: top level must be a mapping of keys to values".into(),
        )),
    }
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

    #[test]
    fn loads_app_config_keys() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(
            tmp.path().join("config.yaml"),
            "scene:\n  meta: { name: x }\n  elements: []\nschedule-rows: 12\n",
        )
        .unwrap();
        let mut ctx = RuntimeContext::default();
        load_app_config(&mut ctx, tmp.path()).unwrap();
        // Structured value preserved as an object; scalar preserved as a number.
        assert_eq!(ctx.config["scene"]["meta"]["name"], "x");
        assert!(ctx.config["scene"]["elements"].is_array());
        assert_eq!(ctx.config["schedule-rows"], 12);
    }

    #[test]
    fn config_resolves_through_the_renderer_end_to_end() {
        // The #230 regression: `{{ config.<key> }}` rendered empty because nothing
        // loaded config.yaml. Prove the whole path now works through `template`.
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("config.yaml"), "scene: { name: tower }\n").unwrap();
        let mut ctx = RuntimeContext::default();
        load_app_config(&mut ctx, tmp.path()).unwrap();
        let out = crate::runtime::template::render("{{ config.scene.name }}", &ctx).unwrap();
        assert_eq!(out, "tower");
    }

    #[test]
    fn missing_app_config_is_not_an_error() {
        let tmp = tempfile::tempdir().unwrap();
        let mut ctx = RuntimeContext::default();
        load_app_config(&mut ctx, tmp.path()).unwrap();
        assert!(ctx.config.is_empty());
    }

    #[test]
    fn empty_or_comment_only_app_config_is_not_an_error() {
        for body in ["", "   \n", "# just a comment\n"] {
            let tmp = tempfile::tempdir().unwrap();
            std::fs::write(tmp.path().join("config.yaml"), body).unwrap();
            let mut ctx = RuntimeContext::default();
            load_app_config(&mut ctx, tmp.path()).unwrap();
            assert!(ctx.config.is_empty(), "body {body:?}");
        }
    }

    #[test]
    fn malformed_or_non_mapping_app_config_is_an_error() {
        // Broken YAML → error (don't silently render an empty namespace).
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("config.yaml"), "scene: [unterminated\n").unwrap();
        let mut ctx = RuntimeContext::default();
        assert!(load_app_config(&mut ctx, tmp.path()).is_err());

        // Valid YAML but a non-mapping top level (a sequence) → error.
        let tmp2 = tempfile::tempdir().unwrap();
        std::fs::write(tmp2.path().join("config.yaml"), "- a\n- b\n").unwrap();
        let mut ctx2 = RuntimeContext::default();
        assert!(load_app_config(&mut ctx2, tmp2.path()).is_err());
    }
}
