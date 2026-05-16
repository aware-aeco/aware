# brep-trim

Lifecycle: single

Trims a brep with an oriented cutter. The parts of the brep that lie inside             (opposite the normal) of the cutter are retained while the parts to the             outside (in the direction of the normal) are discarded.  If the Cutter is             closed, then a connected component of the Brep that does not intersect the             cutter is kept if and only if it is contained in the inside of cutter.             That is the region bounded by cutter opposite from the normal of cutter,             If cutter is not closed all these components are kept.
