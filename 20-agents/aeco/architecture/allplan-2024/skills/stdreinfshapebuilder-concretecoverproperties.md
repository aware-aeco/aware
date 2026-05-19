---
name: allplan-stdreinfshapebuilder-concretecoverproperties
description: This skill encodes the allplan 2024.0 surface of the StdReinfShapeBuilder.ConcreteCoverProperties namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: ConcreteCoverProperties.
---

# StdReinfShapeBuilder.ConcreteCoverProperties

Auto-generated from vendor docs for allplan 2024.0. 1 types in this namespace.

## ConcreteCoverProperties (class)

Implementation of the concrete cover properties class

[Vendor docs](https://pythonparts.allplan.com/2024/api_reference/StdReinfShapeBuilder/ConcreteCoverProperties/ConcreteCoverProperties/)

### Constructors
- `ConcreteCoverProperties(left=0, right=0, top=0, bottom=0)` — Set the properties of the concrete cover

### Methods
#### `__repr__()`

convert to string

**Remarks:** convert to string

[Docs](https://pythonparts.allplan.com/2024/api_reference/StdReinfShapeBuilder/ConcreteCoverProperties/ConcreteCoverProperties/#StdReinfShapeBuilder.ConcreteCoverProperties.ConcreteCoverProperties.__repr__)

#### `all(concrete_cover)`

Set the properties of the concrete cover for all borders

**Remarks:** Set the properties of the concrete cover for all borders

**Parameters:**
- `concrete_cover` (float) — concrete cover for all sides

**Returns:** `ConcreteCoverProperties` — concrete cover properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/StdReinfShapeBuilder/ConcreteCoverProperties/ConcreteCoverProperties/#StdReinfShapeBuilder.ConcreteCoverProperties.ConcreteCoverProperties.all)

#### `from_reinforcement_definition(reinf_def)`

Get the cover from the reinforcement definition

**Remarks:** Get the cover from the reinforcement definition

**Parameters:**
- `reinf_def` (ReinforcementDefinition) — reinforcement definition

[Docs](https://pythonparts.allplan.com/2024/api_reference/StdReinfShapeBuilder/ConcreteCoverProperties/ConcreteCoverProperties/#StdReinfShapeBuilder.ConcreteCoverProperties.ConcreteCoverProperties.from_reinforcement_definition)

#### `left_right(left, right)`

Set the properties of the concrete cover

**Remarks:** Set the properties of the concrete cover

**Parameters:**
- `left` (float) — description
- `right` (float) — description

**Returns:** `ConcreteCoverProperties` — concrete cover properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/StdReinfShapeBuilder/ConcreteCoverProperties/ConcreteCoverProperties/#StdReinfShapeBuilder.ConcreteCoverProperties.ConcreteCoverProperties.left_right)

#### `left_right_bottom(left, right, bottom)`

Set the properties of the concrete cover

**Remarks:** Set the properties of the concrete cover

**Parameters:**
- `left` (float) — description
- `right` (float) — description
- `bottom` (float) — description

**Returns:** `ConcreteCoverProperties` — concrete cover properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/StdReinfShapeBuilder/ConcreteCoverProperties/ConcreteCoverProperties/#StdReinfShapeBuilder.ConcreteCoverProperties.ConcreteCoverProperties.left_right_bottom)

#### `top_bottom(top, bottom)`

Set the properties of the concrete cover

**Remarks:** Set the properties of the concrete cover

**Parameters:**
- `top` (float) — description
- `bottom` (float) — description

**Returns:** `ConcreteCoverProperties` — concrete cover properties

[Docs](https://pythonparts.allplan.com/2024/api_reference/StdReinfShapeBuilder/ConcreteCoverProperties/ConcreteCoverProperties/#StdReinfShapeBuilder.ConcreteCoverProperties.ConcreteCoverProperties.top_bottom)

### Properties
- `bottom` (float, get/set) — Get the cover at the bottom border
- `left` (float, get/set) — Get the cover at the left border
- `right` (float, get/set) — Get the cover at the right border
- `top` (float, get/set) — Get the cover at the top border

