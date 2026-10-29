# entity.list-by-tag

List entities on the active model filtered by their layer (which SketchUp 2020+ calls "tags"). Returns one row per entity with id, class, and bbox.

## Inputs

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `tag-name` | string | yes | — | Layer/tag name to match. Use `"*"` for all tags. |
| `visible-only` | boolean | no | `true` | When true, skips entities on hidden tags. |

## Outputs

```json
{
  "tag-name": "WALLS",
  "count":    142,
  "entities": [
    { "id": 12345, "class": "Sketchup::Group", "bbox": { "min": { "x": 0, "y": 0, "z": 0 }, "max": { "x": 1000, "y": 500, "z": 3000 } } },
    ...
  ]
}
```

## Implementation (shipped through `aware-sketchup exec`)

```ruby
model = Sketchup.active_model
tag_name      = args["tag-name"].to_s
visible_only  = args.fetch("visible-only", true)

# Resolve target layer (or :all)
target = nil
if tag_name == "*"
  target = :all
else
  model.layers.each { |l| target = l if l.name == tag_name }
  unless target
    return { "ok" => false, "error" => "no layer/tag named '#{tag_name}' in active model" }
  end
end

rows = []
model.active_entities.each do |e|
  next unless e.respond_to?(:layer)
  next if visible_only && e.layer && !e.layer.visible?
  if target == :all || e.layer == target
    bb = e.bounds
    rows << {
      "id"    => e.entityID,
      "class" => e.class.name,
      "bbox"  => {
        "min" => { "x" => bb.min.x, "y" => bb.min.y, "z" => bb.min.z },
        "max" => { "x" => bb.max.x, "y" => bb.max.y, "z" => bb.max.z },
      },
    }
  end
end

{ "ok" => true, "tag_name" => tag_name, "count" => rows.size, "entities" => rows }
```

## Gotchas

- SketchUp 2020+ renamed Layers to Tags in the UI but the Ruby API is still `model.layers`. Both terms refer to the same object.
- Only the top-level `model.active_entities` is scanned — entities inside groups/components are not enumerated. Recurse via `definition.entities` if you need that.
- `entityID` is a Ruby-runtime id, not a persistent SketchUp model id. It changes between sessions.
- For very large models (>10k matching entities), serialize-on-output may be slow; project to fewer columns or filter harder.

## See also

- [`component.list-instances`](./component.list-instances.md) — read-side component audit
- [`material.bulk-apply`](./material.bulk-apply.md) — common write-side pairing
