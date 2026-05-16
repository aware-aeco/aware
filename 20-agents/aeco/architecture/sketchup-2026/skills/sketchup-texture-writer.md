---
name: yard-sketchup-texture-writer
description: Sketchup::TextureWriter API reference (YARD)
---

# Sketchup::TextureWriter API reference

The TextureWriter class is used primarily for writing the textures used in a SketchUp model out to files as part of an export for use in another application. These methods are usually invoked in this order:

## Methods

- `count` — The #length method is used to determine the number of textures loaded into the texture writer.
- `filename` — The filename method is used to retrieve the original filename for a particular texture.
- `handle` — The handle method is used to retrieve a handle or index for a specific texture in the texture writer.
- `length` — The #length method is used to determine the number of textures loaded into the texture writer.
- `load` — If you are passing a face in as the entity argument when loading a texture you will have to specify the second boolean argument, side. The argument side specifies which side of the face the texture will be loaded from.
- `write` — If you are passing a face in as the entity argument when writing a texture you will have to specify the boolean argument, side. The argument side controls the side of the face from which the texture will be sampled before writing it.
- `write_all` — The write_all method is used to write all of the textures within the texture writer to files. It will return one of three status numbers:
