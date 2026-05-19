# selection.by-attribute

Select every object whose user-text attribute matches a value (or regex). The "find every part tagged X" / "select by mark" primitive — pairs naturally with `user-text.bulk-write`.

## Inputs

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `key` | string | yes | — | User-text key to read (e.g. `"AWARE_RUN_ID"`). |
| `value` | string | one-of | — | Exact value to match (case-insensitive). |
| `pattern` | string | one-of | — | .NET regex to match the value. |
| `add-to-selection` | boolean | no | `false` | When true, ADDS to current selection; when false, replaces. |

Exactly one of `value` or `pattern` must be supplied.

## Outputs

```json
{ "matched-count": 47, "ids": ["abc...", "def...", ...] }
```

## Implementation (shipped through `aware-rhino exec`)

```csharp
var doc = Rhino.RhinoDoc.ActiveDoc;
var key      = args["key"]!.ToString();
var exact    = args.TryGetValue("value",   out var v) ? v?.ToString() : null;
var pattern  = args.TryGetValue("pattern", out var p) ? p?.ToString() : null;
bool addTo   = args.TryGetValue("add-to-selection", out var a) && Convert.ToBoolean(a);

if (string.IsNullOrEmpty(exact) && string.IsNullOrEmpty(pattern))
    return new { ok = false, error = "supply value OR pattern" };

System.Text.RegularExpressions.Regex? rx = null;
if (!string.IsNullOrEmpty(pattern))
    rx = new System.Text.RegularExpressions.Regex(pattern!,
            System.Text.RegularExpressions.RegexOptions.IgnoreCase);

if (!addTo)
    doc.Objects.UnselectAll();

var matched = new List<string>();
foreach (var obj in doc.Objects)
{
    var stored = obj.Attributes.GetUserString(key);
    if (string.IsNullOrEmpty(stored)) continue;

    bool isMatch = rx != null
        ? rx.IsMatch(stored)
        : string.Equals(stored, exact, StringComparison.OrdinalIgnoreCase);

    if (isMatch)
    {
        obj.Select(true);
        matched.Add(obj.Id.ToString());
    }
}
doc.Views.Redraw();
return new { ok = true, matched_count = matched.Count, ids = matched };
```

## Gotchas

- User-text is also called "attribute user data" in Rhino. Stored per object via `obj.Attributes.SetUserString(key, value)`.
- Matching is case-insensitive for `value`; specify the regex flag explicitly if you want case-sensitive `pattern` matching.
- Hidden/locked objects are returned in the match but not selected (Rhino refuses to select them) — the count reflects matches, the ids reflect what's now in the selection.
- For very large models (>50k objects), this iterates every object; consider scoping to a layer first via a separate filter pass.

## See also

- [`user-text.bulk-write`](./user-text.bulk-write.md) — the write-side pair
- `_-SelByUserText` (Rhino command) — interactive equivalent
