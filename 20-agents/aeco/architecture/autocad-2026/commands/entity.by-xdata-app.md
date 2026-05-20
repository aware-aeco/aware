# entity.by-xdata-app

Find every entity that has XData attached by a named application. The "find every part stamped by this app" primitive — pairs with `xdata.bulk-write` for v0.11 audit-stamp workflows.

## Inputs

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `app-name` | string | yes | — | Registered app name to filter by (e.g. "AWARE") |
| `space` | enum | no | `all` | `model`, `paper`, or `all` — which space(s) to scan |

## Implementation (via aware-autocad exec)

```csharp
using Autodesk.AutoCAD.ApplicationServices;
using Autodesk.AutoCAD.DatabaseServices;
using Autodesk.AutoCAD.EditorInput;

var doc = Application.DocumentManager.MdiActiveDocument;
var ed  = doc.Editor;
var db  = doc.Database;
var appName = args["app-name"].ToString();
var space   = args.TryGetValue("space", out var s) ? s?.ToString() ?? "all" : "all";

// Build a SelectionFilter: filter -3 = "has XData for app X"
var filterList = new List<TypedValue> {
    new TypedValue(-3, appName),
};
// Add space filter (DXF code 67: 0 = model, 1 = paper)
if (space == "model") filterList.Add(new TypedValue(67, 0));
else if (space == "paper") filterList.Add(new TypedValue(67, 1));

// Wrap the -3 in <and..and> with the space filter for clarity
var wrapped = new List<TypedValue>();
if (filterList.Count > 1)
{
    wrapped.Add(new TypedValue((int)DxfCode.Operator, "<and"));
    wrapped.AddRange(filterList);
    wrapped.Add(new TypedValue((int)DxfCode.Operator, "and>"));
}
else
{
    wrapped.AddRange(filterList);
}

var selRes = ed.SelectAll(new SelectionFilter(wrapped.ToArray()));
if (selRes.Status != PromptStatus.OK || selRes.Value == null)
    return new { matched_count = 0, handles = new string[0] };

using (var t = db.TransactionManager.StartTransaction())
{
    var handles = selRes.Value.GetObjectIds()
        .Select(id => {
            var e = t.GetObject(id, OpenMode.ForRead) as Entity;
            return e?.Handle.ToString();
        })
        .Where(h => h != null)
        .ToList();
    t.Commit();
    return new {
        matched_count = handles.Count,
        handles       = handles,
    };
}
```

## See also

- [autocad-selection-and-filters](../skills/autocad-selection-and-filters.md)
- [autocad-extension-dictionaries-and-xdata](../skills/autocad-extension-dictionaries-and-xdata.md)
- [xdata.bulk-write](./xdata.bulk-write.md) — the write-side pair
