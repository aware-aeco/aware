# mesh-unify-normals

Lifecycle: single

Attempts to fix inconsistencies in the directions of mesh faces in a mesh. This function             does not modify mesh vertex normals, it rearranges the mesh face winding and face             normals to make them all consistent. Note, you may want to call Mesh.Normals.ComputeNormals()             to recompute vertex normals after calling this functions.
