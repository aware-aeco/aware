# layout-document-render-mode-override

Lifecycle: single

The #render_mode_override= method sets the override setting for output render modes of SketchUpModels in the Layout::Document. Setting this to NO_OVERRIDE will prevent overriding the individual SketchUpModel render mode setting during export. This override will only affect raster rendered SketchUpModels, if a viewport is set to vector or hybrid, it will retain that render mode as its output render mode.
