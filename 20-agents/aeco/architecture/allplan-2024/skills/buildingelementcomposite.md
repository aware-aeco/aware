---
name: allplan-buildingelementcomposite
description: This skill encodes the allplan 2024.0 surface of the BuildingElementComposite namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: BuildingElementComposite.
---

# BuildingElementComposite

Auto-generated from vendor docs for allplan 2024.0. 1 types in this namespace.

## BuildingElementComposite (class)

Implementation of class BuildingElementComposite

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementComposite/BuildingElementComposite/)

### Methods
#### `get_build_ele_index(build_ele_id)`

get the index of the building element by the ID

**Remarks:** get the index of the building element by the ID

**Parameters:**
- `build_ele_id` (str) — id fo find

**Returns:** `int` — index of the ID

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementComposite/BuildingElementComposite/#BuildingElementComposite.BuildingElementComposite.get_build_ele_index)

#### `get_composite_build_ele_list(sub_ele_index, build_ele_list)`

Get a tuple with the - list with the sub building elements - list with the indices of the sub building elements

**Remarks:** Get a tuple with the - list with the sub building elements - list with the indices of the sub building elements

**Parameters:**
- `sub_ele_index` (int) — Index of the sub element
- `build_ele_list` (List[BuildingElement]) — List with the building elements

**Returns:** `Tuple[List[BuildingElement], List[int]]` — tuple with the lists

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementComposite/BuildingElementComposite/#BuildingElementComposite.BuildingElementComposite.get_composite_build_ele_list)

#### `get_element_palette_data(element_id)`

Set the palette data for the element (control index, page text, expander text)

**Remarks:** Set the palette data for the element (control index, page text, expander text)

**Parameters:**
- `element_id` (str) — element ID

**Returns:** `PaletteData` — palette data

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementComposite/BuildingElementComposite/#BuildingElementComposite.BuildingElementComposite.get_element_palette_data)

#### `get_script(sub_ele_index)`

Get the script of the sub element Return: script

**Remarks:** Get the script of the sub element Return: script

**Parameters:**
- `sub_ele_index` (int) — Index of the sub element

**Returns:** `object` — script name

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementComposite/BuildingElementComposite/#BuildingElementComposite.BuildingElementComposite.get_script)

#### `get_sub_element_name(sub_ele_index)`

Get the name of the sub element Return: name

**Remarks:** Get the name of the sub element Return: name

**Parameters:**
- `sub_ele_index` (int) — Index of the sub element

**Returns:** `str` — sub element name

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementComposite/BuildingElementComposite/#BuildingElementComposite.BuildingElementComposite.get_sub_element_name)

#### `is_element_visible(ele_index, build_ele_list)`

Check for a visible element Return: the element is visible: True/False

**Remarks:** Check for a visible element Return: the element is visible: True/False

**Parameters:**
- `ele_index` (int) — element index
- `build_ele_list` (List[BuildingElement]) — Building element list

**Returns:** `bool` — element visible state

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementComposite/BuildingElementComposite/#BuildingElementComposite.BuildingElementComposite.is_element_visible)

#### `set_connection_persistence(build_ele_list)`

Set the persistence for the connected parameter

**Remarks:** Set the persistence for the connected parameter

**Parameters:**
- `build_ele_list` (List[BuildingElement]) — building element list

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementComposite/BuildingElementComposite/#BuildingElementComposite.BuildingElementComposite.set_connection_persistence)

#### `set_element_visible(ele_index, value)`

Set the visible state for the element

**Remarks:** Set the visible state for the element

**Parameters:**
- `ele_index` (int) — element index
- `value` (str) — value

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementComposite/BuildingElementComposite/#BuildingElementComposite.BuildingElementComposite.set_element_visible)

