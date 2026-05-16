---
name: yard-sketchup-extensions-manager
description: Sketchup::ExtensionsManager API reference (YARD)
---

# Sketchup::ExtensionsManager API reference

The ExtensionsManager class provides a way of accessing the SketchupExtensions that have been registered via the Sketchup.register_extension method.

## Methods

- `[]` — The [] method is used to get an extension by name, index or ID.
- `count` — Since SketchUp 2014 the count method is inherited from Ruby's Enumerable mix-in module. Prior to that the #count method is an alias for #length.
- `each` — The #each method is used to iterate through extensions.
- `keys` — The keys method is used to get a list of keys in the ExtensionsManager, which are the same as the names of the extensions.
- `length` — The #length method returns the number of SketchupExtension objects inside this ExtensionsManager.
- `size` — The #size method is an alias of #length.
