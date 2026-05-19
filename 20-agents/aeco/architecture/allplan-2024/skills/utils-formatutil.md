---
name: allplan-utils-formatutil
description: This skill encodes the allplan 2024.0 surface of the Utils.FormatUtil namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: FormatUtil.
---

# Utils.FormatUtil

Auto-generated from vendor docs for allplan 2024.0. 1 types in this namespace.

## FormatUtil (class)

Implementation of the format utilities

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/Utils/FormatUtil/FormatUtil/)

### Methods
#### `get_layer_pen_stroke_color(layer)`

get the pen, stroke and color from the layer

**Remarks:** get the pen, stroke and color from the layer

**Parameters:**
- `layer` (int | str) — number of the layer

**Returns:** `tuple[bool, int, int, int]` — tuple(pen, stroke, color)

[Docs](https://pythonparts.allplan.com/2024/api_reference/Utils/FormatUtil/FormatUtil/#Utils.FormatUtil.FormatUtil.get_layer_pen_stroke_color)

#### `init_by_global_properties(build_ele)`

initialize the values by global properties

**Remarks:** initialize the values by global properties

**Parameters:**
- `build_ele` (BuildingElement) — building element with the parameter properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/Utils/FormatUtil/FormatUtil/#Utils.FormatUtil.FormatUtil.init_by_global_properties)

#### `init_list_by_global_properties(build_ele)`

initialize the values by global properties

**Remarks:** initialize the values by global properties

**Parameters:**
- `build_ele` (BuildingElement) — building element with the parameter properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/Utils/FormatUtil/FormatUtil/#Utils.FormatUtil.FormatUtil.init_list_by_global_properties)

#### `update_by_layer(color, color_by_layer, pen, pen_by_layer, stroke, stroke_by_layer, layer)`

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

[Docs](https://pythonparts.allplan.com/2024/api_reference/Utils/FormatUtil/FormatUtil/#Utils.FormatUtil.FormatUtil.update_by_layer)

