---
name: allplan-stdreinfshapebuilder-reinforcementshapeproperties
description: This skill encodes the allplan 2025.0 surface of the StdReinfShapeBuilder.ReinforcementShapeProperties namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: ReinforcementShapeProperties.
---

# StdReinfShapeBuilder.ReinforcementShapeProperties

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## ReinforcementShapeProperties (class)

Reinforcement shape properties class representing the properties relevant for a reinforcement shape, such as bar diameter, steel grade or diameter of the bending roller.

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/StdReinfShapeBuilder/ReinforcementShapeProperties/)

### Constructors
- `ReinforcementShapeProperties( diameter: float, bending_roller: float, steel_grade: int, concrete_grade: int, bending_shape_type: BendingShapeType, mesh_type: str, mesh_bending_direction: MeshBendingDirection, )` — Default constructor

### Methods
#### `__eq__(other)`

comparison operator

**Remarks:** comparison operator

[Docs](https://pythonparts.allplan.com/2025/api_reference/StdReinfShapeBuilder/ReinforcementShapeProperties/#StdReinfShapeBuilder.ReinforcementShapeProperties.ReinforcementShapeProperties.__eq__)

#### `deep_copy() -> 'ReinforcementShapeProperties'`

Execute a deep copy of the data

**Remarks:** Execute a deep copy of the data

**Returns:** `'ReinforcementShapeProperties'` — copy of the object

[Docs](https://pythonparts.allplan.com/2025/api_reference/StdReinfShapeBuilder/ReinforcementShapeProperties/#StdReinfShapeBuilder.ReinforcementShapeProperties.ReinforcementShapeProperties.deep_copy)

#### `mesh( mesh_type: str, mesh_bending_direction: MeshBendingDirection, bending_roller: float, concrete_grade: int, bending_shape_type: BendingShapeType, ) -> ReinforcementShapeProperties`

Constructs reinforcement shape properties with data relevant for a mesh

**Remarks:** Constructs reinforcement shape properties with data relevant for a mesh

**Parameters:**
- `mesh_type` (str) — Mesh type
- `mesh_bending_direction` (MeshBendingDirection) — Mesh bending direction
- `concrete_grade` (int) — Concrete grade as index of the global list starting from 0. Set to -1 to use global value from the settings.
- `bending_roller` (float) — Default bending roller as a multiple of bar diameter
- `bending_shape_type` (BendingShapeType) — Bending shape type

**Returns:** `ReinforcementShapeProperties` — Reinforcement shape properties for a mesh

[Docs](https://pythonparts.allplan.com/2025/api_reference/StdReinfShapeBuilder/ReinforcementShapeProperties/#StdReinfShapeBuilder.ReinforcementShapeProperties.ReinforcementShapeProperties.mesh)

#### `rebar( diameter: float, bending_roller: float, steel_grade: int, concrete_grade: int, bending_shape_type: BendingShapeType, ) -> ReinforcementShapeProperties`

Constructs reinforcement shape properties with data relevant for a rebar

**Remarks:** Constructs reinforcement shape properties with data relevant for a rebar

**Parameters:**
- `diameter` (float) — Bar diameter
- `bending_roller` (float) — Default bending roller as a multiple of bar diameter
- `steel_grade` (int) — Steel grade as index of the global list
- `concrete_grade` (int) — Concrete grade as index of the global list starting from 0. Set to -1 to use global value from the settings.
- `bending_shape_type` (BendingShapeType) — Bending shape type

**Returns:** `ReinforcementShapeProperties` — Reinforcement shape properties for a mesh

[Docs](https://pythonparts.allplan.com/2025/api_reference/StdReinfShapeBuilder/ReinforcementShapeProperties/#StdReinfShapeBuilder.ReinforcementShapeProperties.ReinforcementShapeProperties.rebar)

#### `to_reinforcementshapebarproperties_string() -> str`

Convert to a string.

**Remarks:** Convert to a string.

**Returns:** `str` — String for the reinforcementshapebarproperties value type

[Docs](https://pythonparts.allplan.com/2025/api_reference/StdReinfShapeBuilder/ReinforcementShapeProperties/#StdReinfShapeBuilder.ReinforcementShapeProperties.ReinforcementShapeProperties.to_reinforcementshapebarproperties_string)

#### `to_reinforcementshapemeshproperties_string() -> str`

Convert to a string.

**Remarks:** Convert to a string.

**Returns:** `str` — String for the reinforcementshapebarproperties value type

[Docs](https://pythonparts.allplan.com/2025/api_reference/StdReinfShapeBuilder/ReinforcementShapeProperties/#StdReinfShapeBuilder.ReinforcementShapeProperties.ReinforcementShapeProperties.to_reinforcementshapemeshproperties_string)

### Properties
- `bending_roller` (float, get/set) — Bending roller diameter as a multiple of bar diameter
- `bending_shape_type` (BendingShapeType, get/set) — Bending shape type
- `concrete_grade` (int, get/set) — Concrete grade as index of the global list starting from 0
- `diameter` (float, get/set) — Bar diameter
- `mesh_bending_direction` (MeshBendingDirection, get/set) — Mesh bending direction
- `mesh_type` (str, get/set) — Mesh type
- `steel_grade` (int, get/set) — Steel grade as index of the global table

