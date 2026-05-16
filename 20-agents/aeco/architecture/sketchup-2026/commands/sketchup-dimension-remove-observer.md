# sketchup-dimension-remove-observer

Lifecycle: single

The remove_observer method is used to remove a DimensionObserver from the dimension. Note that, if the given observer responds to 'onTextChanged', it will be removed as a DimensionObserver. If not, the base Entity.remove_observer will be called.
