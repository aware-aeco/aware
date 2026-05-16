# wire-set-vertex

Lifecycle: single

Sets the position of a given vertex.    If the vertex is start or end point, and the wire connects to electrical device, the wire end offset will be set according to the given vertex.    If the vertex is start or end point, and the wire connects to other wire, user can't set the vertex and exception will be thrown.    If the vertex is start or end point, and the wire connects to nothing, the vertex will be set as the given vertex.
