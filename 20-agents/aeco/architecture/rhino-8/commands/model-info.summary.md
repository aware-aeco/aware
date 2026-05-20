# model-info.summary

Compact model summary: object counts by class, layer count, block-definition count, units, bounding box, named-view count. The "what's in this 3dm" one-shot inventory.

## Inputs

None.

## Outputs

```json
{
  "units":                  "Millimeters",
  "object-count":           1247,
  "objects-by-class":       { "Curve": 412, "Brep": 631, "Mesh": 88, "InstanceObject": 116 },
  "layer-count":            87,
  "block-definition-count": 14,
  "named-view-count":       6,
  "bounding-box": {
    "min": { "x": -5000, "y": -3000, "z":    0 },
    "max": { "x":  8000, "y":  4500, "z": 3000 }
  }
}
```

## Implementation (shipped through `aware-rhino exec`)

```csharp
var doc = Rhino.RhinoDoc.ActiveDoc;

// Object counts by Geometry concrete type name.
var counts = new Dictionary<string, int>();
int total = 0;
var bbox = Rhino.Geometry.BoundingBox.Empty;
foreach (var obj in doc.Objects)
{
    total++;
    var name = obj.Geometry?.GetType().Name ?? "Unknown";
    counts[name] = (counts.TryGetValue(name, out var c) ? c : 0) + 1;
    if (!obj.IsHidden)
        bbox.Union(obj.Geometry.GetBoundingBox(true));
}

return new {
    ok                       = true,
    units                    = doc.ModelUnitSystem.ToString(),
    object_count             = total,
    objects_by_class         = counts,
    layer_count              = doc.Layers.Count,
    block_definition_count   = doc.InstanceDefinitions.Count,
    named_view_count         = doc.NamedViews.Count,
    bounding_box = bbox.IsValid ? (object)new {
        min = new { x = bbox.Min.X, y = bbox.Min.Y, z = bbox.Min.Z },
        max = new { x = bbox.Max.X, y = bbox.Max.Y, z = bbox.Max.Z },
    } : null,
};
```

## Gotchas

- Bounding box excludes hidden objects (matches what a user "sees"). Set `obj.IsHidden` to false beforehand if you want the all-inclusive bbox.
- `objects_by_class` uses .NET class names (e.g. `Brep`, `Mesh`, `InstanceObject`) — not user-facing terms.
- Block-definition count includes both used and unused definitions. Use `doc.InstanceDefinitions.Count(d => d.InUse(0))` for in-use only.

## See also

- [`block.list-instances`](./block.list-instances.md) — drill into a specific block
- [`view.capture-bitmap`](./view.capture-bitmap.md) — pair model-info with a thumbnail in reports
