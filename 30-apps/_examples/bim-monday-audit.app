app:           bim-monday-audit
version:       0.1.0
display-name:  Monday Model Audit
description: |
  The BIM manager's killer app from the 2026-05-17 persona audit.
  Every Monday at 7am, walks the active Revit project(s), produces a
  model-health rollup, posts it to Teams + emails the PM.

  Per-model checks:
    - file size
    - last-saved-by / last-saved-when
    - warnings count
    - unplaced rooms count
    - sheets count
    - sheets-changed-this-week count

  Cross-model checks:
    - shared coordinates baseline match
    - level / grid naming match
    - federation alignment delta

  Output: Teams card + Excel attachment + PDF to design-lead's Outlook.

  Read-only against the model — no `safety:` blocks needed.

exposes-as-agent: false

requires:
  - revit-2026@2026.x
  - microsoft-365@1.x
  - html-report@1.x

requires-permissions:
  filesystem:
    - read: '{{ inputs.projects-yaml }}'
    - write: '{{ inputs.report-dir }}'
  network:
    - https://graph.microsoft.com

schedule:
  cron: '0 7 * * MON'
  timezone: 'Europe/London'

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
      - id: open
        agent: revit-2026
        command: link.reload-all
        inputs:
          include-cad-links: true
        safety:
          transaction-group: monday-audit
          snapshot: true
          worksharing:
            check: true
            fail-if-other-user: true
          audit-stamp:
            uda-prefix: AWARE_

      - id: sheets
        agent: revit-2026
        command: sheet.list

      - id: stale-sheets
        inline:
          kind: predicate
          description: Sheets changed in the last 7 days
          atom: 'atom://generic/is-newer-than'
          inputs:
            item: '{{ sheets.sheets }}'
            threshold: '{{ last-week.iso }}'

      - id: missing-fire
        agent: revit-2026
        command: schedule.find-rows-missing
        inputs:
          schedule-name: 'Door Schedule'
          parameters: ['Fire Rating', 'Acoustic']

  - id: rollup
    agent: html-report
    command: render
    inputs:
      template: 'monday-audit'
      data:
        projects: '{{ each-project.outputs }}'
      output-path: '{{ inputs.report-dir }}/monday-audit-{{ run.date }}.html'

  - id: teams-post
    agent: microsoft-365
    command: teams.channel.post-with-card
    inputs:
      team-id: '{{ inputs.teams-team-id }}'
      channel-id: 'coordination'
      title: 'Monday model audit — {{ run.date }}'
      rows: '{{ each-project.outputs }}'
    safety:
      transaction-group: notify
      snapshot: false

  - id: email-pm
    agent: microsoft-365
    command: outlook.mail.send-with-attachment
    inputs:
      to: ['{{ inputs.pm-email }}']
      subject: 'Monday model audit — {{ run.date }}'
      content-type: html
      body: |
        Weekly model-audit attached. Highlights:
          - {{ stale-sheets.length }} sheets changed in the last 7 days
          - {{ missing-fire.missing.length }} door rows missing fire rating
      attachments:
        - path: '{{ rollup.output-path }}'
          filename: 'monday-audit-{{ run.date }}.html'
    safety:
      transaction-group: notify
      snapshot: false

connections:
  - { from: projects,      to: each-project }
  - { from: each-project,  to: rollup }
  - { from: rollup,        to: teams-post }
  - { from: rollup,        to: email-pm }
