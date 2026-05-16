---
name: yard-layout-layer-instance
description: Layout::LayerInstance API reference (YARD)
---

# Layout::LayerInstance API reference

References an instance of a Layer. A LayerInstance provides access to the Entitys that are on it, as well as their draw order. Groups are not included in the list of Entitys associated with a LayerInstance, since the group hierarchy has no effect on entity draw order. Each Page has one LayerInstance for each Layer in the Document. Non-shared LayerInstances are unique per Page, while shared LayerInstances are shared across all Pages of the Document.

## Methods

- `==` — The #== method checks to see if the two Layout::LayerInstances are equal. This checks whether the Ruby Objects are pointing to the same internal object.
- `definition` — The #definition method returns the Layout::Layer of the Layout::LayerInstance.
- `entities` — The #entities method returns the Entities on the Layout::LayerInstance.
- `entity_index` — The #entity_index method returns the index of the Entity on the Layout::LayerInstance.
- `reorder_entity` — The #reorder_entity method moves the Entity to the specified index.
