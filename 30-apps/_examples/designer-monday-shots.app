app:           designer-monday-shots
version:       0.1.0
display-name:  Monday Concept Shots
description: |
  The designer's killer app from the 2026-05-17 persona audit.
  Every Monday at 7am, opens the active Rhino model, captures 3
  named perspective views at consistent settings, posts them to
  the design-review Teams channel.

  Pairs with `enscape-prep` if the designer wants photoreal renders
  alongside the schematic shots (out of scope for this example;
  see enscape-prep agent skills).

exposes-as-agent: false

requires:
  - rhino-8@8.x
  - microsoft-365@1.x

requires-permissions:
  filesystem:
    - read: '{{ inputs.rhino-file }}'
    - write: '{{ inputs.shot-dir }}'
  network:
    - https://graph.microsoft.com

schedule:
  cron: '0 7 * * MON'
  timezone: 'Europe/London'

layout: dag

nodes:
  - id: shot-se
    agent: rhino-8
    command: view.capture
    inputs:
      view-name: 'SE Perspective'
      output-path: '{{ inputs.shot-dir }}/{{ run.date }}-se.png'
      width-pixels: 2560
      height-pixels: 1440
      transparent: false

  - id: shot-nw
    agent: rhino-8
    command: view.capture
    inputs:
      view-name: 'NW Perspective'
      output-path: '{{ inputs.shot-dir }}/{{ run.date }}-nw.png'
      width-pixels: 2560
      height-pixels: 1440

  - id: shot-aerial
    agent: rhino-8
    command: view.capture
    inputs:
      view-name: 'Aerial Bird-eye'
      output-path: '{{ inputs.shot-dir }}/{{ run.date }}-aerial.png'
      width-pixels: 2560
      height-pixels: 1440

  - id: post
    agent: microsoft-365
    command: teams.channel.post-with-screenshot
    inputs:
      team-id: '{{ inputs.teams-team-id }}'
      channel-id: 'design-review'
      text: |
        Monday concept shots — {{ project.name }} {{ run.date }}
      screenshot-path: '{{ shot-se.path }}'
    safety:
      transaction-group: notify
      snapshot: false

connections:
  - { from: shot-se,     to: post }
  - { from: shot-nw,     to: post }
  - { from: shot-aerial, to: post }
