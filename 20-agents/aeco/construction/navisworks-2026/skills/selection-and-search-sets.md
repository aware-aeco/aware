---
name: navisworks-selection-and-search-sets
description: This skill should be used when composing snippets that select items in a federated Navisworks model — by class, by property, by appearance, by spatial filter, or any combination. Encodes Navisworks's distinctive Selection Sets vs Search Sets dichotomy (the key insight that differentiates it from Revit/Rhino selection), the `Search` / `SearchCondition` / `Find` patterns, the `ModelItem` hierarchy, and the gotcha that selection state is NOT persisted across sessions but Sets are.
---

# Navisworks selection and search sets

Navisworks's selection model is fundamentally different from Revit's or Rhino's because federated models can contain millions of items from a dozen source files. The vocabulary you must internalize:

| Concept | Class | Persists | Notes |
|---|---|---|---|
| Current Selection | `Document.CurrentSelection` | No — session-only | What's visually highlighted right now |
| Selection Set | `Document.SelectionSets` (a `SelectionSet`) | Yes — saved with the .nwd/.nwf | "Frozen" list of items: snapshot at save time. Doesn't update as items are added/removed. |
| Search Set | `Document.SelectionSets` (a `SavedItemReference` with `IsSearchSet == true`) | Yes — saved with the .nwd/.nwf | A SAVED QUERY: re-evaluates when accessed. Adapts to model changes. |

**This is the key insight:** "Selection Sets" in Navisworks are stale snapshots; "Search Sets" are live queries. Coordinators always prefer Search Sets because they survive model updates.

## The ModelItem hierarchy

Navisworks's `ModelItem` is the unit of selection. Items form a tree:

```
Document.Models[i].RootItem        — top-level container (one per loaded file)
├── ModelItem                       — file
│   ├── ModelItem                   — layer
│   │   ├── ModelItem               — IFC space
│   │   │   ├── ModelItem           — wall (leaf or has its own geometry children)
│   │   │   │   └── (geometry inside)
```

Walking the tree: `item.Children` (immediate) or `item.DescendantsAndSelf` (recursive).

## Selecting current selection programmatically

```csharp
using Autodesk.Navisworks.Api;

var doc = Autodesk.Navisworks.Api.Application.ActiveDocument;
var allItems = doc.Models[0].RootItem.DescendantsAndSelf;
var walls = allItems.Where(i => i.ClassDisplayName == "Wall");

doc.CurrentSelection.AddRange(walls);
```

Current selection is in-memory; closing the document loses it.

## Creating a Search Set

```csharp
using Autodesk.Navisworks.Api.ComApi;        // many Navisworks classes are COM-backed

// 1. Build the search condition
var search = new Search();
search.Selection.SelectAll();                 // start from everything

var condition = SearchCondition
    .HasPropertyByDisplayName("Item", "Layer")        // check Item/Layer
    .EqualValue(VariantData.FromDisplayString("WALLS")); // == "WALLS"
search.SearchConditions.Add(condition);

// 2. Save as a Search Set
var sset = new SavedItemReference(...);   // (API varies; the high-level wrapper is HiddenSelectionSet vs SelectionSet)
sset.Name = "All Walls";
sset.IsSearchSet = true;
doc.SelectionSets.AddCopy(sset);
```

The COM-backed older API and the modern Roamer .NET API differ; check the namespace (`Autodesk.Navisworks.Api` vs `Autodesk.Navisworks.Api.ComApi`).

## Querying by property

Properties on Navisworks items live in **PropertyCategories**. Each item has many categories (Item, Element, Geometry, Material, plus IFC PSets, plus custom):

```csharp
var item = ... ;
var category = item.PropertyCategories.FindCategoryByName("Element");
var nameProperty = category?.Properties.FindPropertyByDisplayName("Name");
var name = nameProperty?.Value.ToDisplayString();
```

For systematic filtering across millions of items, use `SearchCondition.HasPropertyByDisplayName(category, property).EqualValue(VariantData)` — the search engine indexes these for speed.

## Variant data

Property values come back as `VariantData` — a tagged union of all possible types (String, Double, Int, Bool, Date, NamedConstant, etc.). Construct one for comparison via:

```csharp
var v = VariantData.FromDisplayString("Concrete");
var n = VariantData.FromDouble(2500.0);
var b = VariantData.FromBoolean(true);
```

Match via `Equals(another)`. Don't compare across types — `FromDouble(100.0).Equals(FromInt32(100))` returns FALSE.

## Visibility and selection

Navisworks separates these concepts:

- **Visibility** — controlled per `ModelItem` via `item.IsHidden`. Affects rendering and clash detection.
- **Selection** — controlled by `Document.CurrentSelection`. Affects highlight color and tool-target.

A hidden item can still be selected (rare). A non-selected item is always shown if visible.

## Common gotchas

- **Search Sets evaluate LAZILY.** Accessing `sset.Items` on a newly-loaded model can take seconds for huge federations.
- **Selection Sets are NOT updated when source models change.** Reloading an .nwf with new geometry will leave the Selection Set referring to (possibly deleted) items by InternalName, which can crash on stale ref.
- **`ModelItem` references are session-scoped.** Save the model, close, reopen — the old `ModelItem` C# reference is invalid. Re-query.
- **The current selection is NOT a `SelectionSet`** — it's a `Selection` object. The two have similar interfaces but different identity.

## Standard pattern

```csharp
var doc = Autodesk.Navisworks.Api.Application.ActiveDocument;

// Find all items where IFC type == IfcWall
var walls = doc.Models
    .SelectMany(m => m.RootItem.DescendantsAndSelf)
    .Where(i => {
        var category = i.PropertyCategories.FindCategoryByName("Element");
        var ifcType = category?.Properties.FindPropertyByDisplayName("Type")?.Value.ToDisplayString();
        return ifcType == "IfcWall";
    })
    .ToList();

// Show count, then highlight them
return new {
    count = walls.Count,
    selected_now = (doc.CurrentSelection.AddRange(walls), walls.Count).Item2,
};
```

## See also

- [clash-detection-workflow](./clash-detection-workflow.md) — using sets to scope clash tests
- [federation-and-coordinates](./federation-and-coordinates.md) — origin issues for multi-source models
