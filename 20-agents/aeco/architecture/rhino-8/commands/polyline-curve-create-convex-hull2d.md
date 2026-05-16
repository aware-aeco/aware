# polyline-curve-create-convex-hull2d

Lifecycle: single

Attempts to create a closed PolylineCurve that is the anti-clockwise planar convex hull of the input points.             In addition, the indices of the extremal points among the input points are returned in correct order.             Possible duplicates among the input points are taken care of.             The input pointsThe indices into the input points such that points[hullIndices[i]] = result[i]. Since the              result is a closed polyline if successful, the start/end index is repeated at the beginning and end of the hullIndices.                          The closed PolylineCurve encompassing the input points, or null if the input points were either too few, or were found to be collinear.
