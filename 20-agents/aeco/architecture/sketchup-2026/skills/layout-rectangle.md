---
name: yard-layout-rectangle
description: Layout::Rectangle API reference (YARD)
---

# Layout::Rectangle API reference

A simple rectangular shape entity.

## Methods

- `initialize` — The #initialize method creates a new normal, lozenge, bulged or rounded Layout::Rectangle, depending on the type passed in.
- `radius` — The #radius method returns the radius of the Layout::Rectangle, or nil if the Layout::Rectangle is not of type Layout::Rectangle::TYPE_BULGED or Layout::Rectangle::TYPE_ROUNDED
- `radius=` — The #radius= method sets the radius of the Layout::Rectangle.
- `type` — The #type method returns the type of the Layout::Rectangle.
- `type=` — The #type= method sets the type of the Layout::Rectangle. If the type is set to Layout::Rectangle::TYPE_ROUNDED or Layout::Rectangle::TYPE_BULGED, the [Layout::Rectangle]'s radius will be set to the default value of 0.25, if the value had not previously been set.
