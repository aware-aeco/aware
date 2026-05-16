---
name: yard-layout-layers
description: Layout::Layers API reference (YARD)
---

# Layout::Layers API reference

The Layers class is a container class for all layers in a Document.

## Methods

- `[]` — The #[] method returns a value from the array of Layout::Layers.
- `active` — The #active method returns the active Layout::Layer in the Document.
- `active=` — The #active= method sets the active Layout::Layer that will be displayed the next time the Document is opened. This value will change whenever the Layout::Layer is changed in the Document in LayOut.
- `add` — The #add method adds a new Layout::Layer to the Document. The newly added Layout::Layer will be the last one in the Document.
- `each` — Don't remove content from this collection while iterating over it with #each. This would change the size of the collection and cause elements to be skipped as the indices change. Instead copy the current collection to an array using to_a and then use each on the array, when removing content.
- `index` — The #index method returns the index of the Layout::Layer, or nil if it doesn't exist in the Document.
- `length` — The #length method returns the number of Layout::Layers.
- `remove` — The #remove method deletes the given Layout::Layer from the Document.
- `reorder` — The #reorder method moves a Layout::Layer to a different index within the Document's list of layers. This will move the Layout::Layer such that its new index becomes new_index.
