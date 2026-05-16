# sketchup-model-edit-transform

Lifecycle: single

Returns the transformation of the current component edit session. If a user has double-clicked to edit a component's geometry, this will return the transformation of that component, relative to its parent's origin. This allows one to correctly calculate “local” transformations of a given entity regardless of whether the user is in edit mode.
