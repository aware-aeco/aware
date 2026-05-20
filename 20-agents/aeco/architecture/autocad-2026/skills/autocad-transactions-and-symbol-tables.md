---
name: autocad-transactions-and-symbol-tables
description: This skill should be used when composing any AutoCAD .NET snippet that reads or writes the drawing — every operation requires a Transaction in AutoCAD's API model. Encodes the TransactionManager / Transaction lifecycle, the OpenMode (ForRead / ForWrite / ForNotify) parameter, the SymbolTable concept (BlockTable, LayerTable, LineTypeTable, DimStyleTable, RegAppTable, TextStyleTable, UcsTable, ViewTable, ViewportTable), and the gotcha that opening the same object twice in one transaction throws unless ForRead-twice.
---

# AutoCAD transactions and symbol tables

AutoCAD's .NET API is **transactional**. Every read AND write of database content goes through a Transaction. This is similar to Revit's transaction model but stricter — there are no implicit-transaction reads.

## The transaction lifecycle

```csharp
using Autodesk.AutoCAD.DatabaseServices;

var db = doc.Database;

using (var t = db.TransactionManager.StartTransaction())
{
    // Open objects via t.GetObject(id, mode)
    var entity = t.GetObject(someId, OpenMode.ForRead) as Entity;
    // ...
    t.Commit();    // or omit Commit() to roll back on dispose
}
```

`using` (or `try/finally` with explicit Dispose) ensures the transaction's resources are released. NEVER hold a Transaction across event handlers or async operations — they expire when their parent code path exits.

## OpenMode

| Mode | Use when |
|---|---|
| `OpenMode.ForRead` | Reading properties; iterating |
| `OpenMode.ForWrite` | Modifying the object |
| `OpenMode.ForNotify` | (Rare) Listening for change events |

Open an object for READ first, then `Upgrade()` to WRITE if needed:

```csharp
var line = t.GetObject(lineId, OpenMode.ForRead) as Line;
if (someCondition)
{
    line.UpgradeOpen();
    line.ColorIndex = 1;   // red
}
```

This pattern avoids unnecessary write-locks. AutoCAD's locking is per-object — exclusive write-lock blocks other transactions.

## The SymbolTable concept

AutoCAD groups document settings into **SymbolTables**. There are 9 of them, all accessible from Database:

| Table | What it holds | Common entries |
|---|---|---|
| `BlockTable` | All block definitions + model/paper space records | `*Model_Space`, `*Paper_Space`, your blocks |
| `LayerTable` | Layers | `0`, custom layers |
| `LineTypeTable` | Line types | `Continuous`, `DASHED`, custom |
| `DimStyleTable` | Dimension styles | `Standard`, custom |
| `RegAppTable` | Registered app names (for XData) | App-specific names |
| `TextStyleTable` | Text styles | `Standard`, custom |
| `UcsTable` | Named UCSs (coordinate systems) | Custom UCSs |
| `ViewTable` | Named views | Custom saved views |
| `ViewportTable` | Viewport configurations | `*Active`, custom |

Walk a table:

```csharp
var layerTable = t.GetObject(db.LayerTableId, OpenMode.ForRead) as LayerTable;
foreach (ObjectId layerId in layerTable)
{
    var layer = t.GetObject(layerId, OpenMode.ForRead) as LayerTableRecord;
    Console.WriteLine($"{layer.Name}  {layer.Color}  visible={!layer.IsOff}");
}
```

Look up by name:

```csharp
if (layerTable.Has("AWARE"))
{
    var awareLayerId = layerTable["AWARE"];
    var awareLayer = t.GetObject(awareLayerId, OpenMode.ForRead) as LayerTableRecord;
}
```

## Adding a SymbolTable entry

To add a new layer:

```csharp
using (var t = db.TransactionManager.StartTransaction())
{
    var layerTable = t.GetObject(db.LayerTableId, OpenMode.ForWrite) as LayerTable;

    if (!layerTable.Has("AWARE"))
    {
        var newLayer = new LayerTableRecord {
            Name = "AWARE",
            Color = Autodesk.AutoCAD.Colors.Color.FromColorIndex(Autodesk.AutoCAD.Colors.ColorMethod.ByAci, 1),
        };
        var newLayerId = layerTable.Add(newLayer);
        t.AddNewlyCreatedDBObject(newLayer, true);   // CRITICAL — registers for commit
    }
    t.Commit();
}
```

The `AddNewlyCreatedDBObject` call is mandatory — without it, the new layer is "orphaned" and won't survive the transaction.

## The "open the same object twice" gotcha

Opening an object you already hold (via the same Transaction) is fine for ForRead — but throws for ForWrite:

```csharp
var obj1 = t.GetObject(id, OpenMode.ForRead);
var obj2 = t.GetObject(id, OpenMode.ForRead);   // OK; returns same object
var obj3 = t.GetObject(id, OpenMode.ForWrite);  // THROWS — already open for read
```

Solution: `obj1.UpgradeOpen()` on the original reference.

## Cross-transaction object identity

ObjectId values are valid across transactions and even across SAVES (they're document-scoped GUIDs internally). The .NET wrapper objects (Entity, BlockTableRecord) are valid ONLY within their owning transaction. After commit, references to those objects are stale.

```csharp
ObjectId modelSpaceId;
using (var t = db.TransactionManager.StartTransaction())
{
    var bt = t.GetObject(db.BlockTableId, OpenMode.ForRead) as BlockTable;
    modelSpaceId = bt[BlockTableRecord.ModelSpace];   // ObjectId — SAFE to capture
    t.Commit();
}

// modelSpaceId is still valid here. Re-open:
using (var t = db.TransactionManager.StartTransaction())
{
    var ms = t.GetObject(modelSpaceId, OpenMode.ForRead) as BlockTableRecord;
    // ... use ms ...
    t.Commit();
}
```

## Common gotchas

- **Always `AddNewlyCreatedDBObject(obj, true)`** when you `Add` to a SymbolTable. Without it, the object disappears on commit.
- **Always `t.AddNewlyCreatedDBObject(entity, true)`** when adding entities to a BlockTableRecord (model space). Same reason.
- **Don't store Entity objects across transactions.** They invalidate. Store ObjectId values and re-open per-transaction.
- **NestedSymbolTables don't exist** — every SymbolTable is flat. BlockTable contains BlockTableRecords which contain Entities; layers don't nest.
- **Layer 0 is special** — it can't be deleted, can't be renamed, and entities on Layer 0 inherit the BLOCK's color/linetype when inside a block reference.

## Standard pattern

```csharp
var doc = Application.DocumentManager.MdiActiveDocument;
var db  = doc.Database;

using (var docLock = doc.LockDocument())
using (var t = db.TransactionManager.StartTransaction())
{
    var bt = t.GetObject(db.BlockTableId, OpenMode.ForRead) as BlockTable;
    var ms = t.GetObject(bt[BlockTableRecord.ModelSpace], OpenMode.ForWrite) as BlockTableRecord;

    var line = new Line(new Point3d(0, 0, 0), new Point3d(1000, 0, 0));
    var id = ms.AppendEntity(line);
    t.AddNewlyCreatedDBObject(line, true);

    t.Commit();
}
```

## See also

- [autocad-document-and-database](./autocad-document-and-database.md) — what owns these tables
- [autocad-block-references](./autocad-block-references.md) — BlockTable + BlockReference in depth
- [autocad-extension-dictionaries](./autocad-extension-dictionaries.md) — for metadata storage
