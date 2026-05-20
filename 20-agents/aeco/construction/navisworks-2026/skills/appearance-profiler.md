---
name: navisworks-appearance-profiler
description: This skill should be used when composing snippets that color items in a federated Navisworks model by property — for QA visualization, work-package highlighting, design-discipline-by-color renders, or any "color by X" requirement. Encodes the AppearanceProfile rule model (property → color mapping), the priority resolution order (last-wins for overlapping rules), the difference between Appearance Profiler and Item Material override, and the gotcha that profile changes don't persist to .nwd unless explicitly saved.
---

# Navisworks Appearance Profiler

Appearance Profiler is the "color my model by property X" tool. Define a rule: "Items where ActivityID starts with 'STR-' get red"; apply; the model recolors. Every coordinator uses this; the AWARE substrate should expose it as a curated workflow.

## The rule model

| Concept | What |
|---|---|
| **Profile** | A named collection of rules. Multiple profiles can be saved per document. |
| **Rule** | One mapping: condition (property test) → color + transparency |
| **Selection target** | The set of items the rule applies to (Selection Set or Search Set or explicit) |
| **Priority** | Order of evaluation; LAST matching rule wins |
| **Reset** | "Clear all overrides" — restore original material |

## Creating a profile programmatically

The Appearance Profiler API is partial; most operations route through the COM bridge or the UI command:

```csharp
// (Pseudocode; specific class names vary across Navisworks versions)
using Autodesk.Navisworks.Api.ComApi;

var doc = Autodesk.Navisworks.Api.Application.ActiveDocument;

// 1. Define rule via a Search condition
var searchSet = doc.SelectionSets.ResolveReference(searchSetRef) as SelectionSet;
var color = new Color(255, 100, 100);   // red

// 2. Apply override
doc.Models.OverridePermanentColor(searchSet.Items, color);
doc.Models.OverridePermanentTransparency(searchSet.Items, 0.0);   // 0 = opaque, 1 = invisible
```

The high-level "Profile" wrapper saves these as a named set; the low-level `Models.OverrideXxx` methods are the actual overrides.

## Reset all overrides

```csharp
doc.Models.ResetAllPermanentMaterials();
doc.Models.ResetAllPermanentTransparency();
```

Resetting affects ALL items in the document. There's no per-Selection reset.

## Persistence

Material overrides are stored in the .nwf as part of the saved viewpoint or as document-level overrides. Saving via `doc.SaveFile(path)` persists overrides. Saving as .nwd freezes the overrides as the embedded geometry's appearance.

Critical: overrides on a SAVED Selection Set follow the snapshot — items added/removed from source models won't auto-pick up the override.

## Three override layers

When an item appears in multiple rules, the resolution order is (last-wins per layer):

1. **Source material** — from the original file (Revit material, IFC color, etc.)
2. **Permanent overrides** — Appearance Profiler / scripted overrides
3. **Temporary overrides** — Color-by-distance, clash-color, simulation-color (these CLEAR on cancel)
4. **Selection highlight** — current-selection color (usually yellow); takes precedence over everything

A `ResetAllPermanentMaterials` does NOT clear temporary overrides; the user must dismiss clash detection / simulation first.

## Common gotchas

- **`OverridePermanentColor` is per-`ModelItem`, not per-rule.** Once applied, the overridden state outlives the Selection Set that defined it.
- **Search Sets recolor lazily.** Changing the Search Set's condition doesn't re-apply the override automatically — the user has to re-run the profile.
- **Cross-discipline profiles compose unpredictably.** Profile A colors by-trade; Profile B colors by-phase. Applying both means Profile B's last write wins on overlaps. Use single comprehensive profiles, not chained.
- **Appearance Profiler doesn't see transparency from materials.** Setting transparency through `OverridePermanentTransparency` overrides the source material's alpha but doesn't blend cleanly with pre-existing translucent materials.

## Standard pattern for AWARE curated 'color-by-property' verb

```csharp
var doc = Autodesk.Navisworks.Api.Application.ActiveDocument;
var propertyName = args["property"].ToString();
var colorMap = args["color-map"] as IDictionary<string, object>;   // { "STR-": "#FF0000", "MEP-": "#00AAFF" }

var allItems = doc.Models.SelectMany(m => m.RootItem.DescendantsAndSelf).ToList();

doc.Models.ResetAllPermanentMaterials();

int touched = 0;
foreach (var item in allItems)
{
    var value = item.PropertyCategories
        .FindCategoryByName("Element")?
        .Properties.FindPropertyByDisplayName(propertyName)?
        .Value.ToDisplayString();

    if (string.IsNullOrEmpty(value)) continue;
    foreach (var kvp in colorMap)
    {
        if (value.StartsWith(kvp.Key.ToString()))
        {
            var hex = kvp.Value.ToString();
            var c = System.Drawing.ColorTranslator.FromHtml(hex);
            var nwColor = new Color(c.R / 255.0, c.G / 255.0, c.B / 255.0);
            doc.Models.OverridePermanentColor(new[] { item }, nwColor);
            touched++;
            break;
        }
    }
}

return new { items_colored = touched };
```

## See also

- [selection-and-search-sets](./selection-and-search-sets.md) — the upstream filter
- [viewpoints-and-markup](./viewpoints-and-markup.md) — saving the colored view as a viewpoint
