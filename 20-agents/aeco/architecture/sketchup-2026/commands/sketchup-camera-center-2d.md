# sketchup-camera-center-2d

Lifecycle: single

The #center_2d method returns a point with the x and y offset of the camera when it's in two-point perspective or math photo mode. When the camera is in two-point perspective and the user pans around, the x and y values will change. These values are in normalized device coordinates, so for instance, the range [-1.0, 1.0] spans the full width or height of the screen.
