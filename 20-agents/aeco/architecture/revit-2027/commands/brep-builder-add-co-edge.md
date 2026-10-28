# brep-builder-add-co-edge

Lifecycle: single

Add a co-edge associated to a previously added edge. A co-edge represents the use of an edge on one    of the edge's faces. BrepBuilder allows at most two faces per edge, hence at most two co-edges per edge,    and the co-edges must have opposite bCoEdgeIsReversed flags. The co-edges in a loop must be added in the    order in which they occur in the loop (i.e., in their topological order).
