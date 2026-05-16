---
name: yard-sketchup-edge
description: Sketchup::Edge API reference (YARD)
---

# Sketchup::Edge API reference

The Edge class contains methods modifying and extracting information for edges.

## Methods

- `all_connected` — The all_connected method retrieves all of the entities connected to an edge, including the edge itself.
- `common_face` — The common_face method is used to identify a face that is common to two edges.
- `curve` — The curve method is used to get the Curve object that this edge belongs to, if any. Note that if the edge is part of an arc instead of a random curve, then this method will return an ArcCurve object.
- `end` — The end method is used to retrieve the Vertex object at the end of the edge.
- `explode_curve` — The explode_curve method is used to explode the curve that the given edge is a part of.
- `faces` — The #faces method is used to retrieve all of the faces common to the edge.
- `find_faces` — The find_faces method is used to create all of the Faces that can be created with this edge. For example, if you use the API to draw three edges that form a triangle, the face between them will not show up because you've only drawn the edges, but if you call find_faces on one of the edges, the triangle will be filled in.
- `length` — The #length method is used to retrieve the length of an edge in current units.
- `line` — The line method is used to retrieve the line defined by the edge. Lines in SketchUp aren't visible entities but geometric constructs represented by an Array with a Point3d and a Vector3d. See the Geom module and the Array class for more information on lines.
- `other_vertex` — The other_vertex method is used to find the opposite vertex given one vertex of the edge.
- `reversed_in?` — The #reversed_in? method is used to determine if the edge is reversed in a face's bounding loop.
- `smooth=` — The soft and smooth properties are normally set in pairs. You can observer this when the Soften/Smooth Edges feature or holding down Ctrl when using the Eraser Tool.
- `smooth?` — The #smooth? method is used to retrieve the current smooth setting for an edge.
- `soft=` — The soft and smooth properties are normally set in pairs. You can observe this when the Soften/Smooth Edges feature or holding down Ctrl when using the Eraser Tool.
- `soft?` — The #soft? method is used to retrieve the current soft setting for an edge.
- `split` — The #split method is used to split an edge into two or more distinct edges. If a Point3d is given, it must be a point that is on the Edge.
- `start` — The start method is used to retrieve the Vertex object at the start of the edge.
- `used_by?` — The used_by? method is used to see if an edge is used by a given Face or Vertex.
- `vertices` — The vertices method is used to retrieve the vertices on the edge.
