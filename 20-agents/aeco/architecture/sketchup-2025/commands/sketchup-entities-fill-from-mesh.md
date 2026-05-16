# sketchup-entities-fill-from-mesh

Lifecycle: single

The #fill_from_mesh method is used to add faces and edges to the collection of entities from a Geom::PolygonMesh. It requires that the entities collection to be filled is empty. It has higher performance than #add_faces_from_mesh, but does less error checking as it builds the geometry.
