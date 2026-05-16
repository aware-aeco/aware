# rhino-viewport-get-pick-transform

Lifecycle: single

Takes a rectangle in screen coordinates and returns a transformation             that maps the 3d frustum defined by the rectangle to a -1/+1 clipping             coordinate box. This takes a single point and inflates it by             Rhino.ApplicationSettings.ModelAidSettings.MousePickBoxRadius to define             the screen rectangle.
