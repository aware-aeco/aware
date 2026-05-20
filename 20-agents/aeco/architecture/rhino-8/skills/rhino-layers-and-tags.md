---
name: rhino-layers-and-tags
description: This skill should be used when composing snippets that read or write Rhino's layer hierarchy — adding a new layer, moving objects between layers, querying layer visibility/lock state, walking the parent/child tree, or interoperating with imported IFC models that use Layer prefixes as a category encoding. Covers `Layer.FullPath` (parent::child syntax) vs `Layer.Name`, the difference between layer color and the per-object plot/print weight, the "Locked" + "Hidden" semantics, the gotcha around the SketchUp/AutoCAD term "tag" (Rhino still calls them layers), and the AWARE curated `layer.set-by-pattern` verb's invariants.
---

# Rhino layers and tags

Layers are Rhino's first-class organizational mechanism. Every object belongs to exactly one layer; layers form a tree (parent / children); attributes inherit from layer to object unless explicitly overridden. Get this wrong and you ship a model where "the BIM team's filters all break".

## Layers vs tags

Rhino has **layers**. SketchUp 2020+ renamed its layers to **tags**. AutoCAD has both **layers** and **groups**. Don't confuse:

| Term | In Rhino API | Behavior |
|---|---|---|
| Layer | `Rhino.DocObjects.Layer` | Hierarchical (parent::child via `FullPath`), exactly one per object, inherits color / linetype / print-weight |
| Group | (not first-class) | Use `doc.Groups` — flat collection, an object can belong to multiple groups |
| Block | `InstanceDefinition` | See [`rhino-blocks-and-references.md`](./rhino-blocks-and-references.md) |

When an AI orchestrator says "tag the imported IFC walls as `IFC-WALL`", the actionable Rhino concept is **layer**, not group.

## FullPath syntax

Layers nest. The full path uses `::` as the separator:

```
Walls
Walls::Exterior
Walls::Exterior::Insulated
Walls::Interior
```

Properties:
- `layer.Name` is the LEAF name only (`Insulated`)
- `layer.FullPath` is the path from root (`Walls::Exterior::Insulated`)
- `layer.ParentLayerId` is the parent layer's GUID; `layer.GetChildren()` walks down

**Always match against `FullPath` for unambiguous targeting.** Matching by `Name` collides if two sub-layers share a leaf name.

## Common operations

```csharp
var doc = Rhino.RhinoDoc.ActiveDoc;

// Find or create a layer by FullPath
var existing = doc.Layers.FindName("Walls::Exterior");
int layerIdx;
if (existing != null)
{
    layerIdx = existing.Index;
}
else
{
    // Auto-creates parent layers if missing.
    layerIdx = doc.Layers.Add(new Rhino.DocObjects.Layer
    {
        Name = "Exterior",
        ParentLayerId = doc.Layers.FindName("Walls")?.Id ?? Guid.Empty,
        Color = System.Drawing.Color.Orange,
    });
}

// Move object to layer
var attrs = obj.Attributes.Duplicate();
attrs.LayerIndex = layerIdx;
doc.Objects.ModifyAttributes(obj, attrs, false);
```

The AWARE curated `layer.set-by-pattern` verb wraps this for bulk moves via regex against `FullPath`.

## Visibility and lock

`Layer` has THREE state knobs:

| Property | What it controls |
|---|---|
| `IsVisible` | Whether objects on the layer render in viewports. UI: layer-list eye icon. |
| `IsLocked` | Whether objects can be selected / edited. UI: layer-list padlock icon. |
| `IsExpanded` | Pure UI state for the layer-tree panel. Doesn't affect rendering. |

Objects can have per-object overrides (`obj.IsHidden`, `obj.IsLocked`) that win over the layer default. The `Layer.Visible` etc. reflect the layer-level state; query `obj.Visible` for the effective state.

`doc.Objects.ModifyAttributes` with `LayerIndex` change succeeds even on locked source/target layers (the API doesn't enforce the UI's protections). If you want UI-equivalent behavior, check `layer.IsLocked` first.

## Color / linetype / plot weight

Layers carry default `Color`, `LinetypeIndex`, `PlotColor`, `PlotWeight`, `RenderMaterialIndex`. Objects without per-object overrides inherit these.

| Where set | Wins when... |
|---|---|
| Layer default | Object's `ColorSource == FromLayer` (etc.) |
| Object override | Object's `ColorSource == FromObject` (etc.) |
| Block parent | Object's `ColorSource == FromParent` and object is inside a block |

For deterministic visualization, set per-object explicit values; for theme-able / re-colorable models, keep at `FromLayer`.

## Imported model conventions

IFC imports typically encode the IFC category as a layer prefix:

```
IFC-WALL::Walls::Exterior::Concrete-200mm
IFC-WALL::Walls::Interior::Stud-120mm
IFC-WINDOW::Doors_Windows::Standard
```

This makes `layer.set-by-pattern` extremely useful — `from-pattern: ^IFC-WALL::` collects every wall regardless of nested categorization.

AutoCAD imports often use layer-prefix conventions for line-weight (`__HIDDEN__::` etc.). DGN imports preserve MicroStation's level-color encoding.

## Gotchas

- **Layer indices are NOT stable across `doc.Layers.Delete` / re-create.** Resolve by GUID or FullPath, not by index, for cross-call durability.
- **Sub-layer visibility composes with parent.** A visible sub-layer under an invisible parent is still invisible. Check effective state via `layer.IsVisible && layer.ParentLayer == null || layer.ParentLayer.IsVisible`.
- **Color is a `System.Drawing.Color`, not a hex string.** Convert via `System.Drawing.ColorTranslator.FromHtml("#FF8800")` when reading from JSON.
- **Default layer is named `Default` and has index 0.** Can't be deleted.

## See also

- `_-Layer` (Rhino command) — interactive layer manager
- The `layer.set-by-pattern` curated verb in `commands/`
