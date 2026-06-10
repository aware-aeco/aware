//! Generic, deterministic HTML report renderer (#201).
//!
//! Turns an *arbitrary* JSON value into a self-contained dark-theme HTML document,
//! applying the palette + layout from the html-report styling contract
//! (`20-agents/_core/html-report/skills/styling.md`). It is a pure function
//! (`(title, data) -> String`) — no clock, no environment — so the same input always
//! renders byte-identical output. This is the engine behind the `html-report` built-in
//! agent's `render` command: an array of objects becomes a table (keys as columns), an
//! object becomes a field/value table, scalars render as a single value, and nested
//! structures are shown as compact JSON inside their cell.

use serde_json::Value;

/// Self-contained dark-theme CSS, inlined into every report. Verbatim from the
/// html-report `styling.md` design contract (palette + tables + cards + print rules).
/// `pub(crate)` so the declarative-UI renderer (#215, `super::ui`) shares the
/// exact same visual language instead of forking the palette.
pub(crate) const STYLE: &str = r#"
  * { margin: 0; padding: 0; box-sizing: border-box; }
  body {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    background: #0f172a;
    color: #e2e8f0;
    padding: 2rem;
    line-height: 1.6;
  }
  .container { max-width: 1200px; margin: 0 auto; }
  .report-header { border-bottom: 2px solid #d946ef; padding-bottom: 1rem; margin-bottom: 2rem; }
  .report-header h1 { font-size: 1.75rem; font-weight: 700; color: #f8fafc; }
  .report-header .subtitle { color: #94a3b8; font-size: 0.875rem; margin-top: 0.25rem; }
  .summary-cards {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
    margin-bottom: 2rem;
  }
  .card { background: #1e293b; border: 1px solid #334155; border-radius: 8px; padding: 1.25rem; }
  .card .label {
    font-size: 0.75rem; text-transform: uppercase; letter-spacing: 0.05em;
    color: #94a3b8; margin-bottom: 0.25rem;
  }
  .card .value {
    font-size: 1.5rem; font-weight: 700; color: #f8fafc;
    font-family: 'Cascadia Code', 'Fira Code', 'Consolas', monospace;
  }
  .card .value.accent { color: #d946ef; }
  .table-wrapper { overflow-x: auto; border-radius: 8px; border: 1px solid #334155; margin-bottom: 2rem; }
  table { width: 100%; border-collapse: collapse; font-size: 0.875rem; }
  thead th {
    background: #1e293b; color: #94a3b8; font-weight: 600; text-transform: uppercase;
    font-size: 0.7rem; letter-spacing: 0.05em; padding: 0.75rem 1rem; text-align: left;
    position: sticky; top: 0; border-bottom: 2px solid #334155;
  }
  tbody td {
    padding: 0.625rem 1rem; border-bottom: 1px solid #1e293b;
    font-family: 'Cascadia Code', 'Fira Code', 'Consolas', monospace; font-size: 0.8125rem;
  }
  tbody th {
    padding: 0.625rem 1rem; border-bottom: 1px solid #1e293b; text-align: left;
    color: #94a3b8; font-weight: 600;
  }
  tbody tr { background: #0f172a; }
  tbody tr:nth-child(even) { background: #162032; }
  tbody tr:hover { background: #1e3a5f; }
  .empty { color: #64748b; font-style: italic; padding: 1rem 0; }
  .report-footer {
    margin-top: 2rem; padding-top: 1rem; border-top: 1px solid #334155;
    color: #64748b; font-size: 0.75rem; text-align: center;
  }
  @media print {
    body { background: #fff; color: #1e293b; padding: 1rem; }
    .card { border: 1px solid #e2e8f0; background: #f8fafc; }
    .card .value { color: #0f172a; }
    .card .value.accent { color: #a21caf; }
    thead th { background: #f1f5f9; color: #475569; border-bottom-color: #cbd5e1; }
    tbody tr, tbody tr:nth-child(even) { background: #fff; }
    tbody tr:hover { background: #fff; }
    tbody td { border-bottom-color: #e2e8f0; color: #1e293b; }
    tbody th { color: #475569; }
    .report-header { border-bottom-color: #a21caf; }
    .report-header h1 { color: #0f172a; }
    .report-footer { color: #94a3b8; }
  }
"#;

/// Optional table presentation controls for the `render` command. `Default`
/// reproduces the legacy behaviour: every key as a column (first-seen order) and
/// rows in input order. Only affects an array-of-objects (the table case).
#[derive(Debug, Default, Clone)]
pub struct TableOptions {
    /// Columns to show, in this exact order. `None` or an empty list → the union of
    /// all keys in first-seen order (legacy behaviour). Lets a report drop noise
    /// (raw URLs, nested JSON blobs, internal ids) and keep what matters.
    pub columns: Option<Vec<String>>,
    /// Key to sort rows by before rendering. `None` → input order. Numbers compare
    /// numerically; everything else by string form (so ISO timestamps sort
    /// chronologically); a missing/null key sorts last in both directions.
    pub sort_by: Option<String>,
    /// Sort descending (e.g. most-recent-first for an ISO timestamp). Ignored unless
    /// `sort_by` is set.
    pub sort_desc: bool,
}

/// Render `data` into a complete, self-contained HTML document titled `title`,
/// with table presentation `opts` (column selection/order and row sort). Pass
/// `&TableOptions::default()` for the legacy all-columns, input-order behaviour.
///
/// Deterministic: identical `(title, data, opts)` always yield identical bytes (no
/// timestamp / no environment), so reports are reproducible and testable.
pub fn render_report_with(title: &str, data: &Value, opts: &TableOptions) -> String {
    let count = item_count(data);
    let title = if title.trim().is_empty() {
        "Report"
    } else {
        title
    };
    let body = render_node(data, opts);
    format!(
        "<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n<meta charset=\"UTF-8\">\n\
<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n\
<title>{title}</title>\n<style>{STYLE}</style>\n</head>\n<body>\n\
<div class=\"container\">\n\
<div class=\"report-header\"><h1>{title}</h1>\
<div class=\"subtitle\">{count} item{plural}</div></div>\n\
{body}\n\
<div class=\"report-footer\">Generated by AWARE</div>\n\
</div>\n</body>\n</html>\n",
        title = esc(title),
        plural = if count == 1 { "" } else { "s" },
    )
}

/// Item count surfaced in the header subtitle: array length, object key count,
/// `0` for null, else `1` (a lone scalar). Also reported by the `render` command
/// as `item-count`.
pub fn item_count(data: &Value) -> usize {
    match data {
        Value::Array(a) => a.len(),
        Value::Object(o) => o.len(),
        Value::Null => 0,
        _ => 1,
    }
}

/// Render a value into a report BODY FRAGMENT (no document shell): array →
/// table (honoring `opts`), object → field/value table, scalar → value card.
/// `pub(crate)` so the declarative-UI renderer (#215, `super::ui`) reuses the
/// exact table/sort/column logic for its `table`/`report` blocks instead of
/// reimplementing it.
pub(crate) fn render_fragment(data: &Value, opts: &TableOptions) -> String {
    render_node(data, opts)
}

/// Render the top-level value into the report body fragment.
fn render_node(data: &Value, opts: &TableOptions) -> String {
    match data {
        Value::Array(items) => render_array(items, opts),
        Value::Object(map) => render_object(map),
        scalar => format!(
            "<div class=\"summary-cards\"><div class=\"card\">\
<div class=\"label\">Value</div><div class=\"value accent\">{}</div></div></div>",
            cell(scalar)
        ),
    }
}

/// An array renders as a table. An array of objects uses [`TableOptions::columns`]
/// (or the union of keys, first-seen order) as columns and may be sorted by
/// [`TableOptions::sort_by`]; any other array is a single `Value` column.
fn render_array(items: &[Value], opts: &TableOptions) -> String {
    if items.is_empty() {
        return "<p class=\"empty\">No items.</p>".to_string();
    }
    if items.iter().all(|v| v.is_object()) {
        // Sort first so column selection and row order stay independent concerns.
        let sorted;
        let items: &[Value] = match &opts.sort_by {
            Some(key) => {
                sorted = sort_items(items, key, opts.sort_desc);
                &sorted
            }
            None => items,
        };
        // Selected columns in the requested order, else the union of all keys.
        let columns: Vec<String> = match &opts.columns {
            Some(cols) if !cols.is_empty() => cols.clone(),
            _ => union_keys(items),
        };
        let head: String = columns
            .iter()
            .map(|c| format!("<th>{}</th>", esc(c)))
            .collect();
        let rows: String = items
            .iter()
            .map(|item| {
                let cells: String = columns
                    .iter()
                    .map(|c| format!("<td>{}</td>", cell(item.get(c).unwrap_or(&Value::Null))))
                    .collect();
                format!("<tr>{cells}</tr>")
            })
            .collect();
        return table(&format!("<tr>{head}</tr>"), &rows);
    }
    // Scalar / mixed array → one `Value` column, one row per element.
    let rows: String = items
        .iter()
        .map(|v| format!("<tr><td>{}</td></tr>", cell(v)))
        .collect();
    table("<tr><th>Value</th></tr>", &rows)
}

/// Sort a copy of `items` by each object's `key`. Numbers compare numerically;
/// other values by string form (ISO timestamps therefore sort chronologically); a
/// missing/null key sorts last in BOTH directions. Stable, so equal keys keep input
/// order and the output stays deterministic. `desc` reverses the order of PRESENT
/// values only.
fn sort_items(items: &[Value], key: &str, desc: bool) -> Vec<Value> {
    let mut out = items.to_vec();
    out.sort_by(|a, b| {
        let (av, bv) = (a.get(key), b.get(key));
        let ord = cmp_cell(av, bv);
        // `desc` reverses the comparison ONLY when both keys are present. A missing/null key
        // sorts last regardless of direction — the old blanket `ord.reverse()` floated blank
        // rows to the TOP under `sort-desc`, contradicting the documented "missing sorts last"
        // (#210 follow-up).
        let both_present = av.is_some_and(|v| !v.is_null()) && bv.is_some_and(|v| !v.is_null());
        if desc && both_present {
            ord.reverse()
        } else {
            ord
        }
    });
    out
}

/// Order two optional cell values: missing/null last (ascending), numbers
/// numerically, everything else by string form.
fn cmp_cell(a: Option<&Value>, b: Option<&Value>) -> std::cmp::Ordering {
    use std::cmp::Ordering;
    let missing = |v: Option<&Value>| matches!(v, None | Some(Value::Null));
    match (missing(a), missing(b)) {
        (true, true) => return Ordering::Equal,
        (true, false) => return Ordering::Greater,
        (false, true) => return Ordering::Less,
        (false, false) => {}
    }
    match (a, b) {
        (Some(Value::Number(x)), Some(Value::Number(y))) => x
            .as_f64()
            .partial_cmp(&y.as_f64())
            .unwrap_or(Ordering::Equal),
        (Some(x), Some(y)) => sort_key(x).cmp(&sort_key(y)),
        _ => Ordering::Equal,
    }
}

/// Comparable string form for a non-numeric sort key (string as-is, else JSON).
fn sort_key(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        other => other.to_string(),
    }
}

/// An object renders as a two-column Field / Value table.
fn render_object(map: &serde_json::Map<String, Value>) -> String {
    if map.is_empty() {
        return "<p class=\"empty\">No fields.</p>".to_string();
    }
    let rows: String = map
        .iter()
        .map(|(k, v)| format!("<tr><th>{}</th><td>{}</td></tr>", esc(k), cell(v)))
        .collect();
    table("<tr><th>Field</th><th>Value</th></tr>", &rows)
}

/// Wrap a header row + body rows in the styled table scaffold.
fn table(head: &str, rows: &str) -> String {
    format!(
        "<div class=\"table-wrapper\"><table>\n<thead>{head}</thead>\n<tbody>\n{rows}\n</tbody>\n</table></div>"
    )
}

/// The union of object keys across `items`, in first-seen order (stable because
/// serde_json preserves object insertion order via the `preserve_order` feature).
fn union_keys(items: &[Value]) -> Vec<String> {
    let mut seen = std::collections::HashSet::new();
    let mut out = Vec::new();
    for item in items {
        if let Value::Object(map) = item {
            for k in map.keys() {
                if seen.insert(k.clone()) {
                    out.push(k.clone());
                }
            }
        }
    }
    out
}

/// One cell's content: scalars print plainly; nested structures print as compact,
/// escaped JSON so they are visible without exploding the layout.
fn cell(v: &Value) -> String {
    match v {
        Value::Null => String::new(),
        Value::String(s) => esc(s),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::Array(_) | Value::Object(_) => esc(&v.to_string()),
    }
}

/// HTML-escape text for safe insertion into element content / attributes.
/// `pub(crate)` so every renderer in the crate (incl. `super::ui`, #215) escapes
/// user-supplied strings through this one function.
pub(crate) fn esc(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// Default-presentation convenience for the legacy table/object/scalar cases.
    fn render_report(title: &str, data: &Value) -> String {
        render_report_with(title, data, &TableOptions::default())
    }

    #[test]
    fn array_of_objects_renders_a_table_with_union_columns() {
        let data = json!([
            {"id": "p1", "name": "Tower A"},
            {"id": "p2", "name": "Tower B", "region": "EU"},
        ]);
        let html = render_report("Projects", &data);
        // Columns are the union of keys in first-seen order.
        assert!(html.contains("<th>id</th><th>name</th><th>region</th>"));
        // Rows carry the values; a missing key is an empty cell.
        assert!(html.contains("<td>p1</td><td>Tower A</td><td></td>"));
        assert!(html.contains("<td>p2</td><td>Tower B</td><td>EU</td>"));
        // Header + count + self-contained shell.
        assert!(html.starts_with("<!DOCTYPE html>"));
        assert!(html.contains("<title>Projects</title>"));
        assert!(html.contains("2 items"));
        assert!(html.contains("<style>"));
    }

    #[test]
    fn object_renders_a_field_value_table() {
        let data = json!({"status": 200, "ok": true});
        let html = render_report("Result", &data);
        assert!(html.contains("<tr><th>Field</th><th>Value</th></tr>"));
        assert!(html.contains("<tr><th>status</th><td>200</td></tr>"));
        assert!(html.contains("<tr><th>ok</th><td>true</td></tr>"));
        assert!(html.contains("2 items"));
    }

    #[test]
    fn scalar_renders_a_value_card_singular_count() {
        let html = render_report("N", &json!("hello"));
        assert!(html.contains("class=\"value accent\">hello</div>"));
        assert!(html.contains("1 item<")); // singular, no trailing 's'
    }

    #[test]
    fn array_of_scalars_renders_single_value_column() {
        let html = render_report("List", &json!(["a", "b", 3]));
        assert!(html.contains("<tr><th>Value</th></tr>"));
        assert!(html.contains("<tr><td>a</td></tr>"));
        assert!(html.contains("<tr><td>3</td></tr>"));
        assert!(html.contains("3 items"));
    }

    #[test]
    fn empty_array_renders_an_empty_note() {
        let html = render_report("Empty", &json!([]));
        assert!(html.contains("No items."));
        assert!(html.contains("0 items"));
    }

    #[test]
    fn values_and_title_are_html_escaped() {
        let data = json!([{"name": "<script>alert(\"x\")</script> & 'co'"}]);
        let html = render_report("A & B <c>", &data);
        assert!(html.contains("<title>A &amp; B &lt;c&gt;</title>"));
        assert!(
            html.contains("&lt;script&gt;alert(&quot;x&quot;)&lt;/script&gt; &amp; &#39;co&#39;")
        );
        // No unescaped script tag leaks into the document body.
        assert!(!html.contains("<script>"));
    }

    #[test]
    fn nested_value_renders_as_compact_escaped_json() {
        let data = json!([{"id": 1, "tags": ["a", "b"]}]);
        let html = render_report("Nested", &data);
        // The array cell is compact JSON, escaped.
        assert!(html.contains("[&quot;a&quot;,&quot;b&quot;]"));
    }

    #[test]
    fn render_is_deterministic() {
        let data = json!([{"id": "p2"}, {"id": "p1"}]);
        assert_eq!(render_report("T", &data), render_report("T", &data));
    }

    #[test]
    fn empty_title_falls_back_to_report() {
        let html = render_report("   ", &json!({}));
        assert!(html.contains("<title>Report</title>"));
    }

    #[test]
    fn columns_option_selects_and_orders_visible_columns() {
        let data = json!([
            {"id": "p1", "name": "Tower A", "secret": "hush", "region": "EU"},
            {"id": "p2", "name": "Tower B", "secret": "hush", "region": "US"},
        ]);
        let opts = TableOptions {
            columns: Some(vec!["name".into(), "region".into()]),
            ..Default::default()
        };
        let html = render_report_with("Projects", &data, &opts);
        // Only the requested columns, in the requested order.
        assert!(html.contains("<th>name</th><th>region</th>"));
        assert!(!html.contains("<th>id</th>"));
        assert!(!html.contains("<th>secret</th>"));
        assert!(!html.contains("hush")); // the dropped column's values are gone too
        assert!(html.contains("<td>Tower A</td><td>EU</td>"));
        // Count still reflects the full array, not the visible column count.
        assert!(html.contains("2 items"));
    }

    #[test]
    fn empty_columns_list_falls_back_to_all_keys() {
        let data = json!([{"a": 1, "b": 2}]);
        let opts = TableOptions {
            columns: Some(vec![]),
            ..Default::default()
        };
        let html = render_report_with("P", &data, &opts);
        assert!(html.contains("<th>a</th><th>b</th>"));
    }

    #[test]
    fn sort_by_descending_orders_rows_most_recent_first() {
        let data = json!([
            {"name": "Old", "modifiedOn": "2023-01-01T00:00:00+0000"},
            {"name": "New", "modifiedOn": "2026-06-01T00:00:00+0000"},
            {"name": "Mid", "modifiedOn": "2024-03-01T00:00:00+0000"},
        ]);
        let opts = TableOptions {
            sort_by: Some("modifiedOn".into()),
            sort_desc: true,
            ..Default::default()
        };
        let html = render_report_with("P", &data, &opts);
        let pos = |needle: &str| html.find(needle).expect("row present");
        assert!(pos("<td>New</td>") < pos("<td>Mid</td>"));
        assert!(pos("<td>Mid</td>") < pos("<td>Old</td>"));
    }

    #[test]
    fn sort_by_numeric_key_compares_numerically_not_lexically() {
        let data = json!([
            {"name": "b", "size": 9},
            {"name": "a", "size": 100},
        ]);
        let opts = TableOptions {
            sort_by: Some("size".into()),
            ..Default::default()
        };
        let html = render_report_with("P", &data, &opts);
        // Ascending numeric: 9 before 100 (a lexical sort would order "100" first).
        assert!(html.find("<td>b</td>").unwrap() < html.find("<td>a</td>").unwrap());
    }

    #[test]
    fn missing_sort_key_sorts_last_even_when_descending() {
        // A row missing the sort key must sort LAST in both directions — `sort-desc` must not
        // float blank rows to the top (#210 follow-up: the manifest documents "missing sorts
        // last", but the old blanket reverse put them first under desc).
        let data = json!([
            {"name": "HasKey-Low",  "modifiedOn": "2023-01-01T00:00:00+0000"},
            {"name": "NoKey"},
            {"name": "HasKey-High", "modifiedOn": "2026-06-01T00:00:00+0000"},
        ]);
        let opts = TableOptions {
            sort_by: Some("modifiedOn".into()),
            sort_desc: true,
            ..Default::default()
        };
        let html = render_report_with("P", &data, &opts);
        let pos = |needle: &str| html.find(needle).expect("row present");
        // desc orders present values High-before-Low …
        assert!(pos("<td>HasKey-High</td>") < pos("<td>HasKey-Low</td>"));
        // … and the missing-key row stays LAST, not first.
        assert!(pos("<td>HasKey-Low</td>") < pos("<td>NoKey</td>"));
    }
}
