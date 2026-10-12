app:           model-to-construction-sequence
version:       0.1.0
display-name:  Model to Construction Sequence
description: |
  IFC in, a construction-sequence animation out: the building rises storey by
  storey, bottom-up, and the MP4 ends on the finished model.

  Nothing in the `blender` agent's fixed recipes makes this picture, so the
  node runs `exec` -- `bpy` code written once when this app was authored,
  pinned in the approved lock, and run unchanged on every run. What varies per
  run (the model, where the video goes, how long each storey holds, the frame
  size) travels as data in `args`; the program itself is never templated, and
  validate, compile and run all refuse a templated `code`.

  The render reports per-frame progress while it works, so a long animation
  never looks stuck from the outside: the run trace carries `node-progress`
  records counting frames up to the total.

exposes-as-agent: false

requires:
  - blender@0.2.x

requires-permissions:
  filesystem:
    - read:  '{{ inputs.ifc-path }}'
    - write: '{{ inputs.output-dir }}'

layout: dag

nodes:
  - id: sequence
    agent: blender
    command: exec
    inputs:
      ifc-path:   '{{ inputs.ifc-path }}'
      output-dir: '{{ inputs.output-dir }}'
      preset:     realistic
      timeout-seconds: 3600
      args:
        seconds-per-storey: '{{ inputs.seconds-per-storey }}'
        fps:    24
        width:  '{{ inputs.width-pixels }}'
        height: '{{ inputs.height-pixels }}'
      code: |
        # Builds the model up storey by storey -- each storey appears in turn,
        # lowest first -- and renders it as an MP4 that ends on the whole building.
        def run(ctx):
            scene = ctx.scene
            fps = int(ctx.args.get("fps") or 24)
            hold = max(1, int(float(ctx.args.get("seconds-per-storey") or 2) * fps))
            elements = ctx.elements()

            # Order storeys bottom-up by the lowest point of anything on them.
            lowest = {}
            for obj, props in elements:
                world = [obj.matrix_world @ v.co for v in obj.data.vertices]
                z = min(p.z for p in world) if world else obj.location.z
                key = props["storey"]
                lowest[key] = min(z, lowest.get(key, z))
            order = sorted(lowest, key=lowest.get)

            ctx.ground(True)
            ctx.frame("hero")
            scene.render.fps = fps
            scene.render.resolution_x = int(ctx.args.get("width") or 1280)
            scene.render.resolution_y = int(ctx.args.get("height") or 720)
            if hasattr(scene, "eevee") and hasattr(scene.eevee, "taa_render_samples"):
                scene.eevee.taa_render_samples = 16
            scene.frame_start = 1
            scene.frame_end = hold * (len(order) + 1)

            # Hidden until its storey's turn, then visible for the rest of the film.
            for obj, props in elements:
                appear = 1 + order.index(props["storey"]) * hold
                obj.hide_render = True
                obj.keyframe_insert("hide_render", frame=1)
                obj.hide_render = False
                obj.keyframe_insert("hide_render", frame=appear)

            ctx.plan_passes(1)
            video = ctx.render_animation("construction-sequence.mp4")
            return {
                "storeys": order,
                "elements": len(elements),
                "frames": video.get("frames"),
                "video": video["path"],
                "verified": video["verified"],
            }
    # `exec` is write-mode: it renders into `output-dir`. It touches no host
    # model, so there is nothing to snapshot or transact -- the block records
    # that, and groups the node's writes under one name for the audit trail.
    safety:
      transaction-group: construction-sequence
      snapshot: false

connections: []
