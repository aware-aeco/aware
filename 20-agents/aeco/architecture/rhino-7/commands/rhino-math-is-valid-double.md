# rhino-math-is-valid-double

Lifecycle: single

Determines whether a  value is valid within the RhinoCommon context.             Rhino does not use Double.NaN by convention, so this test evaluates to true if:x is not equal to RhinoMath.UnsetValueSystem.Double.IsNaN(x) evaluates to falseSystem.Double.IsInfinity(x) evaluates to false
