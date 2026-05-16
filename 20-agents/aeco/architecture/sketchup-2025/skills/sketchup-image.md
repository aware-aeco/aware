---
name: yard-sketchup-image
description: Sketchup::Image API reference (YARD)
---

# Sketchup::Image API reference

An Image object represents a raster image placed in the Model.

## Methods

- `explode` — The explode method is used to explode an image into a face with a texture on it.
- `glued_to` — The #glued_to method is used to retrieve the entity that this image is glued to.
- `glued_to=` — The #glued_to= method glues this image to a drawing element. When moving this other drawing elment with the Move tool, the image moves with it.
- `height` — The height method is used to retrieve the height of the image.
- `height=` — The height= method is used to set the height of the image.
- `image_rep` — The #image_rep method returns a copy of a Sketchup::ImageRep object representing the pixel data.
- `normal` — The normal method is used to retrieve the 3D Vector that is perpendicular to the plane of the image.
- `origin` — The origin method is used to retrieve the 3D point that defines the origin of the image.
- `origin=` — The #origin= method is used to set the 3D point as the origin of the image.
- `path` — The path method is used to retrieve the path of the file defining the image.
- `pixelheight` — The pixelheight method is used to retrieve the height of the image file in pixels.
- `pixelwidth` — The pixelwidth method is used to retrieve the width of the image file in pixels.
- `size=` — The size= method is used to set the width and height of the image, in inches.
- `transform!` — The transform! method is used to apply a transformation to the image.
- `transformation` — The transformation method is used to retrieve the transformation for the image.
- `transformation=` — The transformation= method is used to set the transformation for the image.
- `width` — The width method is used to retrieve the width of the image.
- `width=` — The width= method is used to set the width of the image.
- `zrotation` — The zrotation method is used to get the angle that the image is rotated about the normal vector from an arbitrary X axis.
