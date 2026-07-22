---
name: blender-headless-rendering
description: This skill should be used when running Blender unattended — composing an app that turns an IFC into a PNG or MP4 with nobody present, writing or debugging a `blender -b -P` command script, or reading a `blender` agent failure. Covers the `<<<AWARE_RESULT>>>` stdout framing and why exit 0 is not a success signal, the named error taxonomy, headless-only camera framing, the `matrix_world` staleness trap, the `__main__` guard every command script needs, and EEVEE vs Cycles on Blender 5.2.
---

# Headless rendering

The `blender` agent is the first visualization agent in the substrate that finishes the
job. Its siblings stop one step short and say so in their manifests: Enscape and
Twinmotion have no headless renderer, so AWARE stages the scene and a human presses
Render. Blender has one:

```bash
blender -b -P scripts/render_still.py -- @inputs.json
```

`-b` (background) starts Blender with no window, no OpenGL context and no event loop.
`-P` runs a Python file inside it with the full `bpy` API available. Everything the agent
does — import, materials, camera, render, save — happens in that one process and then it
exits.

## What headless gives you

| Property | Why it matters for an agent |
|---|---|
| No display server, no window manager | Runs on a build agent, a container, a scheduled task on a locked workstation |
| Two production renderers in the box | EEVEE for drafts and turntables, Cycles for hero stills — no third-party plugin |
| One process per command | Nothing to keep alive between calls; a crash costs one command, not a session |
| Deterministic argv contract | `-- <json>` or `-- @<file>.json`; no GUI state to reproduce |
| Real Python | `ifcopenshell` and the standard library are available inside the render process |

## What it costs

- **Nothing that needs a viewport works.** Anything under `bpy.ops.view3d.*` needs a 3D
  view context that does not exist under `-b`. See *Framing without a viewport* below.
- **Blender floods stdout.** Progress, warnings, memory statistics and render timings all
  land there. Structured output has to be recovered from that noise — hence the sentinel
  protocol.
- **The exit code lies.** See *Exit 0 is not success* below. This is the single most
  expensive thing on this page.
- **No `--factory-startup`.** The transport runs exactly the command line that was proven
  against Blender 5.2 and adds no flags, so the user's startup file and enabled add-ons
  are loaded. Deviating from a proven command line is how a working script starts failing
  for reasons unrelated to the script.
- **Wall-clock is unbounded by default.** A Cycles still at high samples, or a turntable
  of several hundred EEVEE frames, is legitimately slow. The transport imposes a budget
  (below) so a *wedged* Blender cannot hang a workflow forever.

## The sentinel result protocol

`scripts/_result.py` frames every payload:

```python
RESULT_BEGIN = "<<<AWARE_RESULT>>>"
RESULT_END = "<<<AWARE_RESULT_END>>>"

def emit(payload: dict) -> None:
    """Frame a payload on stdout so the transport can recover it."""
    sys.stdout.flush()
    print(RESULT_BEGIN)
    print(json.dumps(payload, indent=2, sort_keys=True))
    print(RESULT_END)
    sys.stdout.flush()
```

`cli/src/render/blender.rs` slices between the first `RESULT_BEGIN` and the first
`RESULT_END` after it. A script emits exactly one frame and exits immediately, so "first"
is also "only" — anchoring on the first marker keeps a payload that happens to *contain*
the sentinel text from re-anchoring the slice.

**Why framing rather than a bare `print(json.dumps(...))`:** Blender's own log is
interleaved on the same stream. A bare print is unrecoverable — there is no way to tell
the payload line from a render statistic that happens to look like JSON, and no way to
find the payload at all if Blender wrote after it. Framing turns "parse the last line and
hope" into an exact slice.

Both success and failure are framed. A named failure emits
`{"ok": false, "code", "message", "hint"}` and exits 1, so the caller always receives
structured data and never has to scrape a Python traceback out of a render log.

The transport also drains stdout and stderr **concurrently** with waiting on the process.
Blender is chatty enough to fill a pipe buffer, and a wait-then-read deadlocks.

## Exit 0 is not success

> Blender can exit 0 with a script that never ran.

This was proven on this build: a broken import inside the script made Blender exit 0 and
emit no result at all. Nothing about the process status distinguished that from a clean
render.

**The sentinel, not the exit code, is the success signal.** When the frame is absent the
transport reports its own failure and carries both output tails plus the exit status,
because a bare "no result" is the worst possible message:

```text
blender render.still: no framed result in Blender's output (exit 0). The script
never reached its result handler — Blender crashed, or failed before it could
report. Note the exit code is not the signal here; the missing <<<AWARE_RESULT>>>
frame is.
```

Any harness that drives these scripts directly must check for the frame the same way.
`tests/run_smoke.py` does, and raises with both tails when it is missing.

## The named error taxonomy

`_result.py` owns the codes. The transport surfaces `code` verbatim, so **these strings are
contract — renaming one is a breaking change.**

| `code` | Raised when | What the caller does |
|---|---|---|
| `ifcopenshell-missing` | `import ifcopenshell` fails inside Blender's Python | Run the install one-liner, which travels in `hint` (see the `ifc-import-ifcopenshell` skill) |
| `ifc-unreadable` | The IFC path does not exist, or `ifcopenshell.open()` raised | Check the path and the producer |
| `ifc-empty` | The geom iterator would not initialize, or zero products imported | The model has no tessellatable geometry; on the second case the `skipped` list rides along in the payload |
| `blend-unreadable` | A command was given a `blend-path` that is not there | Re-run `scene.import` |
| `invalid-inputs` | Missing/unknown input, unparseable JSON, no `--` separator | Fix the node's inputs |
| `render-failed` | No EEVEE engine, camera fit impossible, or the file was not written | Check the scene actually contains mesh geometry |
| `unexpected-error` | Last-resort catch in `_result.run` | Read the `traceback` field — it only rides along on this code |

`invalid-inputs` is the caller's mistake and maps to the CLI's validation class (exit 3).
Every other code is an agent-runtime failure (exit 4).

Four further failures come from the transport, not the script, and so have no `code`:

| Failure | Message shape |
|---|---|
| No Blender executable | Names the whole search order — `$AWARE_BLENDER`, `blender` on PATH, then the platform defaults — and the download URL |
| `AWARE_BLENDER` set but wrong | Explicit override never silently falls through to discovery |
| Agent not installed / damaged | "not installed" and "script missing" are different messages because the fix differs (`agent install` vs `agent update`) |
| Timeout | Kills Blender and, on Windows, its whole process tree; a leaked headless Blender holds a GPU context and the output file open |

Budgets: 1800 s for `render.still` / `render.turntable`, 600 s for everything else,
overridable per call with the `timeout-seconds` input.

## Framing without a viewport

`bpy.ops.view3d.camera_to_view_selected()` — the obvious way to fit a camera — **does not
work in background mode.** It needs a 3D viewport, and `-b` has none.

`scripts/_framing.py` computes the fit instead. It fits the model's bounding **sphere**,
not its projected box:

```python
lo, hi = scene_bounds()
centre = (lo + hi) / 2.0
radius = (hi - lo).length / 2.0
...
half_fov = min(half_x, half_y)
distance = (radius / math.sin(half_fov)) * margin
```

The sphere is rotation-invariant, so a turntable orbit cannot clip the model at some angle
the framing pass never sampled. The cost is a slightly loose crop, tuned by `margin`
(1.10 by default). `half_x` / `half_y` are derived from `camera.data.angle` and the render
aspect, and the narrower of the two is used — fitting to the wider one would crop.

The receipt (`centre`, `radius`, `distance`, `direction`) travels back in the command's
payload, which matters for the next section.

## The `matrix_world` staleness trap

After setting `.location` or `.rotation_euler` on an object, **`matrix_world` is not
refreshed until the next depsgraph evaluation.** Reading it before that gives a stale
value — for a freshly created camera, the identity matrix.

This cost real debugging time: a correct camera fit looked completely broken. A frustum
check run immediately after `frame_camera()` reported *every* vertex outside the frustum,
which is exactly what a broken fit looks like.

Two consequences, and they differ:

- **The render path is safe as-is.** `bpy.ops.render.render()` forces its own depsgraph
  evaluation, so pixels are always rasterized from the correct placement. No explicit sync
  is needed to render correctly.
- **Anything that *reads* the transform is not safe.** Diagnostics,
  `bpy_extras.object_utils.world_to_camera_view`, and re-parenting all read `matrix_world`
  and all see the stale value.

`render_turntable.py` hits the second case for real — it copies the camera's world matrix
in order to re-parent the camera under a pivot without moving it:

```python
# `frame_camera()` set camera.location/.rotation_euler directly, and
# pivot.location was just assigned above -- in background mode neither is
# reflected in .matrix_world until the next depsgraph evaluation. Without
# forcing one here, `world_matrix` below reads a stale (pre-framing) value,
# so the re-parented camera ends up wherever it was before framing (the
# origin, for a freshly created camera) instead of at the solved position.
bpy.context.view_layer.update()

world_matrix = camera.matrix_world.copy()
camera.parent = pivot
camera.matrix_parent_inverse = pivot.matrix_world.inverted()
camera.matrix_world = world_matrix
```

Without that `view_layer.update()` the turntable renders from the world origin — a video
of nothing, or of the inside of a beam.

**Rule:** log the values `frame_camera()` already returned (`centre` / `radius` /
`distance` / `direction`) — they are correct with no sync. If a transform must be read
back, call `bpy.context.view_layer.update()` first.

## Every command script needs a `__main__` guard

```python
if __name__ == "__main__":
    _result.run(main)
```

This is not style. `sys.argv` is process-global, so an unguarded module-scope `run(main)`
means that *importing* a command script re-parses the **importer's** argv as its own
inputs, executes, and calls `sys.exit()` — killing the caller before its own `main()` ever
runs.

`render_turntable.py` imports `render_still.py` to reuse `load_scene`, the lighting setup
and the engine resolution, so this is a live path, not a hypothetical. Unguarded, a
turntable request silently wrote a file named `turn.mp4.png` and died with the wrong
command's error message.

Verified on Blender 5.2 that `-P` sets `__name__ == "__main__"` for the directly-invoked
script, so the guard is safe. **Re-check that before porting these scripts to a different
Blender**: if it ever stops holding, the guard turns every command into a silent no-op
that exits 0 having done nothing — which the sentinel check above would then be the only
thing to catch.

## EEVEE vs Cycles

| | EEVEE | Cycles |
|---|---|---|
| Identifier on 5.2 | `BLENDER_EEVEE` | `CYCLES` |
| Kind | Rasterizer | Path tracer |
| Used by | `quality: draft` stills, **all** turntables | `quality: production` stills |
| Default samples | 32 (still) / 16 (turntable), via `scene.eevee.taa_render_samples` | 128, via `scene.cycles.samples` |
| Right when | Iterating, previewing framing, anything with a frame count | One hero image where reflections and soft shadows have to be correct |

`render.turntable` is EEVEE-only by construction. Hundreds of path-traced frames is not a
turntable, it is a render farm job.

**The identifier is `BLENDER_EEVEE` on 5.2, not `BLENDER_EEVEE_NEXT`.** Blender 4.2
introduced `BLENDER_EEVEE_NEXT`; 5.x reverted to the plain name. `_eevee_engine()` probes
for both, newest first, so one script spans the 4.x/5.x split.

**Enum introspection under-reports — never build a Cycles availability check on it.**
Measured on this install:

```python
bpy.types.RenderSettings.bl_rna.properties["engine"].enum_items
# -> ['BLENDER_EEVEE']
```

while the real validator set is `('BLENDER_EEVEE', 'BLENDER_WORKBENCH', 'CYCLES')` —
confirmed by direct assignment and `addon_utils.check('cycles')`. `_eevee_engine()`'s
membership test still works because `BLENDER_EEVEE` *is* in the under-reported list, but a
guard like `if "CYCLES" not in identifiers: raise` would report Cycles missing on a machine
that has it. Assign the engine directly and let it fail if genuinely absent.

## Blender 5.2 API breaks worth knowing

**Video output is gated behind `media_type`.** `image_settings` gained a
`media_type` switch (`IMAGE` / `MULTI_LAYER_IMAGE` / `VIDEO`) that decides which
`file_format` values are valid. It defaults to `IMAGE`, so assigning `file_format =
"FFMPEG"` first raises `TypeError: enum "FFMPEG" not found in (...)`:

```python
if hasattr(scene.render.image_settings, "media_type"):
    scene.render.image_settings.media_type = "VIDEO"
scene.render.image_settings.file_format = "FFMPEG"
```

The `hasattr` guard keeps the script working on 4.x, where the property does not exist.

**`Action.fcurves` is gone.** The 4.4 layered-action redesign moved fcurves to
`action.layers[].strips[].channelbags[].fcurves`. On 5.2 the flat attribute is absent
entirely and reaching for it raises
`AttributeError: 'Action' object has no attribute 'fcurves'`. `_iter_fcurves()` yields
from whichever shape exists, so keyframe interpolation can be set on both.

## Measured environment

| | Value |
|---|---|
| Blender | 5.2.0 LTS, build hash `fbe6228777e7` |
| Bundled Python | 3.13.13, at `…\Blender 5.2\5.2\python\bin\python.exe` |
| `ifcopenshell` | 0.8.5 |
| EEVEE identifier | `BLENDER_EEVEE` |
| Geometry units out of `ifcopenshell` | metres (`unit-scale` stays 1.0) |

Everything on this page was checked against that build. On a different Blender, re-run the
probe rather than assuming — especially the `__main__` guard behaviour and the EEVEE
identifier.

## Worked example

IFC in, hero still and turntable out, unattended:

```yaml
nodes:
  - id: stage
    agent: blender
    command: scene.import
    inputs:
      ifc-path: '{{ inputs.model-ifc }}'
      blend-path: '{{ run.tmp-dir }}/model.blend'

  - id: look
    agent: blender
    command: scene.apply-look
    inputs:
      blend-path: '{{ nodes.stage.blend-path }}'
      preset: realistic

  - id: hero
    agent: blender
    command: render.still
    inputs:
      blend-path: '{{ nodes.look.blend-path }}'
      output-path: '{{ inputs.out-dir }}/{{ run.date }}-hero.png'
      quality: production
      direction: iso
      width-pixels: 3840
      height-pixels: 2160
      samples: 256
      timeout-seconds: 3600

  - id: turntable
    agent: blender
    command: render.turntable
    inputs:
      blend-path: '{{ nodes.look.blend-path }}'
      output-path: '{{ inputs.out-dir }}/{{ run.date }}-turntable.mp4'
      duration-seconds: 8
      fps: 30
      width-pixels: 1920
      height-pixels: 1080
```

No human touches this. `stage` and `look` are cheap and cacheable; the two renders are the
expensive half and can run in parallel off the same staged `.blend`.

For a one-shot preview, `render.still` also takes `ifc-path` directly and does the import
and look application on the fly — the same picture with no staged file:

```yaml
  - id: preview
    agent: blender
    command: render.still
    inputs:
      ifc-path: '{{ inputs.model-ifc }}'
      output-path: '{{ inputs.out-dir }}/preview.png'
      quality: draft
      preset: clay
      width-pixels: 960
      height-pixels: 540
```

`render.still` and `render.turntable` return the framing receipt alongside the file, so a
downstream node can log the fit that produced the image. The payload's shape:

```json
{
  "ok": true,
  "path": "…/2026-07-22-hero.png",
  "size-bytes": 412915,
  "engine": "CYCLES",
  "quality": "production",
  "framing": { "centre": [x, y, z], "radius": r, "distance": d, "direction": "iso" }
}
```

`radius` is the half-diagonal of the scene's bounding box and `distance` is derived from it
by the fit, so `radius` is the number to watch: one far larger than the model's real
half-diagonal means the scene contains stray geometry — an imported opening, a leftover
default cube — and the crop will look wrong long before anything errors.
