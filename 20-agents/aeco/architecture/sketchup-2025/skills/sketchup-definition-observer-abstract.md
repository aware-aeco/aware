---
name: yard-sketchup-definition-observer-abstract
description: Sketchup::DefinitionObserver   Abstract API reference (YARD)
---

# Sketchup::DefinitionObserver   Abstract API reference

To implement this observer, create a Ruby class of this type, override the desired methods, and add an instance of the observer to the definitions of interests.

## Methods

- `onComponentInstanceAdded` — The #onComponentInstanceAdded method is called when a new component instance is added to a model.
- `onComponentInstanceRemoved` — Due to the underlying way that the SketchUp Move Tool is implemented, this method is fired on a Move + Copy operation even though no ComponentInstance is apparently removed.
