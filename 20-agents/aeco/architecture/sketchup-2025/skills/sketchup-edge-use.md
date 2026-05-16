---
name: yard-sketchup-edge-use
description: Sketchup::EdgeUse API reference (YARD)
---

# Sketchup::EdgeUse API reference

The EdgeUse class defines how an Edge is used in the definition of a Face.

## Methods

- `edge` — The edge method is used to retrieve the edge for the edge use.
- `end_vertex_normal` — The end_vertex_normal method is used to retrieve the vertex normal for the end point of this edgeuse.
- `face` — The face method is used to retrieve the face used by this edge use.
- `loop` — The loop method is used to retrieve the loop for this edge use.
- `next` — The next method is used to retrieve the next edge use in a loop.
- `partners` — The partners method is used to retrieve all of the partner edge uses that uses the same edge.
- `previous` — The previous method is used to retrieve the previous edge use in a loop.
- `reversed?` — The reversed? method is used to determine if the edge direction is opposite of the edge use direction. The edge use direction is the same as the loop it belongs to.
- `start_vertex_normal` — The start_vertex_normal method is used to retrieve the vertex normal for the start point of this edgeuse.
