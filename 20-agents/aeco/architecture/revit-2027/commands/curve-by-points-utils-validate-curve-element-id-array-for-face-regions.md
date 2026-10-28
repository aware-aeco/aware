# curve-by-points-utils-validate-curve-element-id-array-for-face-regions

Lifecycle: single

Validates that the input CurveElements can define FaceRegions.    The CurveElements must be CurveByPoints.  Each curve must be entirely hosted by a single Face or hosts related to a common    Face (for example, Edges of a common Face, other CurveElements hosted by a common Face). To be added to the FaceRegion definition,    a CurveElement must have the SketchOnSurface attribute set.
