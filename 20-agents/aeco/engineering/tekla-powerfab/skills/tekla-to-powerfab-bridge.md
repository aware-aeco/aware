# The Tekla → EPM bridge

The detailer's #1 daily ritual: a fresh Tekla model save triggers a fresh BOM that should flow into EPM to update procurement + production planning. Until now this was a 30-minute manual export-CSV / import-CSV dance per cycle.

## The wedge

```yaml
app: tekla-to-epm-sync
version: 0.1.0
description: |
  Watch Tekla saves on the active model; for each save, export the BOM
  CSV and re-import to the active EPM job. Notify the yard manager if
  any new section profiles appear (they may need to be ordered).

requires:
  - tekla@2026.x
  - tekla-powerfab@24.2.x
  - microsoft-365@1.x

nodes:
  - id: tekla-watch
    agent: tekla
    command: watch
    inputs:
      filter: all
      events: [save]

  - id: bom-export
    agent: tekla
    command: report-create        # promoted to curated in v0.16
    inputs:
      template: 'aware-fabricator-bom'
      output-path: '{{ run.tmp-dir }}/{{ tekla-watch.event-id }}.csv'

  - id: bom-import
    agent: tekla-powerfab
    command: bom.import-tekla
    inputs:
      job-id: '{{ secrets.active-epm-job-id }}'
      tekla-bom-csv: '{{ bom-export.path }}'
    safety:
      transaction-group: tekla-to-epm-sync
      snapshot: false  # EPM has its own change-log

  - id: new-profiles
    inline:
      kind: predicate
      description: Were any imported profiles previously unknown?
      code: |
        result => result.warnings.some(w => w.startsWith("new-profile:"))

  - id: notify-yard
    agent: microsoft-365
    command: teams.channel.post-message
    inputs:
      team-id: 'Project Acme'
      channel-id: 'fab-yard'
      text: |
        New profile(s) in BOM — please confirm stock:
        {{ bom-import.warnings | join: ', ' }}
    safety:
      transaction-group: notify
      snapshot: false
```

## BOM CSV column convention

The `bom.import-tekla` verb expects the Tekla-side CSV to have these
columns (the EPM importer ignores extras):

```
PieceMark, Profile, Grade, Length_mm, Quantity, Weight_kg
```

Tekla's default `aware-fabricator-bom` report template (ships with the
v0.16 Tekla curated update) emits exactly these columns.

## Mapping mismatches

EPM has a stricter profile catalogue than Tekla. If Tekla emits `HEA300`
but EPM only knows `HEA 300` (with space), the import logs a
`profile-mismatch:HEA300:HEA 300` warning and uses the closest match.
Configure the canonical mapping in EPM's *Settings → Profile aliases*.

## Idempotency

`bom.import-tekla` is idempotent within an EPM job — re-importing the
same CSV updates piece counts rather than duplicating rows. Safe to
re-run the watcher app on every save.
