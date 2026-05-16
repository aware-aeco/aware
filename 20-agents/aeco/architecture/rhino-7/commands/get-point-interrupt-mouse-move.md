# get-point-interrupt-mouse-move

Lifecycle: single

If you have lengthy computations in OnMouseMove() and/or DymanicDraw()             overrides, then periodically call InterruptMouseMove() to see if you             should interrupt your work because the mouse has moved again.
