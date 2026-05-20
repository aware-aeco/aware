# block.list-instances

Enumerate every BlockReference for a given block definition, returning insertion point, scale, rotation, and AttributeReferences.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `definition-name` | string | yes | Block definition name (case-insensitive) |

## Implementation (via aware-autocad exec)

```csharp
using Autodesk.AutoCAD.ApplicationServices;
using Autodesk.AutoCAD.DatabaseServices;

var doc = Application.DocumentManager.MdiActiveDocument;
var db  = doc.Database;
var defName = args["definition-name"].ToString();

using (var t = db.TransactionManager.StartTransaction())
{
    var bt = t.GetObject(db.BlockTableId, OpenMode.ForRead) as BlockTable;
    if (!bt.Has(defName))
        return new { ok = false, error = $"no block definition named '{defName}'" };

    var defId = bt[defName];
    var def = t.GetObject(defId, OpenMode.ForRead) as BlockTableRecord;
    var refIds = def.GetBlockReferenceIds(directOnly: true, forceValidity: true);

    var rows = new List<object>();
    foreach (ObjectId refId in refIds)
    {
        var blockRef = t.GetObject(refId, OpenMode.ForRead) as BlockReference;
        if (blockRef == null) continue;

        var attrs = new Dictionary<string, string>();
        foreach (ObjectId attRefId in blockRef.AttributeCollection)
        {
            var attRef = t.GetObject(attRefId, OpenMode.ForRead) as AttributeReference;
            if (attRef != null) attrs[attRef.Tag] = attRef.TextString;
        }

        rows.Add(new {
            handle   = blockRef.Handle.ToString(),
            position = new { x = blockRef.Position.X, y = blockRef.Position.Y, z = blockRef.Position.Z },
            rotation = blockRef.Rotation * 180.0 / Math.PI,
            scale    = new { x = blockRef.ScaleFactors.X, y = blockRef.ScaleFactors.Y, z = blockRef.ScaleFactors.Z },
            attributes = attrs,
        });
    }
    t.Commit();
    return new {
        definition_name = defName,
        instance_count  = rows.Count,
        instances       = rows,
    };
}
```

## See also

- [autocad-block-references](../skills/autocad-block-references.md)
- [autocad-transactions-and-symbol-tables](../skills/autocad-transactions-and-symbol-tables.md)
