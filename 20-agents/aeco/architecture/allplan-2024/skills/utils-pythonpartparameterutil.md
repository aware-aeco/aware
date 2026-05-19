---
name: allplan-utils-pythonpartparameterutil
description: This skill encodes the allplan 2024.0 surface of the Utils.PythonPartParameterUtil namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: PythonPartParameterUtil.
---

# Utils.PythonPartParameterUtil

Auto-generated from vendor docs for allplan 2024.0. 1 types in this namespace.

## PythonPartParameterUtil (class)

implementation of the PythonPart parameter utilities

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/Utils/PythonPartParameterUtil/PythonPartParameterUtil/)

### Constructors
- `PythonPartParameterUtil(python_part_ele, str_table_service)` — initialize

### Methods
#### `__create_attribute_id_label_data(doc, prop, ctrl_props, label_data, prop_pal_ctrl_service)`

create the label data for the value type AttributeID

**Remarks:** create the label data for the value type AttributeID

**Parameters:**
- `doc` (DocumentAdapter) — document of the Allplan drawing files
- `prop` (ParameterProperty) — parameter property
- `ctrl_props` (ControlProperties) — control properties
- `label_data` (ParameterValueList) — label data
- `prop_pal_ctrl_service` (PropertyPaletteControlService) — property palette control service

[Docs](https://pythonparts.allplan.com/2024/api_reference/Utils/PythonPartParameterUtil/PythonPartParameterUtil/#Utils.PythonPartParameterUtil.PythonPartParameterUtil.__create_attribute_id_label_data)

#### `get_label_data(doc)`

get the data for a label from the parameters

**Remarks:** get the data for a label from the parameters

**Parameters:**
- `doc` (DocumentAdapter) — document of the Allplan drawing files

**Returns:** `ParameterValueList` — parameter value data

[Docs](https://pythonparts.allplan.com/2024/api_reference/Utils/PythonPartParameterUtil/PythonPartParameterUtil/#Utils.PythonPartParameterUtil.PythonPartParameterUtil.get_label_data)

