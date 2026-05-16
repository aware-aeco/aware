---
name: yard-sketchup-axes
description: Sketchup::Axes API reference (YARD)
---

# Sketchup::Axes API reference

SketchUp's drawing axes consist of three colored lines (red, green, blue), usually perpendicular to each other, displayed in the drawing area. The exception is when the user open an instance with a non-orthogonal transformation. The root model transformation is always orthogonal.

## Methods

- `axes` — The axes method returns the vectors representing the directions of the axes.
- `origin` — The #origin method returns the origin of the axes.
- `set` — The #set method allows the axes to be manipulated. The axes must always be orthogonal, otherwise an error is thrown.
- `sketch_plane` — The sketch_plane method returns a plane representing the ground plane of the axes.
- `to_a` — The axes method returns the origin and vectors representing the axes.
- `transformation` — The #transformation method returns the transformation of the axes. This is useful when creating tools that respect the model's drawing axes.
- `xaxis` — The #xaxis method returns the x axis of the axes.
- `yaxis` — The #yaxis method returns the y axis of the axes.
- `zaxis` — The #zaxis method returns the z axis of the axes.
