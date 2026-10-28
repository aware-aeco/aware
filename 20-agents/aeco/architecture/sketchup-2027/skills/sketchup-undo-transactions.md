---
name: sketchup-undo-transactions
description: This skill should be used when composing snippets that mutate SketchUp's model state — adding / removing / modifying entities, materials, layers, components, or anything reachable from `Sketchup.active_model`. Encodes `start_operation` / `commit_operation` / `abort_operation` semantics, why every multi-step write MUST be wrapped in one operation (so the user gets single-undo), the `disable_ui` / `transparent` parameters and when each matters, the v0.11 AWARE safety-contract integration, and the failure mode where an unhandled raise mid-operation leaves the model in an inconsistent state.
---

# SketchUp undo and transactions

SketchUp's undo stack is **per-operation**, not per-API-call. Without `start_operation` / `commit_operation`, every single `entities.add_line` or `entity.material =` creates a separate undo entry — and the user has to press Ctrl+Z 47 times to undo your bulk script. With proper operation wrapping, one Ctrl+Z reverts the whole script. This is non-negotiable for professional UX.

## The three operation methods

```ruby
model = Sketchup.active_model

# Begin: name your operation (shown in the Edit -> Undo menu)
model.start_operation("AWARE: bulk material apply", true)
#                       └── name                    └── disable_ui (true = faster)

# Multiple writes here — all coalesce into one undo entry
model.entities.add_face(...)
selection.each { |e| e.material = mat }
# ...

# Commit: succeed
model.commit_operation

# OR abort: roll back everything inside this operation
model.abort_operation
```

If you `raise` between `start_operation` and `commit_operation` without calling `abort_operation`, the SketchUp UI's undo stack ends up in a confused state — the next user Ctrl+Z may revert too much or nothing at all. Always wrap in `begin / rescue / ensure`:

```ruby
model.start_operation("AWARE: <verb-name>", true)
begin
  # ... writes ...
  model.commit_operation
rescue => e
  model.abort_operation
  raise   # re-throw so AWARE's exec layer surfaces the error
end
```

## The four `start_operation` params

```ruby
model.start_operation(name, disable_ui, next_transparent, transparent)
```

| Param | Default | What it does |
|---|---|---|
| `name` | required | Shown in the Edit > Undo menu and in the v0.11 audit-stamp UDA. Use a stable verb-prefixed string (`"AWARE: layer.set-by-pattern"`) so the user can correlate. |
| `disable_ui` | false | When true, SketchUp suppresses viewport redraws during the operation. Massive speedup for bulk writes (>10× on large selections). **Pass true for any AWARE script that writes more than a handful of entities.** |
| `next_transparent` | false | The NEXT operation chains onto this one in the undo stack. Used by interactive tools that issue multiple sub-operations. AWARE scripts typically don't need this. |
| `transparent` | false | This operation chains onto the PREVIOUS one. Used to compose with a parent operation. AWARE scripts that compose `exec` calls within another exec can use this. |

For the canonical AWARE curated workflow verbs, use `start_operation("AWARE: <verb-name>", true)` and don't pass the chain params.

## Nesting

Nested `start_operation` calls (without `transparent`/`next_transparent`) silently become PART of the outer operation. There's no error, and `commit_operation` on the inner does nothing — only the outer commit actually fires. This means you can call other Ruby code (or other AWARE verbs from a composing script) inside an operation and the whole thing is one undo.

## Side-effects that DON'T undo

A few SketchUp operations cannot be undone, even inside `start_operation`:

- **File I/O** (`view.write_image`, `model.export(...)`) — already-written files stay on disk
- **HTTP requests** (`Sketchup::Http::Request#start`) — already-sent requests can't be unsent
- **External process spawns** (`system(...)`, `IO.popen(...)`)
- **Console prints** (`puts`, `print`, `Sketchup.send_to_console`)
- **Material library imports** (`materials.load(...)`)

The user pressing Ctrl+Z restores the model to its pre-operation state but the file/HTTP/etc side-effects persist.

## Relation to AWARE v0.11 safety contract

The v0.11 app-spec mandates that write-mode nodes declare a `safety:` block including a `transaction-group: <name>` field. The AWARE runtime is responsible for naming the operation when it ships the snippet to SketchUp — but the SCRIPT itself must ALSO wrap its writes in `start_operation` / `commit_operation`. Two layers of "undoability":

1. **AWARE layer** (`transaction-group: ...`) — coordinates multiple write nodes in an app's DAG; supports the `aware app rollback` command which replays inverse operations across the whole topology.
2. **SketchUp layer** (`model.start_operation(...)`) — coordinates multiple Ruby API calls within ONE exec invocation; supports the user's Ctrl+Z in the SketchUp UI.

Both are required for correct behavior. The AWARE layer doesn't make the SketchUp Ctrl+Z work without the in-script operation wrapping.

## Standard pattern for AWARE curated write verbs

```ruby
model = Sketchup.active_model
verb_name = "<the verb name>"   # e.g. "AWARE: material.bulk-apply"

model.start_operation(verb_name, true)
begin
  # ... user code body ...
  result = { ok: true, ... }
  model.commit_operation
  result
rescue => e
  model.abort_operation
  raise
end
```

## Performance gotcha

`disable_ui: true` skips viewport redraws BUT still raises Ruby observers. If you have heavyweight Ruby observers attached (e.g. an entity-add observer that does expensive work), they still fire on every write. Detach observers explicitly during bulk ops if profiling shows them dominating.

## See also

- [`attribute.bulk-write`](../commands/attribute.bulk-write.md) — uses this pattern for v0.11 audit-stamping
- [`material.bulk-apply`](../commands/material.bulk-apply.md) — uses this pattern for safe bulk writes
- [`group.flatten`](../commands/group.flatten.md) — multi-pass operation wrapped in one operation
