# sketchup-app-observer-abstract-expects-startup-model-notifications

Lifecycle: single

Prior to SketchUp 2014, #onNewModel and #onOpenModel were not being called for the startup models. This issue is now fixed but observers still need to express their intent to receive these calls. This is for back-compatibility with existing scripts which worked around these missing calls by other means. For new code, this method should be implemented and should return true.
