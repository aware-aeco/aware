---
name: yard-sketchup-shadow-info-observer-abstract
description: Sketchup::ShadowInfoObserver   Abstract API reference (YARD)
---

# Sketchup::ShadowInfoObserver   Abstract API reference

To implement this observer, create a Ruby class of this type, override the desired methods, and add an instance of the observer to the ShadowInfo object.

## Methods

- `onShadowInfoChanged` — The #onShadowInfoChanged method is invoked whenever the user alters a setting inside the Shadows and Model Info dialogs. The type parameter contains a number identifying which option was altered. Here are the types to expect:
