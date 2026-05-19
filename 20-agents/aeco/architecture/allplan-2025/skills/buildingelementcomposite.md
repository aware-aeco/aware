---
name: allplan-buildingelementcomposite
description: This skill encodes the allplan 2025.0 surface of the BuildingElementComposite namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: BuildingElementComposite.
---

# BuildingElementComposite

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## BuildingElementComposite (class)

Implementation of class BuildingElementComposite

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementComposite/)

### Methods
#### `connect_building_element_values( build_ele_list: list[BuildingElement], end_index: int = -1, check_modified: bool = True, ) -> bool`

Connect the values of the building element from the element data Return: the connection is modified: True/False

**Remarks:** Connect the values of the building element from the element data Return: the connection is modified: True/False

**Parameters:**
- `build_ele_list` (list[BuildingElement]) — Building element list
- `end_index` (int) — end index
- `check_modified` (bool) — check modified state

**Returns:** `bool` — modified state

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementComposite/#BuildingElementComposite.BuildingElementComposite.connect_building_element_values)

#### `get_build_ele_index(build_ele_id: str) -> int`

get the index of the building element by the ID

**Remarks:** get the index of the building element by the ID

**Parameters:**
- `build_ele_id` (str) — id fo find

**Returns:** `int` — index of the ID

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementComposite/#BuildingElementComposite.BuildingElementComposite.get_build_ele_index)

#### `get_composite_build_ele_list( sub_ele_index: int, build_ele_list: list[BuildingElement] ) -> tuple[list[BuildingElement], list[int]]`

Get a tuple with the - list with the sub building elements - list with the indices of the sub building elements

**Remarks:** Get a tuple with the - list with the sub building elements - list with the indices of the sub building elements

**Parameters:**
- `sub_ele_index` (int) — Index of the sub element
- `build_ele_list` (list[BuildingElement]) — list with the building elements

**Returns:** `tuple[list[BuildingElement], list[int]]` — tuple with the lists

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementComposite/#BuildingElementComposite.BuildingElementComposite.get_composite_build_ele_list)

#### `get_element_palette_data(element_id: str) -> PaletteData`

Set the palette data for the element (control index, page text, expander text)

**Remarks:** Set the palette data for the element (control index, page text, expander text)

**Parameters:**
- `element_id` (str) — element ID

**Returns:** `PaletteData` — palette data

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementComposite/#BuildingElementComposite.BuildingElementComposite.get_element_palette_data)

#### `get_script(sub_ele_index: int) -> object`

Get the script of the sub element Return: script

**Remarks:** Get the script of the sub element Return: script

**Parameters:**
- `sub_ele_index` (int) — Index of the sub element

**Returns:** `object` — script name

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementComposite/#BuildingElementComposite.BuildingElementComposite.get_script)

#### `get_sub_element_name(sub_ele_index: int) -> str`

Get the name of the sub element Return: name

**Remarks:** Get the name of the sub element Return: name

**Parameters:**
- `sub_ele_index` (int) — Index of the sub element

**Returns:** `str` — sub element name

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementComposite/#BuildingElementComposite.BuildingElementComposite.get_sub_element_name)

#### `get_values_from_list(value_list: list[Any], sub_name: str) -> list[Any]`

get the values from a list

**Remarks:** get the values from a list

**Parameters:**
- `value_list` (list[Any]) — value list
- `sub_name` (str) — sub value name

**Returns:** `list[Any]` — value

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementComposite/#BuildingElementComposite.BuildingElementComposite.get_values_from_list)

#### `is_element_visible( ele_index: int, build_ele_list: list[BuildingElement] ) -> bool`

Check for a visible element Return: the element is visible: True/False

**Remarks:** Check for a visible element Return: the element is visible: True/False

**Parameters:**
- `ele_index` (int) — element index
- `build_ele_list` (list[BuildingElement]) — Building element list

**Returns:** `bool` — element visible state

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementComposite/#BuildingElementComposite.BuildingElementComposite.is_element_visible)

#### `set_connection_persistence(build_ele_list: list[BuildingElement])`

Set the persistence for the connected parameter

**Remarks:** Set the persistence for the connected parameter

**Parameters:**
- `build_ele_list` (list[BuildingElement]) — building element list

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementComposite/#BuildingElementComposite.BuildingElementComposite.set_connection_persistence)

#### `set_element_visible(ele_index: int, value: str)`

Set the visible state for the element

**Remarks:** Set the visible state for the element

**Parameters:**
- `ele_index` (int) — element index
- `value` (str) — value

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementComposite/#BuildingElementComposite.BuildingElementComposite.set_element_visible)

