---
name: revit-categories-and-families
description: This skill should be used when composing snippets that work with Revit's category system — picking elements by category, placing FamilyInstances, navigating Family-Symbol-Instance relationships, or interoperating with imported IFC models that map categories to IFC classes. Encodes the `BuiltInCategory` enum vs `Category` object, the OST_ prefix convention, the Family → FamilySymbol → FamilyInstance hierarchy, the gotcha that you must `Activate()` a FamilySymbol before placing an instance of it, and the relationship between Revit categories and IFC classes.
---

# Revit categories and families

Categories are Revit's top-level taxonomy — every element belongs to exactly one (Walls, Doors, Windows, Furniture, etc.). Families are the BIM equivalent of "parts catalogs" — externally-defined .rfa files containing parametric templates. Getting these wrong is the #1 source of "the script ran but nothing showed up in my model" bugs.

## BuiltInCategory vs Category

| Type | Use for |
|---|---|
| `BuiltInCategory` (enum) | Stable, locale-independent identifier. **Always use this for filters.** |
| `Category` (object) | Live model-aware object with Name, Id, IsSubcategory, etc. Use for display, hierarchy walking. |

```csharp
// Filter by category — ALWAYS use BuiltInCategory
var doors = new FilteredElementCollector(doc)
    .OfCategory(BuiltInCategory.OST_Doors)
    .WhereElementIsNotElementType()
    .ToList();

// Display category name — use Category
var category = element.Category;
return new { name = category.Name, id = category.Id.IntegerValue };
```

## The OST_ prefix

`BuiltInCategory` values start with `OST_` (Original SubType — a historical artifact from Revit's 1990s codebase). Always there, no spaces:

```
BuiltInCategory.OST_Walls
BuiltInCategory.OST_Doors
BuiltInCategory.OST_Windows
BuiltInCategory.OST_Floors
BuiltInCategory.OST_Roofs
BuiltInCategory.OST_StructuralFraming   // beams
BuiltInCategory.OST_StructuralColumns
BuiltInCategory.OST_Stairs
BuiltInCategory.OST_Railings
BuiltInCategory.OST_Levels
BuiltInCategory.OST_Grids
BuiltInCategory.OST_Sheets
BuiltInCategory.OST_Views
BuiltInCategory.OST_ProjectInformation
```

There are ~400 of them. The Revit API Docs is the canonical reference.

## Category hierarchy

Categories form a tree:

```
Walls
├── Walls               (the primary category)
├── Stacked Walls       (subcategory — different rendering)
└── Curtain Walls       (subcategory — composed of panels + mullions)

Furniture
├── Furniture
└── Furniture Systems   (subcategory)
```

Walk via `category.SubCategories`:

```csharp
var wallCat = Category.GetCategory(doc, BuiltInCategory.OST_Walls);
foreach (Category sub in wallCat.SubCategories)
{
    // sub.Name, sub.Id, sub.LineColor, sub.LineWeight, etc.
}
```

When filtering for "all walls including curtain walls", use the parent — Revit's collector descends into subcategories automatically.

## Family / Symbol / Instance — the three levels

| Level | Class | What it is |
|---|---|---|
| **Family** | `Family` | The .rfa file (e.g. "M_Single-Flush") loaded into the project |
| **FamilySymbol** | `FamilySymbol` | A variation within the family (e.g. "0762x2032mm"). Multiple symbols per family. Aka "type". |
| **FamilyInstance** | `FamilyInstance` | A placed copy in the model. Multiple instances per symbol. |

```csharp
var instance = doc.GetElement(elemId) as FamilyInstance;
var symbol   = instance.Symbol;         // FamilySymbol
var family   = symbol.Family;           // Family

// All instances of this exact symbol
var siblings = new FilteredElementCollector(doc)
    .OfClass(typeof(FamilyInstance))
    .Cast<FamilyInstance>()
    .Where(i => i.Symbol.Id == symbol.Id)
    .ToList();
```

## The Activate() gotcha

`FamilySymbol`s loaded from a family file are INACTIVE by default. You MUST call `symbol.Activate()` before placing an instance:

```csharp
using (var t = new Transaction(doc, "AWARE: place-family"))
{
    t.Start();

    var symbol = new FilteredElementCollector(doc)
        .OfClass(typeof(FamilySymbol))
        .OfCategory(BuiltInCategory.OST_Doors)
        .Cast<FamilySymbol>()
        .First(s => s.Family.Name == "M_Single-Flush" && s.Name == "0762x2032mm");

    if (!symbol.IsActive) symbol.Activate();   // CRITICAL — otherwise NewFamilyInstance returns null
    doc.Regenerate();                          // also commonly needed

    var instance = doc.Create.NewFamilyInstance(
        new XYZ(10, 5, 0),
        symbol,
        hostWall,
        Structure.StructuralType.NonStructural
    );

    t.Commit();
}
```

Without `Activate()`, `NewFamilyInstance` silently returns `null` — no exception, no warning. Just a null result. This catches everyone the first time.

## Categories ↔ IFC classes

| Revit category | IFC class |
|---|---|
| Walls | IfcWall, IfcCurtainWall |
| Doors | IfcDoor |
| Windows | IfcWindow |
| StructuralFraming | IfcBeam |
| StructuralColumns | IfcColumn |
| Floors | IfcSlab |
| Roofs | IfcRoof |
| Stairs | IfcStair |
| Railings | IfcRailing |
| MEP_Conduit | IfcCableCarrierSegment |
| (User-defined) | User-mapped via IFC settings dialog |

IFC export respects user-defined mappings in the IFC settings dialog. To override per-instance, set the `IfcExportAs` parameter on the element.

Importing an IFC file creates Revit elements categorized as `OST_<closest-match>` with a `IfcGUID` shared parameter populated. To find the IFC-origin of an imported element:

```csharp
var ifcGuidParam = element.LookupParameter("IfcGUID");
var ifcGuid = ifcGuidParam?.AsString();
```

## System families vs loadable families

Revit has THREE family categories that have different lifecycle rules:

| Family type | Examples | Editable | Loadable |
|---|---|---|---|
| System | Walls, Floors, Roofs, Pipes, Conduit | Type-properties only (not geometry) | No — built into Revit |
| Loadable | Doors, Windows, Furniture, Equipment | Full .rfa editing | Yes — `doc.LoadFamily()` |
| In-place | One-off Generic Models | Edit in-place | No — bound to one project |

Loadable families are what you `LoadFamily(path)` into a project from a .rfa file. System families have no .rfa equivalent; you create new types in the project itself.

## Standard pattern in `aware-revit exec` snippets

```csharp
var doc = uiapp.ActiveUIDocument.Document;
var categoryArg = args["category"].ToString();   // e.g. "OST_Doors"

if (!Enum.TryParse<BuiltInCategory>(categoryArg, out var bic))
    return new { ok = false, error = $"unknown category '{categoryArg}'" };

var elements = new FilteredElementCollector(doc)
    .OfCategory(bic)
    .WhereElementIsNotElementType()
    .Select(e => new {
        id          = e.Id.IntegerValue,
        name        = e.Name,
        family_name = (e is FamilyInstance fi) ? fi.Symbol.Family.Name : "",
        symbol_name = (e is FamilyInstance fi2) ? fi2.Symbol.Name : "",
    })
    .ToList();

return new { count = elements.Count, elements = elements };
```

## See also

- [revit-element-collector](./revit-element-collector.md) — querying by category
- [revit-transactions](./revit-transactions.md) — required for any placement
- [revit-parameters](./revit-parameters.md) — reading/writing element data
