app:           model-to-renders
version:       0.1.0
display-name:  Model to Renders
description: |
  IFC in, finished renders out — the substrate's first visualization chain
  that completes with no human in the loop.

  Stages the model once, assigns materials from what the IFC already states
  (product class plus material grade), then renders a hero still and a 360
  turntable from that same staged scene. Every earlier visualization example
  stopped at "…and now a designer presses Render"; this one does not, because
  Blender renders headlessly and Enscape/Twinmotion cannot.

  Point it at any IFC: one written by the `ifc` agent from a takeoff, a Tekla
  export, a Revit export. The agent is generic by construction — material
  associations resolve through layer-set, profile-set and type-level wrappers,
  not just the trivial direct case.

  The final `scene.info` node is not decoration. It is the receipt: what
  actually imported, what was skipped, and what was deliberately excluded
  (openings and spaces never render). A render you cannot account for is a
  render you cannot trust.

exposes-as-agent: false

requires:
  - blender@0.1.x

requires-permissions:
  filesystem:
    - read:  '{{ inputs.ifc-path }}'
    - write: '{{ inputs.output-dir }}'

layout: dag

nodes:
  - id: stage
    agent: blender
    command: scene.import
    inputs:
      ifc-path:   '{{ inputs.ifc-path }}'
      blend-path: '{{ inputs.output-dir }}/staged.blend'

  - id: look
    agent: blender
    command: scene.apply-look
    inputs:
      blend-path: '{{ stage.blend-path }}'
      preset:     '{{ inputs.preset }}'

  - id: hero
    agent: blender
    command: render.still
    inputs:
      blend-path:    '{{ look.blend-path }}'
      output-path:   '{{ inputs.output-dir }}/hero.png'
      quality:       '{{ inputs.quality }}'
      direction:     iso
      width-pixels:  1920
      height-pixels: 1080

  - id: turntable
    agent: blender
    command: render.turntable
    inputs:
      blend-path:       '{{ look.blend-path }}'
      output-path:      '{{ inputs.output-dir }}/turntable.mp4'
      duration-seconds: 8
      fps:              30
      width-pixels:     1920
      height-pixels:    1080

  - id: inventory
    agent: blender
    command: scene.info
    inputs:
      blend-path: '{{ look.blend-path }}'

connections:
  - { from: stage, to: look }
  - { from: look,  to: hero }
  - { from: look,  to: turntable }
  - { from: look,  to: inventory }
