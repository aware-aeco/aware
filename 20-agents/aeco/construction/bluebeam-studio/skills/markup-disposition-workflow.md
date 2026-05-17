# Markup disposition workflow

The architect's bread-and-butter Studio loop:

1. Owner / consultant marks up a tender PDF in a Studio Session
2. Architect reviews each markup, sets disposition (Accepted / Rejected / Cancelled / Completed)
3. At cycle end, Session is archived — flattened PDFs become the record

`bluebeam-studio.markups.pull` + `bluebeam-studio.markups.set-status` automate steps 2 + parts of 3.

## Example: weekly markup-disposition pass

```yaml
nodes:
  - id: list-sessions
    agent: bluebeam-studio
    command: session.list
    inputs:
      status: active

  - id: markups
    agent: bluebeam-studio
    command: markups.pull
    inputs:
      session-id: '{{ list-sessions.sessions[0].id }}'
      since: '{{ last-friday.iso }}'

  - id: report
    agent: html-report
    command: render
    inputs:
      template: 'markup-digest'
      data: '{{ markups.markups }}'
      output-path: '{{ run.tmp-dir }}/markup-digest-{{ run.date }}.html'

  - id: post
    agent: microsoft-365
    command: teams.channel.post-with-card
    inputs:
      team-id: 'Project Acme'
      channel-id: 'arch-team'
      title: 'New markups since {{ last-friday.date }}'
      rows: '{{ markups.markups | groupBy: status }}'
```

## Disposition statuses

| Status | When to set |
|---|---|
| `None` | Default — markup is open, not yet reviewed |
| `Accepted` | "Yes, valid — I'll make the change" |
| `Rejected` | "No, this is wrong / not in scope" |
| `Cancelled` | "Not applicable any more (e.g. drawing superseded)" |
| `Completed` | "Done — change applied in the model/drawing" |

The `checkmark` field on a markup is the user-visible Bluebeam checkmark — separate from disposition. Most workflows ignore it.

## Batching writes

`markups.set-status` accepts an array of markup ids — batch up to 100 per call. Studio's API rate-limits to ~10 writes/sec; the transport stays under that.

## Race condition

If a consultant adds a new markup to a Session while your app is in the middle of a `markups.pull` → `set-status` cycle, the new markup won't be in the batch (no race fence in Studio's API). Use the `since:` parameter on subsequent pulls to catch up incrementally rather than re-pulling the whole Session.

## End-of-cycle archive

```yaml
- id: archive
  agent: bluebeam-studio
  command: session.archive
  inputs:
    session-id: '{{ list-sessions.sessions[0].id }}'
    output-dir: '\\office\projects\acme\record-set\{{ run.date }}\'
  safety:
    transaction-group: cycle-close
    snapshot: false  # external state — Studio's own archive process logs this
```

After `archive` the Session is locked (no new markups). The output-dir contains one flattened PDF per Session file with all annotations baked in.
