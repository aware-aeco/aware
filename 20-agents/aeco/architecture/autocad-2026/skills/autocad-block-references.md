---
name: autocad-block-references
description: This skill should be used when composing AutoCAD snippets that work with blocks — defining new blocks, placing block references, querying block-instance counts, modifying block attributes, or working with dynamic blocks. Encodes the BlockTable → BlockTableRecord → BlockReference hierarchy, the difference between the model-space record (`*Model_Space`) and user-defined blocks, the dynamic block lifecycle (`AnonymousBlocks` for variations), the AttributeReference lifecycle on insertion, and the cross-drawing block-import pattern via `Database.Insert`.
---

# AutoCAD block references

Blocks are AutoCAD's first-class organizational primitive — like Rhino's `InstanceDefinition` + `InstanceObject` but with deeper integration (every drawing has a BlockTable; model space itself IS a BlockTableRecord).

## The block hierarchy

| Class | What it is |
|---|---|
| `BlockTable` | Container of all block-definitions in the database |
| `BlockTableRecord` | One block-definition: name + entities + attributes |
| `BlockReference` | A placed copy of a definition: position + scale + rotation + attribute overrides |
| `AttributeDefinition` | Inside the BlockTableRecord: the attribute template |
| `AttributeReference` | Inside the BlockReference: the per-instance attribute value |
| `DynamicBlockReferenceProperty` | Per-instance dynamic-block parameter values |

`*Model_Space` and `*Paper_Space` are special BlockTableRecord names — these are the model space and paper space layouts themselves. EVERY entity in a drawing lives inside SOME BlockTableRecord (either model space, paper space, or a user block).

## Listing block definitions

```csharp
using (var t = db.TransactionManager.StartTransaction())
{
    var bt = t.GetObject(db.BlockTableId, OpenMode.ForRead) as BlockTable;
    foreach (ObjectId btrId in bt)
    {
        var btr = t.GetObject(btrId, OpenMode.ForRead) as BlockTableRecord;
        if (btr.IsLayout) continue;            // skip model/paper space
        if (btr.Name.StartsWith("*"))         // skip anonymous + system blocks
            continue;

        Console.WriteLine($"{btr.Name}  used_count={btr.GetBlockReferenceIds(true, true).Count}");
    }
    t.Commit();
}
```

`GetBlockReferenceIds(directOnly: true, forceValidity: true)` returns all instances of this definition.

## Inserting a block reference

```csharp
using (var t = db.TransactionManager.StartTransaction())
{
    var bt = t.GetObject(db.BlockTableId, OpenMode.ForRead) as BlockTable;
    if (!bt.Has("CHAIR")) return "block CHAIR not found";

    var ms = t.GetObject(bt[BlockTableRecord.ModelSpace], OpenMode.ForWrite) as BlockTableRecord;
    var blockDefId = bt["CHAIR"];

    var blockRef = new BlockReference(new Point3d(1000, 500, 0), blockDefId);
    blockRef.Rotation = Math.PI / 4;             // 45 degrees (radians)
    blockRef.ScaleFactors = new Scale3d(1.5);    // 1.5x scale uniform

    var refId = ms.AppendEntity(blockRef);
    t.AddNewlyCreatedDBObject(blockRef, true);

    // Now the attributes: instantiate AttributeReferences from the definition's AttributeDefinitions
    var blockDef = t.GetObject(blockDefId, OpenMode.ForRead) as BlockTableRecord;
    foreach (ObjectId attDefId in blockDef)
    {
        var attDef = t.GetObject(attDefId, OpenMode.ForRead) as AttributeDefinition;
        if (attDef == null || attDef.Constant) continue;

        var attRef = new AttributeReference();
        attRef.SetAttributeFromBlock(attDef, blockRef.BlockTransform);
        // Override the value if you want:
        if (attDef.Tag == "ROOM_NUMBER") attRef.TextString = "101";

        blockRef.AttributeCollection.AppendAttribute(attRef);
        t.AddNewlyCreatedDBObject(attRef, true);
    }

    t.Commit();
}
```

The attribute lifecycle is the most-forgotten step — without explicit `AttributeReference` creation, the block places but its attributes are missing.

## Dynamic blocks

Dynamic blocks have parametric variations (different lengths, shapes, etc.). Each visual variation is technically a separate anonymous BlockTableRecord behind the scenes — but they share a dynamic block "lookup table".

```csharp
var blockRef = t.GetObject(blockRefId, OpenMode.ForWrite) as BlockReference;
if (blockRef.IsDynamicBlock)
{
    var props = blockRef.DynamicBlockReferencePropertyCollection;
    foreach (DynamicBlockReferenceProperty p in props)
    {
        if (p.PropertyName == "Width")
        {
            p.Value = 1200.0;       // typed value
        }
    }
}
```

Each `DynamicBlockReferenceProperty` exposes a typed Value (double, int, string). The lookup-table internally swaps to the appropriate anonymous block.

## Block import (cross-drawing)

To copy a block definition from one drawing to another:

```csharp
using (var sourceDb = new Database(false, true))
{
    sourceDb.ReadDwgFile(@"C:\Libraries\Furniture.dwg", FileShare.Read, true, "");

    using (var t = sourceDb.TransactionManager.StartTransaction())
    {
        var sourceBt = t.GetObject(sourceDb.BlockTableId, OpenMode.ForRead) as BlockTable;
        var chairBlockId = sourceBt["CHAIR"];

        // Insert into the active drawing's database
        var ids = new ObjectIdCollection { chairBlockId };
        var mapping = new IdMapping();
        sourceDb.WblockCloneObjects(ids, db.BlockTableId, mapping, DuplicateRecordCloning.Replace, false);

        t.Commit();
    }
}
```

`WblockCloneObjects` is the canonical "copy block from source to target" call. Handles dependent records (layers, linetypes referenced by block geometry).

## Common gotchas

- **`bt[BlockTableRecord.ModelSpace]` returns the ObjectId of the model space BTR.** Don't confuse with model space ENTITIES.
- **Anonymous blocks** (names starting with `*`) include dynamic-block variations and dimension-style blocks. Filter out for user-facing block lists.
- **AttributeReferences are children of BlockReference, NOT of BlockTableRecord.** Don't try to walk attributes off the definition.
- **`BlockReference.Bounds` is the AXIS-ALIGNED bounding box of the placed instance** — useful for visibility checks, NOT for tight collision.
- **`blockRef.GetBlockReferenceIds(...)` is NOT a method on BlockReference** — it's on BlockTableRecord. Common typo source.
- **Erasing the BlockTableRecord doesn't auto-erase its BlockReferences.** AutoCAD throws "block has references" unless you erase references first.

## Standard pattern

```csharp
var doc = Application.DocumentManager.MdiActiveDocument;
var db  = doc.Database;

using (var docLock = doc.LockDocument())
using (var t = db.TransactionManager.StartTransaction())
{
    var bt = t.GetObject(db.BlockTableId, OpenMode.ForRead) as BlockTable;
    var blocks = new List<object>();

    foreach (ObjectId btrId in bt)
    {
        var btr = t.GetObject(btrId, OpenMode.ForRead) as BlockTableRecord;
        if (btr.IsLayout || btr.Name.StartsWith("*")) continue;

        var refCount = btr.GetBlockReferenceIds(true, false).Count;
        blocks.Add(new {
            name = btr.Name,
            id = btr.ObjectId.Handle.ToString(),
            instance_count = refCount,
        });
    }

    t.Commit();
    return new { definition_count = blocks.Count, blocks = blocks };
}
```

## See also

- [autocad-transactions-and-symbol-tables](./autocad-transactions-and-symbol-tables.md) — the BlockTable is one of these
- [autocad-document-and-database](./autocad-document-and-database.md) — what owns the BlockTable
