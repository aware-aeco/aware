# sketchup-entities-add-group

Lifecycle: single

Calling add_group with entities in its parameters has been known to crash SketchUp before version 8.0. It is preferable to create an empty group and then add things to its Entities collection.
