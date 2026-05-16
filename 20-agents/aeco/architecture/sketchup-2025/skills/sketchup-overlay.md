---
name: yard-sketchup-overlay
description: Sketchup::Overlay API reference (YARD)
---

# Sketchup::Overlay API reference

An Overlay provides contextual model information directly in the viewport. This can be presented in 2D and 3D.

## Methods

- `initialize` — 
- `description` — This is a short user facing description of the overlay that will appear in the UI.
- `description=` — Sets a short user facing description of the overlay that will appear in the UI. Set this before adding to the Sketchup::OverlaysManager.
- `draw` — It is called whenever the view updates.
- `enabled=` — In most cases, extensions doesn't need to expose any new UI for enabling them. This can be done from the Overlays panel. However, in some cases the extension might have additional UI related to the overlays and might want to offer a way to toggle its overlays along with the rest of the UI.
- `enabled?` — 
- `getExtents` — The method should be implementing sub-classes ensure what is drawn in 3D space doesn't appear clipped. If the overlay only draws in 2D this isn't needed.
- `name` — This is a user facing display name that will appear in the UI.
- `onMouseEnter` — It can be used by implementing sub-classes to react to mouse movement in the viewport.
- `onMouseLeave` — It can be used by implementing sub-classes to react to mouse movement in the viewport.
- `onMouseMove` — It can be used by implementing sub-classes to react to mouse movement in the viewport.
- `overlay_id` — 
- `source` — Describes the source associated with the overlay. This is automatically inferred when the overlay instance is initialized.
- `start` — It can be used by implementing sub-classes to react when the overlay becomes active, for instance when the user turns it on.
- `stop` — It can be used by implementing sub-classes to react when the overlay becomes inactive, for instance when the user turns it off.
- `valid?` — Indicates whether the overlay is valid. An overlay becomes invalid after being removed from the model and cannot be reused.
