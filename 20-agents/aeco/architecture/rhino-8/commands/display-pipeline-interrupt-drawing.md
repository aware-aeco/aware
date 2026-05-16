# display-pipeline-interrupt-drawing

Lifecycle: single

Tests to see if the pipeline should stop drawing more geometry and just show what it has so far.              If a drawing operation is taking a long time, this function will return true and tell Rhino it should just              finish up and show the frame buffer. This is used in dynamic drawing operations.
