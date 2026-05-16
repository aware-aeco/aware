# curve-make-closed

Lifecycle: single

If IsClosed, just return true. Otherwise, decide if curve can be closed as              follows: Linear curves polylinear curves with 2 segments, NURBS with 3 or less              control points cannot be made closed. Also, if tolerance > 0 and the gap between              start and end is larger than tolerance, curve cannot be made closed.              Adjust the curve's endpoint to match its start point.
