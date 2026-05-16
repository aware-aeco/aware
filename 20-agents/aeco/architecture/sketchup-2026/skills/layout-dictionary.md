---
name: yard-layout-dictionary
description: Layout::Dictionary API reference (YARD)
---

# Layout::Dictionary API reference

This is the interface to a LayOut dictionary. A Dictionary wraps key/value pairs.

## Methods

- `initialize` — The #initialize method creates a new Layout::Dictionary.
- `[]` — The #[] method retrieves the value for a given key.
- `[]=` — The #[]= method sets a value for a given key.
- `delete_key` — The #delete_key method deletes a key/value pair from the dictionary.
- `each` — The #each_pair method is an alias for #each.
- `each_key` — The #each_key method iterates through all of the dictionary keys.
- `each_pair` — The #each_pair method is an alias for #each.
- `empty?` — The #empty? method checks if the dictionary is empty.
- `keys` — The #keys method retrieves an array with all of the dictionary keys.
- `length` — The #length method retrieves the size (number of elements) of a dictionary.
- `size` — The #length method retrieves the size (number of elements) of a dictionary.
- `values` — The #values method retrieves an array with all of the dictionary values.
