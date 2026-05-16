# polyline-collapse-short-segments

Lifecycle: single

Collapses all segments until none are shorter than tolerance.              This function is significantly slower than DeleteShortSegments,              since it recursively operates on the shortest segment.              When a segment is collapsed the end-points are placed in the center of the segment.
