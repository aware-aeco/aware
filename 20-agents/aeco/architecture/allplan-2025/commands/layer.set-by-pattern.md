# layer.set-by-pattern

Move every element whose **current layer name matches a regex** onto a **target layer** (which should already exist — see the layer-creation gotcha). Returns the target layer id, the number of elements moved, and whether the target had to be created — the bulk "re-home everything on `*_TEMP` onto `AR_WALL`" primitive.

**WRITE-mode.**

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `from-pattern` | string | yes | Regex matched against each element's resolved layer name. |
| `to-layer` | string | yes | Target layer (short name) to move matching elements onto. |
| `to-layer-color-aci` | number | no (default `7`) | Default colour for the target layer. **See the gotcha** — Allplan has no ACI palette; this maps onto Allplan's integer Colour id. |

## Output

```yaml
target-layer-id: "312"
moved-count:     47
created-target:  false
```

## Implementation (Allplan Python API)

Runs as an **interactor** PythonPart (it mutates existing elements, which the standard palette shape cannot do). The read/match half is fully supported; the **create + reassign half has documented gaps you must respect** (see Gotchas).

1. **Enumerate + match** (verified — same machinery as `element.list-by-trade`):

   ```python
   import NemAll_Python_BaseElements as AllplanBaseEle
   doc_id = doc.GetDocumentID()
   for ele in AllplanBaseEle.ElementsSelectService.SelectAllElements(doc):
       name = AllplanBaseEle.LayerService.GetNameByID(ele.GetCommonProperties().Layer, doc_id)
       if re.search(from_pattern, name):
           matched.append(ele)
   ```

2. **Resolve the target layer id** by short name:

   ```python
   target_id = AllplanBaseEle.LayerService.GetIDByShortName(to_layer, doc)
   ```

3. **Reassign** the matched elements to `target_id`. The element's layer lives in `CommonProperties.Layer` (a *writable* int); the documented bulk-mutation path for format attributes on existing elements is `ElementsAttributeService.ChangeAttribute(attributeNumber, newValue, elements)`. **Confirm the exact reassignment call against an `ArchitectureExamples/ModifyObjects/*.py` example before relying on it** — the docs do not show a worked "move existing element to another layer" sample, so treat the write call as needs-confirmation.

## Gotchas

- **Layer *creation* has no documented Python API — this is the soft spot.** `LayerService` exposes name/id lookups but no `Create`/`Add`; `AllplanGlobalSettings` has only `GetCurrentLayerId` (no setter). The realistic way to provision a layer is to import a **layer favorite** with `LayerService.LoadFromFavoriteFile(doc, fileName)`. Until that path is confirmed end-to-end, **do not assume `created-target: true` is achievable** — if the target layer is absent, `GetIDByShortName` behaviour (return 0 / invalid vs. create) is unverified, and the verb may need to fail with "target layer must already exist".
- **"ACI color" is not Allplan terminology.** ACI (AutoCAD Color Index) is an AutoCAD concept. Allplan's format model is **Pen / Stroke / Colour** integer ids plus per-element `ColorByLayer` / `PenByLayer` / `StrokeByLayer` flags. `to-layer-color-aci` therefore maps onto Allplan's integer **Colour** id; `CommonProperties.GetColorPenStrokeByLayerFromLayerNumber(layerNumber)` returns a layer's default `[pen, stroke, color]`.
- **`...ByLayer` flags change appearance implicitly.** If a moved element has `ColorByLayer = True`, relocating it to a new layer silently changes its displayed colour. Worth surfacing in the result.
- **Layer is an id, not a name, end-to-end.** Every match needs `LayerService.GetNameByID(id, documentID)` against the correct `documentID`.
- **Adapters are read-views.** You cannot mutate through a `BaseElementAdapter` directly; the change must go through a documented mutating service and be re-written to the model.

Sources: [LayerService](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/LayerService/) · [CommonProperties](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/CommonProperties/) · [ElementsAttributeService](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ElementsAttributeService/) · [AllplanGlobalSettings](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_AllplanSettings/AllplanGlobalSettings/) · [ModifyObjects examples](https://github.com/NemetschekAllplan/PythonPartsExamples)

## See also

- [element.list-by-trade](./element.list-by-trade.md) — the read-side counterpart (same enumerate + layer-resolve machinery)
- [nemall_python_baseelements](../skills/nemall_python_baseelements.md) — LayerService / ElementsAttributeService
- [baseinteractor](../skills/baseinteractor.md) — the interactor PythonPart shape required to mutate existing elements
