# sketchup-view-draw-points

Lifecycle: single

Prior to SketchUp 2025.0 this method accepted the size as physical pixels. As of SketchUp 2025.0 the points are expected to be in logical pixels. Older versions need to apply the scaling factor from UI.scale_factor to the size before passing them to this method.
