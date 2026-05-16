# mesh-face-list-cull-degenerate-faces

Lifecycle: single

Attempts to removes degenerate faces from the mesh.             Degenerate faces are faces that contains such a combination of indices,             that their final shape collapsed in a line or point.Before returning, this method also attempts to repair faces by juggling             vertex indices.
