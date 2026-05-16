---
name: yard-sketchup-rendering-options
description: Sketchup::RenderingOptions API reference (YARD)
---

# Sketchup::RenderingOptions API reference

The RenderingOptions class contains method to extract the rendering information for a model. The majority of the rendering information returned exists in the Styles dialog. The following rendering information keys are maintained in SketchUp:

## Methods

- `each_key` — The #each_key method iterates through all of the rendering options keys.
- `keys` — The keys method returns an array with all of the attribute keys.
- `[]` — The #[] method is used to get the value of a rendering option.
- `[]=` — The set value []= method is used to set the value in the array of rendering options.
- `add_observer` — The add_observer method is used to add an observer to the current object.
- `count` — The #count method is inherited from the Enumerable mix-in module.
- `each` — The #each method iterates through all of the rendering options key/value pairs.
- `each_key` — The #each_key method iterates through all of the rendering options keys.
- `each_pair` — The #each_pair method is an alias for #each.
- `keys` — The keys method returns an array with all of the attribute keys.
- `length` — The #length method returns the number of options in the rendering options collection.
- `remove_observer` — The remove_observer method is used to remove an observer from the current object.
- `size` — The #size method is an alias for #length.
