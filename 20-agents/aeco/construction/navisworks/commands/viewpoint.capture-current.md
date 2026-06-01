# viewpoint.capture-current

Capture the current Navisworks view as a PNG — used by the BIM-manager Monday-clash-digest flow to attach a screenshot to a Teams post or ACC Issue.

Read-only.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `document-id` | string | yes | The open federation. |
| `output-path` | string | yes | PNG output path. |
| `width-pixels` | number | no (default `1920`) | Image width. |
| `height-pixels` | number | no (default `1080`) | Image height. |

## Output

```yaml
path:          "C:/digest/view.png"
width-pixels:  1920
height-pixels: 1080
```

## Implementation (Navisworks .NET API)

There are **two paths, and which one applies is version-dependent**:

- **Navisworks 2021+ (.NET):** `Application.ActiveDocument.ActiveView.GenerateImage(ImageGenerationStyle, width, height)` returns a `System.Drawing.Bitmap`; save it as PNG:

  ```csharp
  var view = Application.ActiveDocument.ActiveView;
  using var bmp = view.GenerateImage(ImageGenerationStyle.ScenePlusOverlay, w, h);
  bmp.Save(outputPath, System.Drawing.Imaging.ImageFormat.Png);
  ```

  `ImageGenerationStyle` is `Scene` or `ScenePlusOverlay`. (2020 had only `GenerateThumbnail`.)

- **Pre-2021 (COM fallback):** drive the `lcodpimage` IO plugin via `ComApiBridge.State` (`InwOpState10`) → `GetIOPluginOptions("lcodpimage")` → `DriveIOPlugin("lcodpimage", path, options)`. **This defaults to 256×256** and there is no documented COM way to set the resolution from code (only a hidden SHIFT+Options UI setting) — so `width-pixels`/`height-pixels` cannot be honored on pre-2021 builds.

## Gotchas

- **Version gate: `GenerateImage` is 2021+.** On older builds you fall back to the awkward COM `lcodpimage` path (256×256 default, no programmatic resolution) — so the `width`/`height` inputs are only fully honored on 2021+.
- **Needs a live, rendered view → run in-process on the UI thread.** Whether `ActiveView` is non-null under headless `NavisworksApplication` automation is unverified (historically image work went through COM there). This is a key reason the sidecar uses an in-process AddInPlugin, not Automation.
- **Very large sizes** (e.g. 3840×2160) hit performance/memory limits.

Sources: [Export viewpoint image — `GenerateImage` (2021) / `GenerateThumbnail` (2020) (Autodesk forum)](https://forums.autodesk.com/t5/navisworks-api/export-viewpoint-image/td-p/9539353) · [Batch-export saved-viewpoint images via `GenerateImage` (Autodesk forum)](https://forums.autodesk.com/t5/navisworks-api-forum/batch-export-saved-viewpoint-images-via-generateimage/td-p/12200296) · [Image resolution of viewpoints via the COM `lcodpimage` path + 256×256 limit (teocomi)](https://teocomi.com/image-resolution-of-viewpoints-exported-via-navisworks-api/)

## See also

- [viewpoint.from-clash](./viewpoint.from-clash.md) — capture framed on a clash
- [viewpoint.list-saved](./viewpoint.list-saved.md)
- [viewpoints-and-markup](../skills/viewpoints-and-markup.md)
