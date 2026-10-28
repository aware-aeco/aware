# rebar-split-rebar

Lifecycle: single

Splits a rebar into multiple sets at specified bar indices.    Given split indices (e.g., {ii, jj}) in barIndices, the function creates three rebar sets: from 0 to ii, ii+1 to jj, and jj+1 to NumberOfBarPositions - 1.    Edge Case: A split at index 0 creates a first set with one bar; a split at the last index does nothing, as no bars follow.    If a split is made at an index where the bar is not included, the split will still occur, and the unincluded bar will remain unchanged in the new set created after the split.    This method applies to a Shape Driven Rebar whose definition is RebarShapeDefinitionBySegments, for 3D Path Free Form Rebar and for Morphed Free Form Rebar.    For Morphed Free Form Rebar that targets Spiral or Lapped Circle shapes, split cannot be made.
