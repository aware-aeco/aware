---
name: yard-sketchup-attribute-dictionary
description: Sketchup::AttributeDictionary API reference (YARD)
---

# Sketchup::AttributeDictionary API reference

The AttributeDictionary class allows you to attach arbitrary collections of attributes to a SketchUp entity. The attributes are defined by key/value pairs where the keys are strings.  An Entity or Model object can have any number of AttributeDictionary objects (see the AttributeDictionaries class).

## Methods

- `[]` — The [] method is used to retrieve the attribute with a given key.
- `[]=` — The set value ([]=) method is used to set the value of an attribute with a given key.
- `count` — The count method is inherited from the Enumerable mix-in module.
- `delete_key` — The delete_key method is used to delete an attribute with a given key.
- `each` — Don't remove content from this collection while iterating over it with #each. This would change the size of the collection and cause elements to be skipped as the indices change. Instead copy the current collection to an array using to_a and then use each on the array, when removing content.
- `each_key` — The #each_key method is used to iterate through all of the attribute keys.
- `each_pair` — The #each_pair method is an alias for #each.
- `empty?` — The #empty? method is used to check if the attribute dictionary is empty.
- `keys` — The keys method is used to retrieve an array with all of the attribute keys.
- `length` — The #length method is used to retrieve the size (number of elements) of an attribute dictionary.
- `name` — The name method is used to retrieve the name of an attribute dictionary.
- `size` — The #size method is an alias of #length.
- `values` — The values method is used to retrieve an array with all of the attribute values.
