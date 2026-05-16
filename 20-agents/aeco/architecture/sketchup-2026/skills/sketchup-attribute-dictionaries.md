---
name: yard-sketchup-attribute-dictionaries
description: Sketchup::AttributeDictionaries API reference (YARD)
---

# Sketchup::AttributeDictionaries API reference

The AttributeDictionaries class is a collection of all of the AttributeDictionary objects that are attached to a given Entity object.

## Methods

- `[]` — Get an AttributeDictionary by name. Returns nil if there is none with the given name.
- `count` — The count method is inherited from the Enumerable mix-in module.
- `delete` — The delete method destroys a given AttributeDictionary. This AttributeDictionary can be passed directly or identified by its string name.
- `each` — The #each method is used to iterate through all of the attributes dictionaries.
- `length` — The #length method returns the number of attribute dictionary objects in the collection.
- `size` — The #size method is an alias of #length.
