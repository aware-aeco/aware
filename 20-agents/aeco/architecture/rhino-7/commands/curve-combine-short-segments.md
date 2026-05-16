# curve-combine-short-segments

Lifecycle: single

Looks for segments that are shorter than tolerance that can be combined.             For NURBS of degree greater than 1, spans are combined by removing             knots. Similarly for NURBS segments of polycurves. Otherwise,             RemoveShortSegments() is called. Does not change the domain, but it will             change the relative parameterization.
