//! Declarative-UI substrate (#215) — schema validation, block catalog, and a
//! deterministic fallback renderer for prompt-composed UI descriptors.
//!
//! The division of labor mirrors `.flo` compose→compile: a host AI *composes* a
//! descriptor (guided by `20-agents/_core/ui/skills/descriptor-authoring.md`),
//! and this module deterministically validates / renders it — no LLM anywhere.
//! Hosts own chrome: they may render descriptors with their own design systems;
//! `ui_render` is the *fallback* renderer producing self-contained HTML (inline
//! CSS, no JS, no external resources), reusing the html-report conventions
//! (`super::html_report`) so the visual language stays consistent.
//!
//! Forward compatibility is part of the contract: an unknown block type is a
//! `validate` WARNING and a `render` placeholder — never an error — so an old
//! renderer degrades gracefully on a descriptor written for a newer schema.

use serde_json::Value;

use crate::error::AwareError;
use crate::json::type_name;
use crate::render::html_report::{TableOptions, esc, render_fragment};

/// The descriptor schema version this build understands.
const SCHEMA_VERSION: &str = "1";

// ── Block specs: the single source of truth ──────────────────────────────────
// Both the validator and the machine-readable catalog are built from this one
// table, so they can never drift apart.

/// One field of a block type's data contract.
struct FieldSpec {
    name: &'static str,
    /// Type tag: `string` | `string|number` | `boolean` | `array<string>` | `object`.
    ty: &'static str,
    required: bool,
    description: &'static str,
}

/// One composable block type.
struct BlockSpec {
    name: &'static str,
    description: &'static str,
    fields: &'static [FieldSpec],
}

/// Descriptor schema v1 block types. Order here is the catalog order.
const BLOCK_SPECS: &[BlockSpec] = &[
    BlockSpec {
        name: "stat",
        description: "A single key metric: label + value, with an optional hint line.",
        fields: &[
            FieldSpec {
                name: "label",
                ty: "string",
                required: true,
                description: "Short metric name shown above the value.",
            },
            FieldSpec {
                name: "value",
                ty: "string|number",
                required: true,
                description: "The metric value (rendered prominently).",
            },
            FieldSpec {
                name: "hint",
                ty: "string",
                required: false,
                description: "Optional fine-print context under the value.",
            },
        ],
    },
    BlockSpec {
        name: "table",
        description: "A data table bound to a named payload from the render-time `data` \
                      map. Columns/sort reuse html-report semantics (#210).",
        fields: &[
            FieldSpec {
                name: "source",
                ty: "string",
                required: true,
                description: "Key into the `data` input map naming the payload to render. \
                              A missing key renders a placeholder, not an error.",
            },
            FieldSpec {
                name: "columns",
                ty: "array<string>",
                required: false,
                description: "Columns to show, in this exact order (array-of-objects \
                              payloads only). Omit to show every key.",
            },
            FieldSpec {
                name: "sort",
                ty: "string",
                required: false,
                description: "Key to sort rows by. Numbers sort numerically; other values \
                              by string form; a missing key sorts last.",
            },
            FieldSpec {
                name: "sort-desc",
                ty: "boolean",
                required: false,
                description: "Sort descending (e.g. most-recent-first). Needs `sort`.",
            },
        ],
    },
    BlockSpec {
        name: "text",
        description: "Escaped prose with paragraph breaks. NO raw HTML pass-through, ever.",
        fields: &[FieldSpec {
            name: "content",
            ty: "string",
            required: true,
            description: "Plain text; blank lines separate paragraphs. HTML is escaped.",
        }],
    },
    BlockSpec {
        name: "report",
        description: "Embeds html-report's generic auto-render of a named data payload \
                      (array → table, object → field/value table, scalar → value card).",
        fields: &[
            FieldSpec {
                name: "source",
                ty: "string",
                required: true,
                description: "Key into the `data` input map naming the payload to render. \
                              A missing key renders a placeholder, not an error.",
            },
            FieldSpec {
                name: "title",
                ty: "string",
                required: false,
                description: "Optional sub-heading above the embedded report.",
            },
        ],
    },
    BlockSpec {
        name: "action",
        description: "A declared intent the HOST wires up: the fallback renderer emits an \
                      INERT disabled-styled button carrying `data-action-id` (and \
                      `data-inputs` when given); the agent never executes anything.",
        fields: &[
            FieldSpec {
                name: "label",
                ty: "string",
                required: true,
                description: "Button label.",
            },
            FieldSpec {
                name: "action-id",
                ty: "string",
                required: true,
                description: "Host-interpreted identifier of the intent to trigger.",
            },
            FieldSpec {
                name: "inputs",
                ty: "object",
                required: false,
                description: "Optional inputs the host passes when triggering the action.",
            },
        ],
    },
];

/// Look up a block spec by type name.
fn block_spec(name: &str) -> Option<&'static BlockSpec> {
    BLOCK_SPECS.iter().find(|b| b.name == name)
}

/// `true` when `v` satisfies the field type tag.
fn type_matches(ty: &str, v: &Value) -> bool {
    match ty {
        "string" => v.is_string(),
        "string|number" => v.is_string() || v.is_number(),
        "boolean" => v.is_boolean(),
        "array<string>" => v
            .as_array()
            .is_some_and(|a| a.iter().all(|x| x.is_string())),
        "object" => v.is_object(),
        _ => false,
    }
}

// ── validate ─────────────────────────────────────────────────────────────────

/// `ui.validate` — deterministic schema check of a descriptor. Returns
/// `{ valid, errors[], warnings[], "schema-version" }`; error/warning strings
/// carry JSON-path-style locations (e.g. `panels[0].blocks[2].type: …`).
///
/// Args-level problems (missing/`non-object` `descriptor`, unsupported
/// `schema-version`) are `AwareError::Validation`; problems *inside* the
/// descriptor are reported in `errors`/`warnings` with `valid: false/true`.
pub fn ui_validate(args: &Value) -> Result<Value, AwareError> {
    let descriptor = required_descriptor(args, "ui validate")?;
    if let Some(sv) = args.get("schema-version") {
        let sv = sv.as_str().ok_or_else(|| {
            AwareError::Validation("ui validate: `schema-version` must be a string".into())
        })?;
        if sv != SCHEMA_VERSION {
            return Err(AwareError::Validation(format!(
                "ui validate: unsupported schema-version {sv:?} (this build supports \
                 {SCHEMA_VERSION:?})"
            )));
        }
    }

    let (errors, warnings) = validate_descriptor(descriptor);
    Ok(serde_json::json!({
        "valid": errors.is_empty(),
        "errors": errors,
        "warnings": warnings,
        "schema-version": SCHEMA_VERSION,
    }))
}

/// Extract the required `descriptor` object input shared by validate/render.
fn required_descriptor<'a>(args: &'a Value, cmd: &str) -> Result<&'a Value, AwareError> {
    match args.get("descriptor") {
        Some(d) if d.is_object() => Ok(d),
        Some(d) => Err(AwareError::Validation(format!(
            "{cmd}: `descriptor` must be an object (got {})",
            type_name(d)
        ))),
        None => Err(AwareError::Validation(format!(
            "{cmd}: missing required input `descriptor` (the descriptor object to {})",
            if cmd.ends_with("render") {
                "render"
            } else {
                "validate"
            }
        ))),
    }
}

/// Walk a descriptor and collect `(errors, warnings)`. Pure — shared by
/// `ui_validate` (reported as data) and `ui_render` (errors refuse the render).
fn validate_descriptor(descriptor: &Value) -> (Vec<String>, Vec<String>) {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    let Some(obj) = descriptor.as_object() else {
        errors.push("descriptor: must be an object".into());
        return (errors, warnings);
    };

    // version: required, must be the integer 1.
    match obj.get("version") {
        None => errors.push("version: missing required field (must be 1)".into()),
        Some(v) if v.as_i64() == Some(1) => {}
        Some(v) => errors.push(format!("version: must be 1 (got {})", compact(v))),
    }

    // panels: required array.
    match obj.get("panels") {
        None => errors.push("panels: missing required field (an array of panels)".into()),
        Some(Value::Array(panels)) => {
            let mut seen_ids: Vec<String> = Vec::new();
            for (i, panel) in panels.iter().enumerate() {
                validate_panel(panel, i, &mut seen_ids, &mut errors, &mut warnings);
            }
        }
        Some(v) => errors.push(format!("panels: must be an array (got {})", type_name(v))),
    }

    // Unknown top-level fields → warning (forward compatibility).
    for key in obj.keys() {
        if key != "version" && key != "panels" {
            warnings.push(format!("{key}: unknown descriptor field"));
        }
    }

    (errors, warnings)
}

fn validate_panel(
    panel: &Value,
    i: usize,
    seen_ids: &mut Vec<String>,
    errors: &mut Vec<String>,
    warnings: &mut Vec<String>,
) {
    let at = format!("panels[{i}]");
    let Some(p) = panel.as_object() else {
        errors.push(format!(
            "{at}: must be an object (got {})",
            type_name(panel)
        ));
        return;
    };

    // id: required, [a-z0-9-]+, unique.
    match p.get("id") {
        None => errors.push(format!("{at}.id: missing required field")),
        Some(Value::String(id)) => {
            let ok = !id.is_empty()
                && id
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-');
            if !ok {
                errors.push(format!("{at}.id: must match [a-z0-9-]+ (got {id:?})"));
            } else if seen_ids.iter().any(|s| s == id) {
                errors.push(format!("{at}.id: duplicate panel id {id:?}"));
            } else {
                seen_ids.push(id.clone());
            }
        }
        Some(v) => errors.push(format!("{at}.id: must be a string (got {})", type_name(v))),
    }

    // slot: required, host-interpreted, non-empty string.
    match p.get("slot") {
        None => errors.push(format!(
            "{at}.slot: missing required field (host-interpreted, e.g. \"dashboard\")"
        )),
        Some(Value::String(s)) if !s.trim().is_empty() => {}
        Some(Value::String(_)) => errors.push(format!("{at}.slot: must not be empty")),
        Some(v) => errors.push(format!(
            "{at}.slot: must be a string (got {})",
            type_name(v)
        )),
    }

    // title: required string.
    match p.get("title") {
        None => errors.push(format!("{at}.title: missing required field")),
        Some(Value::String(_)) => {}
        Some(v) => errors.push(format!(
            "{at}.title: must be a string (got {})",
            type_name(v)
        )),
    }

    // blocks: required array of block objects.
    match p.get("blocks") {
        None => errors.push(format!(
            "{at}.blocks: missing required field (an array of blocks)"
        )),
        Some(Value::Array(blocks)) => {
            for (j, block) in blocks.iter().enumerate() {
                validate_block(block, &format!("{at}.blocks[{j}]"), errors, warnings);
            }
        }
        Some(v) => errors.push(format!(
            "{at}.blocks: must be an array (got {})",
            type_name(v)
        )),
    }

    // Unknown panel fields → warning.
    for key in p.keys() {
        if !matches!(key.as_str(), "id" | "slot" | "title" | "blocks") {
            warnings.push(format!("{at}.{key}: unknown panel field"));
        }
    }
}

fn validate_block(block: &Value, at: &str, errors: &mut Vec<String>, warnings: &mut Vec<String>) {
    let Some(b) = block.as_object() else {
        errors.push(format!(
            "{at}: must be an object (got {})",
            type_name(block)
        ));
        return;
    };
    let ty = match b.get("type") {
        None => {
            errors.push(format!("{at}.type: missing required field"));
            return;
        }
        Some(Value::String(t)) => t,
        Some(v) => {
            errors.push(format!(
                "{at}.type: must be a string (got {})",
                type_name(v)
            ));
            return;
        }
    };
    let Some(spec) = block_spec(ty) else {
        // Forward compatibility: an unknown block type validates with a WARNING
        // (render shows a placeholder) so old validators pass newer descriptors.
        warnings.push(format!(
            "{at}.type: unknown block type {ty:?} (forward-compatible: renders as a placeholder)"
        ));
        return;
    };

    // Known type: validate declared fields strictly.
    for f in spec.fields {
        match b.get(f.name) {
            None if f.required => errors.push(format!(
                "{at}.{}: missing required field for block type {:?}",
                f.name, spec.name
            )),
            None => {}
            Some(v) if type_matches(f.ty, v) => {}
            Some(v) => errors.push(format!(
                "{at}.{}: must be {} (got {})",
                f.name,
                f.ty,
                type_name(v)
            )),
        }
    }
    // Unknown EXTRA fields on a known block → warning (forward compatibility).
    for key in b.keys() {
        if key != "type" && !spec.fields.iter().any(|f| f.name == key) {
            warnings.push(format!(
                "{at}.{key}: unknown field for block type {:?}",
                spec.name
            ));
        }
    }
}

/// Compact JSON form for "got …" messages (so `\"2\"` vs `2` stay distinguishable).
fn compact(v: &Value) -> String {
    v.to_string()
}

// ── catalog ──────────────────────────────────────────────────────────────────

/// `ui.catalog` — the machine-readable list of composable block types with
/// their typed data contracts, built from the SAME table the validator uses,
/// so a host AI on any front-end can discover what it may compose.
pub fn ui_catalog(_args: &Value) -> Result<Value, AwareError> {
    let blocks: Vec<Value> = BLOCK_SPECS
        .iter()
        .map(|b| {
            serde_json::json!({
                "name": b.name,
                "description": b.description,
                "fields": b.fields.iter().map(|f| serde_json::json!({
                    "name": f.name,
                    "type": f.ty,
                    "required": f.required,
                    "description": f.description,
                })).collect::<Vec<_>>(),
            })
        })
        .collect();

    Ok(serde_json::json!({
        "schema-version": SCHEMA_VERSION,
        "slots": {
            "note": "Slot names are host-interpreted strings (e.g. \"dashboard\", \
                     \"sidebar\"): the descriptor declares WHERE a panel wants to live, \
                     the host decides what that means in its own chrome. The fallback \
                     renderer simply labels each panel with its slot."
        },
        "blocks": blocks,
    }))
}

// ── render ───────────────────────────────────────────────────────────────────

/// Extra CSS for descriptor rendering, appended to html-report's base style so
/// the fallback renderer keeps the same visual language. Deterministic, inline,
/// no external resources, no JS.
const UI_STYLE: &str = r#"
  .panel { margin-bottom: 2.5rem; }
  .panel-header {
    display: flex; align-items: baseline; gap: 0.75rem;
    border-bottom: 1px solid #334155; padding-bottom: 0.5rem; margin-bottom: 1rem;
  }
  .panel-header h2 { font-size: 1.25rem; font-weight: 700; color: #f8fafc; }
  .slot-badge {
    font-size: 0.7rem; text-transform: uppercase; letter-spacing: 0.05em;
    color: #94a3b8; background: #1e293b; border: 1px solid #334155;
    border-radius: 9999px; padding: 0.125rem 0.625rem;
  }
  .card .hint { font-size: 0.75rem; color: #64748b; margin-top: 0.25rem; }
  .text-block { margin-bottom: 2rem; max-width: 72ch; }
  .text-block p { margin-bottom: 0.75rem; }
  .block-title { font-size: 1rem; font-weight: 600; color: #f8fafc; margin-bottom: 0.75rem; }
  .action-row { margin-bottom: 2rem; }
  .action {
    display: inline-block; background: #1e293b; color: #94a3b8;
    border: 1px solid #334155; border-radius: 6px; padding: 0.5rem 1rem;
    font-size: 0.875rem; font-weight: 600; cursor: not-allowed; opacity: 0.7;
  }
  .placeholder {
    border: 1px dashed #475569; border-radius: 8px; padding: 1rem;
    margin-bottom: 2rem; color: #94a3b8; font-style: italic; background: #131c2e;
  }
"#;

/// `ui.render` — the deterministic FALLBACK renderer: descriptor (+ optional
/// named `data` payloads) → one self-contained HTML document. Hosts with their
/// own design system render descriptors themselves; this exists for hosts
/// without a renderer and for `.flo` nodes emitting a composed dashboard.
///
/// - A structurally invalid descriptor is refused (`AwareError::Validation`).
/// - An unknown block type renders a visible placeholder — never an error.
/// - A `table`/`report` `source` missing from `data` renders a placeholder.
/// - `output-path` is written on a real run only; `dry_run` skips the write
///   exactly like `html-report.render`.
pub fn ui_render(args: &Value, dry_run: bool) -> Result<Value, AwareError> {
    let descriptor = required_descriptor(args, "ui render")?;
    let (errors, _warnings) = validate_descriptor(descriptor);
    if !errors.is_empty() {
        return Err(AwareError::Validation(format!(
            "ui render: descriptor failed validation — {} (run `ui.validate` for the full list)",
            errors.join("; ")
        )));
    }
    let data = match args.get("data") {
        None | Some(Value::Null) => None,
        Some(Value::Object(m)) => Some(m),
        Some(v) => {
            return Err(AwareError::Validation(format!(
                "ui render: `data` must be an object mapping payload names to values (got {})",
                type_name(v)
            )));
        }
    };

    let html = render_descriptor_html(descriptor, data);

    let mut out = serde_json::Map::new();
    out.insert("html".into(), Value::String(html.clone()));
    out.insert("bytes".into(), Value::from(html.len() as u64));

    if let Some(path) = args
        .get("output-path")
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        // Real run only: a preview (--dry-run / --simulate) returns the HTML and
        // the would-be path but never touches disk (same contract as html-report).
        if !dry_run {
            if let Some(parent) = std::path::Path::new(path).parent()
                && !parent.as_os_str().is_empty()
            {
                std::fs::create_dir_all(parent).map_err(|e| {
                    AwareError::Internal(format!("ui render: create {}: {e}", parent.display()))
                })?;
            }
            std::fs::write(path, html.as_bytes())
                .map_err(|e| AwareError::Internal(format!("ui render: write {path}: {e}")))?;
        }
        out.insert("output-path".into(), Value::String(path.to_string()));
    }

    Ok(Value::Object(out))
}

/// Render a VALIDATED descriptor into a complete HTML document. Deterministic:
/// no clock, no environment — identical inputs yield identical bytes.
fn render_descriptor_html(
    descriptor: &Value,
    data: Option<&serde_json::Map<String, Value>>,
) -> String {
    let panels = descriptor
        .get("panels")
        .and_then(|p| p.as_array())
        .cloned()
        .unwrap_or_default();

    // Document title: a single panel lends its title; otherwise a neutral one.
    let doc_title = if panels.len() == 1 {
        panels[0]
            .get("title")
            .and_then(|t| t.as_str())
            .unwrap_or("AWARE UI")
            .to_string()
    } else {
        "AWARE UI".to_string()
    };

    let body: String = panels.iter().map(|p| render_panel(p, data)).collect();
    let body = if body.is_empty() {
        "<p class=\"empty\">No panels.</p>".to_string()
    } else {
        body
    };

    format!(
        "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n<meta charset=\"UTF-8\">\n\
<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n\
<title>{title}</title>\n<style>{style}{ui_style}</style>\n</head>\n<body>\n\
<div class=\"container\">\n\
{body}\n\
<div class=\"report-footer\">Generated by AWARE</div>\n\
</div>\n</body>\n</html>\n",
        title = esc(&doc_title),
        style = crate::render::html_report::STYLE,
        ui_style = UI_STYLE,
    )
}

fn render_panel(panel: &Value, data: Option<&serde_json::Map<String, Value>>) -> String {
    let id = panel.get("id").and_then(|v| v.as_str()).unwrap_or("");
    let slot = panel.get("slot").and_then(|v| v.as_str()).unwrap_or("");
    let title = panel.get("title").and_then(|v| v.as_str()).unwrap_or("");
    let blocks = panel
        .get("blocks")
        .and_then(|b| b.as_array())
        .cloned()
        .unwrap_or_default();

    // Consecutive `stat` blocks share one summary-cards grid so a row of metrics
    // lays out like html-report's card row instead of stacking full-width.
    let mut out = String::new();
    let mut stat_run: Vec<String> = Vec::new();
    let flush_stats = |run: &mut Vec<String>, out: &mut String| {
        if !run.is_empty() {
            out.push_str(&format!(
                "<div class=\"summary-cards\">{}</div>\n",
                run.join("")
            ));
            run.clear();
        }
    };
    for block in &blocks {
        let ty = block.get("type").and_then(|v| v.as_str()).unwrap_or("");
        if ty == "stat" {
            stat_run.push(render_stat(block));
            continue;
        }
        flush_stats(&mut stat_run, &mut out);
        out.push_str(&render_block(ty, block, data));
        out.push('\n');
    }
    flush_stats(&mut stat_run, &mut out);

    format!(
        "<div class=\"panel\" data-panel-id=\"{id}\" data-slot=\"{slot}\">\n\
<div class=\"panel-header\"><h2>{title}</h2><span class=\"slot-badge\">{slot}</span></div>\n\
{out}</div>\n",
        id = esc(id),
        slot = esc(slot),
        title = esc(title),
    )
}

fn render_block(ty: &str, block: &Value, data: Option<&serde_json::Map<String, Value>>) -> String {
    match ty {
        "table" => render_table(block, data),
        "text" => render_text(block),
        "report" => render_report_block(block, data),
        "action" => render_action(block),
        // Validation guarantees `ty` is a non-empty string; anything not matched
        // above is an unknown type → a visible placeholder, never an error.
        other => format!(
            "<div class=\"placeholder\">unknown block type &#39;{}&#39; — needs a newer \
             renderer</div>",
            esc(other)
        ),
    }
}

/// One `stat` card (the caller wraps runs of these in a `summary-cards` grid).
fn render_stat(block: &Value) -> String {
    let label = block.get("label").and_then(|v| v.as_str()).unwrap_or("");
    let value = match block.get("value") {
        Some(Value::String(s)) => s.clone(),
        Some(Value::Number(n)) => n.to_string(),
        _ => String::new(),
    };
    let hint = block
        .get("hint")
        .and_then(|v| v.as_str())
        .map(|h| format!("<div class=\"hint\">{}</div>", esc(h)))
        .unwrap_or_default();
    format!(
        "<div class=\"card\"><div class=\"label\">{}</div>\
<div class=\"value accent\">{}</div>{hint}</div>",
        esc(label),
        esc(&value)
    )
}

/// Resolve a block's `source` against the render-time data map. Missing data
/// renders a placeholder, not an error (the descriptor may outlive any one run).
fn resolve_source<'a>(
    block: &Value,
    data: Option<&'a serde_json::Map<String, Value>>,
) -> Result<(&'a Value, String), String> {
    let source = block
        .get("source")
        .and_then(|v| v.as_str())
        .unwrap_or_default();
    match data.and_then(|m| m.get(source)) {
        Some(v) => Ok((v, source.to_string())),
        None => Err(format!(
            "<div class=\"placeholder\">no data for &#39;{}&#39;</div>",
            esc(source)
        )),
    }
}

fn render_table(block: &Value, data: Option<&serde_json::Map<String, Value>>) -> String {
    let (payload, _) = match resolve_source(block, data) {
        Ok(p) => p,
        Err(placeholder) => return placeholder,
    };
    // Reuse html-report's table semantics (#210): columns selection + row sort.
    let opts = TableOptions {
        columns: block.get("columns").and_then(|v| v.as_array()).map(|a| {
            a.iter()
                .filter_map(|x| x.as_str().map(str::to_string))
                .collect()
        }),
        sort_by: block
            .get("sort")
            .and_then(|v| v.as_str())
            .map(str::to_string),
        sort_desc: block
            .get("sort-desc")
            .and_then(|v| v.as_bool())
            .unwrap_or(false),
    };
    render_fragment(payload, &opts)
}

fn render_text(block: &Value) -> String {
    let content = block.get("content").and_then(|v| v.as_str()).unwrap_or("");
    // Escaped text with paragraph breaks: blank lines split paragraphs, single
    // newlines become <br>. No raw HTML pass-through, ever.
    let paragraphs: String = content
        .split("\n\n")
        .filter(|p| !p.trim().is_empty())
        .map(|p| format!("<p>{}</p>", esc(p.trim()).replace('\n', "<br>")))
        .collect();
    format!("<div class=\"text-block\">{paragraphs}</div>")
}

fn render_report_block(block: &Value, data: Option<&serde_json::Map<String, Value>>) -> String {
    let (payload, _) = match resolve_source(block, data) {
        Ok(p) => p,
        Err(placeholder) => return placeholder,
    };
    let title = block
        .get("title")
        .and_then(|v| v.as_str())
        .map(|t| format!("<div class=\"block-title\">{}</div>", esc(t)))
        .unwrap_or_default();
    // html-report's generic auto-render of the payload (its legacy default shape).
    format!(
        "{title}{}",
        render_fragment(payload, &TableOptions::default())
    )
}

fn render_action(block: &Value) -> String {
    let label = block.get("label").and_then(|v| v.as_str()).unwrap_or("");
    let action_id = block
        .get("action-id")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let inputs_attr = block
        .get("inputs")
        .filter(|v| v.is_object())
        .map(|v| format!(" data-inputs=\"{}\"", esc(&v.to_string())))
        .unwrap_or_default();
    // INERT by contract: a disabled-styled button carrying the declared intent;
    // the HOST decides what (if anything) `action-id` triggers.
    format!(
        "<div class=\"action-row\"><button class=\"action\" disabled \
data-action-id=\"{}\"{inputs_attr}>{}</button></div>",
        esc(action_id),
        esc(label)
    )
}

// ── tests ────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// Join a JSON string-array result field into one searchable raw string
    /// (avoids `Value::to_string()`'s JSON escaping of embedded quotes).
    fn joined(v: &Value) -> String {
        v.as_array()
            .unwrap()
            .iter()
            .map(|e| e.as_str().unwrap())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// A descriptor exercising every v1 block type.
    fn full_descriptor() -> Value {
        json!({
            "version": 1,
            "panels": [{
                "id": "overview",
                "slot": "dashboard",
                "title": "Model Overview",
                "blocks": [
                    { "type": "stat", "label": "Parts", "value": 128, "hint": "phase 2" },
                    { "type": "stat", "label": "Status", "value": "OK" },
                    { "type": "text", "content": "First paragraph.\n\nSecond paragraph." },
                    { "type": "table", "source": "parts", "columns": ["name", "weight"],
                      "sort": "weight", "sort-desc": true },
                    { "type": "report", "source": "summary", "title": "Run summary" },
                    { "type": "action", "label": "Re-run", "action-id": "rerun",
                      "inputs": { "phase": 2 } },
                ],
            }],
        })
    }

    // ── validate ────────────────────────────────────────────────────────────

    #[test]
    fn valid_descriptor_passes() {
        let out = ui_validate(&json!({ "descriptor": full_descriptor() })).unwrap();
        assert_eq!(out["valid"], json!(true), "errors: {}", out["errors"]);
        assert_eq!(out["errors"], json!([]));
        assert_eq!(out["warnings"], json!([]));
        assert_eq!(out["schema-version"], json!("1"));
    }

    #[test]
    fn missing_required_fields_error_with_path() {
        let d = json!({
            "version": 1,
            "panels": [{
                "slot": "dashboard",
                "blocks": [{ "type": "stat", "value": 1 }],
            }],
        });
        let out = ui_validate(&json!({ "descriptor": d })).unwrap();
        assert_eq!(out["valid"], json!(false));
        let errors = out["errors"].as_array().unwrap();
        let has = |needle: &str| errors.iter().any(|e| e.as_str().unwrap().contains(needle));
        assert!(has("panels[0].id: missing required field"), "{errors:?}");
        assert!(has("panels[0].title: missing required field"), "{errors:?}");
        assert!(
            has("panels[0].blocks[0].label: missing required field for block type \"stat\""),
            "{errors:?}"
        );
    }

    #[test]
    fn missing_version_and_panels_error() {
        let out = ui_validate(&json!({ "descriptor": {} })).unwrap();
        assert_eq!(out["valid"], json!(false));
        let errs = out["errors"].to_string();
        assert!(errs.contains("version: missing required field"), "{errs}");
        assert!(errs.contains("panels: missing required field"), "{errs}");
    }

    #[test]
    fn unknown_block_type_is_warning_not_error() {
        let d = json!({
            "version": 1,
            "panels": [{
                "id": "p", "slot": "s", "title": "T",
                "blocks": [{ "type": "gauge", "value": 42 }],
            }],
        });
        let out = ui_validate(&json!({ "descriptor": d })).unwrap();
        assert_eq!(out["valid"], json!(true), "unknown type must stay valid");
        assert_eq!(out["errors"], json!([]));
        let warnings = joined(&out["warnings"]);
        assert!(
            warnings.contains("panels[0].blocks[0].type: unknown block type \"gauge\""),
            "{warnings}"
        );
    }

    #[test]
    fn unknown_extra_field_on_known_block_is_warning() {
        let d = json!({
            "version": 1,
            "panels": [{
                "id": "p", "slot": "s", "title": "T",
                "blocks": [{ "type": "stat", "label": "L", "value": 1, "color": "red" }],
            }],
        });
        let out = ui_validate(&json!({ "descriptor": d })).unwrap();
        assert_eq!(out["valid"], json!(true));
        let warnings = joined(&out["warnings"]);
        assert!(
            warnings.contains("panels[0].blocks[0].color: unknown field for block type \"stat\""),
            "{warnings}"
        );
    }

    #[test]
    fn duplicate_panel_id_and_bad_id_error() {
        let d = json!({
            "version": 1,
            "panels": [
                { "id": "a", "slot": "s", "title": "A", "blocks": [] },
                { "id": "a", "slot": "s", "title": "B", "blocks": [] },
                { "id": "Bad ID", "slot": "s", "title": "C", "blocks": [] },
            ],
        });
        let out = ui_validate(&json!({ "descriptor": d })).unwrap();
        let errs = joined(&out["errors"]);
        assert!(
            errs.contains("panels[1].id: duplicate panel id \"a\""),
            "{errs}"
        );
        assert!(
            errs.contains("panels[2].id: must match [a-z0-9-]+"),
            "{errs}"
        );
    }

    #[test]
    fn wrong_field_types_error_with_expected_type() {
        let d = json!({
            "version": 1,
            "panels": [{
                "id": "p", "slot": "s", "title": "T",
                "blocks": [
                    { "type": "stat", "label": 7, "value": true },
                    { "type": "table", "source": "x", "columns": "name" },
                ],
            }],
        });
        let out = ui_validate(&json!({ "descriptor": d })).unwrap();
        let errs = out["errors"].to_string();
        assert!(
            errs.contains("blocks[0].label: must be string (got number)"),
            "{errs}"
        );
        assert!(
            errs.contains("blocks[0].value: must be string|number (got boolean)"),
            "{errs}"
        );
        assert!(
            errs.contains("blocks[1].columns: must be array<string> (got string)"),
            "{errs}"
        );
    }

    #[test]
    fn missing_descriptor_is_an_args_error() {
        let err = ui_validate(&json!({})).unwrap_err();
        assert!(matches!(err, AwareError::Validation(_)), "got: {err:?}");
        assert!(format!("{err}").contains("descriptor"));
    }

    #[test]
    fn unsupported_schema_version_is_an_args_error() {
        let err = ui_validate(&json!({
            "descriptor": full_descriptor(),
            "schema-version": "2",
        }))
        .unwrap_err();
        assert!(matches!(err, AwareError::Validation(_)), "got: {err:?}");
        assert!(format!("{err}").contains("schema-version"));
    }

    // ── catalog ─────────────────────────────────────────────────────────────

    #[test]
    fn catalog_lists_all_five_block_types_with_contracts() {
        let out = ui_catalog(&json!({})).unwrap();
        assert_eq!(out["schema-version"], json!("1"));
        assert!(
            out["slots"]["note"]
                .as_str()
                .unwrap()
                .contains("host-interpreted")
        );
        let blocks = out["blocks"].as_array().unwrap();
        let names: Vec<&str> = blocks.iter().map(|b| b["name"].as_str().unwrap()).collect();
        assert_eq!(names, vec!["stat", "table", "text", "report", "action"]);
        // Every block carries a typed field contract a host AI can read.
        for b in blocks {
            assert!(b["description"].as_str().is_some_and(|d| !d.is_empty()));
            for f in b["fields"].as_array().unwrap() {
                assert!(f["name"].is_string());
                assert!(f["type"].is_string());
                assert!(f["required"].is_boolean());
            }
        }
        // Spot-check one required field from the table the validator shares.
        let action = &blocks[4];
        let action_id = action["fields"]
            .as_array()
            .unwrap()
            .iter()
            .find(|f| f["name"] == "action-id")
            .expect("action-id field");
        assert_eq!(action_id["required"], json!(true));
    }

    // ── render ──────────────────────────────────────────────────────────────

    fn render_args() -> Value {
        json!({
            "descriptor": full_descriptor(),
            "data": {
                "parts": [
                    { "name": "Beam", "weight": 120, "guid": "x" },
                    { "name": "Column", "weight": 300, "guid": "y" },
                ],
                "summary": { "total": 2, "status": "ok" },
            },
        })
    }

    #[test]
    fn render_produces_self_contained_deterministic_html() {
        let a = ui_render(&render_args(), false).unwrap();
        let b = ui_render(&render_args(), false).unwrap();
        assert_eq!(a["html"], b["html"], "render must be deterministic");
        let html = a["html"].as_str().unwrap();
        assert!(html.starts_with("<!DOCTYPE html>"));
        assert!(html.contains("<style>"), "inline CSS only");
        assert!(!html.contains("<script"), "no JS");
        assert!(
            !html.contains("http://") && !html.contains("https://"),
            "no external resources"
        );
        assert_eq!(a["bytes"].as_u64().unwrap() as usize, html.len());
        // Panel chrome: id/slot/title all present.
        assert!(html.contains("data-panel-id=\"overview\""));
        assert!(html.contains("data-slot=\"dashboard\""));
        assert!(html.contains("<h2>Model Overview</h2>"));
        // Single panel lends the document title.
        assert!(html.contains("<title>Model Overview</title>"));
    }

    #[test]
    fn render_resolves_table_source_with_columns_and_sort() {
        let out = ui_render(&render_args(), false).unwrap();
        let html = out["html"].as_str().unwrap();
        // Selected columns only, in order; dropped column's values are gone.
        assert!(html.contains("<th>name</th><th>weight</th>"));
        assert!(!html.contains("<th>guid</th>"));
        // sort-desc on weight: 300 before 120.
        assert!(html.find("<td>Column</td>").unwrap() < html.find("<td>Beam</td>").unwrap());
        // report block auto-renders the object payload with its sub-title.
        assert!(html.contains("Run summary"));
        assert!(html.contains("<tr><th>total</th><td>2</td></tr>"));
    }

    #[test]
    fn render_stat_text_action_blocks() {
        let out = ui_render(&render_args(), false).unwrap();
        let html = out["html"].as_str().unwrap();
        // Stats: label + value (+ hint), consecutive stats share one card grid.
        assert!(html.contains("<div class=\"label\">Parts</div>"));
        assert!(html.contains("<div class=\"value accent\">128</div>"));
        assert!(html.contains("<div class=\"hint\">phase 2</div>"));
        assert_eq!(
            html.matches("<div class=\"summary-cards\">").count(),
            1,
            "consecutive stats share one card grid"
        );
        // Text: escaped paragraphs.
        assert!(html.contains("<p>First paragraph.</p><p>Second paragraph.</p>"));
        // Action: inert (disabled) with the declared intent + inputs.
        assert!(html.contains("data-action-id=\"rerun\""));
        assert!(html.contains("disabled"));
        assert!(html.contains("data-inputs=\"{&quot;phase&quot;:2}\""));
        assert!(!html.contains("onclick"), "actions are inert");
    }

    #[test]
    fn render_missing_source_is_a_placeholder_not_an_error() {
        let args = json!({
            "descriptor": {
                "version": 1,
                "panels": [{
                    "id": "p", "slot": "s", "title": "T",
                    "blocks": [{ "type": "table", "source": "nope" }],
                }],
            },
        });
        let out = ui_render(&args, false).unwrap();
        let html = out["html"].as_str().unwrap();
        assert!(html.contains("no data for &#39;nope&#39;"), "{html}");
    }

    #[test]
    fn render_unknown_block_is_a_visible_placeholder() {
        let args = json!({
            "descriptor": {
                "version": 1,
                "panels": [{
                    "id": "p", "slot": "s", "title": "T",
                    "blocks": [{ "type": "gauge", "value": 42 }],
                }],
            },
        });
        let out = ui_render(&args, false).unwrap();
        let html = out["html"].as_str().unwrap();
        assert!(
            html.contains("unknown block type &#39;gauge&#39; — needs a newer renderer"),
            "{html}"
        );
    }

    #[test]
    fn render_refuses_an_invalid_descriptor() {
        let err = ui_render(&json!({ "descriptor": { "version": 2 } }), false).unwrap_err();
        assert!(matches!(err, AwareError::Validation(_)), "got: {err:?}");
        let msg = format!("{err}");
        assert!(msg.contains("version: must be 1"), "{msg}");
    }

    #[test]
    fn html_injection_in_every_string_field_is_escaped() {
        let inj = "<script>alert(\"x\")</script>";
        let args = json!({
            "descriptor": {
                "version": 1,
                "panels": [{
                    "id": "p", "slot": "s", "title": inj,
                    "blocks": [
                        { "type": "stat", "label": inj, "value": inj, "hint": inj },
                        { "type": "text", "content": inj },
                        { "type": "action", "label": inj, "action-id": "x" },
                        { "type": "table", "source": inj },
                        { "type": inj },
                    ],
                }],
            },
            "data": { "rows": [{ "v": inj }] },
        });
        let out = ui_render(&args, false).unwrap();
        let html = out["html"].as_str().unwrap();
        assert!(
            !html.contains("<script>"),
            "injection must be escaped: {html}"
        );
        assert!(html.contains("&lt;script&gt;"));
    }

    #[test]
    fn render_dry_run_echoes_path_but_skips_write() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("ui.html");
        let ps = path.to_string_lossy().to_string();
        let mut args = render_args();
        args["output-path"] = json!(ps.clone());
        let out = ui_render(&args, true).unwrap();
        assert_eq!(out["output-path"].as_str().unwrap(), ps);
        assert!(out["bytes"].as_u64().unwrap() > 0);
        assert!(!path.exists(), "a dry run must not write the file");
    }

    #[test]
    fn render_real_run_writes_the_file_and_creates_parents() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("sub").join("ui.html");
        let ps = path.to_string_lossy().to_string();
        let mut args = render_args();
        args["output-path"] = json!(ps);
        let out = ui_render(&args, false).unwrap();
        assert!(path.exists(), "a real run writes the file");
        let written = std::fs::read_to_string(&path).unwrap();
        assert_eq!(written, out["html"].as_str().unwrap());
    }

    // ── the structural arms the validator has but nothing exercised ─────────
    //
    // `validate_descriptor` / `validate_panel` / `validate_block` each carry a
    // "wrong shape entirely" arm beside the "missing field" arm that the tests
    // above cover. Those arms are what stand between a malformed descriptor and
    // an `unwrap()` in the renderer — `ui_render` refuses only on `errors`, so
    // an arm that pushes nothing lets the shape through to `render_panel`,
    // which reads every field with `.and_then(|v| v.as_str()).unwrap_or("")`
    // and would silently emit a blank panel instead of saying what was wrong.

    /// Validate `d` as a descriptor and return its errors joined into one
    /// searchable string. Unescaped, unlike `Value::to_string()`.
    fn errors_for(d: Value) -> String {
        joined(&ui_validate(&json!({ "descriptor": d })).unwrap()["errors"])
    }

    #[test]
    fn panels_of_the_wrong_shape_are_named_rather_than_skipped() {
        // `panels` itself not an array: the whole block is unreadable.
        let errs = errors_for(json!({ "version": 1, "panels": { "id": "p" } }));
        assert!(
            errs.contains("panels: must be an array (got object)"),
            "{errs}"
        );
        // A non-object ENTRY is reported at its own index, and the panels around
        // it still validate — index drift here would point a user at the wrong
        // panel, which is worse than no message.
        let errs = errors_for(json!({
            "version": 1,
            "panels": [
                { "id": "ok", "slot": "s", "title": "T", "blocks": [] },
                "not-a-panel",
                { "id": "ok", "slot": "s", "title": "T", "blocks": [] },
            ],
        }));
        assert!(
            errs.contains("panels[1]: must be an object (got string)"),
            "{errs}"
        );
        // …and the duplicate id at index 2 is still caught, so the bad entry did
        // not abort the walk.
        assert!(
            errs.contains("panels[2].id: duplicate panel id \"ok\""),
            "{errs}"
        );
    }

    #[test]
    fn a_slot_that_is_present_but_says_nothing_is_refused() {
        // `slot` is the one field the fallback renderer does not interpret — the
        // HOST does. An empty or whitespace-only slot therefore renders a
        // perfectly fine-looking badge with nothing in it while the host has no
        // idea where the panel wants to live, so it has to fail at validation.
        let panel = |slot: Value| {
            json!({
                "version": 1,
                "panels": [{ "id": "p", "slot": slot, "title": "T", "blocks": [] }],
            })
        };
        assert!(errors_for(panel(json!(""))).contains("panels[0].slot: must not be empty"));
        assert!(errors_for(panel(json!("   "))).contains("panels[0].slot: must not be empty"));
        let errs = errors_for(panel(json!(3)));
        assert!(
            errs.contains("panels[0].slot: must be a string (got number)"),
            "{errs}"
        );
        // A slot with real content is accepted — otherwise the two asserts above
        // would pass against a rule that rejects every slot.
        assert_eq!(errors_for(panel(json!("dashboard"))), "");
    }

    #[test]
    fn a_block_whose_type_cannot_be_read_is_refused_not_treated_as_unknown() {
        // The forward-compatibility rule (unknown type → warning) must not
        // swallow a block whose type is missing or isn't a string: those are
        // malformed, not "written for a newer renderer", and letting them
        // through as warnings would render them as placeholders forever.
        let blocks = |b: Value| {
            json!({
                "version": 1,
                "panels": [{ "id": "p", "slot": "s", "title": "T", "blocks": [b] }],
            })
        };
        let errs = errors_for(blocks(json!("stat")));
        assert!(
            errs.contains("panels[0].blocks[0]: must be an object (got string)"),
            "{errs}"
        );
        assert!(errors_for(blocks(json!({ "label": "L" }))).contains("blocks[0].type: missing"));
        let errs = errors_for(blocks(json!({ "type": 7 })));
        assert!(
            errs.contains("blocks[0].type: must be a string (got number)"),
            "{errs}"
        );
    }

    #[test]
    fn a_stringified_version_is_refused_and_the_message_shows_the_quotes() {
        // `"1"` is the mistake a descriptor composed by a prompt actually makes,
        // and it is the reason `compact()` exists rather than `type_name()`: a
        // message reading `must be 1 (got string)` leaves the author staring at
        // a value that looks correct. The quotes are the whole point.
        let errs = errors_for(json!({ "version": "1", "panels": [] }));
        assert!(errs.contains("version: must be 1 (got \"1\")"), "{errs}");
        // The integer 1 is what passes; 1.0 is not an i64 and does not.
        assert_eq!(errors_for(json!({ "version": 1, "panels": [] })), "");
        assert!(errors_for(json!({ "version": 1.5, "panels": [] })).contains("version: must be 1"));
    }

    #[test]
    fn fields_nobody_declared_are_warnings_at_every_level() {
        // Forward compatibility is a contract, not a courtesy: a descriptor
        // written for a newer schema must still VALIDATE on an old build. The
        // block-level half is covered above; the descriptor and panel levels
        // each have their own loop and neither was exercised.
        let out = ui_validate(&json!({
            "descriptor": {
                "version": 1,
                "theme": "dark",
                "panels": [{
                    "id": "p", "slot": "s", "title": "T", "blocks": [],
                    "collapsed": true,
                }],
            },
        }))
        .unwrap();
        assert_eq!(out["valid"], json!(true), "errors: {}", out["errors"]);
        let warnings = joined(&out["warnings"]);
        assert!(
            warnings.contains("theme: unknown descriptor field"),
            "{warnings}"
        );
        assert!(
            warnings.contains("panels[0].collapsed: unknown panel field"),
            "{warnings}"
        );
    }

    #[test]
    fn an_array_of_string_field_checks_the_elements_not_just_the_array() {
        // `wrong_field_types_error_with_expected_type` passes a plain string for
        // `columns`, which fails on `as_array()` alone — so the element check
        // could be deleted and that test would still pass. This one only passes
        // if the elements are actually inspected.
        let columns = |v: Value| {
            json!({
                "version": 1,
                "panels": [{
                    "id": "p", "slot": "s", "title": "T",
                    "blocks": [{ "type": "table", "source": "x", "columns": v }],
                }],
            })
        };
        let errs = errors_for(columns(json!(["name", 2])));
        assert!(
            errs.contains("columns: must be array<string> (got array)"),
            "{errs}"
        );
        assert_eq!(errors_for(columns(json!(["name", "weight"]))), "");
        assert_eq!(
            errors_for(columns(json!([]))),
            "",
            "an empty list is a valid selection"
        );
    }

    #[test]
    fn a_non_string_schema_version_is_refused_before_the_descriptor_is_walked() {
        // `schema-version` is compared against a `&str`; a caller sending the
        // number 1 must get a clear args error rather than the version check
        // being skipped and an unsupported descriptor validating anyway.
        let err = ui_validate(&json!({
            "descriptor": full_descriptor(),
            "schema-version": 1,
        }))
        .unwrap_err();
        assert!(
            format!("{err}").contains("`schema-version` must be a string"),
            "got: {err}"
        );
    }

    // ── renderer branches that only fire on shapes the tests above never built ─

    #[test]
    fn a_multi_panel_document_gets_a_neutral_title_and_keeps_every_panel() {
        // One panel lends its title (covered above); more than one must NOT —
        // titling a whole dashboard after whichever panel happens to be first is
        // the failure this guards, and it is invisible in a single-panel test.
        let args = json!({
            "descriptor": {
                "version": 1,
                "panels": [
                    { "id": "a", "slot": "s1", "title": "First", "blocks": [] },
                    { "id": "b", "slot": "s2", "title": "Second", "blocks": [] },
                ],
            },
        });
        let html = ui_render(&args, false).unwrap()["html"]
            .as_str()
            .unwrap()
            .to_string();
        assert!(html.contains("<title>AWARE UI</title>"), "{html}");
        assert!(!html.contains("<title>First</title>"));
        // Both panels are still rendered, in declaration order.
        assert!(
            html.find("data-panel-id=\"a\"").unwrap() < html.find("data-panel-id=\"b\"").unwrap()
        );
        assert!(html.contains("<h2>Second</h2>"));
    }

    #[test]
    fn a_descriptor_with_no_panels_says_so_instead_of_rendering_a_blank_page() {
        // `panels: []` is valid, so it reaches the renderer. An empty body would
        // be a self-contained HTML document that looks like a broken page.
        let html = ui_render(
            &json!({ "descriptor": { "version": 1, "panels": [] } }),
            false,
        )
        .unwrap()["html"]
            .as_str()
            .unwrap()
            .to_string();
        assert!(html.contains("No panels."), "{html}");
        assert!(html.starts_with("<!DOCTYPE html>"));
    }

    #[test]
    fn a_stat_run_is_broken_by_any_other_block() {
        // The existing test proves consecutive stats SHARE a grid, which a
        // renderer that wrapped every stat run and a renderer that wrapped all
        // stats in the panel would both satisfy. Interleaving is what separates
        // them: three stats around a text block must produce TWO grids, in
        // source order, not one grid with the text pushed out of place.
        let args = json!({
            "descriptor": {
                "version": 1,
                "panels": [{
                    "id": "p", "slot": "s", "title": "T",
                    "blocks": [
                        { "type": "stat", "label": "A", "value": 1 },
                        { "type": "text", "content": "divider" },
                        { "type": "stat", "label": "B", "value": 2 },
                        { "type": "stat", "label": "C", "value": 3 },
                    ],
                }],
            },
        });
        let html = ui_render(&args, false).unwrap()["html"]
            .as_str()
            .unwrap()
            .to_string();
        assert_eq!(
            html.matches("<div class=\"summary-cards\">").count(),
            2,
            "a non-stat block must close the open grid: {html}"
        );
        let at = |needle: &str| html.find(needle).expect("block rendered");
        assert!(at("<div class=\"label\">A</div>") < at("<p>divider</p>"));
        assert!(at("<p>divider</p>") < at("<div class=\"label\">B</div>"));
        assert!(at("<div class=\"label\">B</div>") < at("<div class=\"label\">C</div>"));
    }

    #[test]
    fn text_keeps_the_line_breaks_an_author_wrote_without_letting_markup_through() {
        // Blank line → new paragraph; single newline → `<br>`. Both are the
        // author's formatting, and the ONLY formatting the block honors: the
        // `<br>` substitution happens after escaping, so a `<br>` the author
        // typed stays literal text.
        let args = json!({
            "descriptor": {
                "version": 1,
                "panels": [{
                    "id": "p", "slot": "s", "title": "T",
                    "blocks": [{ "type": "text",
                                 "content": "one\ntwo\n\n\n   \n\nthree <br> four" }],
                }],
            },
        });
        let html = ui_render(&args, false).unwrap()["html"]
            .as_str()
            .unwrap()
            .to_string();
        assert!(html.contains("<p>one<br>two</p>"), "{html}");
        // A whitespace-only run between blank lines produces no empty paragraph.
        assert!(!html.contains("<p></p>"), "{html}");
        // The author's literal `<br>` is escaped, not honored as markup.
        assert!(html.contains("<p>three &lt;br&gt; four</p>"), "{html}");
    }

    #[test]
    fn an_action_carries_data_inputs_only_when_the_inputs_are_an_object() {
        // `inputs` is optional and declared `object`. A block that omits it must
        // not emit an empty `data-inputs` attribute — a host reading the
        // attribute would parse `""` as an intent payload.
        let action = |block: Value| {
            ui_render(
                &json!({
                    "descriptor": {
                        "version": 1,
                        "panels": [{ "id": "p", "slot": "s", "title": "T", "blocks": [block] }],
                    },
                }),
                false,
            )
            .unwrap()["html"]
                .as_str()
                .unwrap()
                .to_string()
        };
        let without = action(json!({ "type": "action", "label": "Go", "action-id": "go" }));
        assert!(without.contains("data-action-id=\"go\""), "{without}");
        assert!(!without.contains("data-inputs"), "{without}");
        let with = action(json!({
            "type": "action", "label": "Go", "action-id": "go", "inputs": { "phase": 2 },
        }));
        assert!(
            with.contains("data-inputs=\"{&quot;phase&quot;:2}\""),
            "{with}"
        );
    }

    #[test]
    fn a_render_with_no_data_map_places_a_holder_for_every_bound_block() {
        // A descriptor outlives any one run, so `data` is optional — both when
        // it is absent and when it is an explicit null (the shape a `.flo` node
        // produces from an upstream that returned nothing). Neither may error,
        // and neither may render an empty table that reads as "zero rows".
        for data in [None, Some(Value::Null)] {
            let mut args = json!({ "descriptor": full_descriptor() });
            if let Some(d) = data.clone() {
                args["data"] = d;
            }
            let html = ui_render(&args, false).unwrap()["html"]
                .as_str()
                .unwrap()
                .to_string();
            assert!(
                html.contains("no data for &#39;parts&#39;"),
                "{data:?}: {html}"
            );
            assert!(
                html.contains("no data for &#39;summary&#39;"),
                "{data:?}: {html}"
            );
            // The blocks that need no data still render.
            assert!(
                html.contains("<div class=\"label\">Parts</div>"),
                "{data:?}"
            );
        }
    }

    #[test]
    fn a_data_map_that_is_not_a_map_is_refused_rather_than_ignored() {
        // Silently treating a non-object `data` as "no data" would render a page
        // full of placeholders and report success — the caller's payload would
        // have vanished with nothing said.
        let mut args = render_args();
        args["data"] = json!([{ "name": "Beam" }]);
        let err = ui_render(&args, false).unwrap_err();
        assert!(matches!(err, AwareError::Validation(_)), "got: {err:?}");
        assert!(
            format!("{err}").contains("`data` must be an object"),
            "{err}"
        );
    }

    #[test]
    fn a_blank_output_path_is_ignored_rather_than_written() {
        // `output-path: ""` is what an unresolved `{{ … }}` config leaf collapses
        // to. Writing it would fail on an empty filename; reporting it back would
        // tell the caller a file exists where none does.
        for blank in ["", "   "] {
            let mut args = render_args();
            args["output-path"] = json!(blank);
            let out = ui_render(&args, false).unwrap();
            assert!(
                out.get("output-path").is_none(),
                "blank path {blank:?} must not be reported as written: {out:?}"
            );
            assert!(out["bytes"].as_u64().unwrap() > 0);
        }
        // A path with surrounding whitespace is trimmed and honored, so the
        // guard above is trimming rather than refusing everything.
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("ui.html");
        let mut args = render_args();
        args["output-path"] = json!(format!("  {}  ", path.to_string_lossy()));
        let out = ui_render(&args, false).unwrap();
        assert_eq!(out["output-path"].as_str().unwrap(), path.to_string_lossy());
        assert!(path.exists(), "a trimmed path is still written");
    }
}
