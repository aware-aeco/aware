---
name: yard-sketchup-behavior
description: Sketchup::Behavior API reference (YARD)
---

# Sketchup::Behavior API reference

The Behavior class is used to control the “behavior” of components, which roughly correlates to the series of options that you see in the Components dialog under the “edit” tab, such as whether it casts shadows, glues to walls, etc.

## Methods

- `always_face_camera=` — The always_face_camera= method is used to set the always_face_camera behavior for a component.
- `always_face_camera?` — The always_face_camera? method is used to retrieve the  always_face_camera behavior for a component.
- `cuts_opening=` — To enable cut opening, also set #is2d= to true.
- `cuts_opening?` — The cuts_opening? method is used to get the status of a component's cut opening behavior.
- `is2d=` — The #is2d= method is used to set whether the component can glue to other entities or not.
- `is2d?` — The #is2d? method is used to get whether the component can glue to other entities or not.
- `no_scale_mask=` — Sets an integer that is really a bit-by-bit description of which scale tool handles are hidden on a given component. This is useful for creating definitions that can only be scaled in particular ways. If a bit contains a a 1, then a certain handle set will be hidden when the user selects the component and activates the Scale tool. Here is the map of which bits control which handles.
- `no_scale_mask?` — The no_scale_mask? method returns an integer that is a bit-by-bit description of which scale tool handles are hidden when the user selects this single component with the scale tool. See the no_scale_mask= method for details on the bit encodings used.
- `shadows_face_sun=` — The shadows_face_sun= method is used to identify whether the component's shadow will be cast from the component's current position as though the component were facing the sun. See the Component entity within the SketchUp User's guide for more information on this feature.
- `shadows_face_sun?` — The shadows_face_sun? method is used to determine whether the component's shadow is being cast from the component's current position (as though the component were facing the sun). See the Component entity within the SketchUp User's guide for more information on this feature.
- `snapto` — The #snapto method is used to see how a component can glue to other entities.
- `snapto=` — To enable gluing, also set #is2d= to true.
