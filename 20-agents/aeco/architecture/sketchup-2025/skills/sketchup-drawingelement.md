---
name: yard-sketchup-drawingelement
description: Sketchup::Drawingelement API reference (YARD)
---

# Sketchup::Drawingelement API reference

Drawingelement is a base class for an item in the model that can be displayed. These items include edges, construction points, construction lines, and images. Arc curves and arcs are not included because they are not drawing elements by themselves, but are a composition of edges.

## Methods

- `bounds` — The #bounds method is used to retrieve the Geom::BoundingBox bounding a Sketchup::Drawingelement.
- `casts_shadows=` — The casts_shadows= method is used to set the Drawingelement to cast shadows.
- `casts_shadows?` — The casts_shadows? method is used to determine if the Drawingelement is casting shadows.
- `erase!` — When erasing multiple elements, it's faster to use Entities#erase_entities and erase in bulk than to iterate individual drawing elements calling #erase!.
- `hidden=` — The hidden= method is used to set the hidden status for an element.
- `hidden?` — The hidden? method is used to determine if the element is hidden.
- `layer` — The layer method is used to retrieve the Layer object of the drawing element.
- `layer=` — The layer= method is used to set the layer for the drawing element.
- `material` — The material method is used to retrieve the material for the drawing element.
- `material=` — The material= method is used to set the material for the drawing element.
- `receives_shadows=` — The receive_shadows= method is used to set the Drawingelement to receive shadows.
- `receives_shadows?` — The receive_shadows? method is used to determine if the Drawingelement is receiving shadows.
- `visible=` — The #visible= method is used to set the visible status for an element. This method performs an opposite function to the hidden= method.
- `visible?` — The #visible? method checks if a Drawingelement object is not explicitly hidden (i.e. its hidden property is false). However, this method's return value alone does not guarantee that the element is visible in the model view. Its tag or parent elements can also be hidden. Some element types can also be hidden by rendering options (Styles).
