# sketchup-definition-observer-abstract-on-component-instance-removed

Lifecycle: single

Due to the underlying way that the SketchUp Move Tool is implemented, this method is fired on a Move + Copy operation even though no ComponentInstance is apparently removed.
