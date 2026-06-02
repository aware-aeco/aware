app:           architect-sheet-status
version:       0.1.0
display-name:  Cross-project Sheet + RFI Ball-in-Court
description: |
  The architect's killer app from the 2026-05-17 persona audit:
  "$400/month subscription to delete a 4hr/week task."

  Every Monday 7am, walks all active Revit projects + their ACC Issues
  registers + their Bluebeam Studio Sessions, produces a single
  ball-in-court Teams card answering the principal's standing question:
  "What's the state of all live projects, who's holding what up?"

exposes-as-agent: false

requires:
  - revit-2026@2026.x
  - acc-issues@1.x
  - bluebeam-studio@1.x
  - microsoft-365@1.x

requires-permissions:
  filesystem:
    - read: '{{ inputs.projects-yaml }}'
  network:
    - https://developer.api.autodesk.com
    - https://studioapi.bluebeam.com
    - https://graph.microsoft.com

schedule:
  cron: '0 7 * * MON'
  timezone: 'America/New_York'

layout: dag

nodes:
  - id: projects
    agent: aware-runtime
    command: read-yaml
    inputs:
      path: '{{ inputs.projects-yaml }}'

  - id: each-project
    for-each: '{{ projects.items }}'
    do:
      - id: sheets
        agent: revit-2026
        command: sheet.list
        inputs:
          filter-issued-for: 'Issued for Construction'

      - id: rfis
        agent: acc-issues
        command: list-issues
        inputs:
          project-id: '{{ item.acc-project-id }}'
          status: 'open'

      - id: markups
        agent: bluebeam-studio
        command: session.list
        inputs:
          status: active

      - id: aging-rfis
        inline:
          kind: predicate
          description: RFIs open > 5 days
          atom: 'atom://generic/at-least'
          inputs:
            value: '{{ rfis.issues[*].age-days }}'
            threshold: 5

  - id: card
    agent: microsoft-365
    command: teams.channel.post-with-card
    inputs:
      team-id: '{{ inputs.teams-team-id }}'
      channel-id: '{{ inputs.coordination-channel }}'
      title: 'Cross-project ball-in-court — {{ run.date }}'
      rows: '{{ each-project.outputs }}'
    safety:
      transaction-group: notify
      snapshot: false

connections:
  - { from: projects,     to: each-project }
  - { from: each-project, to: card }
