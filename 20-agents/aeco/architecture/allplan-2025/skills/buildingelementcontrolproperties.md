---
name: allplan-buildingelementcontrolproperties
description: This skill encodes the allplan 2025.0 surface of the BuildingElementControlProperties namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: BuildingElementControlProperties.
---

# BuildingElementControlProperties

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## BuildingElementControlProperties (class)

implementation of the container for the controls properties of the building element

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementControlProperties/)

### Constructors
- `BuildingElementControlProperties(control_props: Iterable[ControlProperties] = iter([]))` — initialize

### Methods
#### `eval_palette_layout_script()`

evaluate the palette layout script

**Remarks:** evaluate the palette layout script

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementControlProperties/#BuildingElementControlProperties.BuildingElementControlProperties.eval_palette_layout_script)

#### `get_property(name: str) -> ControlProperties | None`

get the control property for the name

**Remarks:** get the control property for the name

**Parameters:**
- `name` (str) — name of the modified property

**Returns:** `ControlProperties | None` — control property

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementControlProperties/#BuildingElementControlProperties.BuildingElementControlProperties.get_property)

#### `is_refresh_control() -> bool`

get the refresh control state

**Remarks:** get the refresh control state

**Returns:** `bool` — refresh of the controls state

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementControlProperties/#BuildingElementControlProperties.BuildingElementControlProperties.is_refresh_control)

#### `reset_refresh_control()`

set the refresh control state

**Remarks:** set the refresh control state

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementControlProperties/#BuildingElementControlProperties.BuildingElementControlProperties.reset_refresh_control)

### Properties
- `palette_layout_dict` (dict[str, Any], get) — get the palette layout dictionary
- `palette_layout_script` (str, get/set) — get the palette layout script

