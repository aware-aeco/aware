---
name: blender-exec-bpy
description: This skill should be used whenever a workflow needs a Blender render the fixed `blender` commands do not make — a construction sequence, a phasing or erection-order animation, a camera fly-through, a sun or shadow study, an exploded view, a "what changed" highlight, or any custom `bpy` scene — and so composes the `blender` agent's `exec` command. Use it when writing or reviewing an `exec` node's `code`, wiring its `args`/`ifc-path`/`output-dir`, budgeting its `timeout-seconds`, reading its `outputs` receipt or its `exec-error`, or deciding why a templated `code` was refused with E_APP_INPUT_MUST_BE_LITERAL. Covers the `run(ctx)` contract, the three scene starting points, per-frame progress, EEVEE as the animation default, and five worked recipes.
---

# `blender/exec` — author-time `bpy`, run unattended

`exec` is how a workflow gets a Blender render nobody pre-built a command for. The
author (usually the terminal AI, while composing the workflow) writes Python that
defines `run(ctx)`. That code is stored **in the workflow file**, approved with the
lock, and runs unchanged on every run, headlessly:

```yaml
- id: sequence
  agent: blender
  command: exec
  inputs:
    ifc-path: '{{ inputs.ifc-path }}'
    output-dir: '{{ inputs.output-dir }}'
    preset: realistic
    args:
      seconds-per-storey: 2
    timeout-seconds: 1800
    code: |
      # Builds the model storey by storey and renders it as an MP4.
      def run(ctx):
          ...
```

The fixed commands (`render.still`, `render.turntable`) stay the right choice when
they fit — they are tested recipes. Reach for `exec` when the picture needs
something they cannot express.

## The literal-code rule — data in `args`, never code

`code` is declared `literal: true`. Validate, compile **and** the runtime refuse a
`code` that contains template syntax (`{{ }}`, `{% %}`, `{# #}`) with
`E_APP_INPUT_MUST_BE_LITERAL`, naming the node and the input.

The reason is approval. The `.lock` hashes the app source; a person approves that
lock; Run refuses when the source drifts. A templated `code` would hash the
*template* and then execute whatever text the template resolved to at run time —
Python nobody approved. So:

- Write the program out in full in `code`.
- Pass everything that varies per run through `args` (templatable), `ifc-path`,
  `blend-path` and `output-dir`. `ctx.args` is a plain dict.
- Never build Python source from `args` and `exec()` it inside `run` — that is the
  same hole by another route.
- Because template delimiters are refused anywhere in `code`, avoid `{{`/`{%`/`{#`
  in the Python itself (an f-string escape like `f"{{"` trips it); use
  `"{" + "{"` or `.format()`-free strings instead.

A changed `code` is a changed program: it needs a fresh compile and approval, which
is the point.

## What happens before `run(ctx)`

The scene is prepared, deterministically, from exactly one source:

| Inputs | Starting scene |
|---|---|
| `ifc-path` | Factory reset to an empty scene, the IFC imported (each object carries `ifc_guid`, `ifc_class`, `ifc_name`, `ifc_material`, `ifc_storey`), the `preset` look applied (`clay`, `realistic`, `section-style`), default studio lighting and a key sun. No camera yet. |
| `blend-path` | That `.blend` opened as-is — its camera, lights and materials are yours. |
| neither | An empty factory scene: no default cube, camera or light. |

`ifc-path` and `blend-path` together is `invalid-inputs`. The user's Blender
startup file never leaks in.

The code is compiled before the scene work, so a syntax error fails in a second
rather than after a long import.

## The `ctx` contract

| Member | What it gives you |
|---|---|
| `ctx.args` | The node's `args` object (a dict; `{}` when absent). |
| `ctx.output_dir` | Absolute real path of `output-dir` (created if missing). Write here. |
| `ctx.scene` | `bpy.context.scene`. |
| `ctx.elements()` | `[(obj, {guid, name, class, material, storey}), …]` for every imported IFC element; staging helpers (the ground) excluded. |
| `ctx.frame(direction="hero")` | Creates the camera if needed and fits it to the model (`hero`, `iso`, `front`, `back`, `left`, `right`, `top`). Returns `{centre, radius, distance, direction}`. |
| `ctx.apply_look(preset)` | Re-shade every element from its IFC class/material. |
| `ctx.environment(name="studio")` | HDRI lighting (`studio`, `courtyard`, `city`, `forest`, `interior`, `night`, `sunrise`, `sunset`, `gradient`, or a path) plus a key sun if none exists. Returns the receipt. |
| `ctx.ground(on=True)` | The fading floor, sized from the model. Call before `ctx.frame` so framing ignores it correctly. |
| `ctx.progress(phase, done=None, total=None, message=None)` | Publish your own progress record (phase ≤ 64 chars). |
| `ctx.plan_passes(n)` | Declare how many `render_*` calls follow, so each pass is labelled `pass i of n` instead of `pass i of ?`. |
| `ctx.render_still(path, quality="draft", samples=None)` | PNG of the current frame. |
| `ctx.render_animation(path, quality="draft", samples=None)` | H.264 MP4 of `scene.frame_start..frame_end`. |
| `ctx.add_output(path, kind="file")` | Register any other file you wrote (`video`, `image`, `blend`, `file`). |

Relative paths given to `render_*`/`add_output` resolve inside `ctx.output_dir`.
`quality="draft"` is EEVEE, `"production"` is Cycles. Without a camera, `render_*`
frames one at `hero` first.

`run(ctx)` returns anything JSON can carry, up to **256 KiB** serialized. Larger
fails as `result-too-large`: write the data to a file in `ctx.output_dir`,
`ctx.add_output(path, "file")` it, and return a summary.

### The node output

```json
{
  "result":  { "…": "whatever run(ctx) returned" },
  "outputs": [
    { "path": "C:\\…\\sequence.mp4", "kind": "video", "size-bytes": 812345,
      "frames": 240, "verified": true }
  ],
  "frames":  240,
  "scene":   { "source": "ifc-path", "imported": 312, "preset": "realistic", "…": "…" }
}
```

`outputs` is an explicit receipt, never a directory listing. Each entry is
verified: the file exists, is a regular non-empty file, and its real path is inside
`output_dir`. One that fails stays in the list with `verified: false` and a
`reason` (`was not written`, `is outside output-dir (…)`, `is empty`). Downstream
nodes should consume only `verified: true` entries.

### Errors

A compile or runtime exception becomes
`{ok: false, code: "exec-error", message, line, traceback}`. `line` and the
traceback count from the **first line of `code`**, and the traceback shows only the
authored frames. `invalid-inputs` (missing `code`/`output-dir`, both scene
sources, non-object `args`) is caught before Blender even starts and exits 3.

## Progress — a long render never looks stuck

Each `render_*` call is one progress segment. While it renders, one record per
frame **written** reaches the run trace as `node-progress`:

```json
{"phase": "render", "done": 143, "total": 240, "message": "pass 1 of 2"}
```

Frames are counted on `bpy.app.handlers.render_write` — measured on Blender 5.2 to
fire exactly once per frame written, for EEVEE and Cycles. Counting is enabled only
on measured versions; on any other Blender a pass reports `phase: render` with its
label and no numbers, so a consumer shows elapsed time instead of a guessed bar.
Earlier phases are `import`, `look` and `setup`. A second `render_*` call starts a
new segment at `done: 1` — it is a new bar, not a step backwards. Above 5,000 frames
in one pass, every Nth frame is reported (plus the last).

Do not count frames yourself with `frame_change_post`; it fires an extra time after
the job and would claim a frame that was never written.

## EEVEE for animation, and budgeting time

- **Default to `quality="draft"` (EEVEE) for animations.** Cycles at production
  samples is minutes per frame; an animation is hundreds of frames.
- **Estimate before running, and tell the user.** Render a 1-frame still at the
  target resolution (or use a known per-frame time), then: frames × seconds/frame
  + import time (≈ 1 s per 50 elements) + ~30 s Blender/EEVEE start-up.
- **Set `timeout-seconds` from that estimate with headroom** (×1.5–2). The default
  is 1800 s; a timeout kills Blender's whole process tree and reports which budget
  ran out, with the progress that had arrived.
- Cut cost with resolution (`scene.render.resolution_x/_y`, or
  `resolution_percentage`), `scene.eevee.taa_render_samples` (16 is plenty for a
  sequence), and fps (24 reads fine for construction).
- An interrupted FFMPEG MP4 is usually unplayable — there is no partial result.

## Recipes

Each opens with a plain-English `#` comment, which is what a workflow's node
Description shows a non-coder.

### Construction sequence by storey (keyframed visibility)

```python
# Builds the model up storey by storey -- each storey appears in turn -- and
# renders it as an MP4 that ends on the finished building.
def run(ctx):
    scene = ctx.scene
    fps = int(ctx.args.get("fps", 24))
    hold = int(ctx.args.get("seconds-per-storey", 2) * fps)
    elements = ctx.elements()

    # Storey order: as given in args, else bottom-up by each storey's lowest point.
    lowest = {}
    for obj, props in elements:
        z = min((obj.matrix_world @ v.co).z for v in obj.data.vertices) if obj.data.vertices else obj.location.z
        lowest[props["storey"]] = min(z, lowest.get(props["storey"], z))
    order = ctx.args.get("storey-order") or sorted(lowest, key=lowest.get)

    ctx.ground(True)
    ctx.frame("hero")
    scene.render.fps = fps
    scene.frame_start = 1
    scene.frame_end = hold * (len(order) + 1)
    scene.render.resolution_x = int(ctx.args.get("width", 1280))
    scene.render.resolution_y = int(ctx.args.get("height", 720))

    for obj, props in elements:
        step = order.index(props["storey"]) if props["storey"] in order else len(order) - 1
        appear = 1 + step * hold
        obj.hide_render = True
        obj.keyframe_insert("hide_render", frame=1)
        obj.hide_render = False
        obj.keyframe_insert("hide_render", frame=appear)

    video = ctx.render_animation("construction-sequence.mp4")
    return {"storeys": order, "frames": video.get("frames"), "verified": video["verified"]}
```

For erection order instead of storeys, pass `args: {sequence: {<guid>: <step>}}`
(from a schedule or a Tekla phase report) and look the step up by `props["guid"]`.
Boolean properties such as `hide_render` key with constant interpolation, so each
element pops in on its frame.

### Camera fly-through

```python
# Flies the camera past the model along a few waypoints and renders the flight.
def run(ctx):
    import bpy
    scene = ctx.scene
    framing = ctx.frame("hero")
    cam = scene.camera
    cx, cy, cz = framing["centre"]
    r = framing["radius"]
    waypoints = ctx.args.get("waypoints") or [
        (cx - 2.5 * r, cy - 2.5 * r, cz + 0.6 * r),
        (cx, cy - 1.8 * r, cz + 0.3 * r),
        (cx + 2.5 * r, cy - 1.2 * r, cz + 0.8 * r),
    ]
    target = bpy.data.objects.new("AwareLookAt", None)
    target.location = (cx, cy, cz)
    scene.collection.objects.link(target)
    track = cam.constraints.new("TRACK_TO")
    track.target = target
    span = int(ctx.args.get("frames-per-leg", 72))
    for i, point in enumerate(waypoints):
        cam.location = point
        cam.keyframe_insert("location", frame=1 + i * span)
    scene.frame_start, scene.frame_end = 1, 1 + (len(waypoints) - 1) * span
    return {"video": ctx.render_animation("fly-through.mp4")}
```

### Sun study

```python
# Sweeps the sun across the sky from morning to evening so the shadows move
# over the model, and renders the day as an MP4.
import math

def run(ctx):
    import bpy
    scene = ctx.scene
    ctx.ground(True)
    ctx.frame("hero")
    sun = next((o for o in bpy.data.objects if o.type == "LIGHT" and o.data.type == "SUN"), None)
    if sun is None:
        sun = bpy.data.objects.new("Sun", bpy.data.lights.new("Sun", "SUN"))
        scene.collection.objects.link(sun)
    frames = int(ctx.args.get("frames", 120))
    scene.frame_start, scene.frame_end = 1, frames
    elevation = math.radians(float(ctx.args.get("max-elevation-deg", 55)))
    for f, t in ((1, 0.0), (frames // 2, 0.5), (frames, 1.0)):
        azimuth = math.radians(90 + 180 * t)          # east -> south -> west
        height = elevation * math.sin(math.pi * t) + math.radians(5)
        sun.rotation_euler = (math.pi / 2 - height, 0.0, azimuth)
        sun.keyframe_insert("rotation_euler", frame=f)
    return {"video": ctx.render_animation("sun-study.mp4")}
```

For a true solar path, compute azimuth/elevation from latitude, date and time in
the code (pure math) and pass the site's latitude and date in `args`.

### Exploded view

```python
# Pulls the storeys apart vertically and back together, so each level can be seen.
def run(ctx):
    scene = ctx.scene
    ctx.frame("iso")
    storeys = sorted({p["storey"] for _, p in ctx.elements()})
    gap = float(ctx.args.get("gap-m", 4.0))
    frames = int(ctx.args.get("frames", 96))
    scene.frame_start, scene.frame_end = 1, frames
    for obj, props in ctx.elements():
        lift = storeys.index(props["storey"]) * gap
        base = obj.location.z
        for f, z in ((1, base), (frames // 2, base + lift), (frames, base)):
            obj.location.z = z
            obj.keyframe_insert("location", index=2, frame=f)
    ctx.frame("iso")   # refit at rest; the camera does not follow the explosion
    return {"video": ctx.render_animation("exploded.mp4")}
```

### Highlight changed elements

```python
# Paints the elements that changed in a bright accent over a neutral clay model
# and renders one still, so a reviewer sees what moved at a glance.
def run(ctx):
    import bpy
    changed = set(ctx.args.get("changed-guids", []))
    ctx.apply_look("clay")
    accent = bpy.data.materials.new("AwareChanged")
    accent.use_nodes = True
    accent.node_tree.nodes["Principled BSDF"].inputs["Base Color"].default_value = (0.95, 0.25, 0.1, 1)
    hits = 0
    for obj, props in ctx.elements():
        if props["guid"] in changed:
            obj.data.materials.clear()
            obj.data.materials.append(accent)
            hits += 1
    ctx.ground(True)
    ctx.frame("hero")
    still = ctx.render_still("changes.png", quality="production", samples=64)
    return {"highlighted": hits, "not-found": len(changed) - hits, "still": still["verified"]}
```

Report `not-found` rather than dropping it: a GUID that is not in the model is a
finding, not noise.

## Gotchas

- `bpy.ops.view3d.*` and anything needing a viewport does not exist headless —
  use `ctx.frame`, which fits the camera from geometry.
- After moving objects or the camera in code, `bpy.context.view_layer.update()`
  before reading `matrix_world`.
- Blender 4.4+ keeps keyframes in layered actions (`action.layers[].strips[]
  .channelbags[].fcurves`); `action.fcurves` no longer exists on 5.x. To change
  interpolation, walk both shapes (see `render_turntable._iter_fcurves`).
- Blender 5.x gates FFMPEG behind `image_settings.media_type = "VIDEO"`;
  `render_animation` handles it — if you configure output yourself, set it first.
- Python writes are not sandboxed; the agent's declared filesystem permission is
  the contract a person approves. Keep writes inside `ctx.output_dir`.
