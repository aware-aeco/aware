---
name: allplan-utils-hideelementsservice
description: This skill encodes the allplan 2025.0 surface of the Utils.HideElementsService namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: HideElementsService.
---

# Utils.HideElementsService

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## HideElementsService (class)

implementation of the hide elements service

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/Utils/HideElementsService/)

### Constructors
- `HideElementsService()` — initialize

### Methods
#### `clear()`

clear the elements

**Remarks:** clear the elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/HideElementsService/#Utils.HideElementsService.HideElementsService.clear)

#### `hide_arch_ground_view_elements(arch_ele: BaseElementAdapter)`

hide the architecture ground view elements

**Remarks:** hide the architecture ground view elements

**Parameters:**
- `arch_ele` (BaseElementAdapter) — architecture element

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/HideElementsService/#Utils.HideElementsService.HideElementsService.hide_arch_ground_view_elements)

#### `hide_element(element: BaseElementAdapter)`

hide the element and the children

**Remarks:** hide the element and the children

**Parameters:**
- `element` (BaseElementAdapter) — element to hide

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/HideElementsService/#Utils.HideElementsService.HideElementsService.hide_element)

#### `hide_element_and_linked(element: BaseElementAdapter)`

hide the element, the children and the linked elements

**Remarks:** hide the element, the children and the linked elements

**Parameters:**
- `element` (BaseElementAdapter) — element to hide

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/HideElementsService/#Utils.HideElementsService.HideElementsService.hide_element_and_linked)

#### `show_elements()`

show the hidden elements

**Remarks:** show the hidden elements

[Docs](https://pythonparts.allplan.com/2025/api_reference/Utils/HideElementsService/#Utils.HideElementsService.HideElementsService.show_elements)

### Properties
- `get_hidden_geo_elements` (list[Any], get) — get the geometry of the hidden elements
- `hidden_elements` (list[BaseElementAdapter], get) — get the elements

