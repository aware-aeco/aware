# federation.append-model

Append a reference model (`.rvt` / `.ifc` / `.dwg` / `.dgn` / `.nwc`) to the active federation. Uses the same model readers as the UI's *Append*.

**WRITE-mode.**

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `document-id` | string | yes | The open federation (from `federation.open`). |
| `path` | string | yes | Source model to append. |
| `mode` | enum `append\|merge\|replace` | no (default `append`) | Intended combine mode. **See the gotcha** — the .NET API does not expose this switch. |

## Output

```yaml
added-model: "struct.ifc"
item-count-delta: 5120
```

## Implementation (Navisworks .NET API)

Append uses `Document.AppendFile(path)` (in-process), `NavisworksApplication.AppendFile(path)` (Automation), or `Document.TryMergeFiles(string[] sources)` to merge several files into the controlled document at once. The source format is handled by the matching model reader (`.rvt`/`.ifc`/`.dwg`/`.dgn`/`.nwc`).

```csharp
doc.AppendFile(@"C:\models\struct.ifc");   // repeat per file, or TryMergeFiles(new[]{a,b})
```

Compute `item-count-delta` by snapshotting the item total before and after the append.

## Gotchas

- **Append does not de-duplicate.** Navisworks will append the same file more than once without warning. Guard against re-appending by checking the already-loaded source files first (via the COM `InwOaPartition.FileName` walk).
- **`mode: append/merge/replace` and shared coordinates are NOT exposed by the .NET API.** No documented surface toggles append-vs-overlay or a coordinate mode — coordinates follow the file readers' defaults. Treat the `mode` input as advisory; do not claim a `.NET` "shared coordinates" switch. (`replace`/`merge` semantics would have to be emulated — e.g. remove-then-append.)
- **IFC append can populate the tree but render empty geometry** on some Navisworks versions (a known Autodesk issue) — verify geometry, not just that tree nodes appeared.
- **Save to persist.** `AppendFile` mutates the in-memory federation; follow with `federation.save` (`.nwf` keeps the appended files external; `.nwd` consolidates them).

Sources: [AU 2012 handout — `app.AppendFile` (Automation)](https://static.au-uw2-prd.autodesk.com/CP2170_handout_2170_au_2012_class_navisworks_simon_bee.pdf) · [phusband/NavisAutomation — `TryMergeFiles`](https://github.com/phusband/NavisAutomation/blob/master/Navisworks.cs) · [Append + de-dupe via `InwOaPartition` (Autodesk forum)](https://forums.autodesk.com/t5/navisworks-api/appending-an-ifc-file-using-appendfile/td-p/4460897) · [IFC-disappears-on-append (Autodesk TS)](https://www.autodesk.com/support/technical/article/caas/sfdcarticles/sfdcarticles/Disappearing-IFC-at-appending-IFC-to-Navisworks-2019.html)

## See also

- [federation.open](./federation.open.md) — open the base federation first
- [federation.save](./federation.save.md) — persist the appended result
- [federation-and-coordinates](../skills/federation-and-coordinates.md) — coordinate behavior + the readers
