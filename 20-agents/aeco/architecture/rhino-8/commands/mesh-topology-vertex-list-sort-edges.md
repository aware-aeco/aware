# mesh-topology-vertex-list-sort-edges

Lifecycle: single

Sorts the edge list for the mesh topology vertex list so that             the edges are in radial order when you call ConnectedTopologyVertices.             A non-manifold edge is treated as a boundary edge with respect             to sorting.  If any boundary or non-manifold edges end at the             vertex, then the first edge will be a boundary or non-manifold edge.
