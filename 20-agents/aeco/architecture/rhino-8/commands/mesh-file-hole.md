# mesh-file-hole

Lifecycle: single

Given a starting "naked" edge index, this function attempts to determine a "hole"             by chaining additional naked edges together until if returns to the start index.             Then it triangulates the closed polygon and either adds the faces to the mesh.
