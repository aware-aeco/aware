---
name: allplan-handleproperties
description: This skill encodes the allplan 2024.0 surface of the HandleProperties namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: HandleProperties.
---

# HandleProperties

Auto-generated from vendor docs for allplan 2024.0. 1 types in this namespace.

## HandleProperties (class)

Implementation of the handle properties class

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/HandleProperties/HandleProperties/)

### Constructors
- `HandleProperties(handle_id, handle_point, ref_point, handle_param_data, handle_move_dir, abs_value=True, distance_factor=1.0, plane=None, dir_vector=None, info_text='', angle_placement=None, show_handles=True)` — Set the properties of a handle

### Methods
#### `__repr__()`

Create class information as string

**Remarks:** Create class information as string

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/HandleProperties/HandleProperties/#HandleProperties.HandleProperties.__repr__)

#### `transform(transformation_matrix)`

Transform the handle

**Remarks:** Transform the handle

**Parameters:**
- `trans_mat` (object) — transformation matrix

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/HandleProperties/HandleProperties/#HandleProperties.HandleProperties.transform)

### Properties
- `abs_value` (bool, get) — Get the absolute value state of the properties
- `angle_placement` (Optional[AllplanGeo.AxisPlacement3D], get/set) — Get the placement for the 3D angle input
- `build_ele_index_list` (List[int], get/set) — Get the index list of the building elements
- `click_state` (bool, get) — Get the click state:
- `dir_vector` (Optional[AllplanGeo.Vector3D], get/set) — Get the direction vector of the handle point movement
- `distance_factor` (float, get/set) — Get the factor for property calculation: -1=show negative value in the input control. Multiply the distance between reference and handle point with the factor to get the property value, e.g. 2.0 if the ref_point is a center point.
- `ele_prop_list` (List[Tuple[str, HandleDirection, bool, bool, bool, Any, Optional[Union[int, List[int]]]]], get) — Get the parameter data as list of tuples
- `handle_angle` (AllplanGeo.Angle, get/set) — Handle angle :getter: Get the handle angle :setter: Set the handle angle
- `handle_id` (Union[str, Tuple[int, str]], get) — Get the handle id
- `handle_move_dir` (HandleDirection, get) — Get the allowed move direction for the handle
- `handle_point` (AllplanGeo.Point3D, get/set) — Get the handle point
- `handle_type` (AllplanIFW.ElementHandleType, get/set) — Handle type :getter: Get the handle type :setter: Set the handle type
- `info_text` (str, get/set) — Info text of the tooltip :getter: Get the info text of the tooltip :setter: Set the info text of the tooltip
- `parameter_data` (List[HandleParameterData], get) — Get tbe list with the parameter data of the handle
- `plane` (Optional[AllplanGeo.Plane3D], get) — Get the Input plane for the handle_point movement
- `ref_point` (AllplanGeo.Point3D, get) — Get the reference point
- `show_handles` (bool, get/set) — Get the show handles state

