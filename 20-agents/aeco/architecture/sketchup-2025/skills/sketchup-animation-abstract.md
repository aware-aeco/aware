---
name: yard-sketchup-animation-abstract
description: Sketchup::Animation   Abstract API reference (YARD)
---

# Sketchup::Animation   Abstract API reference

Implement the methods described in this class to create a an animation. You can not sub-class this class because it is not defined by the API.

## Methods

- `nextFrame` — The #nextFrame method is invoked by SketchUp to tell the animation to display its next frame. This method should set up the camera and then call View#show_frame.
- `pause` — The user interface for pausing and resuming animations isn't integrated with the Ruby API in the current version, so this method is probably not useful to you.
- `resume` — The user interface for pausing and resuming animations isn't integrated with the Ruby API in the current version, so this method is probably not useful to you.
- `stop` — Do not call Sketchup::Animation#Sketchup#Sketchup::View#Sketchup::View#animation= from this method. This will cause a recursive loop and crash SketchUp 2017 and earlier versions. As of SketchUp 2018 this will raise a RunTimeError.
