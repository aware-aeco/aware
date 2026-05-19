---
name: allplan-pythonpartpreview
description: This skill encodes the allplan 2025.0 surface of the PythonPartPreview namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: PythonPartPreview.
---

# PythonPartPreview

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## PythonPartPreview (class)

Implementation of the PythonPart transaction

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPartPreview/)

### Methods
#### `close()`

close the preview

**Remarks:** close the preview

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPartPreview/#PythonPartPreview.PythonPartPreview.close)

#### `execute( doc: DocumentAdapter, placement_matrix: Matrix3D, model_ele_list: list[Any], direct_draw: bool = False, asso_ref_object: BaseElementAdapter | None = None, use_system_angle: bool = True, as_static_preview: bool = False, )`

execute the preview draw

**Remarks:** execute the preview draw

**Parameters:**
- `doc` (DocumentAdapter) — document of the Allplan drawing files
- `placement_matrix` (Matrix3D) — placement matrix
- `model_ele_list` (list[Any]) — list with the model elements
- `direct_draw` (bool) — direct draw of the preview
- `asso_ref_object` (BaseElementAdapter | None) — associative view reference object
- `use_system_angle` (bool) — use the system angle state
- `as_static_preview` (bool) — draw as static preview state

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPartPreview/#PythonPartPreview.PythonPartPreview.execute)

#### `set_preview_draw_lock(preview_draw_lock: bool)`

set the preview draw lock state

**Remarks:** set the preview draw lock state

**Parameters:**
- `preview_draw_lock` (bool) — preview draw lock state

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/PythonPartPreview/#PythonPartPreview.PythonPartPreview.set_preview_draw_lock)

