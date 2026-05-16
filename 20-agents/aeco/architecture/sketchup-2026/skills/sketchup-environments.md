---
name: yard-sketchup-environments
description: Sketchup::Environments API reference (YARD)
---

# Sketchup::Environments API reference

An Environments object is a collection of Environment objects. It is used to manage the environments in a model.

## Methods

- `[]` — The #[] method is used to retrieve an Sketchup::Environment by name.
- `add` — The supported file formats are HDR, EXR and SKE.
- `add_observer` — The #add_observer method is used to add an observer to the environments collection.
- `current` — The #current method is used to get the current environment in the Sketchup::Environments.
- `current=` — The #current= method is used to set the current environment in the Sketchup::Environments.
- `each` — The #each method is used to iterate over all the environments in the Sketchup::Environments.
- `purge_unused` — The #purge_unused method is used to remove unused environments.
- `remove` — The #remove method removes an Sketchup::Environment from the Sketchup::Environments.
- `remove_observer` — The #remove_observer method is used to remove an observer from the current object.
- `size` — The #size method retrieves the number of environments in the Sketchup::Environments.
