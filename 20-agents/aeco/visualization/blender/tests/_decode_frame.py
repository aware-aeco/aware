"""Decode frames of an MP4 to PNGs, inside Blender in background mode (a test helper, not a command).

Blender decodes video through its sequencer: the active scene holds the movie as a strip, each frame is made
current, and a still render of the sequencer output writes exactly that decoded frame. Nothing here needs
an ffmpeg binary on PATH -- the one Blender links against does the work.

Run: blender -b --factory-startup -P _decode_frame.py -- '{"mp4": "a.mp4", "frames": [1, 2], "out-dir": "d"}'
Writes <out-dir>/frame-<n>.png for each frame.
"""

import json
import os
import sys

import bpy

args = json.loads(sys.argv[sys.argv.index("--") + 1])
# The ACTIVE scene: in background mode a frame set on any other scene is not what a render evaluates.
# With a strip present and use_sequencer on, the render is the sequencer output, not its 3D content.
scene = bpy.context.scene
editor = scene.sequence_editor_create()
# Blender 4.x calls it `sequences`; 5.x renamed the collection `strips`.
strips = editor.strips if hasattr(editor, "strips") else editor.sequences
strip = strips.new_movie("clip", args["mp4"], channel=1, frame_start=1)
scene.render.resolution_x = strip.elements[0].orig_width if strip.elements else 320
scene.render.resolution_y = strip.elements[0].orig_height if strip.elements else 180
scene.render.resolution_percentage = 100
scene.render.use_sequencer = True
scene.render.use_stamp = False
if hasattr(scene.render.image_settings, "media_type"):
    scene.render.image_settings.media_type = "IMAGE"
scene.render.image_settings.file_format = "PNG"
scene.frame_start = 1
scene.frame_end = strip.frame_final_duration
for frame in args["frames"]:
    scene.frame_set(int(frame))
    scene.render.filepath = os.path.join(args["out-dir"], f"frame-{int(frame)}.png")
    bpy.ops.render.render(write_still=True)
print("AWARE_DECODED_FRAME", json.dumps({"frames": strip.frame_final_duration}))
