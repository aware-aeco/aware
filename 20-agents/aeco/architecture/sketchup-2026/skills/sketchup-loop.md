---
name: yard-sketchup-loop
description: Sketchup::Loop API reference (YARD)
---

# Sketchup::Loop API reference

Loop is a low level topology class that will not need to be used often. A Loop is a chain of Edges that bound a Face.

## Methods

- `convex?` — Determine if the loop is convex.
- `edges` — Get an array of the edges that define the loop in an ordered sequence.
- `edgeuses` — Get an array of the EdgeUse objects that define this loop in an ordered sequence.
- `face` — Get the Face object that is bounded by this loop.
- `outer?` — Determine if this is an outer loop. Each face has one outer loop, and will have one loop for each hole.
- `vertices` — Get an array of the vertices that define the loop in an ordered sequence.
