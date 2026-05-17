# Reference apps

Seven worked examples spanning every persona from the 2026-05-17 audit. Each `.flo` file is plain text — read it in Notepad, edit by hand, diff in git, share as an email attachment.

## The seven

| App | Persona | Layout | What it demonstrates |
|---|---|---|---|
| [Welded → TC Uploader](./welded-to-tc.flo) | Detailer | linear (3 nodes) | The canonical 60-second-demo. Stateful trigger → inline glue → stateless sink. Idempotency by Mark. `exposes-as-agent: true`. |
| [QA Drawings to Tekla](./qa-drawings-to-tekla.flo) | Detailer | DAG (7 nodes, 6 edges) | Two parallel triggers + fan-in + fan-out. Named connection labels. App-level skills. |
| [Monday Model Audit](./bim-monday-audit.flo) | **BIM manager** (killer app) | DAG (6 nodes) | v0.19 `for-each` + v0.19 `schedule:` + v0.20 atom references + v0.11 `safety:` + v0.12 Teams card + Outlook attachment |
| [Monday Concept Shots](./designer-monday-shots.flo) | **Designer** (killer app) | DAG (4 nodes) | v0.17 `rhino-8.view.capture` × 3 + v0.12 Teams post-with-screenshot. Replaces the Sunday-night "what did I change" panic. |
| [Cross-project Sheet+RFI Ball-in-Court](./architect-sheet-status.flo) | **Architect** (killer app) | DAG (6 nodes) | v0.19 `schedule:` + `for-each` + v0.14 acc-issues + v0.15 bluebeam-studio. The "$400/month subscription to delete a 4hr/week task" app. |
| [Peer-review Delta (TSD)](./engineer-peer-review-delta.flo) | **Engineer** (killer app) | DAG (6 nodes) | v0.19 `snapshot:` + `compare:` + v0.21 engineering envelope (`output-seal:`) + receipt JSON. Replaces 100 engineer-hours/year. |
| [Issue Drawing Pack](./detailer-issue-pack.flo) | **Detailer** (killer app) | DAG (9 nodes) | v0.16 Tekla curated verbs + v0.16 peddinghaus-translator + v0.11 `safety:` + v0.12 Teams card. Wednesday-afternoon panic → 1 command. |

## How to run

```bash
# Install
aware app install ./bim-monday-audit.flo

# Run (one-shot)
aware app run bim-monday-audit -- \
  --projects-yaml '\\office\projects\active.yaml' \
  --pm-email 'pm@acme.com' \
  --teams-team-id 'Project Acme' \
  --report-dir '\\office\reports'

# Or just install + let the schedule: cron trigger fire
aware app install ./bim-monday-audit.flo
# (Monday 7am: app runs automatically)
```

Pre-flight a write run with `--dry-run` to preview side effects:

```bash
aware app run detailer-issue-pack --dry-run -- \
  --drawing-marks '["A-100","A-101","A-200"]' \
  --revision 'E' \
  --phase '2' \
  --output-dir 'C:\fab\issued-rev-E'
```

`aware app explain detailer-issue-pack` prints reads / writes / external posts / required permissions in one screen.

## Why these five killer apps

From the 2026-05-17 persona audit:

> Each persona named one specific "next-week app" they would ship if their top gaps closed. These are the **adoption wedges** for each professional segment. All five killer apps are read-mostly; none requires write-back to a live model; all five were blocked today by missing curated workflow verbs + missing comms primitives.

The first six audit phases (v0.10 → v0.16) closed those gaps. v0.17–v0.21 made the substrate robust enough that the killer apps compose cleanly. v0.22 ships them.

## Authoring your own

1. Open Claude Code (or codex, opencode) with `aware-aeco` installed
2. Describe what you want in plain English
3. The AI composes the `.flo` file using the installed agents + atoms
4. Inspect it visually with FloLess (the commercial canvas) when it ships, or via `aware app show <name>`

See [`CONTRIBUTING.md`](../../CONTRIBUTING.md) for how to publish your apps to the community registry.
