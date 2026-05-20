# federation.save

Save the active federation as `.nwf` (federation reference — appended models stay external) or `.nwd` (consolidated single file).

**WRITE-mode.**

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `document-id` | string | yes | The open federation. |
| `path` | string | yes | Output path. The extension determines the format. |
| `format` | enum `nwf\|nwd` | no (default `nwf`) | Should agree with the `path` extension. |

## Output

```yaml
path:       "C:/coord/tower.nwd"
size-bytes: 48213004
```

## Implementation (Navisworks .NET API)

Two paths:
- **`Document.SaveFile(path, DocumentFileVersion)`** — the general save; the `DocumentFileVersion` enum pins the on-disk version (`Navisworks2010…`, `Current`).
- **`Document.PublishFile(path, PublishProperties)`** — the publish-to-`.nwd` path (`PublishProperties`: Author / Title / Comments / PublishDate / AllowResave).

`.nwf` vs `.nwd` is selected by the **file extension** of `path` (`.nwf` saves references only; `.nwd` consolidates geometry into one file). This extension-drives-format behavior is the consistent, observed mechanism but is not stated verbatim in the docs — treat it as strongly-implied, and keep `format` aligned with the extension.

**Navisworks 2026 adds** a dedicated NWD export: `Document.ExportToNwd(path, NwdExportOptions)` (and a `PublishFile` overload taking `NwdExportOptions { ExcludeHiddenItems, PreventObjectPropertyExport, FileVersion, EmbedXrefs }`) — supports "only visible items / only visible models". Guard it by version (2026+).

## Gotchas

- **`.nwf` is fragile off-machine.** An `.nwf` saved where the source models later go missing won't reload geometry — prefer `.nwd` for a deliverable/archive.
- **`PublishFile` with `AllowResave = false` locks the `.nwd`** against further saving.
- **`ExportToNwd` / `NwdExportOptions` is 2026+ only** — calling it on an earlier build fails; gate by `Application` version.
- **`SaveFile` takes `DocumentFileVersion`; `PublishFile` does not** (it takes `PublishProperties`). Don't cross the two signatures.

Sources: [AU 2012 handout — `SaveFile` / `PublishFile`](https://static.au-uw2-prd.autodesk.com/CP2170_handout_2170_au_2012_class_navisworks_simon_bee.pdf) · [phusband/NavisAutomation — `PublishFile` + `PublishProperties` + `DocumentFileVersion`](https://github.com/phusband/NavisAutomation/blob/master/Navisworks.cs) · [Autodesk dev blog — `NwdExportOptions` in Navisworks 2026](https://blog.autodesk.io/navisworks-api-introducing-nwdexportoptions-in-navisworks-2026/)

## See also

- [federation.open](./federation.open.md) · [federation.append-model](./federation.append-model.md)
- [publish-and-export](../skills/publish-and-export.md) — `.nwf`/`.nwd` semantics + the 2026 export options
