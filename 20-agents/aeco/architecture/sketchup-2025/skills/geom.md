---
name: yard-geom
description: Geom API reference (YARD)
---

# Geom API reference

Lines and Planes are infinite.

## Methods

- `closest_points` — The closest_points method is used to compute the closest points on two lines.
- `fit_plane_to_points` — The fit_plane_to_points method is used to compute a plane that is a best fit to an array of points. If more than three points are given some of the points may not be on the plane.
- `intersect_line_line` — The intersect_line_line computes the intersection of two lines.
- `intersect_line_plane` — The intersect_line_plane method is used to compute the intersection of a line and a plane.
- `intersect_plane_plane` — The intersect_plane_plane method is used to compute the intersection of two planes.
- `linear_combination` — The linear_combination method is used to compute the linear combination of points or vectors.
- `point_in_polygon_2D` — The point_in_polygon_2D method is used to determine whether a point is inside a polygon. The z component of both the point you're checking and the points in the polygon are ignored, effectively making it a 2-d check.
- `tesselate` — The winding order of the polygons matter. The outer loop should be in counter-clockwise order. To cut an empty hole the inner loops should be in clockwise order, otherwise they will create a loop filled with triangles.
