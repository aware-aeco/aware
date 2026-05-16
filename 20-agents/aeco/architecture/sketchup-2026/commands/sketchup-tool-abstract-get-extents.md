# sketchup-tool-abstract-get-extents

Lifecycle: single

In order to accurately draw things, SketchUp needs to know the extents of what it is drawing. If the tool is doing its own drawing, it may need to implement this method to tell SketchUp the extents of what it will be drawing. If you don't implement this method, you may find that part of what the tool is drawing gets clipped to the extents of the rest of the model.
