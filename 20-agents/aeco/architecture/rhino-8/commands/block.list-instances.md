# block.list-instances

Enumerate every `InstanceObject` for a given block definition (by name OR id), returning location, scale, and rotation. The block-audit primitive every BIM coordinator reaches for.

## Inputs

Exactly one of `definition-name` or `definition-id` must be supplied.

| Field | Type | Required | Description |
|---|---|---|---|
| `definition-name` | string | one-of | Block definition name (exact, case-insensitive). |
| `definition-id` | string | one-of | Block definition GUID. |

## Outputs

```json
{
  "definition-name": "DoorSingle",
  "definition-id":   "8b3e...d2",
  "instance-count":  142,
  "instances": [
    { "id": "abc...", "position": { "x": 1000.0, "y": 2500.0, "z": 0.0 }, "rotation": 90.0, "scale": { "x": 1, "y": 1, "z": 1 } },
    ...
  ]
}
```

## Implementation (shipped through `aware-rhino exec`)

```csharp
var doc = Rhino.RhinoDoc.ActiveDoc;
var defName = args.TryGetValue("definition-name", out var n) ? n?.ToString() : null;
var defIdRaw = args.TryGetValue("definition-id",   out var i) ? i?.ToString() : null;

Rhino.DocObjects.InstanceDefinition? idef = null;
if (!string.IsNullOrEmpty(defName))
{
    foreach (var d in doc.InstanceDefinitions)
    {
        if (d != null && string.Equals(d.Name, defName, StringComparison.OrdinalIgnoreCase))
        {
            idef = d;
            break;
        }
    }
}
else if (!string.IsNullOrEmpty(defIdRaw) && Guid.TryParse(defIdRaw, out var defId))
{
    foreach (var d in doc.InstanceDefinitions)
    {
        if (d != null && d.Id == defId) { idef = d; break; }
    }
}
else
{
    return new { ok = false, error = "supply definition-name or definition-id" };
}
if (idef == null) return new { ok = false, error = "no matching block definition in active doc" };

var rows = new List<object>();
foreach (var obj in doc.Objects)
{
    if (obj is not Rhino.DocObjects.InstanceObject inst) continue;
    if (inst.InstanceDefinition.Id != idef.Id) continue;
    var xform = inst.InstanceXform;
    var origin = new Rhino.Geometry.Point3d(xform.M03, xform.M13, xform.M23);
    // Scale extracted from the diagonal lengths of the upper-left 3x3.
    double sx = Math.Sqrt(xform.M00 * xform.M00 + xform.M10 * xform.M10 + xform.M20 * xform.M20);
    double sy = Math.Sqrt(xform.M01 * xform.M01 + xform.M11 * xform.M11 + xform.M21 * xform.M21);
    double sz = Math.Sqrt(xform.M02 * xform.M02 + xform.M12 * xform.M12 + xform.M22 * xform.M22);
    // Rotation about Z, atan2 of the normalized first-column XY components.
    double rotZ = Math.Atan2(xform.M10 / Math.Max(sx, 1e-9), xform.M00 / Math.Max(sx, 1e-9)) * 180.0 / Math.PI;
    rows.Add(new {
        id = inst.Id.ToString(),
        position = new { x = origin.X, y = origin.Y, z = origin.Z },
        rotation = Math.Round(rotZ, 4),
        scale    = new { x = Math.Round(sx, 6), y = Math.Round(sy, 6), z = Math.Round(sz, 6) },
    });
}
return new {
    ok = true,
    definition_name = idef.Name,
    definition_id   = idef.Id.ToString(),
    instance_count  = rows.Count,
    instances       = rows,
};
```

## Gotchas

- `rotation` is extracted as Euler-Z only. Non-axis-aligned rotations are approximated.
- Scale extraction assumes orthogonal axes. Sheared transforms produce non-physical numbers.
- Nested block definitions (blocks-inside-blocks) are not unrolled — only top-level instances of the queried definition are returned.
- For very large models (>10k instances), serialize-on-output may exceed `MaxDepth=6`. Project to fewer columns if needed.

## See also

- `_-BlockManager` (Rhino command) — interactive equivalent
- [`layer.set-by-pattern`](./layer.set-by-pattern.md) — bulk-attribute write follow-up
