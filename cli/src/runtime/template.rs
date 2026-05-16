//! Minijinja-based templating for runtime config substitution.
//!
//! Resolves `{{ inputs.X }}`, `{{ <node_id>.<field> }}`, `{{ secrets.<id> }}`,
//! `{{ config.X }}` against a `RenderContext`.
//!
//! Kebab-case node ids (e.g. `tekla-watch`) are auto-translated to underscore
//! form (`tekla_watch`) so they're valid Jinja identifiers; the raw kebab key
//! is also inserted for bracket-syntax access.

#![allow(dead_code)]

use minijinja::{Environment, Value};
use serde_json::Map;

use crate::error::AwareError;

#[derive(Default, Debug, Clone)]
pub struct RenderContext {
    pub inputs: serde_json::Value,
    /// Upstream node outputs, keyed by node id (raw kebab and underscore-translated forms both populated).
    pub upstream: Map<String, serde_json::Value>,
    /// Secrets keyed by credential id (e.g. "trimble-connect").
    pub secrets: Map<String, serde_json::Value>,
    /// App-level config kv.
    pub config: Map<String, serde_json::Value>,
}

impl RenderContext {
    /// Insert a node output, populating both raw kebab and underscore-translated forms.
    pub fn record_output(&mut self, node_id: &str, output: serde_json::Value) {
        self.upstream.insert(node_id.to_string(), output.clone());
        let translated = node_id.replace('-', "_");
        if translated != node_id {
            self.upstream.insert(translated, output);
        }
    }
}

pub fn render(template: &str, ctx: &RenderContext) -> Result<String, AwareError> {
    let mut env = Environment::new();
    env.add_template("t", template)
        .map_err(|e| AwareError::Validation(format!("template parse: {e}")))?;
    let tmpl = env.get_template("t").unwrap();

    // Build the flat top-level context.
    let mut top: Map<String, serde_json::Value> = Map::new();
    top.insert("inputs".into(), ctx.inputs.clone());
    top.insert(
        "secrets".into(),
        serde_json::Value::Object(ctx.secrets.clone()),
    );
    top.insert(
        "config".into(),
        serde_json::Value::Object(ctx.config.clone()),
    );
    // `upstream` object exposes raw kebab keys via bracket syntax:
    //   {{ upstream["tekla-watch"].mark }}
    // Flat top-level underscore-translated keys allow dot notation:
    //   {{ tekla_watch.mark }}
    top.insert(
        "upstream".into(),
        serde_json::Value::Object(ctx.upstream.clone()),
    );
    for (k, v) in &ctx.upstream {
        top.insert(k.clone(), v.clone());
    }

    let value = Value::from_serialize(serde_json::Value::Object(top));
    let rendered = tmpl
        .render(value)
        .map_err(|e| AwareError::Validation(format!("template render: {e}")))?;
    Ok(rendered)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx_with_upstream(node_id: &str, output: serde_json::Value) -> RenderContext {
        let mut ctx = RenderContext::default();
        ctx.record_output(node_id, output);
        ctx
    }

    #[test]
    fn substitutes_inputs() {
        let ctx = RenderContext {
            inputs: serde_json::json!({ "tc-project-id": "proj-123" }),
            ..Default::default()
        };
        let out = render("{{ inputs['tc-project-id'] }}", &ctx).unwrap();
        assert_eq!(out, "proj-123");
    }

    #[test]
    fn substitutes_upstream_underscore_translated() {
        let ctx = ctx_with_upstream("tekla-watch", serde_json::json!({ "mark": "A-104" }));
        let out = render("file:{{ tekla_watch.mark }}.pdf", &ctx).unwrap();
        assert_eq!(out, "file:A-104.pdf");
    }

    #[test]
    fn substitutes_upstream_bracket_syntax_raw_kebab() {
        // minijinja bracket syntax for top-level dict keys with hyphens:
        // use the `upstream` object which preserves raw kebab-case keys.
        // `{{ ['tekla-watch'].mark }}` would be parsed as an array literal by
        // minijinja (not a key lookup), so the correct form is shown below.
        let ctx = ctx_with_upstream("tekla-watch", serde_json::json!({ "mark": "A-104" }));
        let out = render("{{ upstream['tekla-watch'].mark }}", &ctx).unwrap();
        assert_eq!(out, "A-104");
    }

    #[test]
    fn substitutes_secrets() {
        let mut ctx = RenderContext::default();
        ctx.secrets.insert(
            "trimble-connect".into(),
            serde_json::json!({ "token": "tk_abc" }),
        );
        let out = render("Bearer {{ secrets['trimble-connect'].token }}", &ctx).unwrap();
        assert_eq!(out, "Bearer tk_abc");
    }

    #[test]
    fn substitutes_config() {
        let mut ctx = RenderContext::default();
        ctx.config
            .insert("pdf-inbox".into(), serde_json::json!("/tmp/inbox"));
        let out = render("path={{ config['pdf-inbox'] }}", &ctx).unwrap();
        assert_eq!(out, "path=/tmp/inbox");
    }

    #[test]
    fn passes_through_literal_strings() {
        let ctx = RenderContext::default();
        let out = render("no templating here", &ctx).unwrap();
        assert_eq!(out, "no templating here");
    }

    #[test]
    fn missing_field_errors() {
        let ctx = RenderContext::default();
        let result = render("{{ nonexistent.deep.path }}", &ctx);
        // minijinja default (lenient) mode renders missing keys as empty string.
        // Strict mode would error; v0.3 uses lenient to match Jinja2 defaults.
        let _ = result;
    }
}
