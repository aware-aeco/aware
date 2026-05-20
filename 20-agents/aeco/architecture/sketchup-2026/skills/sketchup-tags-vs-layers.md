---
name: sketchup-tags-vs-layers
description: This skill should be used when composing snippets that read or write SketchUp's layer system — assigning entities to layers (tags), filtering by tag, walking tag folders, or interoperating with imported IFC/Revit models that use tag prefixes as a category encoding. Encodes the 2020+ UI rename (Layers → Tags) and the fact that the Ruby API still uses `Sketchup::Layers` / `Sketchup::Layer`, the difference between LAYER and PRIMARY-classification, tag folders (LayerFolder, post-2020), the gotcha that EVERY entity is on a tag (default "Untagged" if not set), and the AWARE curated `entity.list-by-tag` verb's invariants.
---

# SketchUp tags vs layers

In SketchUp 2020+ the UI calls them **tags**. The Ruby API still calls them **layers**. They are the same thing. This causes confusion in any cross-version codebase.

| Where | Term | API name |
|---|---|---|
| SketchUp 2020+ UI | "Tag" | `Sketchup::Layer` |
| SketchUp 2019 and earlier UI | "Layer" | `Sketchup::Layer` |
| Ruby API (all versions) | always "Layer" | `Sketchup::Layer` |
| IFC import nomenclature | "Tag" (mapped to IFC PSets) | `Sketchup::Layer` |

When an AI orchestrator says "tag the imported walls as `IFC-WALL`", the actionable Ruby concept is `model.layers.add("IFC-WALL")` + `entity.layer = the_layer`.

## What a layer/tag actually does

In SketchUp, **layers are visibility filters, not group containers**. Entities have a single `layer` property (`entity.layer = some_layer`); turning a layer off hides every entity assigned to it. Layers DO NOT contain entities the way Rhino's layers do — entities live in `entities` collections (groups, components, top-level), and the layer is just a tag for filtering.

This is fundamentally different from Rhino, AutoCAD, and Revit where layers ARE containers / organizational hierarchies.

## The default "Layer0" / "Untagged"

Every model has a special tag named `Layer0` (in 2019) or `Untagged` (in 2020+). It is the default for new entities. It cannot be deleted. The Ruby API exposes it as `model.layers[0]` or `model.layers["Untagged"]`.

## Tag folders (2020+)

SketchUp 2020 added tag folders for organization (`Sketchup::LayerFolder`). Tags can nest inside folders, but folders themselves don't filter visibility — they're purely UI. The Ruby API exposes them via `model.layers.folders` and `layer.folder`.

## Common operations

```ruby
model = Sketchup.active_model

# Find or create a tag
existing = model.layers["WALLS"]
target = existing || model.layers.add("WALLS")

# Assign every selected entity to that tag
model.selection.each do |e|
  e.layer = target  if e.respond_to?(:layer=)
end

# Toggle visibility
target.visible = false
```

## Visibility semantics

| Property | What it controls |
|---|---|
| `layer.visible?` | Whether entities on this layer render |
| `entity.hidden?` | Per-entity override (independent of layer state) |
| `entity.visible?` (in 2020+) | Effective state combining entity.hidden + layer.visible |

`layer.visible = false` doesn't change `entity.hidden`; the entity is just filtered out. Toggling the layer back on restores visibility unless the entity is independently hidden.

## Common gotchas

- **`entity.layer = nil` resets to the default tag (Untagged).** Don't use `nil` as "no tag" — it means "default tag".
- **Layer references are by object, not by name.** Renaming a layer keeps existing assignments intact; using a stale `Sketchup::Layer` object after deletion is undefined.
- **Imported IFC layers often use SketchUp-illegal characters.** SketchUp silently translates `/`, `:` etc. to `_`. Match imported tags by full sanitized name, not the raw IFC type.
- **`model.layers.add(name)` returns the EXISTING layer if name is already used** (doesn't error or create a duplicate). Idempotent.
- **Folder filtering is independent of tag filtering.** Collapsing a folder in the UI doesn't hide entities on its tags.

## IFC import conventions

Imported IFC models commonly use one tag per IFC class:

```
IFC-WALL
IFC-COLUMN
IFC-BEAM
IFC-WINDOW
IFC-DOOR
```

Mass-categorize imports via [`material.bulk-apply`](../commands/material.bulk-apply.md) with `from-pattern: "^IFC-"` or filter for audit via [`entity.list-by-tag`](../commands/entity.list-by-tag.md).

## See also

- [`entity.list-by-tag`](../commands/entity.list-by-tag.md) — read-side filter
- [`material.bulk-apply`](../commands/material.bulk-apply.md) — write-side bulk
- [`outliner.tree`](../commands/outliner.tree.md) — structural view (orthogonal to tags)
