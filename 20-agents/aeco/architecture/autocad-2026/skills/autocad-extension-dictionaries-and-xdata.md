---
name: autocad-extension-dictionaries-and-xdata
description: This skill should be used when composing AutoCAD snippets that store custom metadata on entities — application tracking data, AWARE audit-stamp UDAs (the v0.11 safety contract), tool palette persistence, or any "attach my own data to this entity" pattern. Encodes the THREE ways AutoCAD can attach metadata (XData, Extension Dictionary, Xrecord) with their trade-offs, the RegAppTable registration requirement for XData, the size limits, and the persistence-across-drawing-save semantics.
---

# AutoCAD extension dictionaries and XData

AutoCAD has THREE distinct mechanisms for attaching custom data to entities. Each has its own API, limits, and use cases. Pick the wrong one and you either lose data on save or hit silent size limits.

## The three mechanisms

| Mechanism | Class | Max size per entity | Use when |
|---|---|---|---|
| **XData** | `ResultBuffer` on entity | 16 KB | Legacy compat; AutoLISP / ObjectARX interop; small typed metadata |
| **Extension Dictionary** | `DBDictionary` attached to entity | Unlimited | Modern; structured nested data; cross-app sharing |
| **Xrecord** | `Xrecord` inside Extension Dictionary | 64 KB per xrecord | The "leaf" of an Extension Dictionary; holds the actual data |

The relationship: an entity HAS an Extension Dictionary; the dictionary CONTAINS named Xrecords (and possibly nested dictionaries); each Xrecord holds typed data.

## XData (legacy)

```csharp
// Register the app name first (one-time per drawing)
using (var t = db.TransactionManager.StartTransaction())
{
    var regAppTable = t.GetObject(db.RegAppTableId, OpenMode.ForRead) as RegAppTable;
    if (!regAppTable.Has("AWARE"))
    {
        regAppTable.UpgradeOpen();
        var newRegApp = new RegAppTableRecord { Name = "AWARE" };
        regAppTable.Add(newRegApp);
        t.AddNewlyCreatedDBObject(newRegApp, true);
    }
    t.Commit();
}

// Attach XData
using (var t = db.TransactionManager.StartTransaction())
{
    var entity = t.GetObject(entityId, OpenMode.ForWrite) as Entity;

    var rb = new ResultBuffer(
        new TypedValue((int)DxfCode.ExtendedDataRegAppName, "AWARE"),
        new TypedValue((int)DxfCode.ExtendedDataAsciiString, "RUN_ID=abc123")
    );
    entity.XData = rb;
    t.Commit();
}

// Read XData
using (var t = db.TransactionManager.StartTransaction())
{
    var entity = t.GetObject(entityId, OpenMode.ForRead) as Entity;
    var rb = entity.GetXDataForApplication("AWARE");
    if (rb != null)
    {
        foreach (TypedValue tv in rb)
            Console.WriteLine($"{tv.TypeCode}: {tv.Value}");
    }
}
```

The `RegAppTable` registration is non-skippable — XData with an unregistered app name silently fails to save.

XData supports typed values (string, int, double, point, layer, etc.) via the `DxfCode` enum codes. Total per-entity per-app is 16 KB.

## Extension Dictionary (modern, structured)

```csharp
using (var t = db.TransactionManager.StartTransaction())
{
    var entity = t.GetObject(entityId, OpenMode.ForWrite) as Entity;

    entity.CreateExtensionDictionary();
    var dict = t.GetObject(entity.ExtensionDictionary, OpenMode.ForWrite) as DBDictionary;

    // Create an Xrecord under the dictionary
    var xrec = new Xrecord {
        Data = new ResultBuffer(
            new TypedValue((int)DxfCode.Text, "RUN_ID=abc123"),
            new TypedValue((int)DxfCode.Text, "APP=tender-bump"),
            new TypedValue((int)DxfCode.Text, "OPERATOR=pawel")
        ),
    };
    dict.SetAt("AWARE_AUDIT", xrec);
    t.AddNewlyCreatedDBObject(xrec, true);

    t.Commit();
}
```

Extension Dictionary is the modern way — supports nested dictionaries, no app-name registration required, can hold many keyed Xrecords on one entity, no per-key size limit (just per-Xrecord).

## Reading Extension Dictionary

```csharp
using (var t = db.TransactionManager.StartTransaction())
{
    var entity = t.GetObject(entityId, OpenMode.ForRead) as Entity;
    if (entity.ExtensionDictionary.IsNull) return null;

    var dict = t.GetObject(entity.ExtensionDictionary, OpenMode.ForRead) as DBDictionary;
    if (!dict.Contains("AWARE_AUDIT")) return null;

    var xrec = t.GetObject(dict.GetAt("AWARE_AUDIT"), OpenMode.ForRead) as Xrecord;
    var data = xrec.Data.AsArray()
        .Select(tv => tv.Value.ToString())
        .ToList();
    return data;
}
```

## Application-named root dictionary

For app-wide metadata (settings, not per-entity), AutoCAD provides a named dictionary off the Database:

```csharp
var namedDict = t.GetObject(db.NamedObjectsDictionaryId, OpenMode.ForWrite) as DBDictionary;
if (!namedDict.Contains("AWARE_SETTINGS"))
{
    var settingsDict = new DBDictionary();
    namedDict.SetAt("AWARE_SETTINGS", settingsDict);
    t.AddNewlyCreatedDBObject(settingsDict, true);
}
```

Useful for storing app-wide preferences that survive the drawing's lifetime.

## Choosing between the three

| Pattern | Choice |
|---|---|
| AWARE v0.11 audit-stamp UDA on every modified entity | Extension Dictionary with one Xrecord per stamp |
| Lightweight tags for AutoLISP interop | XData |
| App-wide preferences | Named dictionary off `NamedObjectsDictionaryId` |
| Cross-app shared metadata (e.g. IFC GUID) | Extension Dictionary with well-known key |
| Cost-of-add: 1 method call | XData |
| Cost-of-add: 5 method calls | Extension Dictionary |

For new AWARE code, **always prefer Extension Dictionary** unless XData is required for legacy compat.

## Persistence across save

Both XData and Extension Dictionary persist to .dwg / .dxf. Some constraints:

- **DXF round-trip:** XData encodes as group codes 1000-1071; Extension Dictionary as group codes 102/330/etc. Both survive.
- **Cross-version:** XData lost from some older XREF clip operations. Extension Dictionary is more robust.
- **Audit / Recover:** AutoCAD's `AUDIT` command can purge "orphaned" Extension Dictionary entries. Always test round-trips.

## Common gotchas

- **`entity.CreateExtensionDictionary()` is a no-op if one exists** — safe to call always.
- **`entity.ExtensionDictionary.IsNull` is the only way to check existence.** No `HasExtensionDictionary` property.
- **Xrecord without `t.AddNewlyCreatedDBObject` disappears** silently on commit (same as block adds).
- **XData size limit 16 KB is per-app per-entity.** Multiple apps can have their own 16 KB.
- **RegApp registration must be in the SAME database** as the entity. Side-loaded databases need their own registration.

## Standard pattern for AWARE v0.11 audit-stamp

```csharp
var doc = Application.DocumentManager.MdiActiveDocument;
var db  = doc.Database;
var runId    = args["run-id"]?.ToString() ?? "";
var appName  = args["app"]?.ToString() ?? "";
var operator_ = args["operator"]?.ToString() ?? "";

using (var docLock = doc.LockDocument())
using (var t = db.TransactionManager.StartTransaction())
{
    int stamped = 0;
    foreach (ObjectId id in selectedIds)
    {
        var entity = t.GetObject(id, OpenMode.ForWrite) as Entity;
        if (entity == null) continue;

        entity.CreateExtensionDictionary();
        var dict = t.GetObject(entity.ExtensionDictionary, OpenMode.ForWrite) as DBDictionary;

        var xrec = new Xrecord {
            Data = new ResultBuffer(
                new TypedValue((int)DxfCode.Text, $"AWARE_RUN_ID={runId}"),
                new TypedValue((int)DxfCode.Text, $"AWARE_APP={appName}"),
                new TypedValue((int)DxfCode.Text, $"AWARE_OPERATOR={operator_}"),
                new TypedValue((int)DxfCode.Text, $"AWARE_TS={DateTime.UtcNow:O}")
            ),
        };
        dict.SetAt("AWARE_AUDIT", xrec);
        t.AddNewlyCreatedDBObject(xrec, true);
        stamped++;
    }
    t.Commit();
    return new { stamped = stamped };
}
```

## See also

- [autocad-transactions-and-symbol-tables](./autocad-transactions-and-symbol-tables.md) — RegAppTable basics
- [autocad-document-and-database](./autocad-document-and-database.md) — where to find NamedObjectsDictionaryId
