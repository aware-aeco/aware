# view.export-plans-to-pdf

Export sheet plans (layouts) in the project to PDF — one combined file or one per plan — optionally filtered by a plan-name regex. The "issue the floor-plan pack" primitive.

Read-only with respect to the model (it produces PDF files).

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `out-path` | string | yes | PDF file path, or a directory when `one-per-plan` is true. |
| `one-per-plan` | boolean | no (default `false`) | One PDF per plan vs. a single combined PDF. |
| `dpi` | number | no (default `300`) | Target resolution. **See the gotcha** — Allplan takes this from a print config, not as an argument. |
| `plan-name-filter` | string | no | Regex; only matching plans are exported. |

## Output

```yaml
exported-count: 6
files:
  - "C:/issue/A-101.pdf"
  - "C:/issue/A-102.pdf"
```

## Implementation (Allplan Python API)

Unlike reports, **plan→PDF export *is* exposed** via `NemAll_Python_BaseElements.LayoutFileService`:

```python
import NemAll_Python_BaseElements as AllplanBaseEle
AllplanBaseEle.LayoutFileService.ExportPDF(doc, layout_file_index, r"C:\issue\A-101.pdf", config_file_name)
```

`ExportPDF(doc, layoutFileIndex, fileName, configFileName)` exports one layout (addressed by an **integer `layoutFileIndex`**) to a PDF. A "plan/layout" is a `LayoutMasterData` object (sheet size, border, stamp, legend); the service operates on layout *files* by index. Companion calls on the same service: `ExportDWG`, `LoadFile`, `AssignPrintProfile`, `CreateMasterLayoutElement`.

Realistic flow: operate on the **layout document** (not a drawing-file/model document); for each target layout index, optionally `LoadFile(...)` + `AssignPrintProfile(...)`, then `ExportPDF(...)`. For `one-per-plan`, call once per index into the `out-path` directory; for a combined PDF, see the gotcha on combined output.

## Gotchas

- **DPI is not an argument — it rides in the print config.** `ExportPDF` has no resolution parameter; output resolution is governed by `configFileName` (a print/plot configuration) and/or the assigned print profile (`AssignPrintProfile`). Honour `dpi` by selecting/preparing the right config file, not by passing a number to the API.
- **Plan-name regex needs a name→index lookup that is not pinned in the docs.** `ExportPDF` addresses layouts by integer index; `DocumentAdapter` is too thin to enumerate layout names. Enumerating plan names (to apply `plan-name-filter`) likely needs `DrawingFileService` / `DocumentNameService` — **confirm before relying on the regex filter.**
- **Combined vs. one-per-plan is unverified at the API level.** The signature takes a single index; whether a combined multi-plan PDF is one call or a post-merge of per-plan PDFs is not documented — verify before promising combined output.
- **You must be on a layout document.** Allplan separates "drawing files" (model) from "layout files" (plans); calling `ExportPDF` in the wrong document context fails.
- **`ExportPDF`'s return type is undocumented.** Don't assume a status code — confirm success by checking the output file exists and is non-empty.

Sources: [LayoutFileService (`ExportPDF`)](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/LayoutFileService/) · [LayoutMasterData](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/LayoutMasterData/) · [DocumentAdapter](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/DocumentAdapter/)

## See also

- [report.export-quantities](./report.export-quantities.md) — the other export verb (computed, not a native engine)
- [nemall_python_baseelements](../skills/nemall_python_baseelements.md) — LayoutFileService and the layout/drawing document split
