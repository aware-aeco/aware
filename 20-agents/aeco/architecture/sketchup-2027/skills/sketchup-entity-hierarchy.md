---
name: sketchup-entity-hierarchy
description: This skill should be used when composing snippets that touch SketchUp's entity model — anything involving Groups, Components, ComponentDefinitions, or the relationship between `Sketchup::Entities` collections. Encodes the difference between Groups (one-off bundles, exploded freely) and Components (definition-backed, instances share a template), the role of the ComponentDefinition (template lives in `model.definitions`, instances live in entity collections), nested-entity walking (`definition.entities` vs `active_entities`), and the SketchUp-specific gotcha that groups are SECRETLY components-with-anonymous-definitions. Critical for any app that audits, walks, or restructures model geometry.
---

# SketchUp entity hierarchy

SketchUp's geometry model has three closely-related container types: **Groups**, **ComponentInstances**, and **ComponentDefinitions**. They look similar in the UI but have very different semantics. Get them wrong and you either duplicate geometry needlessly OR delete things people care about.

## The three containers

| Type | Class | What it is | Sharing |
|---|---|---|---|
| Group | `Sketchup::Group` | One-off bundle of entities — like a "lite" component | NEVER shared — each group has its own internal entities |
| ComponentInstance | `Sketchup::ComponentInstance` | A placement of a `ComponentDefinition` at a transform | Multiple instances share one definition |
| ComponentDefinition | `Sketchup::ComponentDefinition` | The template that instances reference | Lives in `model.definitions`; not directly placed in the model |

**The secret:** `Sketchup::Group` is actually a ComponentInstance behind the scenes, with a hidden ComponentDefinition that's marked as anonymous (`definition.group? == true`). The Ruby API exposes a different class, but the storage model is identical.

## The two `entities` collections

| Property | What it returns |
|---|---|
| `model.entities` (alias `model.active_entities`) | The currently-open entity collection — top-level by default, but switches when the user enters a Group/Component edit context |
| `model.active_entities` | Same as `model.entities` — the editable context |
| `definition.entities` | The entities INSIDE the definition (i.e. what one instance contains) |
| `group.entities` / `instance.definition.entities` | Same thing — the group's contents OR the instance's template contents |

**Walking the entire model** is therefore recursive:

```ruby
def walk_all(entities, depth = 0)
  entities.each do |e|
    yield e, depth
    case e
    when Sketchup::Group
      walk_all(e.entities, depth + 1) { |child, d| yield child, d }
    when Sketchup::ComponentInstance
      walk_all(e.definition.entities, depth + 1) { |child, d| yield child, d }
    end
  end
end
```

Note that an `each` on a ComponentInstance's definition is shared — if 100 instances reference the same definition, the inner entities exist ONCE in storage but are conceptually present 100 times.

## Auditing instance count

`definition.count_used_instances` returns the number of in-model placements. `definition.instances` (where available) returns the Array.

`definition.count_used_instances == 0` for an in-use group's anonymous definition is a known quirk — use `definition.group?` to detect group-defs and skip them.

## Editing a definition vs editing an instance

- **Edit a definition** (`definition.entities.add_face(...)`) → ALL instances of that definition update visually.
- **Edit an instance** (`instance.move!(transform)`) → only that instance moves.
- **Explode an instance** (`instance.explode`) → instance's entities are pasted into the parent entity collection, instance reference removed, definition unchanged.
- **Explode a group** (`group.explode`) → same as explode-instance + the anonymous group-def is purged (since nothing else references it).

## Common gotchas

- **`active_entities` switches when the user enters edit-context.** A script that calls `model.active_entities` after the user double-clicks into a group sees the GROUP's contents, not the top-level. Use `model.entities` (which is an alias but doesn't track edit-context in all SketchUp versions) OR pin to `model.entities` at the start of the script.
- **`group.entities` returns the same entities each call.** Edits are immediately visible.
- **ComponentDefinition GUID is stable** across saves; instance entityID is NOT.
- **A definition with `count_used_instances == 0` AND `definition.group? == false` is purge-candidate.** Use `model.definitions.purge_unused` to clean.
- **Nested groups inside components inside groups inside components** is allowed — recursion depth has no architectural limit, only practical one.

## When to use each

- **Group** — Quick one-off bundling for selection / move-as-one. No re-use intent.
- **Component** — Anything that repeats: doors, windows, furniture, equipment. Edit-once-update-everywhere.
- **Naked entities** (Edges, Faces directly in `model.entities`) — The actual geometry. Groups/Components are organizational layers above them.

## See also

- [`component.list-instances`](../commands/component.list-instances.md) — read-side audit
- [`group.flatten`](../commands/group.flatten.md) — destructive group-to-naked-entities conversion
- [`outliner.tree`](../commands/outliner.tree.md) — nested hierarchy view as JSON
