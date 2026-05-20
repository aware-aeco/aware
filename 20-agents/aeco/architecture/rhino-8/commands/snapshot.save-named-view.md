# snapshot.save-named-view

Save the current active viewport (camera + display mode) as a named view. The "capture this perspective for later" primitive.

`mode: write` — modifies the doc's named-view list. Apps should declare safety: even though no geometry changes.

## Inputs

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `view-name` | string | yes | — | Name for the new named view. |
| `overwrite` | boolean | no | `true` | If `false` and the name exists, error out instead of overwriting. |

## Outputs

```json
{
  "view-name":   "Lobby-NE",
  "overwritten": true,
  "camera-location": { "x": -5000, "y": -8000, "z": 1800 },
  "camera-target":   { "x":     0, "y":     0, "z":  900 }
}
```

## Implementation (shipped through `aware-rhino exec`)

```csharp
var doc = Rhino.RhinoDoc.ActiveDoc;
var name      = args["view-name"]!.ToString();
bool overwrite = !args.TryGetValue("overwrite", out var o) || Convert.ToBoolean(o);

var active = doc.Views.ActiveView;
if (active == null) return new { ok = false, error = "no active view" };

// Detect existing named view of the same name.
int existingIndex = -1;
for (int i = 0; i < doc.NamedViews.Count; i++)
{
    if (string.Equals(doc.NamedViews[i].Name, name, StringComparison.OrdinalIgnoreCase))
    {
        existingIndex = i;
        break;
    }
}

bool overwritten = false;
if (existingIndex >= 0)
{
    if (!overwrite) return new { ok = false, error = $"named view '{name}' exists; pass overwrite=true to replace" };
    doc.NamedViews.Delete(existingIndex);
    overwritten = true;
}

// Add the view. NamedViews.Add returns the new index; -1 on failure.
int newIdx = doc.NamedViews.Add(name, active.ActiveViewportID);
if (newIdx < 0)
    return new { ok = false, error = "NamedViews.Add returned -1 (active view may not be saveable)" };

var vp  = active.ActiveViewport;
var loc = vp.CameraLocation;
var tgt = vp.CameraTarget;
return new {
    ok = true,
    view_name       = name,
    overwritten     = overwritten,
    camera_location = new { x = loc.X, y = loc.Y, z = loc.Z },
    camera_target   = new { x = tgt.X, y = tgt.Y, z = tgt.Z },
};
```

## Gotchas

- Capture is from the ACTIVE viewport at call time. If the caller wants to capture a specific viewport other than the active one, switch via `doc.Views.ActiveView = <view>` first.
- The named view records camera + display mode + viewport state at the moment of `Add`. Subsequent geometry edits don't change the saved camera.
- Overwriting deletes by index, then adds with the same name; the new index may differ from the old one. The script returns the camera shot, not the index.

## See also

- [`view.capture-bitmap`](./view.capture-bitmap.md) — render the named view to PNG
- `_-NamedView` (Rhino command) — interactive equivalent
