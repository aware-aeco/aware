# user-text.bulk-write

Write multiple user-text key/value pairs onto every selected object. The "stamp the run id" / "tag-by-mark" primitive — the v0.11 safety contract uses this pattern for `audit-stamp:` UDA writes.

`mode: write` — the composing app must declare a `safety:` block per v0.11.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `entries` | object | yes | Map of `key -> value` (both strings) to write on every selected object. |

## Outputs

```json
{ "objects-stamped": 47, "keys-written": 4 }
```

## Implementation (shipped through `aware-rhino exec`)

```csharp
var doc = Rhino.RhinoDoc.ActiveDoc;
var entries = args["entries"] as IDictionary<string, object?>
              ?? throw new InvalidOperationException("entries must be a string→string map");
if (entries.Count == 0)
    return new { ok = false, error = "entries is empty (nothing to write)" };

int stamped = 0;
int keysWritten = 0;
foreach (var obj in doc.Objects.GetSelectedObjects(false, false))
{
    var attrs = obj.Attributes.Duplicate();
    bool anyChange = false;
    foreach (var kvp in entries)
    {
        var val = kvp.Value?.ToString() ?? "";
        attrs.SetUserString(kvp.Key, val);
        anyChange = true;
        keysWritten++;
    }
    if (anyChange && doc.Objects.ModifyAttributes(obj, attrs, false))
        stamped++;
}
doc.Views.Redraw();
return new { ok = true, objects_stamped = stamped, keys_written = keysWritten };
```

## Gotchas

- `ModifyAttributes` returns false on locked objects — the count reflects successful writes only.
- Setting a user-text key to empty string removes the key entirely (per Rhino's convention).
- This rewrites attribute user-data; it does NOT touch block instance attributes inside the definition.
- For very large selections (>10k objects), wrap the loop in a `RhinoApp.Wait()` / progress feedback in the caller — there's no built-in transaction grouping for attribute writes.

## See also

- [`selection.by-attribute`](./selection.by-attribute.md) — the read-side pair
- `_-SetUserText` (Rhino command) — interactive equivalent for one key at a time
