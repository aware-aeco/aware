# sketchup-model-latlong-to-point

Lifecycle: single

The latlong_to_point method converts a latitude and longitude to a Point3d object in the model. It does not actually work with a LatLong object, but operates on a 2-element array. The returned point will always be on the ground (z=0).
