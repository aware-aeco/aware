---
name: allplan-utils-pythonpart-modifypythonpartparameterutil
description: This skill encodes the allplan 2025.0 surface of the Utils.PythonPart.ModifyPythonPartParameterUtil namespace — 3 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Functions, ModifyPythonPartParameterUtil, ModifiedParameterData.
---

# Utils.PythonPart.ModifyPythonPartParameterUtil

Auto-generated from vendor docs for allplan 2025.0. 3 types in this namespace.

## Functions (static-class)

Module-level functions of Utils.PythonPart.ModifyPythonPartParameterUtil

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/Utils/PythonPart/ModifyPythonPartParameterUtil/)

## ModifiedParameterData (class)

data for a modified parameter

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/Utils/PythonPart/ModifyPythonPartParameterUtil/ModifiedParameterData/)

## ModifyPythonPartParameterUtil (class)

implementation of the utility class for the PythonPart parameter modification

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/Utils/PythonPart/ModifyPythonPartParameterUtil/ModifyPythonPartParameterUtil/)

### Methods
#### `execute( python_part: BaseElementAdapter, coord_input: CoordinateInput, mod_param_data: list[ModifiedParameterData], )`

execute the parameter modification

**Remarks:** execute the parameter modification

**Parameters:**
- `python_part` (BaseElementAdapter) — PythonPart to modify
- `coord_input` (CoordinateInput) — API object for the coordinate input, element selection, ... in the Allplan view
- `mod_param_data` (list[ModifiedParameterData]) — data of the modified parameter

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/PythonPart/ModifyPythonPartParameterUtil/ModifyPythonPartParameterUtil/#Utils.PythonPart.ModifyPythonPartParameterUtil.ModifyPythonPartParameterUtil.execute)

#### `get_modified_parameters( old_parameter: list[str], python_part_ele: BaseElementAdapter, str_table: BuildingElementStringTable, material_str_table: BuildingElementMaterialStringTable, ) -> list[ModifiedParameterData]`

get the modified parameters

**Remarks:** get the modified parameters

**Parameters:**
- `old_parameter` (list[str]) — old parameter
- `python_part_ele` (BaseElementAdapter) — PythonPart element
- `str_table` (BuildingElementStringTable) — global string table
- `material_str_table` (BuildingElementMaterialStringTable) — global material string table

**Returns:** `list[ModifiedParameterData]` — returns

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/PythonPart/ModifyPythonPartParameterUtil/ModifyPythonPartParameterUtil/#Utils.PythonPart.ModifyPythonPartParameterUtil.ModifyPythonPartParameterUtil.get_modified_parameters)

#### `get_overtake_parameters( python_part_ele: BaseElementAdapter, str_table: BuildingElementStringTable, material_str_table: BuildingElementMaterialStringTable, ) -> list[ModifiedParameterData]`

get the overtake parameters

**Remarks:** get the overtake parameters

**Parameters:**
- `python_part_ele` (BaseElementAdapter) — PythonPart element
- `str_table` (BuildingElementStringTable) — global string table
- `material_str_table` (BuildingElementMaterialStringTable) — global material string table

**Returns:** `list[ModifiedParameterData]` — overtake parameter

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/PythonPart/ModifyPythonPartParameterUtil/ModifyPythonPartParameterUtil/#Utils.PythonPart.ModifyPythonPartParameterUtil.ModifyPythonPartParameterUtil.get_overtake_parameters)

