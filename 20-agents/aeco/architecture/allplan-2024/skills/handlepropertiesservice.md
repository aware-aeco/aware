---
name: allplan-handlepropertiesservice
description: This skill encodes the allplan 2024.0 surface of the HandlePropertiesService namespace — 13 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: HandlePropertiesService, _Angle, _BaseHandlePropUpdate, _Point, _CheckBox, _DecrementButton, _IncrementButton, _XDistance, and 5 more types.
---

# HandlePropertiesService

Auto-generated from vendor docs for allplan 2024.0. 13 types in this namespace.

## HandlePropertiesService (class)

implementation of the service for the handle properties

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/HandlePropertiesService/HandlePropertiesService/)

### Methods
#### `update_property_value(build_ele, handle_prop, input_pnt)`

Update the property value

**Remarks:** Update the property value

**Parameters:**
- `build_ele` (BuildingElement) — building element with the parameter properties
- `handle_prop` (HandleProperties) — handle property
- `input_pnt` (Point3D) — input point

**Returns:** `bool` — update palette state

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/HandlePropertiesService/HandlePropertiesService/#HandlePropertiesService.HandlePropertiesService.update_property_value)

## _Angle (class)

modify the angle

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/HandlePropertiesService/_Angle/)

### Methods
#### `__call__()`

execute the value update

**Remarks:** execute the value update

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/HandlePropertiesService/_Angle/#HandlePropertiesService._Angle.__call__)

## _BaseHandlePropUpdate (interface)

base class for the handle property update

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/HandlePropertiesService/_BaseHandlePropUpdate/)

### Constructors
- `_BaseHandlePropUpdate(build_ele, handle_prop, input_pnt, param_data)` — initialize

### Methods
#### `__call__()`

execute the value update

**Remarks:** execute the value update

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/HandlePropertiesService/_BaseHandlePropUpdate/#HandlePropertiesService._BaseHandlePropUpdate.__call__)

#### `get_property_data()`

get the property data

**Remarks:** get the property data

**Returns:** `Tuple[Optional[ParameterProperty], str, str]` — parameter property, item_name with index, parameter name

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/HandlePropertiesService/_BaseHandlePropUpdate/#HandlePropertiesService._BaseHandlePropUpdate.get_property_data)

#### `get_z_min_max()`

get the min and max parameter of the z coordinate

**Remarks:** get the min and max parameter of the z coordinate

**Returns:** `Tuple[Optional[ParameterProperty], Optional[ParameterProperty]]` — min/max parameter of the z coordinate

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/HandlePropertiesService/_BaseHandlePropUpdate/#HandlePropertiesService._BaseHandlePropUpdate.get_z_min_max)

#### `update_property_value(value)`

update the property value

**Remarks:** update the property value

**Parameters:**
- `value` (Any) — new value

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/HandlePropertiesService/_BaseHandlePropUpdate/#HandlePropertiesService._BaseHandlePropUpdate.update_property_value)

### Properties
- `update_palette` (bool, get) — get the update palette state

## _CheckBox (class)

modify the value by checkbox toggle

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/HandlePropertiesService/_CheckBox/)

### Methods
#### `__call__()`

execute the value update

**Remarks:** execute the value update

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/HandlePropertiesService/_CheckBox/#HandlePropertiesService._CheckBox.__call__)

## _DecrementButton (class)

modify the value by decrement

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/HandlePropertiesService/_DecrementButton/)

### Methods
#### `__call__()`

execute the value update

**Remarks:** execute the value update

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/HandlePropertiesService/_DecrementButton/#HandlePropertiesService._DecrementButton.__call__)

## _IncrementButton (class)

modify the value by increment

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/HandlePropertiesService/_IncrementButton/)

### Methods
#### `__call__()`

execute the value update

**Remarks:** execute the value update

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/HandlePropertiesService/_IncrementButton/#HandlePropertiesService._IncrementButton.__call__)

## _Point (class)

modify the point

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/HandlePropertiesService/_Point/)

### Methods
#### `__call__()`

execute the value update

**Remarks:** execute the value update

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/HandlePropertiesService/_Point/#HandlePropertiesService._Point.__call__)

## _PointDistance (class)

modify the point distance

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/HandlePropertiesService/_PointDistance/)

### Methods
#### `__call__()`

execute the value update

**Remarks:** execute the value update

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/HandlePropertiesService/_PointDistance/#HandlePropertiesService._PointDistance.__call__)

## _VectorDistance (class)

modify the value by a vector distance

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/HandlePropertiesService/_VectorDistance/)

### Methods
#### `__call__()`

execute the value update

**Remarks:** execute the value update

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/HandlePropertiesService/_VectorDistance/#HandlePropertiesService._VectorDistance.__call__)

## _XDistance (class)

modify the x distance

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/HandlePropertiesService/_XDistance/)

### Methods
#### `__call__()`

execute the value update

**Remarks:** execute the value update

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/HandlePropertiesService/_XDistance/#HandlePropertiesService._XDistance.__call__)

## _YDistance (class)

modify the y distance

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/HandlePropertiesService/_YDistance/)

### Methods
#### `__call__()`

execute the value update

**Remarks:** execute the value update

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/HandlePropertiesService/_YDistance/#HandlePropertiesService._YDistance.__call__)

## _ZCoord (class)

modify the z coordinate

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/HandlePropertiesService/_ZCoord/)

### Methods
#### `__call__()`

execute the value update

**Remarks:** execute the value update

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/HandlePropertiesService/_ZCoord/#HandlePropertiesService._ZCoord.__call__)

## _ZDistance (class)

modify the z distance

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/HandlePropertiesService/_ZDistance/)

### Methods
#### `__call__()`

execute the value update

**Remarks:** execute the value update

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/HandlePropertiesService/_ZDistance/#HandlePropertiesService._ZDistance.__call__)

