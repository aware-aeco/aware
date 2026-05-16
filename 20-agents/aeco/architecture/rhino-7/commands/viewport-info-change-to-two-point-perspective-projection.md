# viewport-info-change-to-two-point-perspective-projection

Lifecycle: single

Changes projections of valid viewports             to a two point perspective.  It will make common additional             adjustments to the frustum and camera location and direction             so the resulting views are similar.             If the current projection is perspective and             IsFrustumIsLeftRightSymmetric is true and             IsFrustumIsTopBottomSymmetric is false, then no changes are             made and true is returned.
