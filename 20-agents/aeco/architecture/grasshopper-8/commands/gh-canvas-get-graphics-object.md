# gh-canvas-get-graphics-object

Lifecycle: single

Gets a graphics object for this control. You are not allowed to draw with this object,   use it only for visibility testing and such. If you're inside a canvas update, use the Graphics() property instead.   You must dispose of the Graphics object returned by this method or resources will be leaked.
