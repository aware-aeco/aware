# sketchup-model-start-operation

Lifecycle: single

Operations in SketchUp are sequential and cannot be nested. If you start a new Ruby operation while another is still open, you will implicitly close the first one.
