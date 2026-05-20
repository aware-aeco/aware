# federation.open

Open a Navisworks federation document (`.nwf`, `.nwd`, or `.nwc`). Required pre-flight for every other Navisworks verb in a run — they all operate on the document this verb opens.

Read-only.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `path` | string | yes | Absolute or workspace-relative path to the `.nwf` / `.nwd` / `.nwc`. |

## Output

```yaml
document-id: "nwd-3f1c…"
model-count: 7
item-count:  148230
version:     "2026"
```

## Implementation (Navisworks .NET API)

The .NET API is four assemblies — `Autodesk.Navisworks.Api` (most functionality), `.Automation`, `.Controls`, `.ComApi` — usable three ways: an in-process **AddInPlugin** (runs inside Navisworks Manage/Simulate against `Application.ActiveDocument`), the out-of-process **Automation** API (`NavisworksApplication`, drives a hidden Navisworks), or the **Controls** engine. A realistic AWARE sidecar uses an **in-process AddInPlugin inside Navisworks Manage** driven over local IPC — same in-process-addin pattern as this repo's revit/tekla exec agents (it gives the full Clash/Timeliner surface and a live view for image capture; the Automation path is licensing-fragile and lacks an interactive view). See the `federation-and-coordinates` skill.

1. Get the document — `Application.ActiveDocument` in a plug-in, or `new NavisworksApplication()` under Automation.
2. Open: `document.OpenFile(path)`.
3. Read the shape from `document.Models` (a collection of `Model`): `Models.Count` is the model count; total item count comes from walking the model trees (`Model.RootItem` → descendants).

## Gotchas

- **`OpenFile` replaces the current document — it does not append.** Use `federation.append-model` to add to an already-open federation. A bad path throws `DocumentFileException`.
- **`.nwf` vs `.nwd` vs `.nwc`.** `.nwf` is a federation *reference* (source models stay external and are re-read on open — their paths must resolve, slower); `.nwd` is a consolidated snapshot; `.nwc` is a single cache. Opening an `.nwf` whose referenced models moved yields a partially-loaded federation — check `model-count` against expectation.
- **Open is the mandatory first node.** Every other verb takes a `document-id`; compose this node first and thread it downstream.
- **Exact `ModelItemCount` property name is unverified** against the SDK reference — the confirmed surface is `doc.Models[0]` and `doc.Models.Count`; derive the item total by walking descendants.

Sources: [Navisworks API overview — assemblies + plug-in/automation/controls (APS)](https://aps.autodesk.com/developer/overview/navisworks-api) · [AU 2012 .NET API class handout (Simon Bee, Autodesk)](https://static.au-uw2-prd.autodesk.com/CP2170_handout_2170_au_2012_class_navisworks_simon_bee.pdf) · [phusband/NavisAutomation — OpenFile + DocumentFileException](https://github.com/phusband/NavisAutomation/blob/master/Navisworks.cs)

## See also

- [federation.append-model](./federation.append-model.md) — add a reference model to the open federation
- [federation.save](./federation.save.md) — persist as `.nwf` or `.nwd`
- [federation-and-coordinates](../skills/federation-and-coordinates.md) — execution model, shared coordinates, `.nwf`/`.nwd`/`.nwc` semantics
