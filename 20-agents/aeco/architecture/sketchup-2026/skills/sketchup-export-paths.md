---
name: sketchup-export-paths
description: This skill should be used when composing snippets that produce files from SketchUp — exporting STL/OBJ/DAE/3DS/DWG/PDF for downstream tools, writing scene images via `view.write_image`, generating reports, or any operation that lands bytes on disk. Encodes the difference between `model.export(path, options)` and the legacy command-line `Sketchup.send_to_console("Export ...")` path, the per-format option dictionaries (3DS, DAE, FBX, OBJ, STL, KMZ, IFC have different keys), the gotcha that `view.write_image` is the ONLY pure-Ruby path for raster output (no PDF API), and the AWARE convention of writing to `%TEMP%/aware-sketchup/<run-id>/` for run-scoped temp artifacts.
---

# SketchUp export paths

SketchUp has two parallel export systems with very different option vocabularies. Pick the wrong one and your IFC export comes out as a static OBJ.

## The two systems

| Path | When to use | API |
|---|---|---|
| **`model.export(path, options)`** | Modern (2014+) Ruby API. Returns true/false. | `Sketchup::Model#export` |
| **`Sketchup.send_to_console("Export <path>")`** | Legacy command-line dispatch. Triggers any export plugin registered by other extensions. | `Sketchup.send_to_console` |

For built-in formats (3DS, DAE, DWG/DXF for SketchUp Pro, FBX, KMZ, OBJ, STL, IFC), `model.export` is the reliable path. For exports provided by third-party extensions (e.g. SimLab DWG, SketchUp's Twinmotion direct link), `send_to_console` may be the only way.

## Format-specific option dictionaries

`model.export(path, options)` takes a hash. The valid keys DEPEND on the file extension SketchUp parses out of the path:

### `.obj` (OBJ)
```ruby
model.export(path, {
  :triangulated_faces        => true,
  :doublesided_faces         => false,
  :edges                     => false,
  :texture_maps              => true,
  :swap_yz                   => false,
  :units                     => "Meters",  # or "Millimeters", "Inches", etc.
  :selectionset_only         => false,
})
```

### `.dae` (Collada)
```ruby
model.export(path, {
  :triangulated_faces  => true,
  :doublesided_faces   => false,
  :edges               => false,
  :texture_maps        => true,
  :preserve_instancing => true,
  :selectionset_only   => false,
})
```

### `.stl` (Stereolithography)
SketchUp's built-in STL exporter ships as a default extension and registers via `model.export`. Options:
```ruby
model.export(path, {
  :units                       => "Millimeters",
  :format                      => "ASCII",   # or "Binary"
  :selectionset_only           => false,
  :swap_yz                     => true,
})
```

### `.3ds`, `.fbx`, `.kmz`, `.ifc`
Each has its own dictionary. Always check the [SketchUp Developer Center](https://developer.trimble.com/docs/desktop-api/) for the current key list — Trimble adds new options between releases.

### Image (`.png`, `.jpg`, `.bmp`, `.tif`)
Image export goes through `view.write_image`, NOT `model.export`:
```ruby
model.active_view.write_image({
  :filename       => path,
  :width          => 1920,
  :height         => 1080,
  :antialias      => true,
  :compression    => 0.95,     # JPEG quality 0..1
  :transparent    => false,
})
```

For multi-image runs across named scenes, iterate `model.pages`, set the active page, redraw, then write_image.

## The PDF gap

**SketchUp has no Ruby PDF export API.** The desktop UI's File > Export > 2D Graphic > PDF is implemented natively and not exposed to Ruby. To produce PDF programmatically:

1. **Export PNGs per scene via `view.write_image`** then convert to PDF outside SketchUp (Ghostscript, ImageMagick, etc.).
2. **Send a console command:** `Sketchup.send_to_console('File.Export.2D-Graphic("' + path + '")')` — but this opens the GUI dialog requesting confirmation in most versions.
3. **Use a third-party extension** (e.g. SimLab) that wraps PDF generation and exposes a Ruby callable.

For AWARE workflows, prefer path (1) — produce PNGs in SketchUp, hand them to a downstream PDF assembler agent.

## AWARE conventions

- **Per-run temp dir:** `%TEMP%/aware-sketchup/<run-id>/` — scoped to the AWARE app's run-id so concurrent runs don't collide. The orchestrator passes the run-id through `args`.
- **Cleanup:** the script that produced the temp artifact does NOT clean up; the AWARE runtime garbage-collects the run-temp dir on completion.
- **Selection vs full model:** verbs that operate on `selection` should pass `selectionset_only: true` to keep the export tight.

## Common gotchas

- **`model.export` returns false silently** on permission denied / disk full. Always check the return value:
  ```ruby
  ok = model.export(path, opts)
  return { ok: false, error: "export returned false (check path + permissions)" } unless ok
  ```
- **Some options are silently ignored** if the format doesn't support them. No warning, no error.
- **Paths must be absolute.** Relative paths resolve against the current working directory of the SketchUp process — which is usually NOT what you want.
- **`view.write_image` requires `:width`+`:height`.** Without them, it uses the current viewport size, which is fragile across user setups.
- **The current page (scene)'s style affects the export.** Set `model.pages.selected_page = some_page` first if you want a specific style baked in.

## Standard pattern

```ruby
require 'fileutils'

model    = Sketchup.active_model
run_id   = args["run-id"].to_s.empty? ? "manual" : args["run-id"]
temp_dir = File.join(ENV["TEMP"] || "/tmp", "aware-sketchup", run_id)
FileUtils.mkdir_p(temp_dir)

target = File.join(temp_dir, "export.stl")
ok = model.export(target, {
  :units => "Millimeters",
  :format => "Binary",
  :selectionset_only => false,
})
return { ok: false, error: "export returned false" } unless ok

{ ok: true, path: target, bytes: File.size(target) }
```

## See also

- [`scene.export-to-images`](../commands/scene.export-to-images.md) — uses `view.write_image` per page
- [`sketchup-units-and-precision`](./sketchup-units-and-precision.md) — units-aware export options
