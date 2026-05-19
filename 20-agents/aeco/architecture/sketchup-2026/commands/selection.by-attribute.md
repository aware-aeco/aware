# selection.by-attribute

Select entities whose AttributeDictionary value matches an exact value or regex. The SketchUp analog of Rhino's `selection.by-attribute`.

## Inputs

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `dictionary` | string | yes | — | AttributeDictionary name (e.g. `"AWARE"`). |
| `key` | string | yes | — | Key within that dictionary. |
| `value` | string | one-of | — | Exact value to match (case-insensitive). |
| `pattern` | string | one-of | — | Ruby regex. |
| `add-to-selection` | boolean | no | `false` | When true, ADDS to current selection. |

## Outputs

```json
{ "matched-count": 47, "ids": [12345, 12346, ...] }
```

## Implementation (shipped through `aware-sketchup exec`)

```ruby
model      = Sketchup.active_model
dict_name  = args["dictionary"].to_s
key        = args["key"].to_s
exact      = args["value"]
pattern    = args["pattern"]
add_to     = args.fetch("add-to-selection", false)

if (exact.nil? || exact.to_s.empty?) && (pattern.nil? || pattern.to_s.empty?)
  return { "ok" => false, "error" => "supply value OR pattern" }
end

rx = pattern && !pattern.to_s.empty? ? Regexp.new(pattern.to_s, Regexp::IGNORECASE) : nil

model.selection.clear unless add_to

matched = []
model.active_entities.each do |e|
  next unless e.respond_to?(:attribute_dictionaries)
  dict = e.attribute_dictionaries && e.attribute_dictionaries[dict_name]
  next if dict.nil?
  stored = dict[key]
  next if stored.nil?
  is_match = rx ? rx.match?(stored.to_s) : stored.to_s.casecmp(exact.to_s).zero?
  if is_match
    model.selection.add(e)
    matched << e.entityID
  end
end
{ "ok" => true, "matched_count" => matched.size, "ids" => matched }
```

## Gotchas

- `attribute_dictionaries[name]` returns `nil` if the dictionary doesn't exist on the entity (not an empty dict). The script checks before keying.
- `model.selection.add` ignores entities that can't be selected (locked/hidden); `matched` still records them.
- Only top-level `active_entities` are scanned — entities inside groups/components aren't walked. Adjust the scope if you need that.

## See also

- [`attribute.bulk-write`](./attribute.bulk-write.md) — the write-side pair
- Compare to Rhino's [`selection.by-attribute`](../../../architecture/rhino-8/commands/selection.by-attribute.md)
