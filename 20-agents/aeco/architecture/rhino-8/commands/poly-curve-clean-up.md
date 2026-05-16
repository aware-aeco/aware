# poly-curve-clean-up

Lifecycle: single

Removes any nesting of polycurves. If this polycurve has just a single segment, the segment is returned.             If, after nest removal, there are adjacent segments which are polylines, they are combined into a single polyline.             The new curve may have a different domain from this polycurve. If the start and end segments of a closed input are polylines,             the result may have a different seam location since the start and end segments will be combined.
