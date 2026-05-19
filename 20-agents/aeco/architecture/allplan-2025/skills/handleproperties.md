---
name: allplan-handleproperties
description: This skill encodes the allplan 2025.0 surface of the HandleProperties namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: HandleProperties.
---

# HandleProperties

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## HandleProperties (class)

Implementation of the handle properties class

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/HandleProperties/)

### Constructors
- `HandleProperties( handle_id: str | HandleProperties() | HandleProperties() | HandleProperties() | HandleProperties() | HandleProperties()` — Set the properties of a handle

### Methods
#### `__repr__() -> str`

Create class information as string

**Remarks:** Create class information as string

**Returns:** `str` — created data string

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/HandleProperties/#HandleProperties.HandleProperties.__repr__)

#### `get_min_max_values(name: str) -> tuple[int | float, int | float, str, str]`

Get the min/max values of the handle property

**Remarks:** Get the min/max values of the handle property

**Parameters:**
- `name` (str) — name of the property

**Returns:** `tuple[int | float, int | float, str, str]` — tuple(min value, max value, value list, interval value)

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/HandleProperties/#HandleProperties.HandleProperties.get_min_max_values)

#### `get_parameter_names() -> list[str]`

get the names of the parameters

**Remarks:** get the names of the parameters

**Returns:** `list[str]` — names of the parameters

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/HandleProperties/#HandleProperties.HandleProperties.get_parameter_names)

#### `set_min_max_values( control_props: list[BuildingElementControlProperties], build_ele_list: list[BuildingElement] | None = None, )`

Set the min/max values of the handle property

**Remarks:** Set the min/max values of the handle property

**Parameters:**
- `control_props` (list[BuildingElementControlProperties]) — control properties
- `build_ele_list` (list[BuildingElement] | None) — list with the building elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/HandleProperties/#HandleProperties.HandleProperties.set_min_max_values)

#### `transform(transformation_matrix: Matrix3D)`

Transform the handle

**Remarks:** Transform the handle

**Parameters:**
- `transformation_matrix` (Matrix3D) — transformation matrix

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/HandleProperties/#HandleProperties.HandleProperties.transform)

### Properties
- `abs_value` (bool, get) — Get the absolute value state of the properties
- `angle_placement` (AxisPlacement3D | None, get/set) — Get the placement for the 3D angle input
- `build_ele_index_list` (list[int], get/set) — Get the index list of the building elements
- `center_point` (Point3D | None, get/set) — Get the center point
- `click_state` (bool, get) — Get the click state:
- `dir_vector` (Vector3D | None, get/set) — Get the direction vector of the handle point movement
- `distance_factor` (float, get/set) — Get the factor for property calculation: -1=show negative value in the input control. Multiply the distance between reference and handle point with the factor to get the property value, e.g. 2.0 if the ref_point is a center point.
- `ele_prop_list` (list[ tuple[ str, HandleParameterType, bool, bool, bool, Any, int | list[int] | None, bool, bool, ] ], get) — Get the parameter data as list of tuples (called from C++)
- `handle_angle` (Angle, get/set) — Handle angle
- `handle_id` (str | tuple[int, str], get) — Get the handle id
- `handle_move_dir` (HandleDirection, get) — Get the allowed move direction for the handle
- `handle_point` (Point3D, get/set) — Get the handle point
- `handle_type` (ElementHandleType, get/set) — Handle type
- `info_text` (str, get/set) — Info text of the tooltip
- `parameter_data` (list[HandleParameterData], get) — Get tbe list with the parameter data of the handle
- `plane` (Plane3D | None, get) — Get the input plane for the handle_point movement
- `ref_point` (Point3D, get/set) — Get the reference point
- `show_handles` (bool, get/set) — Get the show handles state

