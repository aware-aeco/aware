---
name: yard-sketchup-frame-change-observer-abstract
description: Sketchup::FrameChangeObserver   Abstract API reference (YARD)
---

# Sketchup::FrameChangeObserver   Abstract API reference

Implement the methods described in this class to create a frame change observer. You can not sub-class this class because it is not defined by the API.

## Methods

- `frameChange` — The from_scene argument into this callback does not appear to be populated on the PC. You can store a variable that keeps track of the to_scene and then use that on a subsequent Scene selection to determine the last Page that the user was on.
