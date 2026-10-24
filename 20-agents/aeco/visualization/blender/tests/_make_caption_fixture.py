"""Build the caption test's staged .blend (a test helper, not a command).

Three boxes, plus two traps a caption must survive:
- the file keyframes its own `render.use_stamp` OFF, which frame evaluation would put back on every frame of a
  turntable unless the caption holds its settings through rendering;
- it asks for the stamp's text from sequencer strips (`metadata_input = STRIPS`) and has one image strip with no
  note. Blender stamps a strip's metadata in place of the note only when the picture is the sequencer output
  (not the case here: `use_sequencer = False` keeps the render the model), so today this trap is inert and
  kept so that a change in Blender's behaviour would show.

Run: blender -b --factory-startup -P _make_caption_fixture.py -- '{"out": "fixture.blend", "strip-png": "s.png"}'
"""

import json
import sys

import bpy

args = json.loads(sys.argv[sys.argv.index("--") + 1])
bpy.ops.wm.read_factory_settings(use_empty=True)
for x, size in ((0, 2), (3, 1), (-3, 1.5)):
    bpy.ops.mesh.primitive_cube_add(size=size, location=(x, 0, size / 2))

scene = bpy.context.scene
render = scene.render
render.use_stamp = False
for frame in (1, 4, 8):
    render.keyframe_insert("use_stamp", frame=frame)

image = bpy.data.images.new("strip", 4, 4)
image.filepath_raw = args["strip-png"]
image.file_format = "PNG"
image.save()
editor = scene.sequence_editor_create()
strips = editor.strips if hasattr(editor, "strips") else editor.sequences
strips.new_image("strip", args["strip-png"], channel=1, frame_start=1)
render.metadata_input = "STRIPS"
render.use_sequencer = False
if args.get("crop-x"):
    # A render cropped to a border: Blender delivers (and stamps) the border's size, not the resolution.
    render.use_border = True
    render.use_crop_to_border = True
    render.border_min_x, render.border_max_x = (float(v) for v in args["crop-x"])
    render.border_min_y, render.border_max_y = 0.0, 1.0

bpy.ops.wm.save_as_mainfile(filepath=args["out"])
print("AWARE_FIXTURE_READY")
