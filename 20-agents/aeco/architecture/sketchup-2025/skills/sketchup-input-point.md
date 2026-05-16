---
name: yard-sketchup-input-point
description: Sketchup::InputPoint API reference (YARD)
---

# Sketchup::InputPoint API reference

The InputPoint class is used to pick 3d points and/or entities that reside under the current cursor location, similar to native Line tool and other drawing tools. Unlike PickHelper, InputPoint uses inference, i.e. “snaps” to vertices and other entities when the cursor is close to them.

## Methods

- `initialize` — Prior to SketchUp 2019 it was not possible to sub-class Sketchup::InputPoint due to a bug in how SketchUp initialized the class.
- `==` — The == method is used to determine if two input points are the same.
- `clear` — The clear method is used to clear the input point.
- `copy!` — The copy! method is used to copy the data from a second input point into this input point.
- `degrees_of_freedom` — The degrees_of_freedom method retrieves the number of degrees of freedom there are for an input point.
- `depth` — The depth method retrieves the depth of an inference if it is coming from a component.
- `display?` — The display? method is used to determine if the input point has anything to draw.
- `draw` — The draw method is used to draw the input point.
- `edge` — The edge method is used to retrieve the edge if the input point is getting its position from a point on an Edge.
- `face` — The InputPoint doesn't necessarily lie on the face, but can be e.g. on an edge in front of the face.
- `instance_path` — The #instance_path method retrieves the instance path for the picked point.
- `pick` — The #pick method is used to get a input point at a specific screen position.
- `position` — The position method is used to get the 3D point from the input point.
- `tooltip` — The tooltip method is used to retrieve the string that is the tool tip to display for the input point.
- `transformation` — The transformation method retrieves the Transformation object for the input point.
- `valid?` — The valid? method is used to determine if an input point has valid data.
- `vertex` — The vertex method returns a Vertex associated with the InputPoint. If the InputPoint is on the end of a line, then it will return the Vertex. If the InputPoint does not select any vertices this method returns nil.
