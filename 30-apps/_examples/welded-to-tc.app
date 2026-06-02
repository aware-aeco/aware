app:           welded-to-tc
version:       0.3.1
display-name:  Welded → TC Uploader
description: |
  Watches the active Tekla model for new welded assemblies. When one is
  detected, fetches its drawing and uploads to a configured Trimble Connect
  folder, idempotent by drawing Mark.

  The canonical AWARE app — three nodes, linear topology, real AECO tools.
  This is the example referenced in the manifesto's 60-second demo.

# This app exposes itself as an agent so other apps can install and use it.
exposes-as-agent: true
exposed-commands:
  start:
    lifecycle: start
    inputs:
      tc-project-id:
        type: string
        description: Trimble Connect project UUID.
      tc-folder-id:
        type: string
        description: Destination folder UUID.
    outputs:
      type: stream
      schema:
        mark:    string
        file-id: string
        url:     string
        replaced: bool

# Pinned agent versions. Minor pinning recommended.
requires:
  - tekla@2025.x
  - trimble-connect@2.x

requires-permissions:
  network:
    - https://app.connect.trimble.com
  software:
    - tekla-structures@2025.x

layout: linear

nodes:
  - id: tekla-watch
    agent: tekla
    command: watch
    config:
      filter: welded                       # see tekla agent's watch command

  - id: filter-welded
    inline:
      kind: predicate
      description: Drop anything that isn't an Assembly with type=Welded.
      code: |
        e => e.type == "Welded" && e.mark != null

  - id: tc-upload
    agent: trimble-connect
    command: upload
    config:
      project-id: "{{ inputs.tc-project-id }}"
      folder-id:  "{{ inputs.tc-folder-id }}"
      filename:   "{{ tekla-watch.mark }}.pdf"
      bytes:      "{{ tekla-watch.drawing-bytes }}"
      properties:
        mark:   "{{ tekla-watch.mark }}"   # see trimble-connect/skills/idempotency.md
        source: "aware-fab-pipeline"

connections:
  - from: tekla-watch
    to:   filter-welded
    label: AssemblyEvent

  - from: filter-welded
    to:   tc-upload
    label: "AssemblyEvent (welded)"

# App-level skills — knowledge specific to operating this app.
skills:
  - configuring.md
  - troubleshooting.md
