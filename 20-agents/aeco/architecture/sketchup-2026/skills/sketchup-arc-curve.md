---
name: yard-sketchup-arc-curve
description: Sketchup::ArcCurve API reference (YARD)
---

# Sketchup::ArcCurve API reference

An ArcCurve is a Curve that makes up part of a circle. This is the underlying class for circles as well.

## Methods

- `center` — The center method is used to retrieve the Point3d that is at the center of the circular arc.
- `circular?` — Checks if the ArcCurve is a circle.
- `end_angle` — A bug in SketchUp 2017 and older will report the end-angle for some circles as more than 360 degrees. In such case, subtract 2 * PI from the end angle value.
- `normal` — The normal method retrieves a Vector3d that is perpendicular to the plane of the arc.
- `plane` — The plane method is used to retrieve the plane of the arc.
- `radius` — The radius method is used to retrieve the radius of the arc.
- `start_angle` — The start_angle method is used to retrieve the angle of the start of the arc, measured from the X axis in radians.
- `xaxis` — The xaxis method is used to retrieve the X axis of the coordinate system for the curve. Note that the length of the returned vector is equal to the radius of the underlying curve.
- `yaxis` — The yaxis method is used to retrieve the Y axis of the coordinate system for the curve. Note that the length of the returned vector is equal to the radius of the underlying curve.
