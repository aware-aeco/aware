# export.layouts-to-pdf

Export every layout in the active doc to a single PDF (or one PDF per layout). The architect "issue the sheet pack" primitive.

## Inputs

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `out-path` | string | yes | — | Output PDF path, or output DIRECTORY when `one-per-layout` is true. |
| `one-per-layout` | boolean | no | false | If true, writes `<out-path>/<layout-name>.pdf` per layout. |
| `dpi` | number | no | 300 | Raster DPI for embedded bitmaps. |
| `layout-name-filter` | string | no | `""` | Optional .NET regex; only layouts whose name matches are exported. |

## Outputs

```json
{ "exported-count": 12, "files": ["C:/issue/A-100.pdf", "C:/issue/A-101.pdf", ...] }
```

## Implementation (shipped through `aware-rhino exec`)

```csharp
var doc = Rhino.RhinoDoc.ActiveDoc;
var outPath  = args["out-path"]!.ToString();
bool perLayout = args.TryGetValue("one-per-layout", out var p) && Convert.ToBoolean(p);
int dpi = Convert.ToInt32(args.TryGetValue("dpi", out var d) ? d : 300);
var nameFilter = args.TryGetValue("layout-name-filter", out var f) ? f?.ToString() ?? "" : "";
var rx = string.IsNullOrEmpty(nameFilter) ? null : new System.Text.RegularExpressions.Regex(nameFilter,
            System.Text.RegularExpressions.RegexOptions.IgnoreCase);

// Collect layouts (page views).
var layouts = new List<Rhino.Display.RhinoPageView>();
foreach (var view in doc.Views)
{
    if (view is Rhino.Display.RhinoPageView page)
    {
        if (rx == null || rx.IsMatch(page.PageName))
            layouts.Add(page);
    }
}
if (layouts.Count == 0) return new { ok = false, error = "no layouts matched in active doc" };

var written = new List<string>();
if (perLayout)
{
    System.IO.Directory.CreateDirectory(outPath);
    foreach (var page in layouts)
    {
        var pdf = Rhino.FileIO.FilePdf.Create();
        var settings = new Rhino.Display.ViewCaptureSettings(
            page, page.PageWidth, page.PageHeight, dpi);
        pdf.AddPage(settings);
        var safeName = System.Text.RegularExpressions.Regex.Replace(page.PageName, @"[\\/:*?""<>|]+", "_");
        var target = System.IO.Path.Combine(outPath, safeName + ".pdf");
        pdf.Write(target);
        written.Add(target);
    }
}
else
{
    System.IO.Directory.CreateDirectory(System.IO.Path.GetDirectoryName(outPath)!);
    var pdf = Rhino.FileIO.FilePdf.Create();
    foreach (var page in layouts)
    {
        var settings = new Rhino.Display.ViewCaptureSettings(
            page, page.PageWidth, page.PageHeight, dpi);
        pdf.AddPage(settings);
    }
    pdf.Write(outPath);
    written.Add(outPath);
}
return new { ok = true, exported_count = layouts.Count, files = written };
```

## Gotchas

- `RhinoPageView` is the layout type; `RhinoView` covers both model and page views. Filter by `is RhinoPageView`.
- `page.PageWidth` / `page.PageHeight` are in document units; Rhino's PDF writer expects them as the target sheet size.
- The PDF writer rasterizes hidden-line views per the print width / weight settings on the page. Vector output requires the layout's "vector" rendering mode.
- Filename sanitization replaces `\/:*?"<>|` with `_`. Pre-process your page names if you want different behavior.

## See also

- `_-Print` (Rhino command line) — interactive equivalent
- [`view.capture-bitmap`](./view.capture-bitmap.md) — for model-space PNG captures
