# selection.by-property

Find every `ModelItem` whose property matches a value — the coordination-side equivalent of Revit's `element.by-parameter-value`. Read-only.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `document-id` | string | yes | The open federation. |
| `category` | string | no | Property category (e.g. `"Element"`, `"LcOaNode"`). |
| `property-name` | string | yes | Property display name (e.g. `"Material"`, `"Family"`). |
| `operator` | enum `equals\|contains\|is-null\|is-not-null` | no (default `equals`) | Match operator. |
| `value` | string | no | Value to match (for equals/contains). |

## Output

```yaml
items:
  - path:         "Tower.nwd/Level 3/Walls/Basic Wall:Exterior"
    display-name: "Basic Wall: Exterior"
```

## Implementation (Navisworks .NET API)

Use the `Search` API (in `Autodesk.Navisworks.Api`) rather than iterating items by hand:

```csharp
Search search = new Search();
search.Selection.SelectAll();                      // REQUIRED — default selection is empty
search.SearchConditions.Add(
    SearchCondition.HasPropertyByDisplayName(category, propertyName)
        .EqualValue(VariantData.FromDisplayString(value)));
ModelItemCollection items = search.FindAll(Application.ActiveDocument, false);
```

Each result `ModelItem` gives `DisplayName` and a path (its ancestor chain). Condition factories: `HasPropertyByDisplayName(catDisplay, propDisplay)` (UI labels) or `HasPropertyByName(internalCat, internalProp)` (internal ids like `LcRevitData`); values via `VariantData.FromDisplayString(...)`.

## Gotchas

- **You MUST call `search.Selection.SelectAll()`** (or set a scope) — the default `Search.Selection` is **empty**, so omitting it returns zero hits. This is the #1 gotcha.
- **`operator`: only `equals` is confirmed** (`.EqualValue`). The `contains` / `is-null` / `is-not-null` operator method names are **unverified** against the SDK reference (the product supports a contains/match-type and has-no-value, but the exact `SearchCondition` method names should be checked against the shipped `.chm` before implementing those three).
- **Internal vs display names differ** — `HasPropertyByName` takes internal ids (`LcRevitData` / `LcRevitPropertyElementCategory`); `HasPropertyByDisplayName` takes UI labels. Pick the right factory for the input.
- **Prefer `Search` over manual iteration** — walking `ModelItem`s is slow (managed↔unmanaged transitions); `Search.FindAll` / `FindIncremental` is the fast path.

Sources: [AU 2012 handout — `Search`/`SearchCondition`/`VariantData`/`FindAll`, empty-selection warning](https://static.au-uw2-prd.autodesk.com/CP2170_handout_2170_au_2012_class_navisworks_simon_bee.pdf) · [xiaodongliang property-database sample — `HasCategoryByDisplayName`/`FindAll`](https://github.com/xiaodongliang/Navisworks-Net-Plugin-Property-Database-Example/blob/master/NetPluginPropertyDatabaseExample/Class1.cs) · [Search ModelItem &amp; collect properties (twentytwo)](https://twentytwo.space/2020/05/28/navisworks-api-search-modelitem-and-collect-properties/)

## See also

- [selection-and-search-sets](../skills/selection-and-search-sets.md) — search sets, scopes
- [property-search-and-quantification](../skills/property-search-and-quantification.md) — property model + take-off
