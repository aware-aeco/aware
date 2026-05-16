---
name: yard-sketchup-options-manager
description: Sketchup::OptionsManager API reference (YARD)
---

# Sketchup::OptionsManager API reference

The OptionsManager class manages various kinds of OptionsProviders on a Model.

## Methods

- `[]` — The [] method is used to get an option provider by name or index.
- `count` — Since SketchUp 2014 the count method is inherited from Ruby's Enumerable mix-in module. Prior to that the #count method is an alias for #length.
- `each` — The #each method is used to iterate through options providers.
- `keys` — The keys method is used to get a list of keys in the OptionsManager.
- `length` — The #length method is an alias of #size.
- `size` — The #size method returns the number of Sketchup::OptionsProvider objects inside this Sketchup::OptionsManager.
