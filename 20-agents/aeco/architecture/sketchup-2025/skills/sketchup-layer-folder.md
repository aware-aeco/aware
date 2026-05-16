---
name: yard-sketchup-layer-folder
description: Sketchup::LayerFolder API reference (YARD)
---

# Sketchup::LayerFolder API reference

As of SketchUp 2020 “Layers” were renamed to “Tags” in the UI. The API retains the use of “Layer” for compatibility and is synonymous with “Tag”.

## Methods

- `<=>` — The #<=> method is used to compare two layer folders based on their names. This enables the Ruby Array#sort method to sort SketchUp layer folders.
- `==` — The #== method is used to determine if two layer folders are the same.
- `add_folder` — The #add_folder method adds or moves a layer folder.
- `add_layer` — The #add_layer method adds a layer to a folder.
- `count_folders` — The #count_folders method retrieves the number of child folders in the folder.
- `count_layers` — The #count_layers method retrieves the number of layers in the folder.
- `each_folder` — The #each_folder method is used to iterate through the folders that are direct children to the folder.
- `each_layer` — The #each_layer method is used to iterate through the layers that are direct children to the folder.
- `folder` — The #folder method is used to return the parent layer folder of a layer folder.
- `folder=` — The #folder= method is used to set the parent layer folder of a layer folder.
- `folders` — The #folders returns the direct child-folders of the folder.
- `layers` — The #layers method retrieves the child layers of a folder.
- `name` — The #name method gets the name of the folder.
- `name=` — The #name= method sets the name of the folder.
- `remove_folder` — The #remove_folder method removes the folder from the model. All children are preserved, but move up one level.
- `remove_layer` — The #remove_layer method removes a layer from a folder. The layer will be parent to the layer manager.
- `visible=` — The #visible= method is used to set if the layer folder is visible.
- `visible?` — The #visible? method is used to determine if the layer folder is visible.
- `visible_on_new_pages=` — The #visible_on_new_pages= method is used to set if the layer folder is by default visible on new pages.
- `visible_on_new_pages?` — The #visible_on_new_pages? method is used to determine if the layer folder is by default visible on new pages.
