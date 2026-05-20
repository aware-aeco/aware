# group.flatten

Explode top-level selected groups one level deep, leaving their contents in the parent context. Useful for cleaning over-grouped imports. Optionally recursive (until no groups remain).

`mode: write` — the composing app must declare a `safety:` block per v0.11.

## Inputs

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `recursive` | boolean | no | `false` | When true, keep exploding until no groups remain in the selection. |
| `include-component-instances` | boolean | no | `false` | When true, also explode component instances (destructive — loses re-use semantics). |

## Outputs

```json
{ "exploded-count": 14, "passes": 1 }
```

## Implementation (shipped through `aware-sketchup exec`)

```ruby
model      = Sketchup.active_model
recursive  = args.fetch("recursive", false)
incl_comps = args.fetch("include-component-instances", false)

# Classify "explodable" — Groups always; ComponentInstance only when opted in.
explodable = ->(e) {
  e.is_a?(Sketchup::Group) || (incl_comps && e.is_a?(Sketchup::ComponentInstance))
}

model.start_operation("AWARE: group.flatten", true)
total = 0
passes = 0
loop do
  passes += 1
  targets = model.selection.select(&explodable)
  break if targets.empty?
  exploded_this_pass = 0
  targets.each do |t|
    children = t.explode
    if children.is_a?(Array)
      # Add newly-exposed groups to the selection so the next pass picks them up.
      model.selection.add(children.select(&explodable)) if recursive
      exploded_this_pass += 1
    end
  end
  total += exploded_this_pass
  break unless recursive
  break if exploded_this_pass == 0
end
model.commit_operation

{ "ok" => true, "exploded_count" => total, "passes" => passes }
```

## Gotchas

- `Sketchup::Group#explode` returns an Array of the new top-level entities OR `false` if the group couldn't be exploded. The script treats `false` as "skip".
- `recursive: true` modifies the selection mid-loop. The lambda re-evaluates on each pass against the (now-larger) selection.
- `include-component-instances: true` is destructive — the component definition stays in the model (use `definitions.purge_unused` to clean up), but ALL instances of that definition lose their re-use link. Use with care.
- Locked groups can't be exploded; they're skipped (`explode` returns false).

## See also

- [`entity.list-by-tag`](./entity.list-by-tag.md) — find groups by tag first
- [`component.list-instances`](./component.list-instances.md) — audit instances before exploding
