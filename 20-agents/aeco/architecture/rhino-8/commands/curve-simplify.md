# curve-simplify

Lifecycle: single

Returns a geometrically equivalent PolyCurve.             The PolyCurve has the following properties             1. All the PolyCurve segments are LineCurve, PolylineCurve, ArcCurve, or NurbsCurve.                          2. The NURBS Curves segments do not have fully multiple interior knots.                          3. Rational NURBS curves do not have constant weights.                          4. Any segment for which IsLinear() or IsArc() is true is a Line,                 Polyline segment, or an Arc.                          5. Adjacent co-linear or co-circular segments are combined.                          6. Segments that meet with G1-continuity have there ends tuned up so                that they meet with G1-continuity to within machine precision.
