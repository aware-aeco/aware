# gh-viewport-limit-unit

Lifecycle: single

Utility function for calculating pixel dimensions in a zoom-aware environment.   The desired value is put through the zoom projection and if the resulting   size (as displayed on the screen) exceeds the visual limits it is clipped.   This function can be used for example to make sure that a certain penwidth   never exceeds visual limits (i.e. it doesn't get too thin or too thick on the screen).
