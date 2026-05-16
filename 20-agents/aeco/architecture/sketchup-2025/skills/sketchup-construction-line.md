---
name: yard-sketchup-construction-line
description: Sketchup::ConstructionLine API reference (YARD)
---

# Sketchup::ConstructionLine API reference

The ConstructionLine class contains methods for modifying construction lines.  Construction lines can be infinite in length, semi-infinite (i.e. infinite in one direction) or finite.

## Methods

- `direction` — The direction method retrieves a 3D vector in the direction of the construction line.
- `direction=` — The direction= method is used to set the direction of the construction line to a 3D vector.
- `end` — The end method retrieves the end point of a construction line in the form of a 3D point.
- `end=` — The end= method is used to set the end point of the construction line. This method will make the length finite at the end.
- `position` — The position method is used to retrieve a 3D point used to create a construction line on an infinite construction line.
- `position=` — The position= method is used to set a 3D point that the construction passes through
- `reverse!` — The reverse! method is used to reverse the direction of the construction line.
- `start` — The start method is used to retrieve the starting point of a construction line.
- `start=` — The start= method is used to set the start point of a construction line making the line's length finite at the start.
- `stipple` — The #stipple method is used to retrieve the stipple pattern used to display the construction line.
- `stipple=` — The #stipple= method is used to set the stipple pattern used to display the construction line. The stipple pattern is given as a string.
