---
name: grasshopper-grasshopper-gui-canvas-tagartists
description: This skill encodes the grasshopper 8.0 surface of the Grasshopper.GUI.Canvas.TagArtists namespace — 3 types with full vendor-documented methods, properties, events, and behavioral remarks. Read when composing apps that touch any of: GH_TagArtist, GH_TagArtist_WirePainter, IGH_TagArtist.
---

# Grasshopper.GUI.Canvas.TagArtists

Auto-generated from vendor docs for grasshopper 8.0. 3 types in this namespace.

## GH_TagArtist (class)

Provides a basic implementation of the IGH_TagArtist interface.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_TagArtists_GH_TagArtist.htm)

### Constructors
- `protected GH_TagArtist(Guid id)` — Initializes a new instance of the GH_TagArtist class

### Methods
#### `public abstract void Paint(GH_Canvas canvas, GH_CanvasChannel channel)`

This method is called whenever a Canvas channel is painted

**Parameters:**
- `canvas` (Grasshopper.GUI.Canvas.GH_Canvas) — Canvas that is currently painting.
- `channel` (Grasshopper.GUI.Canvas.GH_CanvasChannel) — Channel that is currently being painted.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_TagArtists_GH_TagArtist_Paint.htm)

### Properties
- `ID` (Guid, get) — Gets the ID of this TagArtist object.

## GH_TagArtist_WirePainter (class)

IGH_TagArtist implementation for drawing individual wires.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_TagArtists_GH_TagArtist_WirePainter.htm)

### Constructors
- `public GH_TagArtist_WirePainter(IGH_Param source, IGH_Param target, Color colour, int width)` — Initializes a new instance of the GH_TagArtist_WirePainter class

### Methods
#### `public override void Paint(GH_Canvas canvas, GH_CanvasChannel channel)`

(Overrides GH_TagArtist.Paint(GH_Canvas, GH_CanvasChannel).)

**Parameters:**
- `canvas` (Grasshopper.GUI.Canvas.GH_Canvas)
- `channel` (Grasshopper.GUI.Canvas.GH_CanvasChannel)

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_TagArtists_GH_TagArtist_WirePainter_Paint.htm)

### Properties
- `ID` (Guid, get) — Gets the ID of this TagArtist object.
- `WirePainter_ID` (Guid, get) — 

## IGH_TagArtist (interface)

Defines the interface for TagArtist objects. TagArtists can be used to paint additional stuff onto a GH_Canvas.

[Vendor docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/T_Grasshopper_GUI_Canvas_TagArtists_IGH_TagArtist.htm)

### Methods
#### `void Paint(GH_Canvas canvas, GH_CanvasChannel channel)`

This method is called whenever a Canvas channel is painted

**Parameters:**
- `canvas` (Grasshopper.GUI.Canvas.GH_Canvas) — Canvas that is currently painting.
- `channel` (Grasshopper.GUI.Canvas.GH_CanvasChannel) — Channel that is currently being painted.

[Docs](https://raw.githubusercontent.com/mcneel/grasshopper-api-docs/gh-pages/api/grasshopper/html/M_Grasshopper_GUI_Canvas_TagArtists_IGH_TagArtist_Paint.htm)

### Properties
- `ID` (Guid, get) — Gets the ID of this Tag Artist object.

