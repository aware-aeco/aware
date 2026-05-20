# report.export-quantities

Export a quantities / bill-of-materials table for elements matching a trade filter, to CSV / XLSX / XML for downstream cost take-off.

Read-only with respect to the model (it produces an output file).

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `filter-trade` | enum `architecture\|structure\|mep\|civil\|all` | no (default `all`) | Restrict the take-off to one trade. |
| `out-path` | string | yes | Output file path. |
| `format` | enum `csv\|xlsx\|xml` | no (default `csv`) | Output serialization. |

## Output

```yaml
out-path:  "C:/takeoff/structure-q.csv"
row-count: 128
bytes:     20413
```

## Implementation (Allplan Python API)

> **Important — read this before composing the verb.** Allplan exposes **no Python API that renders its native reports** (`.rdlc` templates) or runs the Nevaris/BCM bill-of-quantities engine. The Python Import/Export surface covers geometry/layouts only (IFC, SKP, OBJ, DWG, PDF). So this verb does **not** call an Allplan "report" service — it **reads the quantity attributes itself and serializes them**. A doc or implementation that implied an Allplan report engine would be fabricating an API.

The verb computes the take-off in three steps, in-process:

1. **Select + trade-filter** the elements (the `element.list-by-trade` machinery).
2. **Read quantity attributes** per element with the *computable* read-state, which pulls computed quantities (area / volume / length / count / material), and resolve names + units:

   ```python
   import NemAll_Python_BaseElements as AllplanBaseEle
   attrs = ele.GetAttributes(AllplanBaseEle.eAttibuteReadState.ReadAllAndComputable)
   for attr_id, value in attrs:
       name = AllplanBaseEle.AttributeService.GetAttributeName(doc, attr_id)
       unit = AllplanBaseEle.AttributeService.GetAttributeUnit(doc, attr_id)
   ```

   For geometric quantities you can also compute directly from the element's solid: `NemAll_Python_Geometry.CalcMass(solid)` returns volume / surface area / centre of gravity, and the length helpers cover curves (see the `GeometryExamples/Operations/QuantityTakeOff` examples).

3. **Aggregate + serialize** in plain Python — `csv`, `xml.etree`, or an XLSX writer. Allplan is not involved in this step.

## Gotchas

- **Do not promise native Allplan report output.** The verb computes and writes the table itself; it does not run `.rdlc` / Nevaris. Keep the contract honest.
- **`ReadAll` omits computed quantities.** Use `eAttibuteReadState.ReadAllAndComputable` for take-off, otherwise area/volume attributes may be missing.
- **Quantity values are localized and unit-bearing.** `GetAttributeUnit(doc, id)` gives the unit; enum/area/volume attributes come back already formatted/resolved — parse before arithmetic.
- **`CalcMass` needs a real solid.** Architecture objects must be converted to model geometry first (`GetModelGeometry()` / `GetElement`); it won't operate on a bare adapter.
- **Trade filtering is attribute-based** (see `element.list-by-trade`) — same locale caveats apply.

Sources: [Import/Export manual (no report engine — IFC/SKP/OBJ/DWG/PDF only)](https://pythonparts.allplan.com/2025/manual/features/import_export/) · [AttributeService (names + units)](https://pythonparts.allplan.com/2025/api_reference/InterfaceStubs/NemAll_Python_BaseElements/AttributeService/) · [element attributes (read states, tuple format)](https://pythonparts.allplan.com/2024/manual/features/attributes/element_attributes/) · [QuantityTakeOff examples (`CalcMass.py`, `CalcLength.py`)](https://github.com/NemetschekAllplan/PythonPartsExamples)

## See also

- [element.list-by-trade](./element.list-by-trade.md) — the selection + trade-filter front half
- [nemall_python_baseelements-attributeservice](../skills/nemall_python_baseelements-attributeservice.md) — attribute id → name / unit
- [view.export-plans-to-pdf](./view.export-plans-to-pdf.md) — the other "export an artifact" verb (which *does* use a native Allplan service)
