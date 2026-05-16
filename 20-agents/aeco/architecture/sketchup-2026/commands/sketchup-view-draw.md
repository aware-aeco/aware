# sketchup-view-draw

Lifecycle: single

If you draw outside the model bounds you need to implement Tool#getExtents which returns a bounding box large enough to include the points you draw. Otherwise your drawing will be clipped.
