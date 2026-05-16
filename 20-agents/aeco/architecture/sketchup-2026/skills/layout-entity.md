---
name: yard-layout-entity
description: Layout::Entity API reference (YARD)
---

# Layout::Entity API reference

An entity is an object shown on a page of a LayOut document.

## Methods

- `==` — The #== method checks to see if the two Layout::Entitys are equal. This checks whether the Ruby Objects are pointing to the same internal object.
- `attribute_dictionary` — The #attribute_dictionary method returns a copy of the entity's attribute dictionary with the given name.
- `bounds` — The #bounds method returns the 2D rectangular bounds of the Layout::Entity.
- `delete_attribute` — The #delete_attribute method is used to delete an attribute from an entity.
- `document` — The #document method returns the Document that the Layout::Entity belongs to, or nil if it is not in a Document.
- `drawing_bounds` — The #drawing_bounds method returns the 2D rectangular drawing bounds of the Layout::Entity.
- `get_attribute` — The #get_attribute method is used to retrieve the value of an attribute in the entity's attribute dictionary.
- `group` — The #group method returns the Group the Layout::Entity belongs to, or nil if it is not in a Group.
- `layer_instance` — Groups are never associated with a LayerInstance.
- `locked=` — The #locked= method sets the Layout::Entity as locked or unlocked. When locked, the Layout::Entity cannot be modified directly.
- `locked?` — The #locked? method returns whether the Layout::Entity is locked or unlocked.
- `move_to_group` — The #move_to_group method moves the Layout::Entity into a Group. If the Layout::Entity is already in a Group, it will be removed from that Group prior to being added to the new one. If this action results in the old Group containing only one Layout::Entity, the old Group will be collapsed and the remaining Layout::Entity will be moved to the old Group's parent.
- `move_to_layer` — The #move_to_layer method moves the Layout::Entity to the given Layer. If the Layer is non-shared and the Layout::Entity is currently on a shared Layer, an array of Pages must be provided to move the Layout::Entity to. In all other cases, passing in an array of Pages is not necessary. The Layout::Entity must belong to the same Document as the the Layer and the Pages.
- `on_shared_layer?` — The #on_shared_layer? method returns whether or not the Layout::Entity is on a shared Layer. This function works for all Layout::Entity types, including Group. Groups do not belong to a specific Layer, but their children are all on either a shared or non-shared Layer.
- `page` — The #page method returns the Page that the Layout::Entity belongs to, or nil if it is on a shared Layer or not in a Document.
- `set_attribute` — The #set_attribute method adds an attribute to the entity's attribute dictionary.
- `style` — The #style method returns the Style of the Layout::Entity. If the Layout::Entity is a Group, nil will be returned, as they do not have a Style.
- `style=` — The #style= method sets the Style of the Layout::Entity.
- `transform!` — The #transform! method transforms the Layout::Entity with a given Geom::Transformation2d.
- `transformation` — The #transformation method returns the explicit Geom::Transformation2d.
- `untransformed_bounds` — The #untransformed_bounds method returns the untransformed bounds of the Layout::Entity. This is the bounds of the Layout::Entity before its explicit Geom::Transformation2d is applied.
- `untransformed_bounds=` — The #untransformed_bounds= method sets the untransformed bounds of the Layout::Entity. This is the bounds of the Layout::Entity before its explicit Geom::Transformation2d is applied.
