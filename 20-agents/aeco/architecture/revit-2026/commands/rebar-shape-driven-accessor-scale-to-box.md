# rebar-shape-driven-accessor-scale-to-box

Lifecycle: single

Move and resize the bar to fit within a specified box.    The arguments are interpreted as an arbitrary    rectangle in 3D with vertices: origin, origin+xVec,    origin+xVec+yVec, origin+yVec. The algorithm then    proceeds as follows. First the bar is given the    default values of the shape parameters from the shape    definition. Then, if it is possible to do so without    violating the shape definition, the parameter values    are scaled so that the width and height of the shape    (including bar thickness) match the lengths of xVec and yVec.    If there is no way to do this within the shape definition    due to overconstraining, a compromise is attempted, such as    scaling the whole shape until either the width or the    height is correct. Finally the shape is rotated to    match the coordinate system of the box. The algorithm    is the same one used in one-click placement.
