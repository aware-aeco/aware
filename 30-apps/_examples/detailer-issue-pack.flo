app:           detailer-issue-pack
version:       0.1.0
display-name:  Issue Drawing Pack
description: |
  The detailer's killer app from the 2026-05-17 persona audit:
  "Wednesday-afternoon panic → 1 command."

  Given a list of drawing marks + a revision letter, issues the full
  pack:
    - sets IsReadyForIssue=true + stamps revision
    - exports PDF + DWG + NC + bolt list
    - groups NC by machine (saw / drill / plasma)
    - uploads to Trimble Connect at the project's naming convention
    - posts a single Slack/Teams summary with deltas vs prior rev

  Demonstrates the v0.16 Tekla curated workflow verbs, v0.16
  peddinghaus-translator, v0.11 safety contract, v0.20 named atoms.

exposes-as-agent: false

requires:
  - tekla@2025.x
  - peddinghaus-translator@1.x
  - trimble-connect@2.x
  - microsoft-365@1.x

requires-permissions:
  filesystem:
    - write: '{{ inputs.output-dir }}'
  network:
    - https://app.connect.trimble.com
    - https://graph.microsoft.com

layout: dag

nodes:
  - id: issue
    agent: tekla
    command: drawing-issue
    inputs:
      marks: '{{ inputs.drawing-marks }}'
      revision: '{{ inputs.revision }}'
      issued-for: 'Issued for Construction'
      issued-by: '{{ inputs.detailer-id }}'
    safety:
      transaction-group: issue-pack
      snapshot: true
      worksharing:
        check: true
        fail-if-other-user: true
      audit-stamp:
        uda-prefix: AWARE_

  - id: export-pdfs
    agent: tekla
    command: drawing-export
    inputs:
      marks: '{{ inputs.drawing-marks }}'
      formats: [pdf, dwg]
      output-dir: '{{ inputs.output-dir }}/drawings'
      naming-template: '{project}-{phase}-{mark}-Rev{rev}.{ext}'
    safety:
      transaction-group: issue-pack
      snapshot: false

  - id: nc-export
    agent: tekla
    command: nc-export-phase
    inputs:
      phase: '{{ inputs.phase }}'
      output-dir: '{{ inputs.output-dir }}/nc-raw'
      kerf-mm: 3.0
    safety:
      transaction-group: issue-pack
      snapshot: false

  - id: bucket
    agent: peddinghaus-translator
    command: group-by-machine
    inputs:
      input-dir: '{{ nc-export.output-dir }}'
      output-dir: '{{ inputs.output-dir }}/nc-shop'
      machines: [saw, drill, plasma]
    safety:
      transaction-group: issue-pack
      snapshot: false

  - id: bolts
    agent: tekla
    command: bolt-list
    inputs:
      scope: phase
      phase: '{{ inputs.phase }}'
      output-path: '{{ inputs.output-dir }}/bolt-list-Rev{{ inputs.revision }}.csv'

  - id: upload
    agent: trimble-connect
    command: upload
    inputs:
      project-id: '{{ inputs.tc-project-id }}'
      folder-id: '{{ inputs.tc-folder-id }}'
      directory: '{{ inputs.output-dir }}'
    safety:
      transaction-group: issue-pack
      snapshot: false

  - id: notify
    agent: microsoft-365
    command: teams.channel.post-with-card
    inputs:
      team-id: '{{ inputs.teams-team-id }}'
      channel-id: 'shop-floor'
      title: 'Phase {{ inputs.phase }} Rev {{ inputs.revision }} issued — {{ run.date }}'
      rows:
        - drawings: '{{ export-pdfs.written.length }}'
          nc-files: '{{ bucket.manifest-csv | rowCount }}'
          bolts:    '{{ bolts.total-bolts }}'
    safety:
      transaction-group: notify
      snapshot: false

connections:
  - { from: issue,       to: export-pdfs }
  - { from: issue,       to: nc-export }
  - { from: nc-export,   to: bucket }
  - { from: issue,       to: bolts }
  - { from: export-pdfs, to: upload }
  - { from: bucket,      to: upload }
  - { from: bolts,       to: upload }
  - { from: upload,      to: notify }
