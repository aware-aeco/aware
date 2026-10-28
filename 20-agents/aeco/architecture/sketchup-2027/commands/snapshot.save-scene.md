# snapshot.save-scene

Save the current active viewport (camera + style + shadows + layers) as a SketchUp scene (page).

`mode: write` — modifies the model's pages list.

## Inputs

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `scene-name` | string | yes | — | Name for the new scene. |
| `overwrite` | boolean | no | `true` | If `false` and the name exists, error out. |

## Outputs

```json
{
  "scene-name":   "Lobby-NE",
  "overwritten":  true,
  "camera-eye":   { "x": -5000, "y": -8000, "z": 1800 },
  "camera-target":{ "x":     0, "y":     0, "z":  900 }
}
```

## Implementation (shipped through `aware-sketchup exec`)

```ruby
model     = Sketchup.active_model
name      = args["scene-name"].to_s
overwrite = args.fetch("overwrite", true)

existing = nil
model.pages.each { |p| existing = p if p.name == name }

overwritten = false
if existing
  unless overwrite
    return { "ok" => false, "error" => "scene '#{name}' exists; pass overwrite=true to replace" }
  end
  model.pages.erase(existing)
  overwritten = true
end

new_page = model.pages.add(name)
return { "ok" => false, "error" => "Pages#add returned nil" } unless new_page

view = model.active_view
cam  = view.camera
{
  "ok"            => true,
  "scene_name"    => name,
  "overwritten"   => overwritten,
  "camera_eye"    => { "x" => cam.eye.x,    "y" => cam.eye.y,    "z" => cam.eye.z    },
  "camera_target" => { "x" => cam.target.x, "y" => cam.target.y, "z" => cam.target.z },
}
```

## Gotchas

- `Pages#add(name)` snapshots the CURRENT viewport state — camera, axes, shadows, layers, style. No way to selectively include/exclude properties via the Ruby API.
- `Pages#erase(page)` removes by reference. The new page goes to the end of the list (page index may differ from the old one).
- The scene's transition time + delay default to the model defaults. Set them after via `new_page.transition_time = x` if needed.

## See also

- [`scene.export-to-images`](./scene.export-to-images.md) — render saved scenes to PNG
- Compare to Rhino's [`snapshot.save-named-view`](../../../architecture/rhino-8/commands/snapshot.save-named-view.md)
