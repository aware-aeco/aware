---
name: yard-sketchup-definitions-observer-abstract
description: Sketchup::DefinitionsObserver   Abstract API reference (YARD)
---

# Sketchup::DefinitionsObserver   Abstract API reference

To implement this observer, create a Ruby class of this type, override the desired methods, and add an instance of the observer to the collection of interest.

## Methods

- `onComponentAdded` — The #onComponentAdded method is called whenever a definition is added to the definitions collection.
- `onComponentPropertiesChanged` — The #onComponentPropertiesChanged method is called whenever a definition's name or description are changed.
- `onComponentRemoved` — This methods fires twice for each Component/Group erased.
- `onComponentTypeChanged` — The #onComponentTypeChanged event is fired when a component is converted to a group or vice versa. (In the underlying implementation, Groups are just a special kind of definition that is allowed to only have a single instance.)
