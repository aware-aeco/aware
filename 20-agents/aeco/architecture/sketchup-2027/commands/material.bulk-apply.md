# material.bulk-apply

Apply a material to every entity whose layer (tag) matches a regex. Creates the material from a CSS hex color if it doesn't exist.

`mode: write` — the composing app must declare a `safety:` block per v0.11.

## Inputs

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `from-pattern` | string | yes | — | Ruby regex matched against the entity's layer name. |
| `material-name` | string | yes | — | Material name. Created if missing. |
| `material-color` | string | no | `"#808080"` | CSS hex color (e.g. `"#FF8800"`). Used only when creating the material. |

## Outputs

```json
{ "material-name": "AWARE-Walls", "created": true, "applied-count": 47 }
```

## Implementation (shipped through `aware-sketchup exec`)

```ruby
model         = Sketchup.active_model
from_pattern  = Regexp.new(args["from-pattern"].to_s)
mat_name      = args["material-name"].to_s
mat_hex       = args.fetch("material-color", "#808080").to_s

# Resolve or create the target material.
target = model.materials[mat_name]
created = false
if target.nil?
  # Parse the hex string to RGB (#RRGGBB or #RRGGBBAA).
  hex = mat_hex.delete_prefix("#")
  r = hex[0..1].to_i(16)
  g = hex[2..3].to_i(16)
  b = hex[4..5].to_i(16)
  a = hex.length == 8 ? hex[6..7].to_i(16) : 255
  target = model.materials.add(mat_name)
  target.color = Sketchup::Color.new(r, g, b, a)
  created = true
end

# Group writes in a single undo step.
model.start_operation("AWARE: material.bulk-apply", true)
applied = 0
model.active_entities.each do |e|
  next unless e.respond_to?(:layer) && e.respond_to?(:material=)
  next unless e.layer && from_pattern.match?(e.layer.name)
  e.material = target
  applied += 1
end
model.commit_operation

{ "ok" => true, "material_name" => mat_name, "created" => created, "applied_count" => applied }
```

## Gotchas

- `start_operation` / `commit_operation` bracket the writes so the user can undo them as a single step (and so `model.abort_operation` rolls back if anything raises).
- Only top-level entities in `active_entities` are scanned. Entities inside groups/components keep their original materials.
- `material.color =` accepts a `Sketchup::Color` OR a CSS string OR a `Hash`. The code above uses the explicit Color for predictability.
- Alpha (transparency) is supported via 8-char hex (`#RRGGBBAA`); 6-char hex is treated as opaque.

## See also

- [`entity.list-by-tag`](./entity.list-by-tag.md) — common read-side pairing
- `model.materials.purge_unused` — cleanup after large mass-changes
