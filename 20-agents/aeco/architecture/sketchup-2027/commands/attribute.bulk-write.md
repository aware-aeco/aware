# attribute.bulk-write

Write multiple key/value pairs into a named AttributeDictionary on every selected entity. Creates the dictionary if missing. The v0.11 audit-stamp primitive for SketchUp.

`mode: write` — the composing app must declare a `safety:` block per v0.11.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `dictionary` | string | yes | AttributeDictionary name (e.g. `"AWARE"`). |
| `entries` | object | yes | Map of `key -> value` (both strings). |

## Outputs

```json
{ "entities-stamped": 47, "keys-written": 4 }
```

## Implementation (shipped through `aware-sketchup exec`)

```ruby
model     = Sketchup.active_model
dict_name = args["dictionary"].to_s
entries   = args["entries"]
unless entries.is_a?(Hash) && !entries.empty?
  return { "ok" => false, "error" => "entries must be a non-empty hash of string -> string" }
end

model.start_operation("AWARE: attribute.bulk-write", true)
stamped = 0
keys_written = 0
model.selection.each do |e|
  next unless e.respond_to?(:set_attribute)
  entries.each do |k, v|
    e.set_attribute(dict_name, k.to_s, v.to_s)
    keys_written += 1
  end
  stamped += 1
end
model.commit_operation

{ "ok" => true, "entities_stamped" => stamped, "keys_written" => keys_written }
```

## Gotchas

- `set_attribute` auto-creates the dictionary if missing — no need to call `attribute_dictionaries.add` first.
- Wrapped in `start_operation` / `commit_operation` so the user can undo all writes as a single step.
- Locked entities silently no-op. The `stamped` count still increments because the call doesn't raise.
- Setting `v` to `nil` deletes the key (per SketchUp's convention). The script converts to string explicitly to avoid surprises.

## See also

- [`selection.by-attribute`](./selection.by-attribute.md) — the read-side pair
- Compare to Rhino's [`user-text.bulk-write`](../../../architecture/rhino-8/commands/user-text.bulk-write.md)
