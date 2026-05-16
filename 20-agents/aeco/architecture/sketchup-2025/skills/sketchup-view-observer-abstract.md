---
name: yard-sketchup-view-observer-abstract
description: Sketchup::ViewObserver   Abstract API reference (YARD)
---

# Sketchup::ViewObserver   Abstract API reference

To implement this observer, create a Ruby class of this type, override the desired methods, and add an instance of the observer to the View object.

## Methods

- `onScaleFactorChange` — The #onScaleFactorChange method is called whenever the view DPI of the view changes. This can be the SketchUp window being moved to another monitor with a different DPI or the user changing the DPI settings of the monitor.
- `onViewChanged` — The #onViewChanged method is called whenever the view is altered, such as when the user uses the Pan, Orbit, or Zoom tools.
