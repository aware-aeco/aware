# viewport-info-change-to-perspective-projection

Lifecycle: single

Use this function to change projections of valid viewports             from parallel to perspective.  It will make common additional             adjustments to the frustum and camera location so the resulting             views are similar.  The camera direction and target point are             not changed.             If the current projection is perspective and symmetricFrustum,             IsFrustumIsLeftRightSymmetric, and IsFrustumIsTopBottomSymmetric             are all equal, then no changes are made and true is returned.
