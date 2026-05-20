# worksession.reload

Reload every worksession-attached model (out-of-process references). The "pull the latest from teammates" primitive. Reports per-file success/failure.

`mode: write` — strictly modifies the cached references; the doc itself isn't altered, but the rendered view will change.

## Inputs

None.

## Outputs

```json
{
  "attached-count": 3,
  "results": [
    { "path": "C:/Project/struct.3dm", "success": true,  "error": "" },
    { "path": "C:/Project/MEP.3dm",    "success": false, "error": "file is locked by AnotherUser" },
    ...
  ]
}
```

## Implementation (shipped through `aware-rhino exec`)

```csharp
var doc = Rhino.RhinoDoc.ActiveDoc;
var ws = doc.Worksession;
if (ws == null || ws.ModelCount == 0)
    return new { ok = true, attached_count = 0, results = new object[0] };

var rows = new List<object>();
// Worksession exposes file refs by 0-based index; AttachedModelCount excludes the active doc.
for (int i = 0; i < ws.ModelCount; i++)
{
    var path = ws.ModelAliasFromIndex(i) ?? ws.ModelPathFromIndex(i);
    bool ok;
    string err = "";
    try
    {
        ok = ws.RefreshAttachedModel(i);
    }
    catch (Exception e)
    {
        ok = false;
        err = e.Message;
    }
    rows.Add(new { path = path, success = ok, error = err });
}
doc.Views.Redraw();
return new { ok = true, attached_count = ws.ModelCount, results = rows };
```

## Gotchas

- A "worksession" in Rhino is the .rws-managed multi-model coordination — distinct from block-instance linking. If your model uses linked blocks only, this verb is a no-op.
- `RefreshAttachedModel` returns false (and the script catches any exception) when the referenced file is locked or unreachable. Errors are surfaced per-row; the overall verb still returns ok=true.
- Index ordering may not be stable across sessions; use the `path` field for identity.
- The doc itself isn't marked dirty by this op — only the cached representation of references is updated.

## See also

- `_-Worksession` (Rhino command) — interactive equivalent
- Compare to Revit's `link.reload-all` for the equivalent primitive
