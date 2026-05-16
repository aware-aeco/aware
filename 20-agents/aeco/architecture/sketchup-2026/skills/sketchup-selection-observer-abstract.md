---
name: yard-sketchup-selection-observer-abstract
description: Sketchup::SelectionObserver   Abstract API reference (YARD)
---

# Sketchup::SelectionObserver   Abstract API reference

To implement this observer, create a Ruby class of this type, override the desired methods, and add an instance of the observer to the objects of interests.

## Methods

- `onSelectionAdded` — This event might not trigger even if a single element is selected. For instance the Selection tool will trigger #onSelectionBulkChange regardless.
- `onSelectionBulkChange` — The #onSelectionBulkChange method is called whenever items are added or removed from the selection set.
- `onSelectionCleared` — The #onSelectionCleared method is called when the selection is completely emptied.
- `onSelectionRemoved` — This event might not trigger even if a single element is deselected. For instance the Selection tool will trigger #onSelectionBulkChange regardless.
