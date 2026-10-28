# sketchup-active-model

Lifecycle: single

The active_model method returns the currently active SketchUp model. On the PC, this is the only model that one can have access to via the API, but Macintosh versions of SketchUp can have multiple models open at once, in which case the method will return the model that the user currently has focused.
