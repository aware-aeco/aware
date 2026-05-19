---
name: allplan-utils-formatutil
description: This skill encodes the allplan 2025.0 surface of the Utils.FormatUtil namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: FormatUtil.
---

# Utils.FormatUtil

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## FormatUtil (class)

Implementation of the format utilities

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/Utils/FormatUtil/)

### Methods
#### `get_layer_pen_stroke_color(layer: int | str) -> tuple[bool, int, int, int]`

get the pen, stroke and color from the layer

**Remarks:** get the pen, stroke and color from the layer

**Parameters:**
- `layer` (int | str) — number of the layer

**Returns:** `tuple[bool, int, int, int]` — tuple(pen, stroke, color)

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/FormatUtil/#Utils.FormatUtil.FormatUtil.get_layer_pen_stroke_color)

#### `init_by_global_properties(build_ele: BuildingElement)`

initialize the values by global properties

**Remarks:** initialize the values by global properties

**Parameters:**
- `build_ele` (BuildingElement) — building element with the parameter properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/FormatUtil/#Utils.FormatUtil.FormatUtil.init_by_global_properties)

#### `init_list_by_global_properties(build_ele: BuildingElement)`

initialize the values by global properties

**Remarks:** initialize the values by global properties

**Parameters:**
- `build_ele` (BuildingElement) — building element with the parameter properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/FormatUtil/#Utils.FormatUtil.FormatUtil.init_list_by_global_properties)

#### `update_build_ele_list_values(build_ele: BuildingElement)`

Update the list values inside the building element

**Remarks:** Update the list values inside the building element

**Parameters:**
- `build_ele` (BuildingElement) — building element with the parameter properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/FormatUtil/#Utils.FormatUtil.FormatUtil.update_build_ele_list_values)

#### `update_build_ele_single_values(build_ele: BuildingElement)`

Update the single values inside the building element

**Remarks:** Update the single values inside the building element

**Parameters:**
- `build_ele` (BuildingElement) — building element with the parameter properties

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/FormatUtil/#Utils.FormatUtil.FormatUtil.update_build_ele_single_values)

#### `update_by_layer( color: int, color_by_layer: bool, pen: int, pen_by_layer: bool, stroke: int, stroke_by_layer: bool, layer: int, ) -> tuple[int, int, int]`

Update the common properties by layer

**Remarks:** Update the common properties by layer

**Parameters:**
- `color` (int) — color
- `color_by_layer` (bool) — color by layer state
- `pen` (int) — pen
- `pen_by_layer` (bool) — pen by layer state
- `stroke` (int) — stroke
- `stroke_by_layer` (bool) — stroke by layer state
- `layer` (int) — layer

**Returns:** `tuple[int, int, int]` — color, pen, stroke

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/FormatUtil/#Utils.FormatUtil.FormatUtil.update_by_layer)

