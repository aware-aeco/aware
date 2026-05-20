# component.list-instances

Enumerate every instance of a component definition (by name OR definition GUID), returning location, scale, and rotation. The block-audit primitive for SketchUp.

## Inputs

Exactly one of `definition-name` or `definition-guid` must be supplied.

| Field | Type | Required | Description |
|---|---|---|---|
| `definition-name` | string | one-of | Component definition name (exact, case-insensitive). |
| `definition-guid` | string | one-of | Component definition GUID. |

## Outputs

```json
{
  "definition-name": "Window-1200x900",
  "definition-guid": "ABCD...",
  "instance-count":  142,
  "instances": [
    { "id": 12345, "position": { "x": 1000.0, "y": 2500.0, "z": 0.0 }, "rotation": 90.0, "scale": { "x": 1, "y": 1, "z": 1 } },
    ...
  ]
}
```

## Implementation (shipped through `aware-sketchup exec`)

```ruby
model = Sketchup.active_model
def_name = args["definition-name"]
def_guid = args["definition-guid"]

target = nil
if def_name && !def_name.to_s.empty?
  model.definitions.each do |d|
    if d.name.downcase == def_name.to_s.downcase
      target = d
      break
    end
  end
elsif def_guid && !def_guid.to_s.empty?
  model.definitions.each do |d|
    if d.guid == def_guid.to_s
      target = d
      break
    end
  end
else
  return { "ok" => false, "error" => "supply definition-name or definition-guid" }
end
return { "ok" => false, "error" => "no matching component definition" } unless target

rows = []
target.instances.each do |inst|
  tr = inst.transformation
  origin = tr.origin
  # Scale: norm of each axis column.
  xa = tr.xaxis
  ya = tr.yaxis
  za = tr.zaxis
  sx = Math.sqrt(xa.x**2 + xa.y**2 + xa.z**2)
  sy = Math.sqrt(ya.x**2 + ya.y**2 + ya.z**2)
  sz = Math.sqrt(za.x**2 + za.y**2 + za.z**2)
  # Rotation about Z, atan2 of normalized x-axis xy components.
  rot_deg = Math.atan2(xa.y / [sx, 1e-9].max, xa.x / [sx, 1e-9].max) * 180.0 / Math::PI

  rows << {
    "id"       => inst.entityID,
    "position" => { "x" => origin.x, "y" => origin.y, "z" => origin.z },
    "rotation" => rot_deg.round(4),
    "scale"    => { "x" => sx.round(6), "y" => sy.round(6), "z" => sz.round(6) },
  }
end

{
  "ok"              => true,
  "definition_name" => target.name,
  "definition_guid" => target.guid,
  "instance_count"  => rows.size,
  "instances"       => rows,
}
```

## Gotchas

- `rotation` is extracted as Euler-Z only. Non-axis-aligned rotations are approximated.
- Scale extraction assumes orthogonal axes. Sheared transforms produce non-physical numbers.
- Nested instances (instances-inside-other-component-definitions) are NOT unrolled — only top-level instances of the queried definition are returned.
- `entityID` is a Ruby-runtime id, not a persistent SketchUp model id. It changes between sessions.

## See also

- [`group.flatten`](./group.flatten.md) — destructive flatten after audit
- Compare to Rhino's [`block.list-instances`](../../../architecture/rhino-8/commands/block.list-instances.md) for the equivalent primitive
