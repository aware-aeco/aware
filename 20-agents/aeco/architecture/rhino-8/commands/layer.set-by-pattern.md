# layer.set-by-pattern

Move every object whose current layer matches a regex onto a target layer. Used for cleaning imported models (e.g., "anything starting with `_IFC_` → layer `IFC-IMPORTS`"). Creates the target layer if it doesn't exist.

## Inputs

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `from-pattern` | string | yes | — | .NET regex matched against the source layer's `FullPath`. |
| `to-layer` | string | yes | — | Target layer's `FullPath`. Created if missing. |
| `to-layer-color` | string | no | `"#808080"` | CSS hex color used only when creating the target layer. |

`mode: write` — this verb mutates the document. The composing app **must** declare a `safety:` block (transaction-group, snapshot, etc.) per v0.11.

## Outputs

```json
{ "target-layer-index": 12, "moved-count": 47, "created-target": true }
```

## Implementation (shipped through `aware-rhino exec`)

```csharp
var doc = Rhino.RhinoDoc.ActiveDoc;
var fromPattern = args["from-pattern"]!.ToString();
var toLayer     = args["to-layer"]!.ToString();
var toColorHex  = args.TryGetValue("to-layer-color", out var c) ? c?.ToString() ?? "#808080" : "#808080";

var rx = new System.Text.RegularExpressions.Regex(fromPattern,
            System.Text.RegularExpressions.RegexOptions.IgnoreCase);

// Resolve or create the target layer.
var existing = doc.Layers.FindName(toLayer);
bool created = false;
int targetIdx;
if (existing != null)
{
    targetIdx = existing.Index;
}
else
{
    var color = System.Drawing.ColorTranslator.FromHtml(toColorHex);
    var layer = new Rhino.DocObjects.Layer { Name = toLayer, Color = color };
    targetIdx = doc.Layers.Add(layer);
    created = targetIdx >= 0;
}
if (targetIdx < 0) return new { ok = false, error = $"could not create layer '{toLayer}'" };

// Walk every object; if its source-layer FullPath matches, move it.
int moved = 0;
foreach (var obj in doc.Objects)
{
    var sourceLayer = doc.Layers[obj.Attributes.LayerIndex];
    if (sourceLayer.Index == targetIdx) continue;  // already on target
    if (!rx.IsMatch(sourceLayer.FullPath)) continue;
    var attrs = obj.Attributes.Duplicate();
    attrs.LayerIndex = targetIdx;
    if (doc.Objects.ModifyAttributes(obj, attrs, false)) moved++;
}
doc.Views.Redraw();
return new { ok = true, target_layer_index = targetIdx, moved_count = moved, created_target = created };
```

## Gotchas

- Regex matches the layer's **full path** (e.g. `Walls::Exterior`), not just the leaf name. Anchor with `^` if you want a top-level-layer match.
- ModifyAttributes returns false if the object is locked or on a locked layer — the moved-count reflects successful moves only.
- The target layer's color is set only on creation. To recolor an existing layer, use a separate `_-Layer` macro.

## See also

- `_-ChangeToCurrentLayer` (Rhino command) — manual equivalent
- [`block.list-instances`](./block.list-instances.md) — read-side block audit
