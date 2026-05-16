# geom-polygon-mesh-initialize

Lifecycle: single

When creating a mesh with normals and/or UVQ data it's critical that the number of points estimated is equal to or higher than the final number of points added. If fewer points are estimated the normals and UVQ data might end up out of sync.
