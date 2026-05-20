# element.list-by-trade

List elements in the active Allplan document filtered by **trade** (architecture / structure / MEP / civil) and, optionally, by a regex matched against the element's **layer name**. Returns `id`, `type`, `layer`, and `trade` per element — the audit primitive for "show me every structural element" or "every MEP object on a `*-DRAFT` layer".

Read-only.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `trade` | enum `architecture\|structure\|mep\|civil\|all` | no (default `all`) | The trade to filter on. Resolved against the element's **Trade attribute**, not its layer. |
| `layer-filter` | string | no | Optional regex matched against the element's resolved layer name. |

## Output

```yaml
count: 128
elements:
  - id:    "9f1c…-uuid"
    type:  "Wall"
    layer: "AR_WALL_EXT"
    trade: "Concreting work"
```

## Implementation (Allplan Python API)

Allplan automation is **in-process**: there is no out-of-process/REST surface. A script runs inside a live Allplan session as a PythonPart. Because this verb reads *existing* model elements, it runs as an **interactor** PythonPart (an `<Interactor>True</Interactor>` `.pyp` whose `.py` defines `create_interactor(...)`) — only the interactor shape can touch elements already in the document; a standard palette PythonPart only sees its own parameters.

The verb composes three things: select all elements, read each element's type + layer + attributes, then filter. **Trade is an element *attribute* (German "Gewerk"), not a layer property** — this is the load-bearing fact.

1. **Select** every element in the active document — non-interactive, no picking:

   ```python
   import NemAll_Python_BaseElements as AllplanBaseEle
   elements = AllplanBaseEle.ElementsSelectService.SelectAllElements(doc)   # BaseElementAdapterList
   ```

2. Per element (`BaseElementAdapter`), read the three axes:

   ```python
   type_name = ele.GetElementAdapterType().GetTypeName()          # "Wall", "Slab", …
   uuid      = str(ele.GetElementUUID())
   layer_id  = ele.GetCommonProperties().Layer                    # an integer id, NOT a name
   layer     = AllplanBaseEle.LayerService.GetNameByID(layer_id, doc.GetDocumentID())
   attrs     = dict(ele.GetAttributes(AllplanBaseEle.eAttibuteReadState.ReadAll))
   ```

3. **Resolve the Trade attribute id by name** rather than hard-coding it, then read the value:

   ```python
   trade_id = AllplanBaseEle.AttributeService.GetAttributeID(doc, "Trade")   # do NOT hard-code 209
   trade    = attrs.get(trade_id)
   ```

4. Keep elements whose `trade` matches the requested trade and (if given) whose `layer` matches `layer-filter`. Emit `{ id, type, layer, trade }`.

For an interactive, pre-filtered pick instead of "whole document", the verified pattern is a `SelectionQuery([predicate])` → `ElementSelectFilterSetting(query, True)` → `InputFunctionStarter.StartElementSelect(...)`, where the predicate inspects `element.GetAttributes(...)`.

## Gotchas

- **Trade ≠ layer.** `trade` comes from the element's attributes; the layer comes from `CommonProperties.Layer`. They are independent filter axes — the manifest exposes both deliberately.
- **Don't hard-code the Trade attribute id.** It is widely reported as **209**, but that literal is not confirmed in the current docs; resolve it at run time with `AttributeService.GetAttributeID(doc, "Trade")` (or the localized "Gewerk"). Hard-coding risks silent breakage across locales/versions.
- **Trade returns a *localized, resolved* string.** Enum attributes come back as their value ("Concreting work" / "Betonarbeiten"), not a numeric key — so the `trade` enum must be mapped to Allplan's trade values, and matching is locale-sensitive.
- **`Layer` is an integer id.** You must call `LayerService.GetNameByID(layer_id, documentID)` before a name regex can be applied.
- **`SelectAllElements` spans every open drawing file**, not just one. Scope with `ele.IsInActiveDocument()` / the `DocumentAdapter` if only the active file is wanted.

Sources: [ElementsSelectService](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/ElementsSelectService/) · [BaseElementAdapter](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_IFW_ElementAdapter/BaseElementAdapter/) · [LayerService](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/LayerService/) · [AttributeService](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeService/) · [element attributes (enum-value behavior)](https://pythonparts.allplan.com/2024/manual/features/attributes/element_attributes/)

## See also

- [nemall_python_baseelements-attributeservice](../skills/nemall_python_baseelements-attributeservice.md) — attribute id ↔ name/unit resolution
- [nemall_python_baseelements](../skills/nemall_python_baseelements.md) — selection + layer services
- [baseinteractor](../skills/baseinteractor.md) — the interactor PythonPart shape this verb needs
- [layer.set-by-pattern](./layer.set-by-pattern.md) — the write-side counterpart that acts on layer membership
