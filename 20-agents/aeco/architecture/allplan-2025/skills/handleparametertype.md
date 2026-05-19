---
name: allplan-handleparametertype
description: This skill encodes the allplan 2025.0 surface of the HandleParameterType namespace — 1 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: HandleParameterType.
---

# HandleParameterType

Auto-generated from vendor docs for allplan 2025.0. 1 types in this namespace.

## HandleParameterType (enum)

The parameter type defines the function to be used to recalculate the value of the parameter property assigned to the handle.

[Vendor docs](https://pythonparts.allplan.com/2025/api_reference/GeneralScripts/HandleParameterType/)

### Values
- `ANGLE` = `9` — Calculate the angle from the xy-plane vector between the reference and handle point
- `CHECK_BOX` = `13` — Use the handle as checkbox
- `DECREMENT_BUTTON` = `15` — Use the handle as button to decrement the value
- `INCREMENT_BUTTON` = `14` — Use the handle as button to increment the value
- `POINT` = `7` — Set the value to the handle point
- `POINT_DISTANCE` = `8` — Calculate the value from the distance between the reference and handle point
- `VECTOR_DISTANCE` = `12` — Calculate the value from the distance between the reference and handle point, transformed to the vector direction assigned to the HandleProperties
- `X_DISTANCE` = `1` — Calculate the value from the x-distance between the reference and handle point
- `Y_DISTANCE` = `2` — Calculate the value from the y-distance between the reference and handle point
- `Z_COORD` = `10` — Calculate the value for the "ZMin" or "ZMax" parameter. The distance between these values is adapted to the distance between the reference and handle point
- `Z_DISTANCE` = `3` — Calculate the value from the z-distance between the reference and handle point

