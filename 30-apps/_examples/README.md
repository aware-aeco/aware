# Reference apps

Seven worked examples spanning every persona from the 2026-05-17 audit. Each app file is plain text — read it in Notepad, edit by hand, diff in git, share as an email attachment.

## The seven

| App | Persona | Layout | What it demonstrates |
|---|---|---|---|
| [Welded → TC Uploader](./welded-to-tc.app) | Detailer | linear (3 nodes) | The canonical 60-second-demo. Stateful trigger → inline glue → stateless sink. Idempotency by Mark. `exposes-as-agent: true`. |
| [QA Drawings to Tekla](./qa-drawings-to-tekla.app) | Detailer | DAG (7 nodes, 6 edges) | Two parallel triggers + fan-in + fan-out. Named connection labels. App-level skills. |
| [Monday Model Audit](./bim-monday-audit.app) | **BIM manager** (killer app) | DAG (6 nodes) | v0.19 `for-each` + v0.19 `schedule:` + v0.20 atom references *(does not install — see below)* + v0.11 `safety:` + v0.12 Teams card + Outlook attachment |
| [Monday Concept Shots](./designer-monday-shots.app) | **Designer** (killer app) | DAG (4 nodes) | v0.17 `rhino-8.view.capture` × 3 + v0.12 Teams post-with-screenshot. Replaces the Sunday-night "what did I change" panic. |
| [Cross-project Sheet+RFI Ball-in-Court](./architect-sheet-status.app) | **Architect** (killer app) | DAG (6 nodes) | v0.19 `schedule:` + `for-each` + v0.14 acc-issues + v0.15 bluebeam-studio + v0.20 atom references *(does not install — see below)*. The "$400/month subscription to delete a 4hr/week task" app. |
| [Peer-review Delta (TSD)](./engineer-peer-review-delta.app) | **Engineer** (killer app) | DAG (6 nodes) | v0.19 `snapshot:` + `compare:` + v0.21 engineering envelope (`output-seal:`) + receipt JSON. Replaces 100 engineer-hours/year. |
| [Issue Drawing Pack](./detailer-issue-pack.app) | **Detailer** (killer app) | DAG (9 nodes) | v0.16 Tekla curated verbs + v0.16 peddinghaus-translator + v0.11 `safety:` + v0.12 Teams card. Wednesday-afternoon panic → 1 command. |

## How to run

> **Two of these apps do not install today.** `bim-monday-audit` and
> `architect-sheet-status` each carry an inline predicate written as an
> `atom://` reference. That URI shape is published in
> [`app-spec.md` § Atom references](../../10-core/app-spec.md), but no resolver
> exists in the CLI, so the predicate has no executable body and
> `aware app validate` / `aware app install` refuse it with
> `E_APP_INLINE_NO_BODY`.
>
> That refusal is the fix, not the bug. Until it landed, a body-less predicate
> defaulted to the literal `true` at run time: the gate passed every item
> through and reported `{"pass": true}`, so the Monday audit called every sheet
> "changed in the last 7 days" and the ball-in-court card named every open RFI
> as aging — with nothing in the run log to say the filter had stopped
> filtering. See #554.
>
> Read those two as reference **topology** until `atom://` resolution ships. The
> other seven apps are unaffected.


```bash
# Install
aware app install ./bim-monday-audit.app

# Run (one-shot)
aware app run bim-monday-audit -- \
  --projects-yaml '\\office\projects\active.yaml' \
  --pm-email 'pm@acme.com' \
  --teams-team-id 'Project Acme' \
  --report-dir '\\office\reports'

# Or just install + let the schedule: cron trigger fire
aware app install ./bim-monday-audit.app
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
3. The AI composes the app file using the installed agents + atoms
4. Inspect it via `aware app show <name>`

See [`CONTRIBUTING.md`](../../CONTRIBUTING.md) for how to publish your apps to the community registry.
