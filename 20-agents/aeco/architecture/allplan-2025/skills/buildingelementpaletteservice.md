---
name: allplan-buildingelementpaletteservice
description: This skill encodes the allplan 2025.0 surface of the BuildingElementPaletteService namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: BuildingElementPaletteService.
---

# BuildingElementPaletteService

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## BuildingElementPaletteService (class)

Definition of class BuildingElementPaletteService

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementPaletteService/)

### Constructors
- `BuildingElementPaletteService( build_ele_list: list[BuildingElement], build_ele_composite: BuildingElementComposite, build_ele_script: Any, build_ele_ctrl_props_list: list[BuildingElementControlProperties], picture_path: str, script_object: BaseScriptObject | None = None, )` — Initialize the data

### Methods
#### `check_building_element_index()`

Check the building element index (e.g. set to 0 for hidden elements)

**Remarks:** Check the building element index (e.g. set to 0 for hidden elements)

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementPaletteService/#BuildingElementPaletteService.BuildingElementPaletteService.check_building_element_index)

#### `close_palette() -> str`

Close the palette

**Remarks:** Close the palette

**Returns:** `str` — text of the active page

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementPaletteService/#BuildingElementPaletteService.BuildingElementPaletteService.close_palette)

#### `get_control_text() -> str`

Get the control data as text

**Remarks:** Get the control data as text

**Returns:** `str` — Controls data as text

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementPaletteService/#BuildingElementPaletteService.BuildingElementPaletteService.get_control_text)

#### `modify_element_property(page: int, name: str, value: Any) -> bool`

Modify property of element

**Remarks:** Modify property of element

**Parameters:**
- `page` (int) — description
- `name` (str) — the name of the property.
- `value` (Any) — new value for property.

**Returns:** `bool` — update palette state

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementPaletteService/#BuildingElementPaletteService.BuildingElementPaletteService.modify_element_property)

#### `on_control_event(event_id: int) -> bool`

On control event

**Remarks:** On control event

**Parameters:**
- `event_id` (int) — event id of control.

**Returns:** `bool` — event was processed state

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementPaletteService/#BuildingElementPaletteService.BuildingElementPaletteService.on_control_event)

#### `refresh_palette( build_ele_list: list[BuildingElement], build_ele_ctrl_props_list: list[BuildingElementControlProperties], )`

refresh the palette

**Remarks:** refresh the palette

**Parameters:**
- `build_ele_list` (list[BuildingElement]) — Building element list
- `build_ele_ctrl_props_list` (list[BuildingElementControlProperties]) — Control properties list

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementPaletteService/#BuildingElementPaletteService.BuildingElementPaletteService.refresh_palette)

#### `set_building_element_index(index: int, script_name: str)`

Set the building element index

**Remarks:** Set the building element index

**Parameters:**
- `index` (int) — Element index
- `script_name` (str) — Script name

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementPaletteService/#BuildingElementPaletteService.BuildingElementPaletteService.set_building_element_index)

#### `set_palette_lock(palette_lock: bool)`

set the palette lock state

**Remarks:** set the palette lock state

**Parameters:**
- `palette_lock` (bool) — palette lock state

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementPaletteService/#BuildingElementPaletteService.BuildingElementPaletteService.set_palette_lock)

#### `set_update_lock(update_lock: bool)`

set the palette lock state

**Remarks:** set the palette lock state

**Parameters:**
- `update_lock` (bool) — palette lock state

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementPaletteService/#BuildingElementPaletteService.BuildingElementPaletteService.set_update_lock)

#### `show_page_for_element(act_page: int, build_ele_index_list: list[int])`

Show the page for the element_index

**Remarks:** Show the page for the element_index

**Parameters:**
- `act_page` (int) — active page index
- `build_ele_index_list` (list[int]) — index list with the connected building elements (geometry, reinforcement, ...) of a sub element

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementPaletteService/#BuildingElementPaletteService.BuildingElementPaletteService.show_page_for_element)

#### `show_palette( part_name: str, show_close_button: bool = True, open_palette: bool = True, active_page_text: str = "", is_visual_script: bool = False, )`

Show the palette

**Remarks:** Show the palette

**Parameters:**
- `part_name` (str) — Name of the PythonPart
- `show_close_button` (bool) — Show close button in palette
- `open_palette` (bool) — open the palette
- `active_page_text` (str) — active page text
- `is_visual_script` (bool) — execution for VisualScripting

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementPaletteService/#BuildingElementPaletteService.BuildingElementPaletteService.show_palette)

#### `update_palette( page_index: int, update_dialog_data: bool, _show_palette_close_btn=True )`

Update the palette

**Remarks:** Update the palette

**Parameters:**
- `page_index` (int) — page index to show, -1 = use current
- `update_dialog_data` (bool) — update the dialog data: True/False
- `_show_palette_close_btn` (object) — show close button in palette: True/False

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/BuildingElementPaletteService/#BuildingElementPaletteService.BuildingElementPaletteService.update_palette)

