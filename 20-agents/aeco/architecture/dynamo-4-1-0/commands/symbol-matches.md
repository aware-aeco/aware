# symbol-matches

Lifecycle: single

Checks if all of the namespace prefixes specified in the given              partially qualified name appear in this namespace in the same order.             For Example:             A full namespace "Com.Autodesk.Designscript.ProtoGeometry.Point"              will match all of the following partial namespaces             Com.Autodesk.Designscript.ProtoGeometry.Point             Point             DesignScript.Point             ProtoGeometry.Point             Autodesk.DesignScript.Point             whereas it won't match Com.DesignScript.Autodesk.Point
