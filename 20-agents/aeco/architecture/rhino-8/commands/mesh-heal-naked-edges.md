# mesh-heal-naked-edges

Lifecycle: single

Attempts to "heal" naked edges in a mesh based on a given distance.               First attempts to move vertices to neighboring vertices that are within that             distance away. Then it finds edges that have a closest point to the vertex within             the distance and splits the edge. When it finds one it splits the edge and             makes two new edges using that point.
