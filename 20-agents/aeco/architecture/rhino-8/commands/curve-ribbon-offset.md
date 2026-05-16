# curve-ribbon-offset

Lifecycle: single

Offsets a closed curve in the following way: pProject the curve to a plane with given normal.             Then, loose Offset the projection by distance + blend_radius and trim off self-intersection.             THen, Offset the remaining curve back in the opposite direction by blend_radius, filling gaps with blends.             Finally, use the elevations of the input curve to get the correct elevations of the result.
