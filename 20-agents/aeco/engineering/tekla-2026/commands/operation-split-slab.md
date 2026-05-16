# operation-split-slab

Lifecycle: single

This command is meant for specifically splitting a concrete slab with             advanced solid operations to create more robust and user friendly results             than the command:             public static ContourPlate Split(ContourPlate Object, Polygon SplitPolygon).             No validation is done for the type, it is the caller's responsibility to call             this only for valid types (slabs). Behavior for non-slabs is undetermined.
