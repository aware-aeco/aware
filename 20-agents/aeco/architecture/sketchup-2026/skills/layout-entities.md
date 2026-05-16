---
name: yard-layout-entities
description: Layout::Entities API reference (YARD)
---

# Layout::Entities API reference

The Entities class is a container class for Entitys. A Entities object is different from a SketchUp::Entities object in that it is read-only. Adding or removing Entitys from a Document happens with the Document#add_entity and Document#remove_entity methods.

## Methods

- `[]` — The #[] method returns the Layout::Entity at the given index. This method is not valid for use when the Layout::Entities object came from a Page.
- `each` — Don't remove content from this collection while iterating over it with #each. This would change the size of the collection and cause elements to be skipped as the indices change. Instead copy the current collection to an array using to_a and then use each on the array, when removing content.
- `length` — The #length method returns the number of Layout::Entitys.
- `reverse_each` — The #reverse_each method iterates through all of the Layout::Entitys in reverse order. When iterating over a LayerInstance's Layout::Entities, it is not necessary to provide a flags Hash. When iterating over a Page's Layout::Entities, the flags Hash is optional; providing no Hash will result in iterating over all Layout::Entitys, including those on hidden or locked Layers. Valid symbols for the Hash are :skip_hidden and :skip_locked.
