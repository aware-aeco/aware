# sketchup-edge-find-faces

Lifecycle: single

The find_faces method is used to create all of the Faces that can be created with this edge. For example, if you use the API to draw three edges that form a triangle, the face between them will not show up because you've only drawn the edges, but if you call find_faces on one of the edges, the triangle will be filled in.
