---
name: autocad-selection-and-filters
description: This skill should be used when composing AutoCAD snippets that select entities programmatically — by layer, by type, by spatial filter, by XData / property, or by user-prompt. Encodes the Editor.SelectAll / SelectCrossingWindow / SelectFence / SelectImplied methods, the SelectionFilter (DxfCode-based filter list) for fast database-level filtering, the difference between selection-set IDs and entity object handles, and the gotcha that selection sets are session-only and become invalid across drawing close.
---

# AutoCAD selection and filters

AutoCAD has a richer programmatic-selection API than most CAD tools — selecting all walls on a given layer is one API call, not a manual iteration. Using these APIs correctly is the difference between a 100-ms script and a 30-second script.

## The Editor.Select methods

| Method | What it selects |
|---|---|
| `ed.SelectAll()` | All entities in model space (or current paper-space layout) |
| `ed.SelectAll(filter)` | All entities matching a filter |
| `ed.SelectCrossingWindow(p1, p2)` | Entities crossing a window |
| `ed.SelectWindow(p1, p2)` | Entities fully inside a window |
| `ed.SelectFence(points)` | Entities crossing a polyline fence |
| `ed.SelectImplied()` | The user's current pickset |
| `ed.GetSelection()` | PROMPTS the user to select; blocks until done |

For non-interactive AWARE scripts, `SelectAll` + filter is almost always the right starting point.

## SelectionFilter — DxfCode-based filtering

```csharp
using Autodesk.AutoCAD.EditorInput;
using Autodesk.AutoCAD.DatabaseServices;

// Filter: type=LINE AND layer="WALLS"
var filterList = new TypedValue[] {
    new TypedValue((int)DxfCode.Operator, "<and"),
        new TypedValue((int)DxfCode.Start, "LINE"),
        new TypedValue((int)DxfCode.LayerName, "WALLS"),
    new TypedValue((int)DxfCode.Operator, "and>"),
};
var filter = new SelectionFilter(filterList);

var selRes = ed.SelectAll(filter);
if (selRes.Status == PromptStatus.OK)
{
    var ids = selRes.Value.GetObjectIds();
    Console.WriteLine($"matched {ids.Length} lines on layer WALLS");
}
```

The filter list uses DXF group codes — the same numbers you'd see in a .dxf file:

| Code | Meaning |
|---|---|
| 0 (`DxfCode.Start`) | Entity type name ("LINE", "CIRCLE", "INSERT", etc.) |
| 8 (`DxfCode.LayerName`) | Layer name |
| 2 (`DxfCode.BlockName`) | Block name (for INSERT entities) |
| 6 (`DxfCode.LinetypeName`) | Line type |
| 62 (`DxfCode.Color`) | Color index |
| 67 (`DxfCode.Int16` for Paper Space) | 1 = paper space, 0 = model space |
| -3 (extension data app name) | XData filter — useful for finding entities tagged by your app |

Operators wrap individual filters:

| Operator | What it does |
|---|---|
| `<and ... and>` | Logical AND |
| `<or ... or>` | Logical OR |
| `<not ... not>` | Logical NOT |
| `<xor ... xor>` | Logical XOR |

## Filter by XData app name

```csharp
// Find every entity that has XData attached by AWARE
var filterList = new TypedValue[] {
    new TypedValue((int)DxfCode.Operator, "<and"),
        new TypedValue(-3, "AWARE"),
    new TypedValue((int)DxfCode.Operator, "and>"),
};
var filter = new SelectionFilter(filterList);
var selRes = ed.SelectAll(filter);
```

The `-3` code is a synthetic "this entity has XData from app X" filter. Useful for finding everything AWARE has stamped.

## SelectionSet → ObjectIds

`SelectionSet.GetObjectIds()` returns the entity ObjectIds. You then need a Transaction to open them:

```csharp
using (var t = db.TransactionManager.StartTransaction())
{
    foreach (ObjectId id in selRes.Value.GetObjectIds())
    {
        var entity = t.GetObject(id, OpenMode.ForRead) as Entity;
        // ...
    }
    t.Commit();
}
```

## SelectionSet lifecycle

SelectionSet is **session-only** — closing the drawing invalidates it. The underlying ObjectIds remain valid, but the SelectionSet wrapper does not. Store the ids if you need cross-call persistence:

```csharp
var ids = selRes.Value.GetObjectIds();   // ObjectId[] — store these

// later, in another transaction:
using (var t = db.TransactionManager.StartTransaction())
{
    foreach (var id in ids)
    {
        if (!id.IsValid || id.IsErased) continue;
        var entity = t.GetObject(id, OpenMode.ForRead) as Entity;
    }
    t.Commit();
}
```

Always check `IsValid` and `IsErased` for cross-call persistence — entities may have been deleted between calls.

## Common patterns

### All entities on a layer

```csharp
var filterList = new TypedValue[] {
    new TypedValue((int)DxfCode.LayerName, "WALLS"),
};
var selRes = ed.SelectAll(new SelectionFilter(filterList));
```

### All BlockReferences of a specific definition

```csharp
var filterList = new TypedValue[] {
    new TypedValue((int)DxfCode.Operator, "<and"),
        new TypedValue((int)DxfCode.Start, "INSERT"),
        new TypedValue((int)DxfCode.BlockName, "CHAIR"),
    new TypedValue((int)DxfCode.Operator, "and>"),
};
```

### All entities NOT on a "frozen" layer

There's no direct DXF filter for "not on a frozen layer". Use a post-filter loop:

```csharp
using (var t = db.TransactionManager.StartTransaction())
{
    var layerTable = t.GetObject(db.LayerTableId, OpenMode.ForRead) as LayerTable;
    var nonFrozenIds = selRes.Value.GetObjectIds().Where(id => {
        var ent = t.GetObject(id, OpenMode.ForRead) as Entity;
        var layer = t.GetObject(layerTable[ent.Layer], OpenMode.ForRead) as LayerTableRecord;
        return !layer.IsFrozen;
    }).ToArray();
    t.Commit();
}
```

### Paper space only vs model space only

```csharp
// Model space entities only
var modelSpaceFilter = new TypedValue[] {
    new TypedValue(67, 0),    // 0 = model space, 1 = paper space
};

// Paper space entities only
var paperSpaceFilter = new TypedValue[] {
    new TypedValue(67, 1),
};
```

## Common gotchas

- **`SelectAll` returns from the CURRENT space.** If the user has switched to paper space, you get paper-space entities. Switch first with `LayoutManager.Current.CurrentLayout = "Model"` if you want model-space.
- **`SelectionSet` from `SelectAll` is null when nothing matches.** Always check `selRes.Status == PromptStatus.OK` AND `selRes.Value != null`.
- **`GetSelection()` PROMPTS the user.** Don't use in non-interactive scripts unless the user is meant to be in the loop.
- **Filter group codes 8 (layer) and 2 (block) are STRING comparisons.** Case-sensitive in some AutoCAD versions; verify.
- **Selection filters do NOT traverse blocks.** A circle INSIDE a block is NOT matched by `SelectAll` with `<Start, "CIRCLE">` — the block reference (INSERT) is what's selected.

## Standard pattern

```csharp
var doc = Application.DocumentManager.MdiActiveDocument;
var ed = doc.Editor;
var db = doc.Database;

var filterList = new TypedValue[] {
    new TypedValue((int)DxfCode.Start, "LINE"),
    new TypedValue((int)DxfCode.LayerName, args["layer"].ToString()),
};
var selRes = ed.SelectAll(new SelectionFilter(filterList));

if (selRes.Status != PromptStatus.OK || selRes.Value == null)
    return new { count = 0 };

using (var t = db.TransactionManager.StartTransaction())
{
    var totalLength = 0.0;
    foreach (ObjectId id in selRes.Value.GetObjectIds())
    {
        var line = t.GetObject(id, OpenMode.ForRead) as Line;
        if (line != null) totalLength += line.Length;
    }
    t.Commit();
    return new { count = selRes.Value.Count, total_length = totalLength };
}
```

## See also

- [autocad-transactions-and-symbol-tables](./autocad-transactions-and-symbol-tables.md) — getting from ObjectId to Entity
- [autocad-extension-dictionaries-and-xdata](./autocad-extension-dictionaries-and-xdata.md) — filtering by XData
- [autocad-block-references](./autocad-block-references.md) — block-aware selection patterns
