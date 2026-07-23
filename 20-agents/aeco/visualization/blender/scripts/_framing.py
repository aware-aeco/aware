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
    "iso": Vector((1.0, -1.0, 0.7)),
    "front": Vector((0.0, -1.0, 0.0)),
    "back": Vector((0.0, 1.0, 0.0)),
    "left": Vector((-1.0, 0.0, 0.0)),
    "right": Vector((1.0, 0.0, 0.0)),
    "top": Vector((0.0, 0.0, 1.0)),
}


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

    # Keep the model comfortably inside the clip range at any orbit angle.
    cam_data.clip_start = max(distance - radius * 4.0, distance / 1000.0)
    cam_data.clip_end = distance + radius * 4.0

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
    camera = bpy.data.objects.new("AwareCamera", cam_data)
    scene.collection.objects.link(camera)
    scene.camera = camera
    return camera
