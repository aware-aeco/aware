---
name: html-report-styling
description: HTML report styling, html file generation, html output, html table, web report, styled html, html report template, generate html, create html file, save html, beautiful report, professional report, dashboard, summary page.
---

# HTML Report Styling Guide

When generating code that creates HTML files or HTML string output for reports, summaries, or data presentation, follow these design rules to produce professional, polished reports.

## Design Principles

- **Self-contained**: ALL CSS inline in a `<style>` tag. No external stylesheets, CDNs, or dependencies.
- **Dark theme**: Dark slate backgrounds with high-contrast text and fuchsia accents.
- **Responsive tables**: Sticky headers, alternating row colors, hover highlights.
- **Summary cards**: Key metrics displayed prominently at the top.
- **Print-friendly**: `@media print` rules for clean paper output.
- **System fonts**: Use the system font stack for fast rendering, monospace for data values.

## Color Palette

| Role | Color | Hex |
|------|-------|-----|
| Page background | Dark navy | `#0f172a` |
| Card/table background | Dark slate | `#1e293b` |
| Alternate row | Darker slate | `#162032` |
| Primary text | Light gray | `#e2e8f0` |
| Secondary text | Medium gray | `#94a3b8` |
| Accent / highlights | Fuchsia | `#d946ef` |
| Success | Green | `#10b981` |
| Warning | Amber | `#f59e0b` |
| Error | Red | `#ef4444` |
| Border | Subtle slate | `#334155` |

## Complete HTML Template

Use this as the base structure for every HTML report:

```html
<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>{REPORT_TITLE}</title>
<style>
  * { margin: 0; padding: 0; box-sizing: border-box; }
  body {
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    background: #0f172a;
    color: #e2e8f0;
    padding: 2rem;
    line-height: 1.6;
  }
  .container { max-width: 1200px; margin: 0 auto; }

  /* Header */
  .report-header {
    border-bottom: 2px solid #d946ef;
    padding-bottom: 1rem;
    margin-bottom: 2rem;
  }
  .report-header h1 {
    font-size: 1.75rem;
    font-weight: 700;
    color: #f8fafc;
  }
  .report-header .subtitle {
    color: #94a3b8;
    font-size: 0.875rem;
    margin-top: 0.25rem;
  }

  /* Summary Cards */
  .summary-cards {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 1rem;
    margin-bottom: 2rem;
  }
  .card {
    background: #1e293b;
    border: 1px solid #334155;
    border-radius: 8px;
    padding: 1.25rem;
  }
  .card .label {
    font-size: 0.75rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: #94a3b8;
    margin-bottom: 0.25rem;
  }
  .card .value {
    font-size: 1.5rem;
    font-weight: 700;
    color: #f8fafc;
    font-family: 'Cascadia Code', 'Fira Code', 'Consolas', monospace;
  }
  .card .value.accent { color: #d946ef; }
  .card .value.success { color: #10b981; }
  .card .value.warning { color: #f59e0b; }

  /* Data Table */
  .table-wrapper {
    overflow-x: auto;
    border-radius: 8px;
    border: 1px solid #334155;
    margin-bottom: 2rem;
  }
  table {
    width: 100%;
    border-collapse: collapse;
    font-size: 0.875rem;
  }
  thead th {
    background: #1e293b;
    color: #94a3b8;
    font-weight: 600;
    text-transform: uppercase;
    font-size: 0.7rem;
    letter-spacing: 0.05em;
    padding: 0.75rem 1rem;
    text-align: left;
    position: sticky;
    top: 0;
    border-bottom: 2px solid #334155;
  }
  tbody td {
    padding: 0.625rem 1rem;
    border-bottom: 1px solid #1e293b;
    font-family: 'Cascadia Code', 'Fira Code', 'Consolas', monospace;
    font-size: 0.8125rem;
  }
  tbody tr { background: #0f172a; }
  tbody tr:nth-child(even) { background: #162032; }
  tbody tr:hover { background: #1e3a5f; }

  /* Status Badges */
  .badge {
    display: inline-block;
    padding: 0.125rem 0.5rem;
    border-radius: 9999px;
    font-size: 0.7rem;
    font-weight: 600;
    text-transform: uppercase;
  }
  .badge-success { background: #064e3b; color: #6ee7b7; }
  .badge-warning { background: #78350f; color: #fcd34d; }
  .badge-error   { background: #7f1d1d; color: #fca5a5; }
  .badge-info    { background: #1e1b4b; color: #a5b4fc; }

  /* Footer */
  .report-footer {
    margin-top: 2rem;
    padding-top: 1rem;
    border-top: 1px solid #334155;
    color: #64748b;
    font-size: 0.75rem;
    text-align: center;
  }

  /* Print Styles */
  @media print {
    body { background: #fff; color: #1e293b; padding: 1rem; }
    .card { border: 1px solid #e2e8f0; background: #f8fafc; }
    .card .label { color: #64748b; }
    .card .value { color: #0f172a; }
    .card .value.accent { color: #a21caf; }
    .card .value.success { color: #047857; }
    .card .value.warning { color: #b45309; }
    thead th { background: #f1f5f9; color: #475569; border-bottom-color: #cbd5e1; }
    tbody tr, tbody tr:nth-child(even) { background: #fff; }
    tbody tr:hover { background: #fff; }
    tbody td { border-bottom-color: #e2e8f0; color: #1e293b; }
    .report-header { border-bottom-color: #a21caf; }
    .report-header h1 { color: #0f172a; }
    .report-footer { color: #94a3b8; }
    .badge-success { background: #d1fae5; color: #065f46; }
    .badge-warning { background: #fef3c7; color: #92400e; }
    .badge-error   { background: #fee2e2; color: #991b1b; }
    .badge-info    { background: #e0e7ff; color: #3730a3; }
  }
</style>
</head>
<body>
<div class="container">
  <div class="report-header">
    <h1>{REPORT_TITLE}</h1>
    <div class="subtitle">Generated {TIMESTAMP} &bull; {ITEM_COUNT} items</div>
  </div>

  <div class="summary-cards">
    <div class="card">
      <div class="label">Total Items</div>
      <div class="value accent">{COUNT}</div>
    </div>
    <!-- Add more cards as needed -->
  </div>

  <div class="table-wrapper">
    <table>
      <thead>
        <tr><th>Column 1</th><th>Column 2</th></tr>
      </thead>
      <tbody>
        <tr><td>Value</td><td>Value</td></tr>
      </tbody>
    </table>
  </div>

  <div class="report-footer">
    Generated by AWARE &bull; {TIMESTAMP}
  </div>
</div>
</body>
</html>
```

## C# StringBuilder Pattern

Build the HTML report in code using StringBuilder:

```csharp
var sb = new StringBuilder();
sb.AppendLine("<!DOCTYPE html>");
sb.AppendLine("<html lang=\"en\">");
sb.AppendLine("<head>");
sb.AppendLine("<meta charset=\"UTF-8\">");
sb.AppendLine($"<title>{reportTitle}</title>");
sb.AppendLine("<style>");
// Paste the full CSS from the template above
sb.AppendLine("</style>");
sb.AppendLine("</head>");
sb.AppendLine("<body>");
sb.AppendLine("<div class=\"container\">");

// Header
sb.AppendLine($"<div class=\"report-header\"><h1>{reportTitle}</h1>");
sb.AppendLine($"<div class=\"subtitle\">Generated {DateTime.Now:yyyy-MM-dd HH:mm} &bull; {items.Count} items</div></div>");

// Summary cards
sb.AppendLine("<div class=\"summary-cards\">");
sb.AppendLine($"<div class=\"card\"><div class=\"label\">Total</div><div class=\"value accent\">{items.Count}</div></div>");
sb.AppendLine("</div>");

// Table
sb.AppendLine("<div class=\"table-wrapper\"><table>");
sb.AppendLine("<thead><tr><th>Column</th></tr></thead>");
sb.AppendLine("<tbody>");
foreach (var item in items)
{
    sb.AppendLine($"<tr><td>{EscapeHtml(item.Value)}</td></tr>");
}
sb.AppendLine("</tbody></table></div>");

// Footer
sb.AppendLine($"<div class=\"report-footer\">Generated by AWARE &bull; {DateTime.Now:yyyy-MM-dd HH:mm}</div>");
sb.AppendLine("</div></body></html>");

var html = sb.ToString();
```

## HTML Escaping

Always escape user data to prevent XSS. Use this helper:

```csharp
private static string EscapeHtml(object value)
{
    if (value is null) return "";
    return value.ToString()
        .Replace("&", "&amp;")
        .Replace("<", "&lt;")
        .Replace(">", "&gt;")
        .Replace("\"", "&quot;");
}
```

## Output Pattern

Always emit the HTML under BOTH keys — `html` (primary, canonical) AND `htmlReport` (secondary, descriptive). This makes the AWARE agent command usable from every downstream consumer without the user having to rename anything:

```csharp
outputs["html"] = html;          // primary — used by HTML Viewer node + short templates {{nodeId.html}}
outputs["htmlReport"] = html;    // alias — descriptive name for Text Panel / logs
outputs["itemCount"] = items.Count;
```

### How downstream nodes consume this

| Consumer | Path | Needs |
|----------|------|-------|
| **Write File → Open File** (web browser) | File path or Write-File content field uses template `{{nodeId.html}}` | `outputs["html"]` key present (TemplateResolutionService substitutes it) |
| **Text Panel** | References `{{nodeId.html}}` or `{{nodeId.htmlReport}}` | Either key present |
| **HTML Viewer node** (phase 63.1) | Direct wire: AWARE agent command `Output` port → `htmlContent` input port | `outputs["html"]` key present (HtmlViewer extracts it from the emitted dictionary) |
| **Any other dict consumer** | `{{nodeId.fieldName}}` template | Matching key in `outputs` |

**Why both keys?** AWARE agent command emits the entire `outputs` dictionary on its single `"Output"` port. Downstream wire-based consumers that expect a string (like HTML Viewer) extract a well-known key from the dict — `html` is the canonical one. Template-based consumers (`{{nodeId.FIELD}}`) look up that exact field name. Emitting both covers both worlds with zero runtime cost.

**Do NOT emit only `htmlReport`** — templates like `{{nodeId.html}}` will render as literal text and HTML Viewer wiring will stay blank.

### Alternative: emitting a file path instead of inline HTML

If your AWARE agent command already writes the report to disk (e.g. to a temp folder, a network share, or as a build artifact), you can wire the **file path** to HTML Viewer's `htmlContent` input port instead of an inline string. HTML Viewer detects existing `.html` and `.htm` paths and renders them via `file://` (relative assets next to the file resolve naturally). The inline-HTML path still wins when present.

Two ways to emit a path:

```csharp
// Option A: scalar wire — AWARE agent command "Output" carries the path string directly
outputs["Output"] = htmlPath;        // must end in .html or .htm and exist on disk

// Option B: dict wire — recognized keys: filePath, htmlFilePath, htmlFile
outputs["filePath"] = htmlPath;      // canonical
outputs["html"] = html;              // optional — inline HTML still wins if present
```

Use this pattern when the file already exists for another reason (sharing, archival, downstream non-AWARE tooling) — otherwise the inline `outputs["html"]` pattern above is preferred since it avoids temp-file management.

## Status Badges in Table Cells

Use status badges for categorical data:

```csharp
string BadgeHtml(string status)
{
    var css = status.ToLower() switch
    {
        "ok" or "pass" or "complete" => "badge-success",
        "warning" or "pending" => "badge-warning",
        "error" or "fail" or "failed" => "badge-error",
        _ => "badge-info"
    };
    return $"<span class=\"badge {css}\">{EscapeHtml(status)}</span>";
}
```

## Numeric Formatting in Cards

Format large numbers for readability:

```csharp
// Weight: "12,450.5 kg"
$"<div class=\"value\">{totalWeight:N1} kg</div>"

// Count: "1,234"
$"<div class=\"value accent\">{count:N0}</div>"

// Percentage: "87.3%"
$"<div class=\"value success\">{percentage:F1}%</div>"
```

