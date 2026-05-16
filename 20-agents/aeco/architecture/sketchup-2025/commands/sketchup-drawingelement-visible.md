# sketchup-drawingelement-visible

Lifecycle: single

The #visible? method checks if a Drawingelement object is not explicitly hidden (i.e. its hidden property is false). However, this method's return value alone does not guarantee that the element is visible in the model view. Its tag or parent elements can also be hidden. Some element types can also be hidden by rendering options (Styles).
