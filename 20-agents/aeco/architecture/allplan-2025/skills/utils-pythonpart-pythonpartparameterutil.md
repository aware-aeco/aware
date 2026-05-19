---
name: allplan-utils-pythonpart-pythonpartparameterutil
description: This skill encodes the allplan 2025.0 surface of the Utils.PythonPart.PythonPartParameterUtil namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: PythonPartParameterUtil.
---

# Utils.PythonPart.PythonPartParameterUtil

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## PythonPartParameterUtil (class)

implementation of the PythonPart parameter utilities

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/Utils/PythonPart/PythonPartParameterUtil/)

### Constructors
- `PythonPartParameterUtil( python_part_ele: BaseElementAdapter, str_table: BuildingElementStringTable, material_str_table: BuildingElementMaterialStringTable, )` — initialize

### Methods
#### `get_label_data( doc: DocumentAdapter, ) -> tuple[ParameterValueList, list[BuildingElement]]`

get the data for a label from the parameters

**Remarks:** get the data for a label from the parameters

**Parameters:**
- `doc` (DocumentAdapter) — document of the Allplan drawing files

**Returns:** `tuple[ParameterValueList, list[BuildingElement]]` — parameter value data, building element list

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/PythonPart/PythonPartParameterUtil/#Utils.PythonPart.PythonPartParameterUtil.PythonPartParameterUtil.get_label_data)

