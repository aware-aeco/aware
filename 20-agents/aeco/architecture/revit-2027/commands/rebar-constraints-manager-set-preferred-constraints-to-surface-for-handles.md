# rebar-constraints-manager-set-preferred-constraints-to-surface-for-handles

Lifecycle: single

For ShapeDriven Rebar it will set a preferred 'ToSurface' RebarConstraint for each input handle.    The surface that will be used by the constraint is the current surface that is used to compute the position of the handle.This function applies only for shape driven Rebar, and will throw exception for free form rebar.
