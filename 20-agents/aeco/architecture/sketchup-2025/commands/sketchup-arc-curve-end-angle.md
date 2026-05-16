# sketchup-arc-curve-end-angle

Lifecycle: single

A bug in SketchUp 2017 and older will report the end-angle for some circles as more than 360 degrees. In such case, subtract 2 * PI from the end angle value.
