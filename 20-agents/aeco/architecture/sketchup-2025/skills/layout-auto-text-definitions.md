---
name: yard-layout-auto-text-definitions
description: Layout::AutoTextDefinitions API reference (YARD)
---

# Layout::AutoTextDefinitions API reference

The AutoTextDefinitions class is a container class for all AutoTextDefinitions in a Document.

## Methods

- `[]` — The #[] method returns a value from the array of Layout::AutoTextDefinitions.
- `add` — The #add method adds an Layout::AutoTextDefinition to the Document.
- `each` — Don't remove content from this collection while iterating over it with #each. This would change the size of the collection and cause elements to be skipped as the indices change. Instead copy the current collection to an array using to_a and then use each on the array, when removing content.
- `index` — The #index method returns the index of the Layout::AutoTextDefinition, or nil if it doesn't exist in the Document.
- `length` — The #length method returns the number of Layout::AutoTextDefinitions.
- `remove` — The #remove method removes an Layout::AutoTextDefinition from the Document.
