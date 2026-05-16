---
name: yard-sketchup-entities-observer-abstract
description: Sketchup::EntitiesObserver   Abstract API reference (YARD)
---

# Sketchup::EntitiesObserver   Abstract API reference

To implement this observer, create a Ruby class of this type, override the desired methods, and add an instance of the observer to the objects of interests.

## Methods

- `onActiveSectionPlaneChanged` — The #onActiveSectionPlaneChanged method is invoked when a section plane within this entities is activated or the active one is deactivated.
- `onElementAdded` — The onElementAdded method is invoked when a single element is added to the Sketchup::Entities collection.
- `onElementModified` — The #onElementModified method is invoked whenever one or more elements in the collection are modified.
- `onElementRemoved` — The #onElementRemoved method is invoked when a single element is removed from the Sketchup::Entities collection.  Note that the entity has been deleted and should not be used in anyway except to know that the entity has been deleted.
- `onEraseEntities` — The #onEraseEntities method is invoked when one or more entities are erased.
