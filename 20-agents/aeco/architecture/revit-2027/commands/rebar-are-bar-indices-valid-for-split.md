# rebar-are-bar-indices-valid-for-split

Lifecycle: single

Verifies that all bar indices in the input set are within the valid range of [0, NumberOfBarPositions - 2].For a 3D Path or Morphed Free Form Rebar, single layout Rebar sets cannot result from a split, so the bar indices in the input set must not be consecutive.For a 3D Path Free Form Rebar, this also validates that each new set created by the split will have at least one bar plane intersecting a path edge.
