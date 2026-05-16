---
name: yard-sketchup-texture
description: Sketchup::Texture API reference (YARD)
---

# Sketchup::Texture API reference

The Texture class contains methods for obtaining information about textures that are part of your materials in your model (within the In-Model section of the Materials Browser). Remember, textures are repeatable images that “tile” when painted on a surface.

## Methods

- `average_color` — The average_color method retrieves a color object with the average color found in the texture.
- `filename` — Since SketchUp 2021.0 this method will append a file extension matching the image format if the file extension is missing from stored filepath.
- `height` — The height method is used to get the height of a repeatable texture image, in inches.
- `image_height` — The image_height method retrieves the height of the repeatable texture image, in pixels.
- `image_rep` — The #image_rep method returns a copy of a ImageRep object representing the texture pixel data.
- `image_width` — The image_width method retrieves the width of the repeatable texture image, in pixels.
- `size=` — The size= method allows you to set the size of the repeatable texture image, in inches,
- `valid?` — The valid? method ensures that a texture is valid.
- `width` — The width method is used to get the width of a repeatable texture image, in inches.
- `write` — Writes the texture to file with option to preserve the color adjustments made by the material.
