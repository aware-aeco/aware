---
name: navisworks-property-search-and-quantification
description: This skill should be used when composing snippets that read property data from federated items in Navisworks — quantification (counting items by IFC class / Revit category / cost code), property auditing, building schedule extracts, or any "count + sum + group" operation across federated geometry. Encodes the Property Category model, the typed VariantData values, the QTO / Quantification module's role, and the gotcha that property values are stored as strings even when they represent numbers (so naive comparisons fail).
---

# Navisworks property search and quantification

Federated models contain millions of items, each with dozens of properties. Property-driven analysis (counting walls by fire rating, summing concrete volume by level, etc.) is one of the most common coordination workflows. Done wrong, it's slow and inaccurate.

## The property model

Every `ModelItem` has multiple **PropertyCategories**. Categories are themed buckets — Item, Element, Geometry, Material, plus per-source-format categories like IFC PSets, Revit Phase, Tekla UDA.

```csharp
var item = ... ;
foreach (PropertyCategory cat in item.PropertyCategories)
{
    Console.WriteLine($"Category: {cat.DisplayName}");
    foreach (DataProperty prop in cat.Properties)
    {
        Console.WriteLine($"  {prop.DisplayName} = {prop.Value.ToDisplayString()}");
    }
}
```

Common categories you'll see across federations:
- **Item** — Navisworks-internal (Layer, Name, Type, GUID, Source File)
- **Element** — common element info (Name, Type, IfcGUID, etc.)
- **Geometry** — bounding box, area, volume (when computable)
- **Material** — applied material name + properties
- **IFC PSets** — one category per IFC PropertySet (Pset_WallCommon, Pset_DoorCommon, etc.)
- **Revit Phase** — Phase Created, Phase Demolished
- **User-defined** — anything imported plus AWARE-stamped via `attribute.bulk-write`

## Finding a property

```csharp
var item = ... ;

// By category name + property display name
var category = item.PropertyCategories.FindCategoryByName("Element");
var property = category?.Properties.FindPropertyByDisplayName("Name");
var displayValue = property?.Value.ToDisplayString();   // STRING — see gotcha below
```

`FindCategoryByName` is **case-sensitive**. Categories created by different importers have slightly different names ("Element" vs "Element ").

## The "everything is a string" gotcha

`property.Value` is a `VariantData` — but `.ToDisplayString()` always returns a string, even for numeric values. If you write naive C#:

```csharp
var areaStr = areaProperty.Value.ToDisplayString();   // "12.50 m²"
if (areaStr > "10.00") ...                              // STRING COMPARISON; wrong
```

You're comparing strings ("12.50 m²" > "10.00 m²" is alphabetical). To compare correctly, use typed accessors:

```csharp
if (areaProperty.Value.IsDouble)
{
    double area = areaProperty.Value.ToDouble();
    if (area > 10.0) ...
}
else if (areaProperty.Value.IsInt32)
{
    int n = areaProperty.Value.ToInt32();
}
else if (areaProperty.Value.IsDisplayString)
{
    // The value is a free-form string. Try to parse.
    if (double.TryParse(areaProperty.Value.ToDisplayString(), out var parsed))
        if (parsed > 10.0) ...
}
```

The IsXxx flags tell you what was stored. **Many imported properties are typed as DisplayString even when they look numeric** — the importer didn't infer the type. Defensive parsing is required.

## VariantData type detection

```csharp
public static double? ReadAsDouble(VariantData v)
{
    if (v.IsDouble) return v.ToDouble();
    if (v.IsInt32)  return v.ToInt32();
    if (v.IsDisplayString && double.TryParse(v.ToDisplayString(), out var d)) return d;
    return null;
}
```

This pattern is unavoidable for cross-vendor coordination work.

## Quantification module

Navisworks's separate **Quantification** module is a richer take on property-driven counting: it adds per-item quantity formulas (e.g. "concrete volume = length × cross-section area × height-modifier") and groups results into work breakdowns (WBS).

```csharp
// (API name varies — typically through the Quantification plugin)
var quantPlugin = doc.GetPlugins().FirstOrDefault(p => p.Name == "Quantification");
var workbook = quantPlugin?.GetActiveWorkbook();
// ... walk items, apply quantification formulas, aggregate ...
```

For most AECO use cases, the simpler `PropertyCategory` walk is enough — Quantification adds value when you need cost-loaded summaries.

## Counting by group

```csharp
var allItems = doc.Models.SelectMany(m => m.RootItem.DescendantsAndSelf);
var groups = allItems
    .Select(i => i.PropertyCategories.FindCategoryByName("Element")?.Properties.FindPropertyByDisplayName("Type")?.Value.ToDisplayString() ?? "Unknown")
    .GroupBy(t => t)
    .Select(g => new { type = g.Key, count = g.Count() })
    .OrderByDescending(g => g.count)
    .ToList();

return new { type_count = groups.Count, by_type = groups };
```

This is the canonical "count items by IFC type" workflow.

## Summing by group

```csharp
double TryReadDouble(DataProperty? p) {
    if (p == null) return 0;
    if (p.Value.IsDouble) return p.Value.ToDouble();
    if (double.TryParse(p.Value.ToDisplayString() ?? "", out var d)) return d;
    return 0;
}

var totalConcreteVolume = doc.Models
    .SelectMany(m => m.RootItem.DescendantsAndSelf)
    .Where(i => {
        var matName = i.PropertyCategories.FindCategoryByName("Material")?
            .Properties.FindPropertyByDisplayName("Name")?.Value.ToDisplayString() ?? "";
        return matName.StartsWith("Concrete");
    })
    .Sum(i => TryReadDouble(
        i.PropertyCategories.FindCategoryByName("Geometry")?
         .Properties.FindPropertyByDisplayName("Volume")));

return new { total_concrete_volume = totalConcreteVolume };
```

## Common gotchas

- **Property values can be NULL** (the category exists but the property is absent). Always null-check.
- **Same property name in different categories** is a common federation quirk. "Name" exists in Item, Element, Material — different categories return different values. Specify the category.
- **Unit suffixes are part of the string value.** "12.50 m²" parses as a double via `TryParse` ONLY if you strip the unit first. Use the typed accessor when available.
- **Federation can have inconsistent property names across source files.** Revit exports "Volume" in one project, "Total Volume" in another. Use Property Sets (Pset_*) for IFC consistency.
- **Property reads on millions of items are SLOW.** Filter the item set BEFORE reading properties — e.g., select walls first, then read their fire ratings, not the other way round.

## Standard pattern

```csharp
var doc = Application.ActiveDocument;
var categoryArg = args["category"]?.ToString();
var propertyArg = args["property"]?.ToString();

var groups = doc.Models
    .SelectMany(m => m.RootItem.DescendantsAndSelf)
    .Select(i => {
        var cat = i.PropertyCategories.FindCategoryByName(categoryArg);
        return cat?.Properties.FindPropertyByDisplayName(propertyArg)?.Value.ToDisplayString() ?? "(missing)";
    })
    .GroupBy(v => v)
    .OrderByDescending(g => g.Count())
    .Select(g => new { value = g.Key, count = g.Count() })
    .ToList();

return new {
    category = categoryArg,
    property = propertyArg,
    distinct_values = groups.Count,
    breakdown = groups,
};
```

## See also

- [selection-and-search-sets](./selection-and-search-sets.md) — selecting on these same properties
- [timeliner-and-4d](./timeliner-and-4d.md) — binding ActivityID property to scheduled tasks
- [appearance-profiler](./appearance-profiler.md) — coloring by these same property reads
