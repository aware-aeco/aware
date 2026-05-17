# Publisher Set workflow

ArchiCAD's Publisher organises every deliverable artefact (PDF sheets,
DWGs, IFC exports, BIMx packages, model previews) into named *sets*.
The architect publishes a set to issue.

## Why this is the right verb

In Revit, the equivalent is "print to PDF" + "export DWG" + sheet-by-
sheet pain. ArchiCAD's Publisher does the configuration once + reruns
on every cycle. This agent's `publish.layout-set` triggers that
configured rerun — the architect-side audit's "issue tender / IFC /
record" verb.

## Setting it up (one-time, in ArchiCAD UI)

1. *Document → Publisher → Publishing Sets*
2. New set → name it (e.g. "Tender Set 2026-05")
3. Drag in layouts from the Layout Book
4. Configure output: PDF / DWG / DWF / BIMx / IFC
5. Set output destination (local folder + naming template)
6. Save

After that, this agent triggers reruns:

```yaml
- id: tender
  agent: archicad-29
  command: publish.layout-set
  inputs:
    publisher-set-name: 'Tender Set 2026-05'
    output-dir: '\\office\projects\acme\tender\{{ run.date }}'
    format: pdf
  safety:
    transaction-group: tender-issue
    snapshot: false
```

## Pairing with downstream agents

The output PDFs feed straight into:

- `acc-docs.sheets.upload-set` — push to the Autodesk CDE
- `bluebeam-studio.session.create` — start a markup cycle
- `aconex.documents.upload` — formal UK document control
- `outlook.mail.send-with-attachment` — informal cc to client

A full Friday-issue app composes them:

```yaml
nodes:
  - id: publish
    agent: archicad-29
    command: publish.layout-set
    inputs:
      publisher-set-name: 'Tender Set'
      output-dir: '{{ run.tmp-dir }}/tender'

  - id: bluebeam
    agent: bluebeam-studio
    command: session.create
    inputs:
      name: 'Tender review — {{ run.date }}'
      pdf-paths: '{{ publish.written[*].path }}'
      invitees: '{{ secrets.consultant-emails }}'

  - id: notify
    agent: microsoft-365
    command: outlook.mail.send
    inputs:
      to: '{{ secrets.client-emails }}'
      subject: 'Tender set issued'
      body: |
        Studio Session: {{ bluebeam.session-id }} — please mark up
        within 5 working days.
```

## Output naming

ArchiCAD's Publisher honours the naming template you set in the UI
(e.g. `{Project}_{Phase}_{Sheet}_{Rev}.pdf`). This agent doesn't
override it — set the template once in ArchiCAD; the agent reruns the
configured set.

If you need *programmatic* per-run naming (different prefix per app
run), use the v0.20 `pdf.rename-batch` atom as a post-step.
