---
name: allplan-utils-baseelementadapterfilter
description: This skill encodes the allplan 2025.0 surface of the Utils.BaseElementAdapterFilter namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: BaseElementAdapterFilter.
---

# Utils.BaseElementAdapterFilter

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## BaseElementAdapterFilter (class)

implementation of the base element adapter filter

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/Utils/BaseElementAdapterFilter/)

### Methods
#### `get_bar_definitions( elements: BaseElementAdapterList, ) -> list[BaseElementAdapter]`

get the elements for all type of bar definitions

**Remarks:** get the elements for all type of bar definitions

**Parameters:**
- `elements` (BaseElementAdapterList) — elements to filter

**Returns:** `list[BaseElementAdapter]` — filtered elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/BaseElementAdapterFilter/#Utils.BaseElementAdapterFilter.BaseElementAdapterFilter.get_bar_definitions)

#### `get_bar_placements( elements: BaseElementAdapterList, ) -> list[BaseElementAdapter]`

get the elements for all type of bar definitions

**Remarks:** get the elements for all type of bar definitions

**Parameters:**
- `elements` (BaseElementAdapterList) — elements to filter

**Returns:** `list[BaseElementAdapter]` — filtered elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/BaseElementAdapterFilter/#Utils.BaseElementAdapterFilter.BaseElementAdapterFilter.get_bar_placements)

#### `get_door_swing(elements: BaseElementAdapterList) -> list[BaseElementAdapter]`

get the elements for door swing

**Remarks:** get the elements for door swing

**Parameters:**
- `elements` (BaseElementAdapterList) — elements to filter

**Returns:** `list[BaseElementAdapter]` — filtered elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/BaseElementAdapterFilter/#Utils.BaseElementAdapterFilter.BaseElementAdapterFilter.get_door_swing)

#### `get_elements_by_type( elements: BaseElementAdapterList, type_uuid: GUID ) -> list[BaseElementAdapter]`

get the elements for the type

**Remarks:** get the elements for the type

**Parameters:**
- `elements` (BaseElementAdapterList) — elements to filter
- `type_uuid` (GUID) — element type

**Returns:** `list[BaseElementAdapter]` — filtered elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/BaseElementAdapterFilter/#Utils.BaseElementAdapterFilter.BaseElementAdapterFilter.get_elements_by_type)

#### `get_mesh_definitions( elements: BaseElementAdapterList, ) -> list[BaseElementAdapter]`

get the elements for all type of mesh definitions

**Remarks:** get the elements for all type of mesh definitions

**Parameters:**
- `elements` (BaseElementAdapterList) — elements to filter

**Returns:** `list[BaseElementAdapter]` — filtered elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/BaseElementAdapterFilter/#Utils.BaseElementAdapterFilter.BaseElementAdapterFilter.get_mesh_definitions)

#### `get_python_part(elements: BaseElementAdapterList) -> BaseElementAdapter`

get PythonPart

**Remarks:** get PythonPart

**Parameters:**
- `elements` (BaseElementAdapterList) — elements to filter

**Returns:** `BaseElementAdapter` — filtered element

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/BaseElementAdapterFilter/#Utils.BaseElementAdapterFilter.BaseElementAdapterFilter.get_python_part)

#### `get_slab_tiers(elements: BaseElementAdapterList) -> list[BaseElementAdapter]`

get the elements for all type of wall tiers definitions

**Remarks:** get the elements for all type of wall tiers definitions

**Parameters:**
- `elements` (BaseElementAdapterList) — elements to filter

**Returns:** `list[BaseElementAdapter]` — filtered elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/BaseElementAdapterFilter/#Utils.BaseElementAdapterFilter.BaseElementAdapterFilter.get_slab_tiers)

#### `get_wall_tiers(elements: BaseElementAdapterList) -> list[BaseElementAdapter]`

get the elements for all type of wall tiers definitions

**Remarks:** get the elements for all type of wall tiers definitions

**Parameters:**
- `elements` (BaseElementAdapterList) — elements to filter

**Returns:** `list[BaseElementAdapter]` — filtered elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/BaseElementAdapterFilter/#Utils.BaseElementAdapterFilter.BaseElementAdapterFilter.get_wall_tiers)

