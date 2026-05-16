---
name: yard-sketchup-pages-observer-abstract
description: Sketchup::PagesObserver   Abstract API reference (YARD)
---

# Sketchup::PagesObserver   Abstract API reference

To implement this observer, create a Ruby class of this type, override the desired methods, and add an instance of the observer to the objects of interests.

## Methods

- `onContentsModified` — The #onContentsModified method is invoked whenever the pages change.
- `onElementAdded` — The #onElementAdded method is invoked when an element is added to a Sketchup::Pages object.
- `onElementRemoved` — The #onElementRemoved method is invoked when an element is removed from a Sketchup::Pages object.
