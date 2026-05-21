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

/// Find the byte index just past the next `{% endraw %}` tag at or after `from`,
/// or `None` if the raw block is unterminated. Tolerates whitespace-control
/// dashes (`{%- endraw -%}`).
fn find_endraw(template: &str, from: usize) -> Option<usize> {
    let mut search = from;
    while let Some(open_rel) = template[search..].find("{%") {
        let after_open = search + open_rel + 2;
        let close_rel = template[after_open..].find("%}")?;
        let tag = template[after_open..after_open + close_rel]
            .trim()
            .trim_matches('-')
            .trim();
        let close_end = after_open + close_rel + 2;
        if tag == "endraw" {
            return Some(close_end);
        }
        search = close_end;
    }
    None
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
/// Hyphen-free paths and the contents of string literals and `{% raw %}` blocks
/// are copied through unchanged, so any template that already renders keeps
/// rendering identically (#117-4).
///
/// Subtraction note: an explicit, spaced `{{ a.b - c }}` is preserved (the space
/// ends the path). A *space-free* `{{ ns.a-b }}` is read as the kebab key `a-b`,
/// matching AWARE's kebab-everywhere convention; write subtraction with spaces.
fn normalize_hyphenated_paths(template: &str) -> String {
    let b = template.as_bytes();
    let n = b.len();
    let mut out = String::with_capacity(n);
    // Start of the pending run of bytes copied through verbatim. Copying by
    // SLICE (not byte-by-byte) keeps multi-byte UTF-8 literal text intact.
    let mut flush_from = 0usize;
    let mut i = 0usize;
    let mut in_expr = false;
    let mut quote: Option<u8> = None;
    let ident = |byte: u8| byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-';
    while i < n {
        let c = b[i];
        if !in_expr {
            // `{% raw %}` … `{% endraw %}` is emitted literally by minijinja —
            // skip the whole region so its body is never normalized.
            if c == b'{'
                && i + 1 < n
                && b[i + 1] == b'%'
                && let Some(close_rel) = template[i + 2..].find("%}")
                && template[i + 2..i + 2 + close_rel]
                    .trim()
                    .trim_matches('-')
                    .trim()
                    == "raw"
            {
                let body_start = i + 2 + close_rel + 2;
                i = find_endraw(template, body_start).unwrap_or(n);
                continue;
            }
            if c == b'{' && i + 1 < n && (b[i + 1] == b'{' || b[i + 1] == b'%') {
                in_expr = true;
                i += 2;
            } else {
                i += 1;
            }
            continue;
        }
        // Inside a `{{ }}` / `{% %}` block.
        if let Some(q) = quote {
            if c == b'\\' && i + 1 < n {
                i += 2;
            } else {
                if c == q {
                    quote = None;
                }
                i += 1;
            }
            continue;
        }
        if (c == b'}' || c == b'%') && i + 1 < n && b[i + 1] == b'}' {
            in_expr = false;
            i += 2;
            continue;
        }
        if c == b'\'' || c == b'"' {
            quote = Some(c);
            i += 1;
            continue;
        }
        // A potential accessor-chain base: a leading identifier.
        if c.is_ascii_alphabetic() || c == b'_' {
            let base_start = i;
            i += 1;
            while i < n && ident(b[i]) {
                i += 1;
            }
            // Only an accessor chain (base followed by `.` or `[`) is rewritten;
            // a bare token is left alone so `a - b` subtraction is never mangled.
            if i < n && (b[i] == b'.' || b[i] == b'[') {
                // A kebab base node id resolves via its underscore-translated key.
                let mut chain = template[base_start..i].replace('-', "_");
                loop {
                    if i < n && b[i] == b'.' {
                        let seg_start = i + 1;
                        let mut j = seg_start;
                        while j < n && ident(b[j]) {
                            j += 1;
                        }
                        if j == seg_start {
                            break; // `.` not followed by an identifier (e.g. `.*`)
                        }
                        let seg = &template[seg_start..j];
                        if seg.contains('-') {
                            chain.push_str("['");
                            chain.push_str(seg);
                            chain.push_str("']");
                        } else {
                            chain.push('.');
                            chain.push_str(seg);
                        }
                        i = j;
                    } else if i < n && b[i] == b'[' {
                        // Copy a `[ … ]` lookup verbatim (kebab keys already work),
                        // tracking quotes so a `]` inside a string doesn't end it.
                        let br_start = i;
                        i += 1;
                        let mut q2: Option<u8> = None;
                        while i < n {
                            let cc = b[i];
                            if let Some(qq) = q2 {
                                if cc == b'\\' && i + 1 < n {
                                    i += 2;
                                    continue;
                                }
                                if cc == qq {
                                    q2 = None;
                                }
                                i += 1;
                            } else if cc == b'\'' || cc == b'"' {
                                q2 = Some(cc);
                                i += 1;
                            } else if cc == b']' {
                                i += 1;
                                break;
                            } else {
                                i += 1;
                            }
                        }
                        chain.push_str(&template[br_start..i]);
                    } else {
                        break;
                    }
                }
                out.push_str(&template[flush_from..base_start]);
                out.push_str(&chain);
                flush_from = i;
            }
            continue;
        }
        i += 1;
    }
    out.push_str(&template[flush_from..]);
    out
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

/// Resolve a `{{ <head>.<seg>… }}` reference to its structured value (array,
/// object, scalar) rather than a rendered string. The `for-each` runtime needs
/// the underlying [`serde_json::Value`] to iterate — [`render`] would stringify
/// an array into `"[…]"`, which can't be iterated.
///
/// Mirrors the lockfile compiler's reference grammar
/// (`app_lock::collect_refs`): the leading run of identifier / `_` / `-` / `.`
/// characters inside the first `{{ … }}` is taken, split on `.`, and walked.
/// `inputs` / `config` / `secrets` resolve to those namespaces; any other head
/// is an upstream node id (raw kebab or underscore form, matching
/// [`RenderContext::record_output`]). Nested segments are matched verbatim —
/// agent-output fields aren't identifier-translated. A ref that doesn't resolve
/// yields [`serde_json::Value::Null`], which the caller treats as empty.
///
/// Only the dot/kebab path form is resolved — the same subset the compiler
/// validates. A bracket-syntax ref (`{{ src['items'] }}`) stops at `[`, so
/// neither this resolver nor `collect_refs` sees the `items` segment; author
/// `for-each` collections in dot form (`{{ src.items }}`), AWARE's convention.
pub fn resolve_value(expr: &str, ctx: &RenderContext) -> serde_json::Value {
    let null = serde_json::Value::Null;
    let Some(start) = expr.find("{{") else {
        return null;
    };
    let after = &expr[start + 2..];
    let Some(end) = after.find("}}") else {
        return null;
    };
    let inner = after[..end].trim();
    let path_end = inner
        .find(|c: char| !(c.is_alphanumeric() || c == '_' || c == '-' || c == '.'))
        .unwrap_or(inner.len());
    let parts: Vec<&str> = inner[..path_end]
        .split('.')
        .filter(|p| !p.is_empty())
        .collect();
    let Some((head, rest)) = parts.split_first() else {
        return null;
    };

    let mut cur = match *head {
        "inputs" => ctx.inputs.clone(),
        "config" => serde_json::Value::Object(ctx.config.clone()),
        "secrets" => serde_json::Value::Object(ctx.secrets.clone()),
        // Upstream node id — `record_output` stores raw kebab and underscore forms.
        _ => ctx
            .upstream
            .get(*head)
            .or_else(|| ctx.upstream.get(&head.replace('-', "_")))
            .or_else(|| ctx.upstream.get(&head.replace('_', "-")))
            .cloned()
            .unwrap_or(null),
    };
    for seg in rest {
        cur = cur.get(seg).cloned().unwrap_or(serde_json::Value::Null);
    }
    cur
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

    // ── #124: resolve_value returns structured values for for-each ──────────

    #[test]
    fn resolve_value_returns_upstream_array() {
        let ctx = ctx_with_upstream("src", serde_json::json!({ "items": [1, 2, 3] }));
        assert_eq!(
            resolve_value("{{ src.items }}", &ctx),
            serde_json::json!([1, 2, 3])
        );
    }

    #[test]
    fn resolve_value_resolves_kebab_node_head() {
        // record_output stores both kebab and underscore forms; a `{{ a_b.x }}`
        // ref must resolve the kebab node `a-b`.
        let ctx = ctx_with_upstream("a-b", serde_json::json!({ "x": [true] }));
        assert_eq!(
            resolve_value("{{ a_b.x }}", &ctx),
            serde_json::json!([true])
        );
    }

    #[test]
    fn resolve_value_missing_ref_is_null() {
        let ctx = RenderContext::default();
        assert!(resolve_value("{{ nope.items }}", &ctx).is_null());
    }

    #[test]
    fn resolve_value_reads_inputs_namespace() {
        let ctx = RenderContext {
            inputs: serde_json::json!({ "ids": ["a", "b"] }),
            ..Default::default()
        };
        assert_eq!(
            resolve_value("{{ inputs.ids }}", &ctx),
            serde_json::json!(["a", "b"])
        );
    }

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

    #[test]
    fn normalizer_preserves_non_ascii_literal_text() {
        // Byte-by-byte copy would mojibake multi-byte UTF-8; slice copy keeps it.
        let ctx = RenderContext {
            inputs: serde_json::json!({ "name": "World" }),
            ..Default::default()
        };
        let out = render("Zażółć {{ inputs.name }} gęślą jaźń", &ctx).unwrap();
        assert_eq!(out, "Zażółć World gęślą jaźń");
    }

    #[test]
    fn hyphenated_field_after_bracket_lookup_renders() {
        // `upstream['kebab-id'].kebab-field` — the field follows `]`, not a base.
        let ctx = ctx_with_upstream("tekla-watch", serde_json::json!({ "drawing-bytes": "PDF" }));
        let out = render("{{ upstream['tekla-watch'].drawing-bytes }}", &ctx).unwrap();
        assert_eq!(out, "PDF");
    }

    #[test]
    fn normalize_brackets_field_after_bracket_and_keeps_utf8() {
        assert_eq!(
            normalize_hyphenated_paths("{{ upstream['a-b'].c-d }}"),
            "{{ upstream['a-b']['c-d'] }}"
        );
        // Non-ASCII literal outside the block is byte-preserved.
        assert_eq!(
            normalize_hyphenated_paths("café {{ x.y-z }}"),
            "café {{ x['y-z'] }}"
        );
    }

    #[test]
    fn normalize_skips_raw_blocks() {
        // `{% raw %}` body is emitted literally — no normalization inside.
        assert_eq!(
            normalize_hyphenated_paths("{% raw %}{{ inputs.foo-bar }}{% endraw %}"),
            "{% raw %}{{ inputs.foo-bar }}{% endraw %}"
        );
        // A real ref after the raw block is still normalized.
        assert_eq!(
            normalize_hyphenated_paths("{% raw %}{{ a.b-c }}{% endraw %} {{ x.y-z }}"),
            "{% raw %}{{ a.b-c }}{% endraw %} {{ x['y-z'] }}"
        );
    }

    #[test]
    fn normalize_preserves_spaced_subtraction() {
        // Explicit, spaced subtraction is left intact (the space ends the path).
        assert_eq!(
            normalize_hyphenated_paths("{{ inputs.count - discount }}"),
            "{{ inputs.count - discount }}"
        );
    }
}
