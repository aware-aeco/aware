app:           engineer-peer-review-delta
version:       0.1.0
display-name:  Peer-review Delta (TSD)
description: |
  The structural engineer's killer app from the 2026-05-17 persona
  audit: "100 engineer-hours/year reclaimed."

  At each peer-review checkpoint, compares the current TSD model
  against the prior snapshot and produces a one-page delta report:
    - every member where section changed, grade changed, length changed
      > 5%, end fixity changed, applied load changed > 10%, or
      utilisation crossed the 0.85 threshold (either direction)
    - load combinations that gained/lost a loadcase factor
    - heatmap of gridlines with the most churn

  Demonstrates the v0.21 engineering envelope — every peer-review pin
  is recorded in the receipt.

exposes-as-agent: false

requires:
  - tsd-26@26.x
  - html-report@1.x
  - microsoft-365@1.x

requires-permissions:
  filesystem:
    - read: '{{ inputs.tsd-file }}'
    - write: '{{ inputs.report-dir }}'
    - read: '~/.aware/snapshots/'
    - write: '~/.aware/snapshots/'
  network:
    - https://graph.microsoft.com

engineering:
  pins:
    code-of-practice:    'eurocode-3@2022+uk-na'
    section-catalogue:   'en-10365@2017'
    material-catalogue:  'en-10025-2@2019'
    psi-factors:         'en-1990@2002+uk-na-2002'
    solver-build:        'tsd-26.0.3-build-19834'
  output-seal:
    artifact:    '{{ delta-report.path }}'
    operator:    '{{ inputs.engineer-id }}'
    credential:  '{{ secrets.ceng-seal }}'

layout: dag

nodes:
  - id: today
    agent: tsd-26
    command: model.open
    inputs:
      path: '{{ inputs.tsd-file }}'

  - id: today-bom
    agent: tsd-26
    command: model.extract-member-list

  - id: previous
    snapshot:
      of:
        agent: tsd-26
        target: '{{ inputs.tsd-file }}'
      name: 'peer-review-{{ inputs.previous-checkpoint }}'

  - id: delta
    compare:
      a-snapshot: '{{ previous.name }}'
      b: '{{ today-bom.members }}'
      by: member-id
      track: [profile, grade, length-mm, end-fixity, applied-load-kN, utilisation]

  - id: delta-report
    agent: html-report
    command: render
    inputs:
      template: 'engineer-peer-review-delta'
      data:
        added:    '{{ delta.added }}'
        removed:  '{{ delta.removed }}'
        changed:  '{{ delta.changed }}'
        pins:     '{{ engineering.pins }}'
      output-path: '{{ inputs.report-dir }}/peer-review-delta-{{ run.date }}.html'

  - id: notify
    agent: microsoft-365
    command: outlook.mail.send-with-attachment
    inputs:
      to: ['{{ inputs.peer-reviewer }}']
      subject: 'Peer-review delta — {{ inputs.project }} — {{ run.date }}'
      content-type: html
      body: |
        Peer-review delta attached. Receipt JSON also enclosed.
        {{ delta.changed.length }} members changed; {{ delta.added.length }} added; {{ delta.removed.length }} removed.
      attachments:
        - path: '{{ delta-report.output-path }}'
          filename: 'peer-review-delta-{{ run.date }}.html'
        - path: '{{ delta-report.output-path | replace: ".html", ".aware-receipt.json" }}'
          filename: 'peer-review-receipt-{{ run.date }}.json'
    safety:
      transaction-group: notify
      snapshot: false

connections:
  - { from: today,        to: today-bom }
  - { from: previous,     to: delta }
  - { from: today-bom,    to: delta }
  - { from: delta,        to: delta-report }
  - { from: delta-report, to: notify }
