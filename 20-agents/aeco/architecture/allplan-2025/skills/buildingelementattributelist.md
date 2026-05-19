---
name: allplan-buildingelementattributelist
description: This skill encodes the allplan 2025.0 surface of the BuildingElementAttributeList namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: BuildingElementAttributeList.
---

# BuildingElementAttributeList

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## BuildingElementAttributeList (class)

Implementation of the attribute list

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementAttributeList/)

### Constructors
- `BuildingElementAttributeList()` — initialize

### Methods
#### `__iadd__(other: BuildingElementAttributeList) -> BuildingElementAttributeList`

implement the += operator

**Remarks:** implement the += operator

**Parameters:**
- `other` (BuildingElementAttributeList) — attribute list to add

**Returns:** `BuildingElementAttributeList` — attribute list

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementAttributeList/#BuildingElementAttributeList.BuildingElementAttributeList.__iadd__)

#### `__repr__() -> str`

create the data as string

**Remarks:** create the data as string

**Returns:** `str` — data string

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementAttributeList/#BuildingElementAttributeList.BuildingElementAttributeList.__repr__)

#### `add_attribute(attribute_id: int, attribute_value: Any)`

add an attribute to the list

**Remarks:** add an attribute to the list

**Parameters:**
- `attribute_id` (int) — attribute ID
- `attribute_value` (Any) — attribute value

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementAttributeList/#BuildingElementAttributeList.BuildingElementAttributeList.add_attribute)

#### `add_attribute_by_unit(attribute_id: int, attribute_value: Any)`

add an attribute to the list, convert the value to the attribute unit

**Remarks:** add an attribute to the list, convert the value to the attribute unit

**Parameters:**
- `attribute_id` (int) — attribute ID
- `attribute_value` (Any) — attribute value

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementAttributeList/#BuildingElementAttributeList.BuildingElementAttributeList.add_attribute_by_unit)

#### `add_attribute_list(attribute_list: list[Attribute])`

add an attribute list

**Remarks:** add an attribute list

**Parameters:**
- `attribute_list` (list[Attribute]) — attribute list

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementAttributeList/#BuildingElementAttributeList.BuildingElementAttributeList.add_attribute_list)

#### `add_attributes(attributes: list[tuple[int, Any]])`

add attributes to the list

**Remarks:** add attributes to the list

**Parameters:**
- `attributes` (list[tuple[int, Any]]) — attribute list

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementAttributeList/#BuildingElementAttributeList.BuildingElementAttributeList.add_attributes)

#### `add_attributes_by_unit(attributes: list[tuple[int, Any]])`

add attributes to the list, convert the value to the attribute unit

**Remarks:** add attributes to the list, convert the value to the attribute unit

**Parameters:**
- `attributes` (list[tuple[int, Any]]) — attribute list

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementAttributeList/#BuildingElementAttributeList.BuildingElementAttributeList.add_attributes_by_unit)

#### `add_attributes_from_parameters(build_ele: BuildingElement)`

add the attributes from the parameter attributes

**Remarks:** add the attributes from the parameter attributes

**Parameters:**
- `build_ele` (BuildingElement) — building element with the parameter properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementAttributeList/#BuildingElementAttributeList.BuildingElementAttributeList.add_attributes_from_parameters)

#### `get_attribute_list() -> list[Attribute]`

get the attribute list

**Remarks:** get the attribute list

**Returns:** `list[Attribute]` — attribute list

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementAttributeList/#BuildingElementAttributeList.BuildingElementAttributeList.get_attribute_list)

#### `get_attributes_list_as_tuples() -> list[tuple[int, Any]]`

Returns a list of attribute ids and values grouped as a tuple.

**Remarks:** Returns a list of attribute ids and values grouped as a tuple.

**Returns:** `list[tuple[int, Any]]` — list[tuple[int, Any]]: list of attribute ids and values as a tuple.

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementAttributeList/#BuildingElementAttributeList.BuildingElementAttributeList.get_attributes_list_as_tuples)

