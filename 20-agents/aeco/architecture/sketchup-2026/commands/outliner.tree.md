# outliner.tree

Export the Outliner hierarchy (groups + component instances, nested) as a JSON tree. The "what does this model contain" structural view.

## Inputs

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `max-depth` | number | no | 8 | Cap on recursion depth. |

## Outputs

```json
{
  "node-count": 87,
  "tree": {
    "name":       "<model-root>",
    "class":      "Sketchup::Model",
    "children": [
      { "name": "Building", "class": "Sketchup::Group", "id": 12345, "children": [ ... ] },
      ...
    ]
  }
}
```

## Implementation (shipped through `aware-sketchup exec`)

```ruby
model     = Sketchup.active_model
max_depth = args.fetch("max-depth", 8).to_i

count = 0

walk = lambda do |entities, depth|
  return [] if depth > max_depth
  nodes = []
  entities.each do |e|
    if e.is_a?(Sketchup::Group)
      count += 1
      nodes << {
        "name"     => e.name.to_s.empty? ? "<unnamed group>" : e.name,
        "class"    => "Sketchup::Group",
        "id"       => e.entityID,
        "children" => walk.call(e.entities, depth + 1),
      }
    elsif e.is_a?(Sketchup::ComponentInstance)
      count += 1
      nodes << {
        "name"        => e.name.to_s.empty? ? e.definition.name : e.name,
        "definition"  => e.definition.name,
        "class"       => "Sketchup::ComponentInstance",
        "id"          => e.entityID,
        "children"    => walk.call(e.definition.entities, depth + 1),
      }
    end
  end
  nodes
end

root = {
  "name"     => model.title.to_s.empty? ? "<model-root>" : model.title,
  "class"    => "Sketchup::Model",
  "children" => walk.call(model.entities, 1),
}

{ "ok" => true, "node_count" => count, "tree" => root }
```

## Gotchas

- Only `Group` and `ComponentInstance` nodes appear in the tree — primitive entities (Edges, Faces, etc.) are excluded for readability.
- ComponentInstances expand through their definition; if two instances share a definition, both subtrees are recursed independently (same shape, different ids).
- `max_depth` truncates silently. Deep models without a depth cap can produce huge JSON output. The default of 8 covers ~99% of real models.
- `entityID` is a Ruby-runtime id, not persistent across sessions.

## See also

- [`component.list-instances`](./component.list-instances.md) — flat view of one definition
- [`model-info.summary`](./model-info.summary.md) — flat summary across the whole model
