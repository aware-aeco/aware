---
name: yard-geom-bounding-box
description: Geom::BoundingBox API reference (YARD)
---

# Geom::BoundingBox API reference

that the bounding box returned for face-me components is the center of its entire range of motion. This behavior changed in SketchUp 7.1. In 7.0 and earlier, the .bounds method would return the bounds around the face-me component's current, visible center.

## Methods

- `initialize` — The new method is used to create a new, empty, bounding box.
- `add` — The add method is used to add a point, vertex, or other bounding boxes to the bounding box. The size of the bounding box will increase as necessary to accommodate the new items.
- `center` — The center method is used to retrieve the Point3d object at the center of the bounding box.
- `clear` — The clear method is used to clear a bounding box.
- `contains?` — This method is used to determine if a bounding box contains a specific Point3d or BoundingBox object.
- `corner` — The corner method is used to retrieve a point object at a specified corner of the bounding box.
- `depth` — In SketchUp's coordinate system, this corresponds to the height.
- `diagonal` — The #diagonal method is used to get the length of the diagonal of the bounding box.
- `empty?` — The empty? method is used to determine if a bounding box is empty (such as if the bounds have not been set.) This returns the opposite of the valid? method.
- `height` — In SketchUp's coordinate system, this corersponds to the depth.
- `intersect` — Prior to SU2015 this method would return incorrect result in some cases. For correct result in these versions you must first check if the boundingboxes actually overlap - then call this to get the resulting boundingbox.
- `max` — The max method is used to retrieve the Point3d object where x, y and z are maximum in the bounding box.
- `min` — The min method is used to retrieve the Point3d where x, y and z are minimum in the bounding box.
- `valid?` — The valid method is used to determine if a bounding box is valid (contains points).
- `width` — The #width method is used to retrieve the X extent of the bounding box.
