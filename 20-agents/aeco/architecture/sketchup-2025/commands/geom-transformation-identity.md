# geom-transformation-identity

Lifecycle: single

As of SketchUp 2018, this now looks at the data to determine if the transformation is identity. Prior to SU2018, this only looks at the flag to see if the transform has not been modified. If the transform has been changed, this will return false even if it is really the identity.
