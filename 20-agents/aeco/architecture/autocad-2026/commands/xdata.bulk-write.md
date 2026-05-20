# xdata.bulk-write

Write XData (with auto-RegApp registration) onto every selected entity. The v0.11 audit-stamp primitive for AutoCAD.

`mode: write` — composing app must declare `safety:` block per v0.11.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `app-name` | string | yes | RegApp name (auto-registered if missing) |
| `entries` | array | yes | Array of `{dxf-code, value}` pairs (typed TypedValues) |

`dxf-code` should be a valid XData group code. Common values:
- `1000` — ASCII string
- `1001` — Application name (auto-supplied; you don't need to provide)
- `1003` — Layer name reference
- `1040` — Real (double)
- `1070` — Integer (16-bit)

## Implementation (via aware-autocad exec)

```csharp
using Autodesk.AutoCAD.ApplicationServices;
using Autodesk.AutoCAD.DatabaseServices;
using Autodesk.AutoCAD.EditorInput;

var doc = Application.DocumentManager.MdiActiveDocument;
var ed  = doc.Editor;
var db  = doc.Database;
var appName = args["app-name"].ToString();
var entries = args["entries"] as IList<object>;
if (entries == null || entries.Count == 0)
    return new { ok = false, error = "entries is empty" };

using (var docLock = doc.LockDocument())
using (var t = db.TransactionManager.StartTransaction())
{
    // 1. Register the app name if missing
    var regAppTable = t.GetObject(db.RegAppTableId, OpenMode.ForRead) as RegAppTable;
    if (!regAppTable.Has(appName))
    {
        regAppTable.UpgradeOpen();
        var newRegApp = new RegAppTableRecord { Name = appName };
        regAppTable.Add(newRegApp);
        t.AddNewlyCreatedDBObject(newRegApp, true);
    }

    // 2. Build the ResultBuffer (app-name first, then user entries)
    var buffer = new List<TypedValue> {
        new TypedValue((int)DxfCode.ExtendedDataRegAppName, appName),
    };
    foreach (var entry in entries)
    {
        if (entry is IDictionary<string, object> d &&
            d.TryGetValue("dxf-code", out var code) &&
            d.TryGetValue("value", out var val))
        {
            buffer.Add(new TypedValue(Convert.ToInt32(code), val));
        }
    }
    var rb = new ResultBuffer(buffer.ToArray());

    // 3. Apply to current selection (or use the input ids if provided)
    var implied = ed.SelectImplied();
    if (implied.Status != PromptStatus.OK || implied.Value == null)
        return new { ok = false, error = "no entities selected" };

    int stamped = 0;
    foreach (ObjectId id in implied.Value.GetObjectIds())
    {
        var ent = t.GetObject(id, OpenMode.ForWrite) as Entity;
        if (ent == null) continue;
        ent.XData = rb;
        stamped++;
    }
    t.Commit();
    return new {
        ok = true,
        entities_stamped = stamped,
    };
}
```

## See also

- [autocad-extension-dictionaries-and-xdata](../skills/autocad-extension-dictionaries-and-xdata.md)
- [entity.by-xdata-app](./entity.by-xdata-app.md) — the read-side pair
