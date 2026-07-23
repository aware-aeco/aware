"""Headless camera framing.

`bpy.ops.view3d.camera_to_view_selected()` requires a 3D viewport that does not
exist in background mode, so the fit is computed from the scene bounds.

The fit uses the model's bounding SPHERE, not its projected box: the sphere is
rotation-invariant, so a turntable orbit cannot clip the model at some angle the
framing pass never sampled. The cost is a slightly loose crop, tuned by `margin`.
"""

import math

import bpy
from mathutils import Vector

import _ifc_import

# Named view directions, in Blender's Z-up world. Each is a unit-ish vector FROM
# the model TOWARD the camera; they are normalized before use.
DIRECTIONS = {
    # A three-quarter view: asymmetric in plan and slightly raised, which reads
    # as photography. `iso`'s exact 45 degrees in plan reads as a CAD screenshot
    # -- it is kept byte-identical for anyone who wants the axonometric.
    "hero": Vector((1.0, -1.7, 0.62)),
    "iso": Vector((1.0, -1.0, 0.7)),
    "front": Vector((0.0, -1.0, 0.0)),
    "back": Vector((0.0, 1.0, 0.0)),
    "left": Vector((-1.0, 0.0, 0.0)),
    "right": Vector((1.0, 0.0, 0.0)),
    "top": Vector((0.0, 0.0, 1.0)),
}

# Blender's default 50mm on a 36mm sensor visibly distorts a building towards
# the frame edges. `distance` below is DERIVED from `camera.data.angle`, so a
# longer lens simply backs the camera off for the same framing -- the
# bounding-sphere fit, and with it the turntable's no-clipping guarantee, is
# untouched by this.
DEFAULT_LENS_MM = 80.0


def scene_bounds() -> tuple[Vector, Vector]:
    """World-space min/max corners over every mesh object."""
    lo = Vector((math.inf,) * 3)
    hi = Vector((-math.inf,) * 3)
    found = False
    for obj in bpy.data.objects:
        if obj.type != "MESH":
            continue
        # Staging helpers (the ground plane) are sized from this very fit, so
        # admitting them would be circular: the floor would inflate the sphere,
        # which would enlarge the floor, and the model would shrink in frame.
        if obj.get(_ifc_import.PROP_HELPER):
            continue
        for corner in obj.bound_box:
            world = obj.matrix_world @ Vector(corner)
            for axis in range(3):
                lo[axis] = min(lo[axis], world[axis])
                hi[axis] = max(hi[axis], world[axis])
            found = True
    if not found:
        raise ValueError("scene contains no mesh geometry to frame")
    return lo, hi


def frame_camera(
    camera: bpy.types.Object,
    direction: str = "iso",
    margin: float = 1.10,
) -> dict:
    """Place `camera` so the whole scene fits, looking at the model centre.

    Returns the framing receipt (centre, radius, distance) for the caller's log.
    """
    lo, hi = scene_bounds()
    centre = (lo + hi) / 2.0
    radius = (hi - lo).length / 2.0
    if radius <= 0.0:
        raise ValueError("scene bounding box is degenerate")

    vector = DIRECTIONS.get(direction)
    if vector is None:
        raise ValueError(
            f"unknown direction `{direction}`; expected one of {sorted(DIRECTIONS)}"
        )

    scene = bpy.context.scene
    render = scene.render
    width = render.resolution_x * (render.pixel_aspect_x or 1.0)
    height = render.resolution_y * (render.pixel_aspect_y or 1.0)

    cam_data = camera.data
    # `angle` is the FOV across the sensor-fit axis; derive the other from aspect.
    if width >= height:
        half_x = cam_data.angle / 2.0
        half_y = math.atan(math.tan(half_x) * height / width)
    else:
        half_y = cam_data.angle / 2.0
        half_x = math.atan(math.tan(half_y) * width / height)

    half_fov = min(half_x, half_y)
    distance = (radius / math.sin(half_fov)) * margin

    camera.location = centre + vector.normalized() * distance
    look = (centre - camera.location).normalized()
    camera.rotation_euler = look.to_track_quat("-Z", "Y").to_euler()

    # Both planes have to clear the GROUND, not just the model.
    #
    # `clip_start` was `max(distance - radius * 4.0, ...)` while nothing existed
    # near the camera -- pushing the near plane out buys depth precision, and
    # the model sits at `distance` +/- `radius`, so it was safe. The ground
    # plane is not: it runs from the model TOWARDS the camera and out past it,
    # so a near plane that far out slices it, and the cut renders as a hard
    # horizontal band across the frame. Seen for real on `direction: front`,
    # where the camera sits at mid-model height and the band cut the bottom
    # sixth of the image.
    #
    # A near plane at 0.1% of the camera distance leaves a depth ratio of a few
    # thousand to one, which is nothing for a 32-bit depth buffer, and Cycles
    # has no depth buffer at all.
    cam_data.clip_start = distance / 1000.0
    cam_data.clip_end = distance + radius * 12.0

    return {
        "centre": [round(v, 6) for v in centre],
        "radius": round(radius, 6),
        "distance": round(distance, 6),
        "direction": direction,
    }


def ensure_camera() -> bpy.types.Object:
    """Return the scene camera, creating one if the .blend has none."""
    scene = bpy.context.scene
    if scene.camera is not None:
        return scene.camera
    cam_data = bpy.data.cameras.new("AwareCamera")
    cam_data.lens = DEFAULT_LENS_MM
    camera = bpy.data.objects.new("AwareCamera", cam_data)
    scene.collection.objects.link(camera)
    scene.camera = camera
    return camera
