# Rulesets and BEP enforcement

Solibri's value is not the engine — it's the **rulesets**. A ruleset
(`.cset`) is a curated bundle of named rules with parameters. Common
patterns:

| Ruleset | What it enforces |
|---|---|
| **BEP Compliance — Client X** | Project-specific naming, parameter, classification rules |
| **IFC 4.0 Compliance** | Schema-level: required Psets, valid GUIDs, no orphan elements |
| **Architectural QA** | Walls clean, openings present, spaces bounded, room areas calculated |
| **Structural QA** | Beam/column continuity, slab edges, foundation extents |
| **MEP Coordination** | Ducts in ceiling void, pipe slopes, equipment access clearances |
| **COBie Compliance** | All required Psets present, classifications complete, contacts populated |

## Workflow

```yaml
nodes:
  - id: open
    agent: solibri
    command: model.open
    inputs:
      path: '\\office\projects\acme\models\acme-arch-v07.ifc'

  - id: bep
    agent: solibri
    command: ruleset.load
    inputs:
      name: 'BEP Compliance — Acme Corp 2026'

  - id: check
    agent: solibri
    command: check.run
    inputs:
      model-id: '{{ open.model-id }}'
      ruleset-id: '{{ bep.ruleset-id }}'
    safety:
      transaction-group: bep-check
      snapshot: false  # check results are non-destructive

  - id: report
    agent: solibri
    command: check.export-bcf
    inputs:
      results-id: '{{ check.results-id }}'
      output-path: '{{ run.tmp-dir }}/bep-{{ run.date }}.bcfzip'
      severity: critical

  - id: post
    agent: microsoft-365
    command: teams.channel.post-with-card
    inputs:
      team-id: 'Project Acme'
      channel-id: 'qa'
      title: 'BEP check — {{ check.summary.critical }} critical, {{ check.summary.moderate }} moderate'
      rows: '{{ check.summary | tabulate }}'
```

## Configuring Solibri for headless use

The Solibri Office REST endpoint listens on `localhost:10876` by
default. The `aware connect solibri` flow asks for an API key — generate
it in Solibri Office under *Settings → REST API*.

For CI / unattended workstations, launch Solibri Office with the
`--rest-headless` flag (Solibri 9.13+); the UI stays minimized.

## What this agent does NOT do

- It does **not** create new rules. Rules are authored in the Solibri UI
  by your BIM authority and packaged into rulesets. This agent
  executes them.
- It does **not** modify the source IFC. All findings are stored in
  Solibri's checking database; reports are BCF / CSV exports.
- It does **not** run "Solibri Anywhere" (the cloud service) — only
  the desktop Solibri Office with REST enabled.

## Performance

A 500 MB federated IFC against a 250-rule BEP runs in 8-15 minutes
on a modern workstation. Schedule via the v0.19 cron primitive overnight
if the run blocks human review time.
