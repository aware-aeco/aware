# sketchup-instance-observer-abstract-on-open

Lifecycle: single

The #onOpen method is called when an instance is “opened,” meaning an end user has double clicked on it to edit its geometry. This is particularly useful if your plugin is dynamically drawing geometry or performing transformations in global space, since when in “edit component” mode all transformations and positions are returned in relation to the current component's origin.
