---
name: yard-sketchup-materials
description: Sketchup::Materials API reference (YARD)
---

# Sketchup::Materials API reference

A collection of Materials objects. Each model contains a Materials collection that can be accessed via Model.materials.

## Methods

- `[]` — The #[] method is used to retrieve a material by index or name.
- `add` — Add a new Material.  When called with no arguments, this will generate a new unique name for the new Material.  If a name is given, it will check to see if there is already a material with that name.  If there is already a material with the given name, then a new unique name is generated using the given name as a base.
- `add_observer` — The add_observer method is used to add an observer to the materials collection.
- `[]` — The #[] method is used to retrieve a material by index or name.
- `count` — Since SketchUp 2014 the count method is inherited from Ruby's Enumerable mix-in module. Prior to that the #count method is an alias for #length.
- `current` — The current method is used to get the current material, i.e. the material that the user has selected in the Materials dialog.
- `current=` — The current= method is used to set the current material.
- `each` — Don't remove content from this collection while iterating over it with #each. This would change the size of the collection and cause elements to be skipped as the indices change. Instead copy the current collection to an array using to_a and then use each on the array, when removing content.
- `length` — The returned number includes Image materials as well. It will not reflect the number of materials yielded by #each. To get the number of non-image materials use #count or materials.to_a.size.
- `load` — The #load method is used to load a material from file into the model.
- `purge_unused` — The purge_unused method is used to remove unused materials.
- `remove` — Remove a given material.
- `remove_observer` — The remove_observer method is used to remove an observer from the materials collection.
- `size` — The returned number includes Image materials as well. It will not reflect the number of materials yielded by #each. To get the number of non-image materials use #count or materials.to_a.size.
- `unique_name` — The #unique_name method is used to retrieve a unique name from the materials collection that is based on the provided one. If provided name is unique it will be returned, otherwise any trailing indices will be replaced by a new index.
