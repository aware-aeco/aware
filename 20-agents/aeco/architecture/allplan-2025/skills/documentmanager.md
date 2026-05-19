---
name: allplan-documentmanager
description: This skill encodes the allplan 2025.0 surface of the DocumentManager namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: DocumentManager.
---

# DocumentManager

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## DocumentManager (class)

Singleton class for the document manager

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/DocumentManager/)

### Constructors
- `DocumentManager()` — Constructor

### Methods
#### `clear_asso_ref_element()`

clear the asso_ref element

**Remarks:** clear the asso_ref element

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/DocumentManager/#DocumentManager.DocumentManager.clear_asso_ref_element)

#### `clear_pythonpart_element()`

clear the PythonPart element

**Remarks:** clear the PythonPart element

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/DocumentManager/#DocumentManager.DocumentManager.clear_pythonpart_element)

#### `get_instance() -> DocumentManager`

Get the one an only instance for the document manager

**Remarks:** Get the one an only instance for the document manager

**Returns:** `DocumentManager` — DocumentManager object

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/DocumentManager/#DocumentManager.DocumentManager.get_instance)

#### `set_pythonpart_element(modification_ele_list: ModificationElementList)`

set the PythonPart element from the model element UUID

**Remarks:** set the PythonPart element from the model element UUID

**Parameters:**
- `modification_ele_list` (ModificationElementList) — list with the modification elements in modification mode

[Docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/DocumentManager/#DocumentManager.DocumentManager.set_pythonpart_element)

### Properties
- `asso_ref_element` (BaseElementAdapter, get/set) — get the asso_ref element
- `document` (DocumentAdapter, get/set) — get the document
- `pythonpart_element` (BaseElementAdapter, get) — get the PythonPart element

