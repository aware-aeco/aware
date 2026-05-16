# sketchup-component-instance-transformation

Lifecycle: single

As of SketchUp 2026, this will raise an error if the Geom::Transformation is not invertible. Prior to 2026 this would silently set the transformation possibly causing rendering or editing problems.
