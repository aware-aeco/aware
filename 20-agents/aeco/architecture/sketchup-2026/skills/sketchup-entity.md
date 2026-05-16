---
name: yard-sketchup-entity
description: Sketchup::Entity API reference (YARD)
---

# Sketchup::Entity API reference

This is the base class for all SketchUp entities. Entities are basically anything that can be contained in a model, including Drawingelements such as Edges, SectionPlanes, Groups, etc. and entities that relate to those Drawingelements, such as Loops, Layers, etc.

## Methods

- `add_observer` — The add_observer method is used to add an observer to the current object.
- `attribute_dictionaries` — The return value may be either nil or an empty AttributeDictionaries collection if this entity has no AttributeDictionarys.
- `attribute_dictionary` — The attribute_dictionary method is used to retrieve an attribute dictionary with a given name that is attached to an Entity.
- `delete_attribute` — In SketchUp 2018, special attribute dictionaries have been added. The name of these dictionaries are “SU_InstanceSet” and “SU_DefinitionSet”. The dictionaries cannot be deleted via ruby and an ArgumentError will be raised. The key/value pairs in the dictionary can be deleted safely.
- `deleted?` — The deleted? method is used to determine if your entity is still valid (not deleted by another script, for example.)
- `entityID` — The entityID method is used to retrieve a unique ID assigned to an entity.
- `get_attribute` — The #get_attribute method is used to retrieve the value of an attribute in the entity's attribute dictionary.
- `inspect` — The #inspect method is used to retrieve the string representation of the entity.
- `model` — The model method is used to retrieve the model for the entity.
- `parent` — The parent method is used to retrieve the parent of the entity.
- `persistent_id` — Only a subset of entity types support PIDs. Refer to the table below for which and when support was added. In general it is entities that you can iterate over in a Sketchup::Entities collection.
- `remove_observer` — The remove_observer method is used to remove an observer from the current object.
- `set_attribute` — The set attribute is used to set the value of an attribute in an attribute dictionary with the given name.
- `to_s` — The #to_s method is used to retrieve the string representation of the entity.
- `typename` — Prefer is_a? over typename when possible as it is faster.
- `valid?` — The #valid? method is used to determine if your entity is still valid (not deleted by another script, for example).
