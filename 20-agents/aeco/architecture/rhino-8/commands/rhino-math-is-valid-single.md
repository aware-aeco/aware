# rhino-math-is-valid-single

Lifecycle: single

Determines whether a  value is valid within the RhinoCommon context.             Rhino does not use Single.NaN by convention, so this test evaluates to true if:x is not equal to RhinoMath.UnsetValue,System.Single.IsNaN(x) evaluates to falseSystem.Single.IsInfinity(x) evaluates to false
