# curve-boolean-regions-region-point-index

Lifecycle: single

If this object were created using the Curve.CreateBooleanRegions override that             accepts a collection of points as input, then you this method to retrieve the             index of the point contained in a curve region.             If this.RegionPointIndex(i) = n, then points[i] is contained in this.RegionCurves(n).             If points[i] is not in any region, then this.RegionPointIndex(i) = -1.
