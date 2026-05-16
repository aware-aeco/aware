# rebar-create-from-rebar-shape

Lifecycle: single

Creates a new shape driven Rebar, as an instance of a RebarShape.    The instance will have the default shape parameters from the RebarShape,    and its location is based on the bounding box of the shape in the shape definition.    Hooks and cranks are removed from the shape before computing its bounding box.    If appropriate hooks and cranks can be found in the document, they will be assigned arbitrarily.
