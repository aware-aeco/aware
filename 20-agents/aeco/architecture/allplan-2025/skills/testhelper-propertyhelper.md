---
name: allplan-testhelper-propertyhelper
description: This skill encodes the allplan 2025.0 surface of the TestHelper.PropertyHelper namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: Functions.
---

# TestHelper.PropertyHelper

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## Functions (static-class)

Module-level functions of TestHelper.PropertyHelper

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/TestHelper/PropertyHelper/)

### Methods
#### `add_property( build_ele: BuildingElement, name: str, value_type: str, value: Any, persist: Persistent, ) -> ParameterProperty`

Add a property to the building element

**Remarks:** Add a property to the building element

**Parameters:**
- `build_ele` (BuildingElement) — building element with the parameter properties
- `name` (str) — name of the modified property
- `value_type` (str) — value type of the property
- `value` (Any) — value of the property
- `persist` (Persistent) — persistent state of the property

**Returns:** `ParameterProperty` — created property

[Docs](https://pythonparts.allplan.com/2025/api_reference/TestHelper/PropertyHelper/#TestHelper.PropertyHelper.add_property)

#### `create_ctrl_property(text: str, name: str) -> ControlProperties`

create a property

**Remarks:** create a property

**Parameters:**
- `text` (str) — control text
- `name` (str) — name of the property

**Returns:** `ControlProperties` — created property

[Docs](https://pythonparts.allplan.com/2025/api_reference/TestHelper/PropertyHelper/#TestHelper.PropertyHelper.create_ctrl_property)

#### `create_param_and_ctrl_prop( text: str, name: str, value: Any, value_type: str ) -> Tuple[ParameterProperty, ControlProperties]`

create a property and a control property

**Remarks:** create a property and a control property

**Parameters:**
- `text` (str) — description
- `name` (str) — name of the modified property
- `value` (Any) — value of the property
- `value_type` (str) — value type of the property

**Returns:** `Tuple[ParameterProperty, ControlProperties]` — created property

[Docs](https://pythonparts.allplan.com/2025/api_reference/TestHelper/PropertyHelper/#TestHelper.PropertyHelper.create_param_and_ctrl_prop)

#### `create_property(name: str, value: Any, value_type: str) -> ParameterProperty`

create a property

**Remarks:** create a property

**Parameters:**
- `name` (str) — name of the modified property
- `value_type` (str) — value type of the property
- `value` (Any) — value of the property

**Returns:** `ParameterProperty` — created property

[Docs](https://pythonparts.allplan.com/2025/api_reference/TestHelper/PropertyHelper/#TestHelper.PropertyHelper.create_property)

