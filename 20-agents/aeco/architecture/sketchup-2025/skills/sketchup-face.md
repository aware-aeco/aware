---
name: yard-sketchup-face
description: Sketchup::Face API reference (YARD)
---

# Sketchup::Face API reference

Faces in SketchUp are flat, 2-sided polygons with 3 or more sides.

## Methods

- `all_connected` — The all_connected method retrieves all of the entities connected to a face.
- `area` — The area method is used to retrieve the area of a face.
- `back_material` — The back_material method is used to retrieve the material assigned to the back side of the face.
- `back_material=` — The back_material= method is used to set the material assigned to the back side of the face.
- `classify_point` — The classify_point method is used to determine if a given Point3d is on the referenced Face.
- `clear_texture_position` — The #clear_texture_position method is used to remove any explicit texture positioning for a face and have SketchUp display it with the default texture positioning.
- `clear_texture_projection` — The #clear_texture_projection method is used to clear the texture projection. This is similar to toggling off Projection from the Position Texture tool in the UI.
- `coplanar_with?` — The #coplanar_with? method is used determine whether a face is coplanar with `other_face`.
- `edges` — The edges method is used to get an array of edges that bound the face.
- `followme` — The #followme method is used to create a shape by making the face follow along an array of edges.
- `get_glued_instances` — The get_glued_instances method returns an Array any ComponentInstances that are glued to the face.
- `get_texture_projection` — The #get_texture_projection method will return a vector representing the projection for either the front or back side of the face.
- `get_UVHelper` — The get_UVHelper object is used to retrieve a UVHelper object for use in texture manipulation on a face.
- `loops` — The loops method is used to get an array of all of the loops that bound the face.
- `material` — The material method is used to retrieve the material assigned to the front of the face. (This method is inherited from the Drawingelement parent class.)
- `material=` — The material= method is used to set the material assigned to the front side of the face. (This method is inherited from the Drawingelement parent class.)
- `mesh` — The mesh method creates a polygon mesh that represents the face. See the Geom::PolygonMesh class for more information.
- `normal` — The normal method is used to retrieve the 3D vector normal to the face in the front direction.
- `outer_loop` — This method is used to retrieve the outer loop that bounds the face.
- `plane` — The plane method is used to retrieve the plane of the face. See the Array class for information on how planes are stored.
- `position_material` — The #position_material method is used to position a material on a face.
- `pushpull` — The pushpull method is used to perform a push/pull on a face.
- `reverse!` — The reverse! method is used to reverse the face's orientation, meaning the front becomes the back.
- `set_texture_projection` — This function never worked correctly. It's not capable of controlling the position and orientation of the texture. In some cases it produced an invalid model. As of SketchUp 2021.1 the method simply raises NotImplementedError.
- `texture_positioned?` — The #texture_positioned? method is used to check if the face has a texture that is positioned.
- `texture_projected?` — The #texture_projected? method is used to check if the face has a texture that is projected.
- `uv_tile_at` — The #uv_tile_at method is used to get the corner positions (model and UV) of a UV tile.
- `vertices` — The vertices method is used to get an array of all of the vertices that bound the face.
