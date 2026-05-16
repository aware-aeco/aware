# sketchup-model-close

Lifecycle: single

As of SketchUp 2024.0 this method will ensure the next model window gets focus if there is one. Before that `Sketchup.active_model` might return `nil` after calling this method even though more models where open.
