# geom-bounding-box-intersect

Lifecycle: single

Prior to SU2015 this method would return incorrect result in some cases. For correct result in these versions you must first check if the boundingboxes actually overlap - then call this to get the resulting boundingbox.
