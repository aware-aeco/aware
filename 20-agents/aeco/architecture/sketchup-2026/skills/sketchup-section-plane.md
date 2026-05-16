---
name: yard-sketchup-section-plane
description: Sketchup::SectionPlane API reference (YARD)
---

# Sketchup::SectionPlane API reference

The SectionPlane class represents a section plane in a SketchUp model. Note that prior to SketchUp 2014 there was no way to create a SectionPlane object using Ruby. For older versions of SketchUp, you must manually create a section plane with the Section Plane Tool in SketchUp and then query the entities array to find the SectionPlane object.

## Methods

- `activate` — The activate method is used to make the section plane the active one of its parent component/group.
- `active?` — The active? method indicates whether the section plane is active or not.
- `get_plane` — The get_plane method is used to retrieve the plane that the section plane is on.
- `name` — The #name method is used to retrieve the name of the section plane.
- `name=` — The #name= method is used to set the name of a section plane.
- `set_plane` — The set_plane method is used to set the plane that the section plane is on.
- `symbol` — The #symbol method is used to retrieve the symbol of the section plane.
- `symbol=` — The symbol must be three characters or less.
