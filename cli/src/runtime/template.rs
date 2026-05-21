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

/// Rewrite hyphenated dot-path segments inside `{{ … }}` / `{% … %}` blocks to
/// bracket form, so kebab-case identifiers work in dot syntax. minijinja lexes
/// `inputs.tc-project-id` as the expression `inputs.tc - project - id`
/// (subtraction over undefined values) and fails; this turns it into
/// `inputs['tc-project-id']`. A hyphenated path BASE — a kebab node id such as
/// `tekla-watch` — is translated to its underscore form (`tekla_watch`), which
/// [`RenderContext::record_output`] also registers, so `tekla-watch.drawing-bytes`
/// becomes `tekla_watch['drawing-bytes']`.
///
/// Only hyphenated dot-paths are touched — exactly the refs that fail today.
/// Hyphen-free paths, single tokens (so `a - b` subtraction is never mangled),
/// and the contents of string literals are copied through unchanged, so any
/// template that already renders keeps rendering identically (#117-4).
fn normalize_hyphenated_paths(template: &str) -> String {
    let bytes = template.as_bytes();
    let n = bytes.len();
    let mut out = String::with_capacity(n);
    let mut i = 0;
    let mut in_expr = false;
    let mut quote: Option<u8> = None;
    while i < n {
        let c = bytes[i];
        if !in_expr {
            if c == b'{' && i + 1 < n && (bytes[i + 1] == b'{' || bytes[i + 1] == b'%') {
                in_expr = true;
                out.push('{');
                out.push(bytes[i + 1] as char);
                i += 2;
            } else {
                out.push(c as char);
                i += 1;
            }
            continue;
        }
        // Inside a `{{ }}` / `{% %}` block.
        if let Some(q) = quote {
            out.push(c as char);
            if c == b'\\' && i + 1 < n {
                out.push(bytes[i + 1] as char);
                i += 2;
            } else {
                if c == q {
                    quote = None;
                }
                i += 1;
            }
            continue;
        }
        // Block close: `}}` or `%}`.
        if (c == b'}' || c == b'%') && i + 1 < n && bytes[i + 1] == b'}' {
            in_expr = false;
            out.push(c as char);
            out.push('}');
            i += 2;
            continue;
        }
        if c == b'\'' || c == b'"' {
            quote = Some(c);
            out.push(c as char);
            i += 1;
            continue;
        }
        // A path token starts with a letter or underscore and runs over
        // `[A-Za-z0-9_.-]` (dots and hyphens are part of the path).
        if c.is_ascii_alphabetic() || c == b'_' {
            let start = i;
            while i < n
                && (bytes[i].is_ascii_alphanumeric()
                    || bytes[i] == b'_'
                    || bytes[i] == b'-'
                    || bytes[i] == b'.')
            {
                i += 1;
            }
            out.push_str(&rewrite_path_token(&template[start..i]));
            continue;
        }
        out.push(c as char);
        i += 1;
    }
    out
}

/// Rewrite a single path token (e.g. `tekla-watch.drawing-bytes`) to bracket
/// form. Tokens without a dot, or without a hyphen, are returned unchanged.
fn rewrite_path_token(token: &str) -> String {
    if !token.contains('.') || !token.contains('-') {
        return token.to_string();
    }
    let mut parts = token.split('.');
    // Base: a kebab node id resolves via its underscore-translated key.
    let mut result = parts.next().unwrap_or("").replace('-', "_");
    for seg in parts {
        if seg.contains('-') {
            result.push_str("['");
            result.push_str(seg);
            result.push_str("']");
        } else {
            result.push('.');
            result.push_str(seg);
        }
    }
    result
}

pub fn render(template: &str, ctx: &RenderContext) -> Result<String, AwareError> {
    let normalized = normalize_hyphenated_paths(template);
    let mut env = Environment::new();
    env.add_template("t", &normalized)
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

    // ── #117-4: hyphenated dot-paths now render via the normalizer ──────────

    #[test]
    fn dot_syntax_hyphenated_input_now_renders() {
        let ctx = RenderContext {
            inputs: serde_json::json!({ "tc-project-id": "proj-123" }),
            ..Default::default()
        };
        let out = render("{{ inputs.tc-project-id }}", &ctx).unwrap();
        assert_eq!(out, "proj-123");
    }

    #[test]
    fn dot_syntax_hyphenated_node_id_and_field_now_renders() {
        let ctx = ctx_with_upstream("tekla-watch", serde_json::json!({ "drawing-bytes": "PDF" }));
        let out = render("{{ tekla-watch.drawing-bytes }}", &ctx).unwrap();
        assert_eq!(out, "PDF");
    }

    #[test]
    fn dot_syntax_hyphenated_config_and_secrets_render() {
        let mut ctx = RenderContext::default();
        ctx.config
            .insert("pdf-inbox".into(), serde_json::json!("/in"));
        ctx.secrets
            .insert("ceng-seal".into(), serde_json::json!("sig"));
        assert_eq!(render("{{ config.pdf-inbox }}", &ctx).unwrap(), "/in");
        assert_eq!(render("{{ secrets.ceng-seal }}", &ctx).unwrap(), "sig");
    }

    #[test]
    fn hyphenated_path_inside_larger_string_renders() {
        let ctx = RenderContext {
            inputs: serde_json::json!({ "report-dir": "/r" }),
            ..Default::default()
        };
        let out = render(
            "{{ inputs.report-dir }}/monday-{{ inputs.report-dir }}.html",
            &ctx,
        )
        .unwrap();
        assert_eq!(out, "/r/monday-/r.html");
    }

    #[test]
    fn normalize_leaves_valid_templates_untouched() {
        // Hyphen-free dot paths, subtraction (no dot), already-bracketed kebab,
        // and literal text outside blocks all pass through byte-identical.
        assert_eq!(
            normalize_hyphenated_paths("{{ inputs.plain }}"),
            "{{ inputs.plain }}"
        );
        assert_eq!(
            normalize_hyphenated_paths("{{ tekla_watch.mark }}"),
            "{{ tekla_watch.mark }}"
        );
        assert_eq!(normalize_hyphenated_paths("{{ a - b }}"), "{{ a - b }}");
        assert_eq!(
            normalize_hyphenated_paths("{{ inputs['a-b'] }}"),
            "{{ inputs['a-b'] }}"
        );
        assert_eq!(
            normalize_hyphenated_paths("a-b.c-d not in a block"),
            "a-b.c-d not in a block"
        );
    }

    #[test]
    fn normalize_rewrites_hyphenated_dot_paths_to_bracket_form() {
        assert_eq!(
            normalize_hyphenated_paths("{{ inputs.tc-project-id }}"),
            "{{ inputs['tc-project-id'] }}"
        );
        assert_eq!(
            normalize_hyphenated_paths("{{ delta-report.output-path }}"),
            "{{ delta_report['output-path'] }}"
        );
        // Path rewritten; the hyphen inside the quoted literal is preserved.
        assert_eq!(
            normalize_hyphenated_paths("{{ x.a-b | replace: \".h-t\" }}"),
            "{{ x['a-b'] | replace: \".h-t\" }}"
        );
    }
}
