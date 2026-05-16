# sketchup-view-load-texture

Lifecycle: single

Avoid loading and releasing textures within the Tool#draw event as that is not efficient.
