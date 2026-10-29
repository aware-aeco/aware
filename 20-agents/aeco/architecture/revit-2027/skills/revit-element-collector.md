---
name: revit-element-collector
description: This skill should be used when composing snippets that query the Revit model — finding elements by category/class/parameter/level, filtering FamilyInstances by symbol, walking nested elements, or building any read-side operation in a Revit `aware-revit exec` script. Encodes the `FilteredElementCollector` (the canonical query API), the difference between `OfClass<T>()` / `OfCategory(BuiltInCategory)` / quick-filter vs slow-filter ordering, common idioms (.NET LINQ-style chaining), and performance gotchas (always filter BEFORE materializing, never call `.ToList()` on an unfiltered collector).
---

# Revit FilteredElementCollector

`FilteredElementCollector` is the canonical query API for Revit elements. Almost every read-side operation routes through it. Using it wrong is the #1 source of "my script is slow" complaints from Revit plugin developers.

## The basic shape

```csharp
using Autodesk.Revit.DB;

var collector = new FilteredElementCollector(doc)
    .OfClass(typeof(Wall))                    // quick filter (uses Revit's index)
    .OfCategory(BuiltInCategory.OST_Walls);    // another quick filter

// At this point nothing has been computed yet — the collector is lazy.

var walls = collector.Cast<Wall>().ToList();   // materialize NOW
```

Two truths to internalise:
1. **The collector is lazy.** No work happens until you call `.ToList()`, `.First()`, `.Any()`, etc.
2. **Filter order matters for speed.** Quick filters use Revit's pre-built indexes (O(1) per index lookup); slow filters scan elements (O(n)). **Always apply quick filters first.**

## Quick filters vs slow filters

| Quick (indexed) | Slow (scan) |
|---|---|
| `OfCategory(BuiltInCategory)` | `WherePasses(ElementParameterFilter)` |
| `OfCategoryId(ElementId)` | `WherePasses(ElementClassFilter)` (overlapping with OfClass — but the IS-A relationship is the slow path) |
| `OfClass(Type)` | `Where(e => ...)` (LINQ — slowest, evaluates every element after collector returns) |
| `WhereElementIsViewIndependent()` / `IsElementType()` | `WhereParameter(...)` |
| `ContainedInDesignOption(id)` | Custom `Predicate<Element>` |

**Always chain quick filters FIRST, then slow filters.** A FilteredElementCollector that starts with `OfClass(typeof(Wall))` then `Where(w => w.WallType.Name == "Exterior")` is fast. The reverse is slow.

## Common idioms

### Get all elements of a category

```csharp
var doors = new FilteredElementCollector(doc)
    .OfCategory(BuiltInCategory.OST_Doors)
    .WhereElementIsNotElementType()    // exclude FamilySymbols (types), keep FamilyInstances
    .Cast<FamilyInstance>()
    .ToList();
```

### Get a specific element by name

```csharp
var wallType = new FilteredElementCollector(doc)
    .OfClass(typeof(WallType))
    .Cast<WallType>()
    .FirstOrDefault(wt => wt.Name == "Generic - 200mm");
```

### Get all elements on a level

```csharp
var levelId = level.Id;
var elementsOnLevel = new FilteredElementCollector(doc)
    .WhereElementIsNotElementType()
    .WhereElementIsViewIndependent()    // model elements only, not annotations
    .Where(e => {
        var levelParam = e.get_Parameter(BuiltInParameter.LEVEL_PARAM);
        return levelParam != null && levelParam.AsElementId() == levelId;
    })
    .ToList();
```

### Get all elements visible in a view

```csharp
var elementsInView = new FilteredElementCollector(doc, view.Id)
    .WhereElementIsNotElementType()
    .ToList();
```

Passing a `viewId` to the constructor returns only elements visible in that view — much faster than `IsHiddenInView` post-filtering.

### Find by GUID or unique-id

```csharp
// By internal id (within this session)
var elem = doc.GetElement(elementId);

// By unique id (stable across sessions)
var elemByUid = doc.GetElement(uniqueId);
```

`GetElement` is O(1) — don't use a collector for this.

## The `OfClass(typeof(T))` vs `T` mismatch

`OfClass(typeof(Wall))` returns instances of `Wall` AND its subclasses. There's no subclassing of `Wall` itself in stock Revit, but for hierarchical classes like `Element`, subclass matches are common.

`OfClass(typeof(Element))` is meaningless (returns everything). Always use the most specific class possible.

## WhereElementIsElementType vs IsNotElementType

Revit distinguishes:
- **ElementType** — the "template" (WallType, FloorType, FamilySymbol). Lives in the project but isn't placed.
- **Element** (non-type) — the placed instance (Wall, Floor, FamilyInstance).

```csharp
// All wall TYPES
var wallTypes = new FilteredElementCollector(doc)
    .OfClass(typeof(WallType))
    .Cast<WallType>();

// All placed walls
var walls = new FilteredElementCollector(doc)
    .OfClass(typeof(Wall))
    .WhereElementIsNotElementType()
    .Cast<Wall>();
```

If you don't specify `WhereElementIsElementType()` / `WhereElementIsNotElementType()`, the collector returns BOTH — almost never what you want.

## FamilyInstance / FamilySymbol gotcha

Loaded families have:
- A `Family` — the .rfa file (e.g. "M_Single-Flush")
- Multiple `FamilySymbol`s per Family — variations within the family ("0762x2032mm", "0813x2134mm")
- Multiple `FamilyInstance`s per Symbol — actual placements in the model

To find all instances of a specific family symbol:

```csharp
var doors = new FilteredElementCollector(doc)
    .OfClass(typeof(FamilyInstance))
    .OfCategory(BuiltInCategory.OST_Doors)
    .Cast<FamilyInstance>()
    .Where(d => d.Symbol.FamilyName == "M_Single-Flush" && d.Symbol.Name == "0762x2032mm")
    .ToList();
```

## Performance pitfalls

- **Don't iterate the same collector twice** — internally it caches, but the re-iteration cost is non-trivial. Materialize to a list once.
- **Don't materialize before filtering** — `new FilteredElementCollector(doc).ToList()` returns EVERY element in the doc. On a real model that's tens of thousands of elements.
- **Use `Any()` not `Count() > 0`** for existence checks — `Any()` short-circuits on the first match.
- **For large models, filter by `OfCategory` first, then by parameter** — the category index halves the search space immediately.

## Standard pattern in `aware-revit exec` snippets

```csharp
var doc = uiapp.ActiveUIDocument.Document;
var categoryArg = args.TryGetValue("category", out var c) ? c?.ToString() : null;

var collector = new FilteredElementCollector(doc)
    .WhereElementIsNotElementType();

if (!string.IsNullOrEmpty(categoryArg) && Enum.TryParse<BuiltInCategory>(categoryArg, out var bic))
    collector = collector.OfCategory(bic);

var rows = collector
    .Select(e => new {
        id       = e.Id.IntegerValue,
        name     = e.Name,
        category = e.Category?.Name ?? "",
    })
    .ToList();

return new { count = rows.Count, elements = rows };
```

## See also

- [revit-parameters](./revit-parameters.md) — accessing element parameters
- [revit-transactions](./revit-transactions.md) — writing back changes
- [`element.by-parameter-value` curated verb](../commands/element.by-parameter-value.md)
