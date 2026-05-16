---
name: yard-layout-pages
description: Layout::Pages API reference (YARD)
---

# Layout::Pages API reference

The Pages class is a container class for all pages in a Document.

## Methods

- `[]` — The #[] method returns a value from the array of Layout::Pages.
- `add` — The #add method adds a new Layout::Page to the Document. The newly added Layout::Page will be the last one in the Document.
- `each` — Don't remove content from this collection while iterating over it with #each. This would change the size of the collection and cause elements to be skipped as the indices change. Instead copy the current collection to an array using to_a and then use each on the array, when removing content.
- `index` — The #index method returns the index of the Layout::Page, or nil if it doesn't exist in the Document.
- `initial` — The #initial method returns the initial Layout::Page that will be displayed the next time the Document is opened. This value will change whenever the Layout::Page is changed in the Document in LayOut.
- `initial=` — The #initial= method sets the initial Layout::Page that will be displayed the next time the Document is opened. This value will change whenever the Layout::Page is changed in the Document in LayOut.
- `length` — The #length method returns the number of Layout::Pages.
- `remove` — The #remove method deletes the given Layout::Page from the Document.
- `reorder` — The #reorder method moves a Layout::Page to a different index within the Document's list of pages. This will move the Layout::Page such that its new index becomes new_index.
