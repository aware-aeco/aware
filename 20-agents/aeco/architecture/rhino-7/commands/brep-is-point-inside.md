# brep-is-point-inside

Lifecycle: single

Determines if point is inside a Brep.  This question only makes sense when             the brep is a closed and manifold.  This function does not check for             closed or manifold, so result is not valid in those cases.  Intersects             a line through point with brep, finds the intersection point Q closest             to point, and looks at face normal at Q.  If the point Q is on an edge             or the intersection is not transverse at Q, then another line is used.
