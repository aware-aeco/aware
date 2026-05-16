# iexport-context-base-on-line-segment

Lifecycle: single

This method is called after unhandled curve was tessellated to line segments and sent to the output.    Note for 2D export: if the export is performed for the view in non-Wireframe display style, then    this method is called outside of view, instance and link begin/end calls but still between OnElementBegin2D/OnElementEnd2D callsthis method is never called for annotation elements, i.e. their geometry should be processed in methods OnCurve and OnPolyline
