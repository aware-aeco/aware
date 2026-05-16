# rectangle3d-create-from-polyline

Lifecycle: single

Attempts to create a rectangle from a polyline. This method only works well for             polylines that already closely resemble rectangles. If the polyline contains             more than four vertices, the least significant ones will be ignored. If the             polylines is non-orthogonal, the discrepancies will be averaged away.             This method should not be used as a Rectangle fitter.
