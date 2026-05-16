---
name: yard-sketchup-environments-observer-abstract
description: Sketchup::EnvironmentsObserver   Abstract API reference (YARD)
---

# Sketchup::EnvironmentsObserver   Abstract API reference

To implement this observer, create a Ruby class of this type, override the desired methods, and add an instance of the observer to the Environment object.

## Methods

- `onEnvironmentAdd` — The #onEnvironmentAdd method is called whenever an environment is added to the Sketchup::Environments.
- `onEnvironmentChange` — The #onEnvironmentChange method is called whenever the environment properties are changed.
- `onEnvironmentRemove` — The #onEnvironmentRemove method is called whenever an environment is removed from the Sketchup::Environments.
- `onEnvironmentSetCurrent` — The #onEnvironmentSetCurrent method is called whenever the current environment is changed.
