# rebar-container-item-set-from-rebar-shape

Lifecycle: single

Set an instance of a RebarContainerItem element, as an instance of a RebarShape.    The instance will have the default shape parameters from the RebarShape,    and its location is based on the bounding box of the shape in the shape definition.    Hooks are removed from the shape before computing its bounding box.    If appropriate hooks can be found in the document, they will be assigned arbitrarily.
