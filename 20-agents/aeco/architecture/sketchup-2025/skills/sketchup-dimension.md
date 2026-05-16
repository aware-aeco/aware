---
name: yard-sketchup-dimension
description: Sketchup::Dimension API reference (YARD)
---

# Sketchup::Dimension API reference

The Dimension class provides base functionality for classes DimensionLinear and DimensionRadial. It's not instantiable.

## Methods

- `add_observer` — If the given observer responds to onTextChanged, it will be added as a Sketchup::DimensionObserver. If not, the base Entity#add_observer will be called.
- `arrow_type` — The #arrow_type method retrieves the current arrow type of the dimension.
- `arrow_type=` — The #arrow_type= method sets the arrow type of the dimension.
- `had_aligned_text=` — The has_aligned_text= method accepts true or false indicating whether the dimension's text is aligned to the dimension or to the screen.
- `has_aligned_text?` — The has_aligned_text method is used to determine whether the dimension's text is aligned to the dimension or to the screen.
- `plane` — The plane method is used to retrieve the plane of the dimension. Refer to the Geom module for information on how planes are represented.
- `remove_observer` — The remove_observer method is used to remove a DimensionObserver from the dimension. Note that, if the given observer responds to 'onTextChanged', it will be removed as a DimensionObserver. If not, the base Entity.remove_observer will be called.
- `text` — The text method is used to retrieve the dimension text.
- `text=` — The text= method is used to set an override on the dimension text.
