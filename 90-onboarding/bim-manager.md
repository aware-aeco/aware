# First hour — BIM manager

Goal: a Monday model audit running by next Monday 7am, no dev help.

## What you need

- Revit 2026 (any active project)
- Microsoft 365 (Teams + Outlook)
- 30 minutes

## 0-10 min: install AWARE

```powershell
# Windows — recommended (no SmartScreen warning):
npm install -g @aware-aeco/cli

aware --version
# aware 0.22.0
```

(macOS/Linux: same `npm` command. Or `curl -fsSL https://raw.githubusercontent.com/aware-aeco/aware/main/scripts/install.sh | bash`.)

Install the BIM-manager bundle:

```powershell
aware agent install aware-aeco
```

That gives you 56+ agents including Revit 2026, ACC Issues / Docs, Microsoft 365, Navisworks 2026, Solibri, BCF, IFC inspector.

## 10-25 min: connect your integrations

```powershell
# M365 — opens the browser. If your tenant restricts cross-tenant identity:
aware connect microsoft-365 --device-code --tenant acme.onmicrosoft.com

# ACC — opens the Autodesk OAuth flow:
aware connect acc --device-code
```

For the Revit agent, no `aware connect` needed — the transport binary talks to Revit locally via its API.

## 25-40 min: install the reference app

```powershell
aware app install https://github.com/aware-aeco/aware/raw/main/30-apps/_examples/bim-monday-audit.flo
```

Inspect what it'll do:

```powershell
aware app explain bim-monday-audit
```

You'll see: which agents are called, what reads happen, what writes happen, what `safety:` blocks are required, what permissions get prompted.

## 40-55 min: customise + dry-run

Edit `~/.aware/apps/bim-monday-audit/bim-monday-audit.flo` and set the inputs at the top:

```yaml
inputs:
  projects-yaml: '\\office\projects\active.yaml'
  pm-email: 'pm@acme.com'
  teams-team-id: 'Project Acme'
  report-dir: '\\office\reports\monday'
```

Create the `active.yaml`:

```yaml
items:
  - id: acme-tower
    rvt: '\\office\projects\acme-tower\central.rvt'
    acc-project-id: 'b.abc-...'
  - id: acme-school
    rvt: '\\office\projects\acme-school\central.rvt'
    acc-project-id: 'b.def-...'
```

Dry-run:

```powershell
aware app run bim-monday-audit --dry-run
```

The `--dry-run` flag walks the full DAG without committing any writes. Read-mode nodes still execute (so the report renders); write-mode nodes emit `would-write:` events you can inspect with `aware app logs bim-monday-audit --replay`.

## 55-60 min: schedule it

The reference app already has:

```yaml
schedule:
  cron: '0 7 * * MON'
  timezone: 'Europe/London'
```

So once you've installed it, it runs Monday 7am automatically. Trigger an ad-hoc one with `aware app run bim-monday-audit` to verify.

## What you've achieved

- Every Monday 7am: walk every active Revit project, produce a model-health rollup, post it to Teams + email the PM
- Replaces the Naviate Audit Panel + manual Dynamo scripts the audit named
- Saves ~4 hours/week of Monday triage

## Next

- Add Navisworks clash digest by wiring in `navisworks-2026.clash.run-all` + `clash.export --format bcf` + `acc-docs.transmittals.create`
- Pair with [Bluebeam Studio markups](../30-apps/_examples/architect-sheet-status.flo) for redline tracking
- Compose your own — describe the workflow in plain English to Claude Code or codex while `aware-aeco` is installed
