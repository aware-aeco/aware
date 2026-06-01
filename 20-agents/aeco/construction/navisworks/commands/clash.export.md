# clash.export

Export clash results to a structured file — the "ACC Issues attachment" / consultant-deliverable primitive. Supports BCF (preferred for round-trip), XML (Navisworks legacy), HTML, and CSV.

Read-only.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `document-id` | string | yes | The open federation. |
| `test-names` | array<string> | no | Tests to export. Empty = all. |
| `format` | enum `bcf\|xml\|html\|csv` | no (default `bcf`) | Output format. |
| `output-path` | string | yes | Output file path. |

## Output

```yaml
path:        "C:/issue/clashes.bcfzip"
clash-count: 56
size-bytes:  184320
```

## Implementation (Navisworks .NET API · Navisworks Manage)

> **Important — read before composing.** The Clash .NET API has **no native results-export** to BCF / XML / HTML / CSV. Per Autodesk's own forum answer: *"Navisworks API does not provide the functions for exporting to HTML and XML, but it does provide the means to extract all information about the clash result (Element IDs, paths, clash points, etc.) **except for the clash image**."* So this verb **walks the `ClashResult`s and serializes the chosen format itself** — it does not call a Navisworks export service. A doc/implementation implying a native exporter would be fabricating an API. (The Clash Detective *UI* can export XML/HTML, but that path is not in the .NET API.)

1. Resolve the tests (all, or those named in `test-names`) via `DocumentClash.TestsData.Tests`.
2. For each `ClashResult` (recurse `ClashResultGroup`s) collect: `DisplayName`, `Status`, `Description`, `ApprovedBy`, `CreatedTime`, `Center`, `BoundingBox`, and the two items' source ids from `Item1`/`Item2` (`FindPropertyByDisplayName(...)` — Revit → `Element ID`, DWG → `Entity Handle`; try both).
3. Serialize to the requested format in code: CSV / XML / HTML are straightforward; **BCF** means writing a valid BCFZIP (markup + viewpoint per topic) yourself.

## Gotchas

- **No native exporter — you write the file.** This is the load-bearing fact; budget for a real BCF/XML/HTML/CSV serializer in the sidecar.
- **BCF specifically has no native writer in the Clash API** — treat BCF support as "implement the BCFZIP yourself" (pair with the `bcf-roundtrip` skill / the `bcf-file` agent). It is the most work of the four formats.
- **No clash *image* is available from the .NET API** — a BCF topic's snapshot must come from `viewpoint.from-clash` (which itself needs a live view), not from the clash result.
- **Source-id property differs by source** (`Element ID` for Revit, `Entity Handle` for DWG) — probe both. Some teams fall back to the COM clash API (`InwOclTestResult.Path1/Path2`) for reliable item paths.
- **Manage-only.**

Sources: [How to export clash results to HTML/XML — "the API does not provide" (Autodesk forum, verbatim)](https://forums.autodesk.com/t5/navisworks-api-forum/how-to-export-clash-test-results-to-html-and-xml-using/td-p/8924705) · [xiaodongliang sample — walking results + source-id extraction (.NET + COM)](https://github.com/xiaodongliang/Forge-Navisworks-ClashTest/blob/master/Navisworks%20Plugin/Class1.cs) · [Community hand-rolled clash exporter](https://github.com/flyingturtle13/Navis-Clash_Data_Exporter)

## See also

- [clash.run-all](./clash.run-all.md) — run tests first
- [viewpoint.from-clash](./viewpoint.from-clash.md) — the per-clash snapshot a BCF topic needs
- [bcf-roundtrip](../skills/bcf-roundtrip.md) — BCF authoring; pairs with the `bcf-file` agent
