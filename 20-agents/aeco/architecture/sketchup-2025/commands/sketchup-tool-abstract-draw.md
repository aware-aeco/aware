# sketchup-tool-abstract-draw

Lifecycle: single

If you draw outside the model bounds you need to implement #getExtents which return a bounding box large enough to include the points you draw. Otherwise your drawing will be clipped.
