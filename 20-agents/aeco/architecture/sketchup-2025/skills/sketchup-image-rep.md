---
name: yard-sketchup-image-rep
description: Sketchup::ImageRep API reference (YARD)
---

# Sketchup::ImageRep API reference

References an image representation object.

## Methods

- `initialize` — The #initialize method creates a new image object. The image object will have no data if a path to the image is not provided.
- `bits_per_pixel` — The #bits_per_pixel method gets the number of bits per pixel in the image.
- `color_at_uv` — The #color_at_uv method returns a color corresponding to the UV texture coordinates. 0.0, 0.0 maps to the bottom left and 1.0, 1.0 to the top right of the image.
- `colors` — The #colors method returns an array of Color for each pixel in the image.
- `data` — The byte order of the pixels are RGB(A) on macOS and BGR(A) on Windows.
- `height` — The #height method returns the height of an image.
- `load_file` — The #load_file method loads image data from the specified file.
- `row_padding` — The #row_padding method returns the size of the row padding of an image in bytes.
- `save_file` — The #save_file method saves an image data object to an image file specified by a path.
- `set_data` — The byte order of the pixels are RGB(A) on macOS and BGR(A) on Windows.
- `size` — The #size method gets the total size of the image data in bytes.
- `width` — The #width method returns the width of an image.
