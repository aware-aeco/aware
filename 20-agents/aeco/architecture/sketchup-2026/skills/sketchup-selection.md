---
name: yard-sketchup-selection
description: Sketchup::Selection API reference (YARD)
---

# Sketchup::Selection API reference

A set of the currently selected drawing elements. Use the Model.selection method to get a Selection object.  Note that the order of drawing elements (selection[0], selection[1] and so on) in the set is in no particular order and should not be assumed to be in the same order as the user selected the drawing elements.

## Methods

- `[]` — The #[] method is used to retrieve a Drawingelement from the selection by index. Index 0 is the first Drawingelement in the selection.
- `add` — The #add method is used to add Drawingelement to the selection. Drawingelements that are added to the Selection are visually indicated by the selection bounding box.
- `add_observer` — The add_observer method is used to add an observer to the selection object.
- `at` — The #at method is an alias for #[].
- `clear` — The clear method is used to clear the selection.
- `contains?` — The #contains? method is and alias of #include?.
- `count` — Since SketchUp 2014 the count method is inherited from Ruby's Enumerable mix-in module. Prior to that the #count method is an alias for #length.
- `each` — Don't remove content from this collection while iterating over it with #each. This would change the size of the collection and cause elemnts to be skipped as the indices change. Instead copy the current collection to an array using to_a and then use each on the array, when removing content.
- `empty?` — The #empty? method is used to determine if there are drawing elements in the selection.
- `first` — The #first method is used to retrieve the first selected Drawingelement
- `include?` — The #include? method is used to determine if a given Drawingelement is in the selection.
- `invert` — The #invert method is used to invert the selection.
- `is_curve?` — The #is_curve? method is used to determine if the selection contains all edges that belong to a single curve.
- `is_surface?` — The #is_surface? method is used to determine if the selection contains only all of the faces that are part of a single curved surface.
- `length` — The #length method is used to retrieve the number of selected drawing elements.
- `model` — The #model method retrieves the model for the selection.
- `nitems` — The #nitems method is an alias for #length.
- `remove` — The #remove method is used to remove Drawingelements from the selection. You can pass it individual Drawingelements or an Array of Drawingelements.
- `remove_observer` — The remove_observer method is used to remove an observer from the selection object.
- `shift` — The #shift method is used to remove the first Drawingelement from the selection and returns it.
- `single_object?` — The #single_object? method is used to determine if the selection contains a single object.
- `size` — The #size method is an alias for #length.
- `toggle` — The #toggle method is used to change whether a Drawingelement is part of the selection. Drawingelements that are not already selected are added. Drawingelements that are already selected are removed.
