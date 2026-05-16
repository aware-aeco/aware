# viewport-info-get-point-depth

Lifecycle: single

Gets the clipping distance of a point. This function ignores the             current value of the viewport's near and far settings. If             the viewport is a perspective projection, then it intersects             the semi infinite frustum volume with the bounding box and             returns the near and far distances of the intersection.             If the viewport is a parallel projection, it intersects the             infinite view region with the bounding box and returns the             near and far distances of the projection.
