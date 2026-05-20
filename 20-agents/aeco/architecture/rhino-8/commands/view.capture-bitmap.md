# view.capture-bitmap

Capture the named view (or active viewport) as a PNG bitmap at a specified resolution. The designer/architect "Monday-shot" + RFI-screenshot primitive.

## Inputs

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `view-name` | string | no | `""` | Named view to activate before capture. Empty = active viewport. |
| `out-path` | string | yes | — | Output PNG file path. |
| `width` | number | no | 1920 | Pixel width. |
| `height` | number | no | 1080 | Pixel height. |
| `transparent-background` | boolean | no | false | If true, writes a 32-bit PNG with transparent background. |

## Outputs

```json
{ "path": "C:/temp/monday-shot.png", "bytes": 482103, "width": 1920, "height": 1080 }
```

## Implementation (shipped through `aware-rhino exec`)

```csharp
var doc = Rhino.RhinoDoc.ActiveDoc;
var viewName  = args.TryGetValue("view-name", out var v) ? (v?.ToString() ?? "") : "";
var outPath   = args["out-path"]!.ToString();
int w         = Convert.ToInt32(args.TryGetValue("width",  out var wv) ? wv : 1920);
int h         = Convert.ToInt32(args.TryGetValue("height", out var hv) ? hv : 1080);
bool transparent = args.TryGetValue("transparent-background", out var t) && Convert.ToBoolean(t);

// Resolve target viewport: named view OR active.
Rhino.Display.RhinoView? targetView = null;
if (!string.IsNullOrEmpty(viewName))
{
    foreach (var view in doc.Views)
    {
        if (string.Equals(view.MainViewport.Name, viewName, StringComparison.OrdinalIgnoreCase))
        {
            targetView = view;
            break;
        }
    }
    if (targetView == null) return new { ok = false, error = $"no view named '{viewName}' in active doc" };
}
targetView ??= doc.Views.ActiveView;
if (targetView == null) return new { ok = false, error = "no active view" };

var settings = new Rhino.Display.ViewCaptureSettings(targetView, new System.Drawing.Size(w, h), 96);
settings.TransparentBackground = transparent;
using var bitmap = Rhino.Display.ViewCapture.CaptureToBitmap(settings);
if (bitmap == null) return new { ok = false, error = "ViewCapture.CaptureToBitmap returned null" };

System.IO.Directory.CreateDirectory(System.IO.Path.GetDirectoryName(outPath)!);
bitmap.Save(outPath, System.Drawing.Imaging.ImageFormat.Png);
var info = new System.IO.FileInfo(outPath);
return new { ok = true, path = outPath, bytes = info.Length, width = w, height = h };
```

## Gotchas

- `Rhino.Display.ViewCapture` requires Rhino to be running with a model open (the viewport is the rendering source).
- `transparent-background` only works for rendered viewports that support it (most do in Rhino 8); falls back to black.
- For very large outputs (>8K px), Rhino may run out of GPU memory; chunk the capture or use the in-Rhino `_-ViewCaptureToFile` command directly.
- DPI is hard-coded to 96 in the settings above. Override if you need print-resolution captures.

## See also

- `_-ViewCaptureToFile` (Rhino command line) — equivalent macro
- [`export.layouts-to-pdf`](./export.layouts-to-pdf.md) — for paper-space sheet output
