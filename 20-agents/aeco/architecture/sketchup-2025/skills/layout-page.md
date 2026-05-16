---
name: yard-layout-page
description: Layout::Page API reference (YARD)
---

# Layout::Page API reference

Class for a single page in a LayOut document.

## Methods

- `==` — The #== method checks to see if the two Layout::Pages are equal. This checks whether the Ruby Objects are pointing to the same internal object.
- `document` — The #document method returns the Document that the Layout::Page belongs to.
- `entities` — The #entities method returns all Entitys that are on the Layout::Page. This is the equivalent of iterating over all LayerInstances and using LayerInstance.entities.
- `in_presentation=` — The #in_presentation= method sets whether the Layout::Page is included in presentations.
- `in_presentation?` — The #in_presentation? method returns whether the Layout::Page is included in presentations.
- `layer_instances` — The #layer_instances method returns an array of the LayerInstances for the Layout::Page.
- `layer_visible?` — The #layer_visible? method returns whether a Layer is visible on the Layout::Page.
- `name` — The #name method returns the name of the Layout::Page.
- `name=` — The #name= method sets the name of a page.
- `nonshared_entities` — The #nonshared_entities method returns the Entities unique to the Layout::Page.
- `set_layer_visibility` — The #set_layer_visibility method sets whether a Layer is visible on the Layout::Page.
