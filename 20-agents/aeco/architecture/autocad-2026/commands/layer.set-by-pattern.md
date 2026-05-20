# layer.set-by-pattern

Move every entity whose current layer matches a regex onto a target layer. Creates target layer if missing.

`mode: write` — composing app must declare `safety:` block per v0.11.

## Inputs

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `from-pattern` | string | yes | — | .NET regex matched against source layer's Name |
| `to-layer` | string | yes | — | Target layer Name (created if missing) |
| `to-layer-color-aci` | number | no | 7 | ACI color (1-255). Used on create only. |

## Implementation (via aware-autocad exec)

```csharp
using Autodesk.AutoCAD.ApplicationServices;
using Autodesk.AutoCAD.DatabaseServices;
using Autodesk.AutoCAD.Colors;

var doc = Application.DocumentManager.MdiActiveDocument;
var db  = doc.Database;
var fromRx = new System.Text.RegularExpressions.Regex(
    args["from-pattern"].ToString(),
    System.Text.RegularExpressions.RegexOptions.IgnoreCase);
var toName = args["to-layer"].ToString();
short colorAci = (short)Convert.ToInt32(args.TryGetValue("to-layer-color-aci", out var c) ? c : 7);

using (var docLock = doc.LockDocument())
using (var t = db.TransactionManager.StartTransaction())
{
    var lt = t.GetObject(db.LayerTableId, OpenMode.ForRead) as LayerTable;
    ObjectId toLayerId;
    bool created = false;
    if (lt.Has(toName))
    {
        toLayerId = lt[toName];
    }
    else
    {
        lt.UpgradeOpen();
        var newLayer = new LayerTableRecord {
            Name = toName,
            Color = Color.FromColorIndex(ColorMethod.ByAci, colorAci),
        };
        toLayerId = lt.Add(newLayer);
        t.AddNewlyCreatedDBObject(newLayer, true);
        created = true;
    }

    int moved = 0;
    var bt = t.GetObject(db.BlockTableId, OpenMode.ForRead) as BlockTable;
    foreach (ObjectId btrId in bt)
    {
        var btr = t.GetObject(btrId, OpenMode.ForRead) as BlockTableRecord;
        foreach (ObjectId entId in btr)
        {
            var ent = t.GetObject(entId, OpenMode.ForRead) as Entity;
            if (ent == null) continue;
            if (!fromRx.IsMatch(ent.Layer)) continue;
            ent.UpgradeOpen();
            ent.LayerId = toLayerId;
            moved++;
        }
    }
    t.Commit();
    return new {
        target_layer_handle = lt[toName].Handle.ToString(),
        moved_count = moved,
        created_target = created,
    };
}
```

## See also

- [autocad-transactions-and-symbol-tables](../skills/autocad-transactions-and-symbol-tables.md)
- [autocad-selection-and-filters](../skills/autocad-selection-and-filters.md)
