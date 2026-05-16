---
name: yard-layout-layer
description: Layout::Layer API reference (YARD)
---

# Layout::Layer API reference

This is the interface to a LayOut Layer Definition. A layer definition specifies the document-wide information about a layer. To access the entities on a layer for a given page, use LayerInstance.

## Methods

- `==` — The #== method checks to see if the two Layout::Layers are equal. This checks whether the Ruby Objects are pointing to the same internal object.
- `document` — The #document method returns the Document that the Layout::Layer belongs to.
- `layer_instance` — The #layer_instance method returns a Layout::LayerInstance from the Layout::Layer. If the Layout::Layer is shared, a Page does not have to be provided.
- `locked=` — The #locked= method sets whether the Layout::Layer is locked.
- `locked?` — The #locked? method returns whether the Layout::Layer is locked.
- `name` — The #name method returns the name of the Layout::Layer.
- `name=` — The #name= sets the name of the Layout::Layer.
- `set_nonshared` — The #set_nonshared method sets the Layout::Layer to non-shared.
- `set_shared` — The #set_shared method sets the Layout::Layer to shared.
- `shared?` — The #shared? method returns whether the Layout::Layer is shared.
