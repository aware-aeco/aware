---
name: allplan-buildingelementdockingpoints
description: This skill encodes the allplan 2024.0 surface of the BuildingElementDockingPoints namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: BuildingElementDockingPoints.
---

# BuildingElementDockingPoints

Auto-generated from vendor docs for allplan 2024.0. 1 types in this namespace.

## BuildingElementDockingPoints (class)

Definition of class BuildingElementDockingPoints

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementDockingPoints/BuildingElementDockingPoints/)

### Constructors
- `BuildingElementDockingPoints(element, path)` — Initialization of class BuildingElementInput

### Methods
#### `get_docking_points(file_name, parameter_data)`

get the docking points

**Remarks:** get the docking points

**Parameters:**
- `file_name` (str) — file name of the pyp file
- `parameter_data` (List[str]) — parameter data of the selected PythonPart

**Returns:** `Tuple[List[Tuple[str, Point3D]], List[Tuple[str, Point3D]], List[Tuple[str, Point3D]]]` — docking points

[Docs](https://pythonparts.allplan.com/2024/api_reference/GeneralScripts/BuildingElementDockingPoints/BuildingElementDockingPoints/#BuildingElementDockingPoints.BuildingElementDockingPoints.get_docking_points)

