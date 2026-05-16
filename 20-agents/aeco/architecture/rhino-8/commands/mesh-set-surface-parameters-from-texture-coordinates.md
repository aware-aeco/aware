# mesh-set-surface-parameters-from-texture-coordinates

Lifecycle: single

If the mesh does not have surface evaluation parameters,             has texture coordinates, and the surface parameters can             be set in a way so the existing texture coordinates can             be computed from the surface parameters, then this function             sets the surface parameters. This is useful when meshes             that have texture coordinates and do not have surface             parameters want to set the surface parameters in a way             so that the texture mapping of type             TextureMappingType.SurfaceParameters             will restore the texture coordinates.
