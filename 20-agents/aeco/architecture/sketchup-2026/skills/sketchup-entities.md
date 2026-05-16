---
name: yard-sketchup-entities
description: Sketchup::Entities API reference (YARD)
---

# Sketchup::Entities API reference

The Entities class is a collection of Entity objects, either in a ComponentDefinition or directly in the Model. A Entities object corresponds to a drawing context in the GUI.

## Methods

- `[]` — The #[] method is used to retrieve an entity by its index in an array of entities. The index is a number between 0 and entities.length - 1. In general, it is preferable to use the #each method to iterate though all of the entities in the collection as it will be much more efficient.
- `active_section_plane` — The active_section_plane method is used to access the currently active section plane in the Entities object.
- `active_section_plane=` — The active_section_plane= method is used to set the active section plane in the Entities object.
- `add_3d_text` — The #add_3d_text method is used to create 3D text. It will be added as edges and faces drawn at the origin.
- `add_arc` — The add_arc method is used to create an arc curve segment.
- `add_circle` — The add_circle method is used to create a circle.
- `add_cline` — The #add_cline method is used to create a construction line. This can be finite or infinite.
- `add_cpoint` — The add_cpoint method is used to create a construction point.
- `add_curve` — The add_curve method is used to create a curve from a collection of edges.
- `add_dimension_linear` — The #add_dimension_linear method adds a linear dimension to the entities.
- `add_dimension_radial` — The add_dimension_radial method adds a radial dimension (i.e arc/circle radius/diameter dimension) to the entities.
- `add_edges` — If the points form a closed loop, the first and last vertex will not merge. If you intend to create a face from the edges, use #add_face directly.
- `add_face` — A special case exists for any face created on the ground plane, in which case the vertex order is ignored and the face is always facing down.
- `add_faces_from_mesh` — The #add_faces_from_mesh method is used to add Face entities to the collection of entities from a Geom::PolygonMesh.
- `add_group` — Calling add_group with entities in its parameters has been known to crash SketchUp before version 8.0. It is preferable to create an empty group and then add things to its Entities collection.
- `add_image` — The add_image method is used to add an image to the collection of entities.
- `add_instance` — The #add_instance method adds a group or component instance to the collection of entities using an existent definition.
- `add_line` — The add_line method is used to add an edge to the collection of entities. This is not to be confused with the concept of a “line” from a geometric sense, which is an invisible object represented by an Array of a point and a vector. (See the Array class for more information on geometric lines in SketchUp.)
- `add_ngon` — The add_ngon method is used to create a multi-sided polygon.
- `add_observer` — The add_observer method is used to add an observer to the current object.
- `add_section_plane` — Adds a section plane object to the entities.
- `add_snap` — The #add_snap method is used to create a new Snap.
- `add_text` — The #add_text method adds a note or label text entity to the entities.
- `at` — The #at method is an alias for #[].
- `build` — While using #build it is important to not add or remove vertices by other means than the builder. Also don't modify the position of the vertices in the Sketchup::Entities collection. Doing so can break the vertex-cache that de-duplicates the vertices.
- `clear!` — The clear! method is used to remove all entities from the collection of entities.
- `count` — Since SketchUp 2014 the count method is inherited from Ruby's Enumerable mix-in module. Prior to that the #count method is an alias for #length.
- `each` — Don't remove content from this collection while iterating over it with #each. This would change the size of the collection and cause elements to be skipped as the indices change. Instead copy the current collection to an array using to_a and then use each on the array, when removing content.
- `erase_entities` — It's faster to use this method and erase in bulk than to iterate individual drawing elements calling Drawingelement#erase!.
- `fill_from_mesh` — The #fill_from_mesh method is used to add faces and edges to the collection of entities from a Geom::PolygonMesh. It requires that the entities collection to be filled is empty. It has higher performance than #add_faces_from_mesh, but does less error checking as it builds the geometry.
- `intersect_with` — The #intersect_with method is used to intersect a Sketchup::Entities, Sketchup::Component, or Sketchup::Group object with a entities object.
- `length` — The #length method is used to retrieve the number of entities in the collection of entities.
- `model` — The model method is used to retrieve the model that contains the collection of entities.
- `parent` — The parent method is used to retrieve the parent or object that contains the collection of entities. A parent can be either a Model or ComponentDefinition object.
- `remove_observer` — The remove_observer method is used to remove an observer from the current object.
- `size` — The #size method is an alias for the #length method.
- `transform_by_vectors` — The transform_by_vectors method is used to apply several vectors to several sub-entities all at once.
- `transform_entities` — The transform_entities method is used to apply a transform to several sub-entities all at once.
- `weld` — The #weld method takes a set of edges and find all possible chains of edges and connect them with a Curve.
