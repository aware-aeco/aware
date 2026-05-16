# curve-create-tween-curves-with-sampling

Lifecycle: single

Creates curves between two open or closed input curves. Use sample points method to make curves compatible.             This is how the algorithm works: Divides the two curves into an equal number of points, finds the midpoint between the              corresponding points on the curves and interpolates the tween curve through those points. There is no matching of curves             direction. Caller must match input curves direction before calling the function.
