# model-info.summary

Compact model summary: entity counts by class, tag count, component-definition count, material count, scene count, units, bounding box.

## Inputs

None.

## Outputs

```json
{
  "units":                       "Millimeters",
  "entity-count":                1247,
  "entities-by-class":           { "Sketchup::Edge": 412, "Sketchup::Face": 631, "Sketchup::Group": 64, "Sketchup::ComponentInstance": 116 },
  "tag-count":                   12,
  "component-definition-count":  14,
  "material-count":              22,
  "scene-count":                 5,
  "bounding-box": {
    "min": { "x": -5000, "y": -3000, "z":    0 },
    "max": { "x":  8000, "y":  4500, "z": 3000 }
  }
}
```

## Implementation (shipped through `aware-sketchup exec`)

```ruby
model = Sketchup.active_model

counts = Hash.new(0)
total  = 0
bb_min = nil
bb_max = nil
model.active_entities.each do |e|
  total += 1
  counts[e.class.name] += 1
  bb = e.respond_to?(:bounds) ? e.bounds : nil
  if bb
    bb_min = bb_min ? Geom::Point3d.new([bb_min.x, bb.min.x].min, [bb_min.y, bb.min.y].min, [bb_min.z, bb.min.z].min) : bb.min
    bb_max = bb_max ? Geom::Point3d.new([bb_max.x, bb.max.x].max, [bb_max.y, bb.max.y].max, [bb_max.z, bb.max.z].max) : bb.max
  end
end

# Units: model.options["UnitsOptions"]["LengthUnit"] gives an integer (0=Inches, 1=Feet, 2=Millimeters, 3=Centimeters, 4=Meters)
unit_map = ["Inches", "Feet", "Millimeters", "Centimeters", "Meters"]
unit_idx = model.options["UnitsOptions"]["LengthUnit"]
units    = unit_map[unit_idx] || "Unknown"

bbox_obj = bb_min && bb_max ? {
  "min" => { "x" => bb_min.x, "y" => bb_min.y, "z" => bb_min.z },
  "max" => { "x" => bb_max.x, "y" => bb_max.y, "z" => bb_max.z },
} : nil

{
  "ok"                          => true,
  "units"                       => units,
  "entity_count"                => total,
  "entities_by_class"           => counts.to_h,
  "tag_count"                   => model.layers.count,
  "component_definition_count"  => model.definitions.count,
  "material_count"              => model.materials.count,
  "scene_count"                 => model.pages.count,
  "bounding_box"                => bbox_obj,
}
```

## Gotchas

- Bounding box union accumulates only top-level `active_entities`. Entities inside groups/components are excluded.
- `entities_by_class` uses Ruby class names (e.g. `Sketchup::Edge`, `Sketchup::Face`).
- `component_definition_count` includes both used and unused definitions. Use `model.definitions.select { |d| d.count_used_instances > 0 }.size` for in-use only.

## See also

- [`outliner.tree`](./outliner.tree.md) — nested view (vs flat summary)
- Compare to Rhino's [`model-info.summary`](../../../architecture/rhino-8/commands/model-info.summary.md)
