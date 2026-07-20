app:           qa-drawings-to-tekla
version:       0.2.0
display-name:  QA Drawings to Tekla
description: |
  Two-trigger DAG: watches a PDF inbox AND an Excel spec sheet. When a drawing
  AND a matching spec arrive (paired by drawing mark), validates them, creates
  the connection in Tekla, and notifies the fab team on Slack.

  Demonstrates the AWARE DAG features: fan-in (drawing + spec → validated job)
  and fan-out (validated job → both Tekla insert AND Slack notify).

  Realistic AECO
  QA workflow shape — software composed from a paragraph.

  NOT RUNNABLE — this is a shape demo, not a working pipeline. It shows the
  DAG topology (fan-in, fan-out) and where the safety contracts on its two external writes belong;
  several of its nodes have no implementation behind them yet:

    - `file.watch`      — declared but `status: planned` (the builtin file
                          agent implements read / write / write-csv, no watch)
    - `excel`           — no agent manifest in the registry at all
    - `think-node`      — no agent manifest in the registry at all
    - `tekla.insert`    — declared in the manifest, but not dispatched by the
                          cli-tekla bridge
    - `./schemas/connection.json` — referenced by `pdf-extract`, not present
                          under `_examples/`
    - `aware-slack`     — the slack agent's `transport.cli.binary`, which is
                          not shipped, bundled, or installable as a sidecar
    - slack write modes — no slack command declares `mode:`, so its writes
                          classify as READ and would execute under `--dry-run`
    - safety enforcement — `safety:` blocks are validated and recorded, but the
                          runtime does not pass them to the transport, so no
                          snapshot or audit stamp is actually performed

  `aware app validate` reports only the `file.watch` error, which understates
  this: it skips unresolved agents and unknown commands entirely, so a clean
  (or nearly clean) validate does NOT mean an app will run. Treat the list
  above as the real prerequisite set.

exposes-as-agent: false      # internal pipeline, not meant to be wrapped

requires:
  - file@1.x
  - excel@1.x
  - think-node@1.x
  - tekla@2025.x
  - slack@1.x

requires-permissions:
  filesystem:
    - read: "{{ config.pdf-inbox }}"
  network:
    - https://slack.com
  software:
    - tekla-structures@2025.x

layout: dag

nodes:
  # ── Triggers (column 1) ──
  - id: file-watch
    agent: file
    command: watch
    row: 1
    col: 1
    config:
      folder:  "{{ config.pdf-inbox }}"
      pattern: "*.pdf"

  - id: excel-watch
    agent: excel
    command: watch-cells
    row: 3
    col: 1
    config:
      sheet:        "{{ config.specs-sheet }}"
      key-column:   "DrawingMark"

  # ── Per-stream preprocessing (column 2) ──
  - id: pdf-extract
    agent: think-node
    command: extract
    row: 1
    col: 2
    config:
      schema: ./schemas/connection.json
      prompt: |
        Read this PDF page. Identify any steel connection callouts.
        Return their type, position (drawing coords), and beam references
        as JSON matching the provided schema.

  - id: excel-lookup
    agent: excel
    command: find-row
    row: 3
    col: 2
    config:
      key:        "{{ excel-watch.changed-key }}"
      key-column: "DrawingMark"

  # ── Fan-in: pair drawing + spec, validate (column 3) ──
  - id: match-build
    agent: think-node      # could be smart-node for deterministic match
    command: pair-and-validate
    row: 2
    col: 3
    inputs:
      drawing: { from: pdf-extract.drawing,   keyed-by: mark }
      spec:    { from: excel-lookup.row,      keyed-by: DrawingMark }
    config:
      tolerance-percent: 5

  # ── Fan-out: validated jobs go to BOTH sinks (column 4) ──
  - id: tekla-insert
    agent: tekla
    command: insert
    row: 1
    col: 4
    config:
      type:     "{{ match-build.connection-type }}"
      position: "{{ match-build.world-position }}"
      beams:    "{{ match-build.beams }}"
    # Mutates the live Tekla model, so the safety contract applies (app-spec §
    # Safety contract). This node IS classified write — but by the legacy exact-name
    # exception `name == "insert"` in manifest::agent::mode_of, not by a manifest
    # `mode:` (it has none) and not by the `.insert` suffix rule (the bare name has no
    # dot). Remove that exception and this gate silently disappears.
    #
    # DECLARED, NOT ENFORCED: the runtime does not pass `safety` to the transport —
    # orchestrator calls invoke_single(agent, command, args), and the block is
    # serialized only into the dry-run `would-write` event, the lock, and validation.
    # So no snapshot is taken and no UDA stamp is written today. It is authored
    # correctly so it starts working when the runtime wires it through.
    safety:
      transaction-group: qa-connection-insert   # rollback boundary for this pipeline
      snapshot: true                            # intent: save the model before writing
      audit-stamp:
        uda-prefix: AWARE_                      # intent: stamp run id / app / operator

  - id: slack-notify
    agent: slack
    command: chat-post-message
    row: 3
    col: 4
    config:
      channel: "#fab-team"
      text:    |
        ✓ Connection {{ match-build.connection-type }} created on {{ match-build.drawing-mark }}.
        Deviation: {{ match-build.deviation-percent }}%.
    # Posting to Slack is an OAuth-protected external write, so the safety contract
    # applies (app-spec § Safety contract). Declared as correct authoring intent — but
    # INERT today, and deliberately not presented as protection: the slack manifest
    # declares no `mode:` on any command, and `chat-post-message` does not match the
    # `*.post` / `*.create` suffix inference, so the runtime classifies this node as
    # READ. A `--dry-run` would therefore really post to #fab-team instead of emitting
    # `would-write`. A node-level `mode: write` cannot fix that from here — it is
    # rejected as E_APP_NODE_MODE_NOT_OVERRIDABLE. The fix belongs in the slack agent.
    safety:
      transaction-group: qa-connection-insert   # same boundary as the Tekla write
      snapshot: false                           # a chat post cannot be snapshotted or undone

connections:
  # Trigger → preprocess
  - from: file-watch
    to:   pdf-extract
    label: PdfFile

  - from: excel-watch
    to:   excel-lookup
    label: RowChange

  # Fan-in at match-build
  - from: pdf-extract
    to:   match-build
    label: Drawing
    input: drawing

  - from: excel-lookup
    to:   match-build
    label: Spec
    input: spec

  # Fan-out from match-build
  - from: match-build
    to:   tekla-insert
    label: ValidatedJob

  - from: match-build
    to:   slack-notify
    label: ValidatedJob

skills:
  - configuring.md
  - tolerance-policy.md
