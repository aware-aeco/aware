# curve-create-arc-line-arc-blend

Lifecycle: single

Creates an arc-line-arc blend curve between two curves.             The output is generally a PolyCurve with three segments: arc, line, arc.             In some cases, one or more of those segments will be absent because they would have 0 length.              If there is only a single segment, the result will either be an ArcCurve or a LineCurve.
