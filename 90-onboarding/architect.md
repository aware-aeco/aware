# First hour — Architect

Goal: cross-project sheet + RFI ball-in-court Teams card every Monday.

## What you need

- Revit 2026 (multiple active projects — that's the point)
- Autodesk Construction Cloud (ACC Issues)
- Microsoft 365 (Teams + Outlook)
- Bluebeam Studio (optional — for markup tracking)
- 30 minutes

## 0-10 min: install AWARE

```powershell
npm install -g @aware-aeco/cli
aware --version

aware agent install aware-aeco
```

## 10-25 min: connect integrations

```powershell
aware connect microsoft-365 --device-code

# ACC — Autodesk's OAuth flow:
aware connect acc --device-code

# Bluebeam Studio (optional):
aware connect bluebeam-studio --device-code
```

## 25-40 min: catalogue your active projects

Create `~/projects/active.yaml`:

```yaml
items:
  - id: acme-tower
    name: 'Acme Tower'
    rvt: '\\office\projects\acme-tower\central.rvt'
    acc-project-id: 'b.acme-tower-123'
    bluebeam-session-id: ''   # optional
  - id: acme-school
    name: 'Acme School'
    rvt: '\\office\projects\acme-school\central.rvt'
    acc-project-id: 'b.acme-school-456'
```

This is the source-of-truth for "which projects am I PM on." The reference app walks all of them on every run.

## 40-55 min: install the reference app

```powershell
aware app install https://github.com/aware-aeco/aware/raw/main/30-apps/_examples/architect-sheet-status.flo
```

Edit inputs at top:

```yaml
inputs:
  projects-yaml: 'C:\Users\you\projects\active.yaml'
  teams-team-id: 'Architecture Studio'
  coordination-channel: 'principals-rollup'
```

## 55-60 min: dry-run + go

```powershell
aware app explain architect-sheet-status
aware app run architect-sheet-status --dry-run
aware app run architect-sheet-status
```

The card lands in your `principals-rollup` Teams channel:

> **Cross-project ball-in-court — 2026-05-17**
> | Project | Open sheets at Rev C | RFIs > 5 days | Markup sessions |
> | --- | --- | --- | --- |
> | Acme Tower | 4 | 2 | 1 active |
> | Acme School | 7 | 0 | none |

## What you've achieved

- The "$400/month subscription to delete a 4hr/week task" from the audit
- Principal asks "what's the state of all live projects" → answer is on screen
- No more weekly Smartsheet copy-paste

## Next

- Add ACC Docs file listing: `acc-docs.files.list since:last-friday` to find sheets touched this week
- Add the Bluebeam markup-disposition workflow: `bluebeam-studio.markups.pull` + `set-status`
- For tender issues, look at the architect's other reference app pattern (`sheet.export-pdfs` + `sheet.stamp` + `acc-docs.transmittals.create`) — write a `tender-issue.flo` and let the AI compose it for you
