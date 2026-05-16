---
name: yard-sketchup-layers
description: Sketchup::Layers API reference (YARD)
---

# Sketchup::Layers API reference

As of SketchUp 2020 “Layers” were renamed to “Tags” in the UI. The API retains the use of “Layer” for compatibility and is synonymous with “Tag”.

## Methods

- `[]` — The #[] method is used to retrieve a layer by index or name.
- `add` — The #add method is used to add a new layer.
- `add_folder` — The #add_folder method adds or moves a layer folder.
- `add_observer` — The #add_observer method is used to add an observer to the layers collection.
- `at` — The #at method is an alias for #[].
- `count` — Since SketchUp 2014 the count method is inherited from Ruby's Enumerable mix-in module. Prior to that the #count method is an alias for #length.
- `count_folders` — The #count_folders method counts the number of folders which are direct children of the layer manager.
- `count_layers` — The #count_layers method retrieves the number of layers not in a folder.
- `each` — Don't remove content from this collection while iterating over it with #each. This would change the size of the collection and cause elements to be skipped as the indices change. Instead copy the current collection to an array using to_a and then use each on the array, when removing content.
- `each_folder` — The #each_folder method is used to iterate through the folders that are direct children to the layer manager.
- `each_layer` — The #each_layer method is used to iterate through the layers that are not inside a layer folder.
- `folders` — This does not return all the folders in the model, only those that are direct children of the layer manager.
- `layers` — The #layers method retrieves the layers not in a folder.
- `length` — The #length method retrieves the number of layers.
- `purge_unused` — The #purge_unused method is used to remove unused layers.
- `purge_unused_folders` — The #purge_unused_folders method is used to remove all layer folder with no children.
- `remove` — Remove the given layer from the model, optionally removing the geometry.
- `remove_folder` — The #remove_folder method removes the folder from the model. All children are preserved, but moved up one level.
- `remove_observer` — The #remove_observer method is used to remove an observer from the current object.
- `size` — The #size method is an alias of #length.
- `unique_name` — The #unique_name method can be used to get a string that will be a unique layer name inside this collection.
