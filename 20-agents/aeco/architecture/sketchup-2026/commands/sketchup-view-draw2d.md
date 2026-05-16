# sketchup-view-draw2d

Lifecycle: single

Prior to SketchUp 2025.0 this method accepted the points as physical screen coordinates. As of SketchUp 2025.0 the points are expected to be in logical screen coordinates. Older versions need to apply the scaling factor from UI.scale_factor to the points before passing them to this method.
