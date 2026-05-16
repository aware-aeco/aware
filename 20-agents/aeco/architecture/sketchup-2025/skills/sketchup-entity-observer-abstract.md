---
name: yard-sketchup-entity-observer-abstract
description: Sketchup::EntityObserver   Abstract API reference (YARD)
---

# Sketchup::EntityObserver   Abstract API reference

To  implement this observer, create a Ruby class of this type, override the desired methods, and add an instance of the observer to the entity of interests.

## Methods

- `onChangeEntity` — The #onChangeEntity method is invoked when your entity is modified.
- `onEraseEntity` — The #onEraseEntity method is invoked when your entity is erased.
