# First hour — Detailer

Goal: Wednesday-afternoon issue panic → one command. Pack ships Friday by itself.

## What you need

- Tekla Structures 2025 or 2026
- Trimble Connect (for the issued pack target)
- Microsoft 365 / Slack (for the notify)
- A Peddinghaus saw/drill line if you want NC routing (optional)
- 30 minutes

## 0-10 min: install AWARE

```powershell
npm install -g @aware-aeco/cli
aware --version

aware agent install aware-aeco
```

Includes the curated `tekla` agent (20 workflow verbs covering issue-pack / NC / bolt-list / drawing-export / numbering-run / IFC / clash-check / UDA ops + 3,179 reflected raw API calls as escape hatch), plus `peddinghaus-translator` (DSTV ↔ Peddimat) and `trimble-connect`.

## 10-25 min: connect Trimble Connect + Teams

```powershell
# Trimble — opens browser to id.trimble.com:
aware connect trimble-connect --device-code

# Teams (or Slack):
aware connect microsoft-365 --device-code
```

For Tekla itself: no `aware connect` needed — the transport opens Tekla via its built-in API.

## 25-40 min: discover your project's identifiers

```powershell
# List your Trimble Connect projects:
aware app run --inline -- "
  agent: trimble-connect
  command: project.list
"
```

Note the `project-id` + the `folder-id` of your fab-drawings folder.

In Tekla, open *Numbering → Modified Objects* once to make sure the model is up-to-date. (Future v0.16.x will let you trigger this from `aware tekla numbering-run` directly.)

## 40-55 min: install + customise the reference app

```powershell
aware app install https://github.com/aware-aeco/aware/raw/main/30-apps/_examples/detailer-issue-pack.flo
```

Edit inputs at top:

```yaml
inputs:
  drawing-marks: ['A-100','A-101','A-200','A-201']  # or pull dynamically from tekla.drawing-list
  revision: 'E'
  phase: '2'
  output-dir: 'C:\fab\acme\issued-rev-E'
  tc-project-id: '...'
  tc-folder-id: '...'
  detailer-id: 'P.Lisowski'
  teams-team-id: 'Acme Fab'
```

## 55-60 min: dry-run + ship

**Always dry-run first** for write-mode apps:

```powershell
aware app run detailer-issue-pack --dry-run
```

The trace will show: `would-write: tekla.drawing-issue (47 drawings)`, `would-write: tekla.drawing-export (47 PDFs + 47 DWGs)`, `would-write: tekla.nc-export-phase`, etc. Inspect with:

```powershell
aware app logs detailer-issue-pack --replay
```

Looks right? Drop the `--dry-run`:

```powershell
aware app run detailer-issue-pack
```

Output:

```
✓ Phase 2 Rev E issued
  → 47 drawings (PDF + DWG)
  → 1,847 NC files routed to saw/drill/plasma queues
  → 14,200-bolt schedule
  → uploaded to Trimble Connect: tc:acme/issued-rev-E/
  → posted to Teams: shop-floor channel
  trace: C:\Users\you\.aware\logs\detailer-issue-pack\default\<run-id>.jsonl
```

## What you've achieved

- Wednesday-afternoon issue ritual → 1 command
- Saw line + drill line + plasma line + paint shop all get their files at the right naming + dialect
- Audit trail: every Tekla model object touched is UDA-stamped with `AWARE_RUN_ID`, `AWARE_APP`, `AWARE_OPERATOR`
- Trimble Connect upload is idempotent by mark — re-runs don't duplicate

## Next

- Wire Tekla EPM (PowerFab): install `tekla-powerfab`, register Trimble Identity OAuth, push BOM after issue
- Erection-schedule export: add a `tekla.uda-get` for `Lift_Week` then a coloured-isometric PDF via the html-report agent
- Per-cycle Slack notify with deltas vs prior rev: add a `compare:` primitive (v0.19) between this run's BoM and the last
