# viewport-info-dolly-camera

Lifecycle: single

DollyCamera() does not update the frustum's clipping planes.             To update the frustum's clipping planes call DollyFrustum(d)             with d = dollyVector o cameraFrameZ.  To convert screen locations             into a dolly vector, use GetDollyCameraVector().             Does not update frustum.  To update frustum use DollyFrustum(d) with d = dollyVector o cameraFrameZ.
