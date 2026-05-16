---
name: yard-sketchup-shadow-info
description: Sketchup::ShadowInfo API reference (YARD)
---

# Sketchup::ShadowInfo API reference

The ShadowInfo class contains method to extract the shadow information for a model. The majority of the shadow information returned exists in the Model Info > Location and Model Info > Shadows dialogs inside SketchUp.

## Methods

- `each_key` — The #each_key method iterates through all of the shadow information keys.
- `keys` — The keys method is a class method that returns an array with all of the attribute keys
- `[]` — The [] method retrieves a value from the array of keys
- `[]=` — The set value []= method is used to set the value in the array of shadow info options.
- `add_observer` — The add_observer method is used to add an observer to the current object.
- `count` — The count method is inherited from the Enumerable mix-in module.
- `each` — The #each method iterates through all of the shadow information key/value pairs.
- `each_key` — The #each_key method iterates through all of the shadow information keys.
- `each_pair` — The #each_pair method is an alias for #each.
- `keys` — The keys method is a class method that returns an array with all of the attribute keys
- `length` — The #length method returns the number of options in the shadow options collection
- `remove_observer` — The remove_observer method is used to remove an observer from the current object.
- `size` — The #size method is an alias for #length.
