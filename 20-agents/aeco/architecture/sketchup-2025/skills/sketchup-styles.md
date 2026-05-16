---
name: yard-sketchup-styles
description: Sketchup::Styles API reference (YARD)
---

# Sketchup::Styles API reference

The Styles class contains methods for manipulating a collection of styles in a model. Typically, you will access this via the active_model:

## Methods

- `[]` — The #[] method is used to retrieves a style by either name or index.
- `active_style` — The #active_style method is used to retrieve the active style.
- `active_style_changed` — The #active_style_changed method tells you if the active style has been edited by the user since it was last saved.
- `add_style` — The #add_style method is used to create and load a style from the given file.
- `count` — Since SketchUp 2014 the count method is inherited from Ruby's Enumerable mix-in module. Prior to that the #count method is an alias for #length.
- `each` — The #each method is used to iterate through styles.
- `length` — The #length method is an alias of #size.
- `parent` — The #parent method is used to return the model for the styles.
- `purge_unused` — The #purge_unused method is used to remove unused styles from the model.
- `selected_style` — The #selected_style method is used to retrieve the style currently selected in the Styles Browser.
- `selected_style=` — The #selected_style= method is used to set the currently selected style.
- `size` — The #size method is used to retrieve the number of styles in the collection.
- `update_selected_style` — The #update_selected_style method commits the current style settings to the style selected in the Style Browser.
