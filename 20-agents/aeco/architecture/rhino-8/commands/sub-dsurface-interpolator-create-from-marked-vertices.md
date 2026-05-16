# sub-dsurface-interpolator-create-from-marked-vertices

Lifecycle: single

Create an interpolator where all the marked vertices (unmarked if             interpolatedVerticesMark is false) in the SubD are free vertices in the linear             system used for interpolation, and the unmarked (marked if interpolatedVerticesMark             is false) are fixed to their initial positions. Free vertices are can move as a             result of the interpolation, and can receive an interpolation target location.
