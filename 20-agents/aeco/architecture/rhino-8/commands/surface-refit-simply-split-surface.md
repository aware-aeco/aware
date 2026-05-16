# surface-refit-simply-split-surface

Lifecycle: single

This routine accepts a 3-d trim curve (trimCrv3d).              The trim curve is assumed to run from one surface edge to the opposite edge;              this is referred to as a "simple" trim curve, roughly parallel to one of the              srf directions (either u or v).  We refer to that parameter as the "trim parameter".                           The routine splits the surface via the trim curve, and then refits either side              ("upper" = "above the trim", and "lower" = "below the trim") as a set of              untrimmed Nurbs surfaces.  The idea is to retain, as much as possible, the               Nurbs structure of srf, especially in the trim parameter.
