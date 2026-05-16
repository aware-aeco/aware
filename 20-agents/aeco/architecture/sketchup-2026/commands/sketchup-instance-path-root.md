# sketchup-instance-path-root

Lifecycle: single

The root of an instance path is the element located closest to the model root. This will be a group or component instance. If you have a non-instance as a leaf with no other parent component this will return `nil`.
