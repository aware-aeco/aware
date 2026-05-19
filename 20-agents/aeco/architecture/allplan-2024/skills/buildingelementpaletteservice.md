---
name: allplan-buildingelementpaletteservice
description: This skill encodes the allplan 2024.0 surface of the BuildingElementPaletteService namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: BuildingElementPaletteService.
---

# BuildingElementPaletteService

Auto-generated from vendor docs for allplan 2024.0. 1 types in this namespace.

## BuildingElementPaletteService (class)

Definition of class BuildingElementPaletteService

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementPaletteService/BuildingElementPaletteService/)

### Constructors
- `BuildingElementPaletteService(build_ele_list, build_ele_composite, build_ele_script, build_ele_ctrl_props_list, picture_path)` — Initialize the data

### Methods
#### `check_building_element_index()`

Check the building element index (e.g. set to 0 for hidden elements)

**Remarks:** Check the building element index (e.g. set to 0 for hidden elements)

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementPaletteService/BuildingElementPaletteService/#BuildingElementPaletteService.BuildingElementPaletteService.check_building_element_index)

#### `close_palette()`

Close the palette

**Remarks:** Close the palette

**Returns:** `str` — text of the active page

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementPaletteService/BuildingElementPaletteService/#BuildingElementPaletteService.BuildingElementPaletteService.close_palette)

#### `get_control_text()`

Get the control data as text

**Remarks:** Get the control data as text

**Returns:** `str` — Controls data as text

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementPaletteService/BuildingElementPaletteService/#BuildingElementPaletteService.BuildingElementPaletteService.get_control_text)

#### `modify_element_property(page, name, value)`

Modify property of element

**Remarks:** Modify property of element

**Parameters:**
- `page` (int) — description
- `name` (str) — the name of the property.
- `value` (Any) — new value for property.

**Returns:** `bool` — update palette state

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementPaletteService/BuildingElementPaletteService/#BuildingElementPaletteService.BuildingElementPaletteService.modify_element_property)

#### `on_control_event(event_id)`

On control event

**Remarks:** On control event

**Parameters:**
- `event_id` (int) — event id of control.

**Returns:** `bool` — event was processed state

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementPaletteService/BuildingElementPaletteService/#BuildingElementPaletteService.BuildingElementPaletteService.on_control_event)

#### `refresh_palette(build_ele_list, build_ele_ctrl_props_list)`

refresh the palette

**Remarks:** refresh the palette

**Parameters:**
- `build_ele_list` (List[BuildingElement]) — Building element list
- `build_ele_ctrl_props_list` (List[BuildingElementControlProperties]) — Control properties list

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementPaletteService/BuildingElementPaletteService/#BuildingElementPaletteService.BuildingElementPaletteService.refresh_palette)

#### `set_building_element_index(index, script_name)`

Set the building element index

**Remarks:** Set the building element index

**Parameters:**
- `index` (int) — Element index
- `script_name` (str) — Script name

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementPaletteService/BuildingElementPaletteService/#BuildingElementPaletteService.BuildingElementPaletteService.set_building_element_index)

#### `set_palette_lock(palette_lock)`

set the palette lock state

**Remarks:** set the palette lock state

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementPaletteService/BuildingElementPaletteService/#BuildingElementPaletteService.BuildingElementPaletteService.set_palette_lock)

#### `show_page_for_element(act_page, build_ele_index_list)`

Show the page for the element_index

**Remarks:** Show the page for the element_index

**Parameters:**
- `act_page` (int) — active page index
- `build_ele_index_list` (List[int]) — index list with the connected building elements (geometry, reinforcement, ...) of a sub element

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementPaletteService/BuildingElementPaletteService/#BuildingElementPaletteService.BuildingElementPaletteService.show_page_for_element)

#### `show_palette(part_name, show_close_button=True, open_palette=True, active_page_text='', is_visual_script=False)`

Show the palette

**Remarks:** Show the palette

**Parameters:**
- `part_name` (str) — Name of the PythonPart
- `show_close_button` (bool) — Show close button in palette
- `open_palette` (bool) — open the palette
- `active_page_text` (str) — active page text
- `is_visual_script` (bool) — execution for VisualScripting

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementPaletteService/BuildingElementPaletteService/#BuildingElementPaletteService.BuildingElementPaletteService.show_palette)

#### `update_palette(page_index, update_dialog_data, _show_palette_close_btn=True)`

Update the palette

**Remarks:** Update the palette

**Parameters:**
- `page_index` (int) — page index to show, -1 = use current
- `update_dialog_data` (bool) — update the dialog data: True/False
- `_show_palette_close_btn` (object) — show close button in palette: True/False

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementPaletteService/BuildingElementPaletteService/#BuildingElementPaletteService.BuildingElementPaletteService.update_palette)

