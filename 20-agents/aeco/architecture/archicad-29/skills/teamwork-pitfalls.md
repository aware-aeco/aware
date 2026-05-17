# Teamwork — the pitfalls AWARE can't automate around

ArchiCAD Teamwork is the worksharing model — multiple users edit a single
BIMcloud-hosted project, each reserving elements. This is the equivalent
of Revit Central, more sophisticated in mechanism (reservation vs check-out).

## What the agent CAN do

- `teamwork.status` — report connection state + reserved count
- `teamwork.send-receive` — trigger sync
- `elements.set-properties` works correctly **only on elements you've
  reserved**. The API refuses writes on other-user elements.

## What the agent CAN'T do (yet)

- **Reserve specific elements before write.** The Graphisoft API
  doesn't expose programmatic reserve/release (UI-only). Tapir is
  slightly better — has `Teamwork.Reserve` — but the contract is
  unstable across versions.
- **Detect mid-flight conflicts.** If two users reserve the same
  element seconds apart, the second's request fails with a vague
  "permission denied" — recovery requires UI inspection.
- **Roll back a partial Teamwork write.** If `elements.set-properties`
  fails on element 47 of 100 (other user holds it), elements 1-46
  are already committed. There's no transactional boundary in
  Teamwork at the API level.

## Implication for AWARE apps

**Don't run bulk Teamwork writes unattended.** Schedule them
during off-hours when other users won't conflict, and inspect the
output `warnings` array before considering the run complete:

```yaml
- id: bulk
  agent: archicad-29
  command: elements.set-properties
  inputs:
    guids: '{{ candidates.guids }}'
    properties:
      'Fire Rating': 'FD60'
  safety:
    transaction-group: bulk-fire-rating
    snapshot: true
    worksharing:
      check: true
      fail-if-other-user: true   # archicad-29 transport respects this
    audit-stamp:
      uda-prefix: AWARE_

- id: gate
  inline:
    kind: predicate
    description: Refuse to proceed if any write was warning-flagged
    code: |
      result => result.warnings.length === 0
```

## When in doubt

- Use `teamwork.status` first to confirm no one else is in the file
- Notify the Teamwork channel before running (`microsoft-365.teams.channel.post-message`)
- For high-stakes ops (revision bump, classification change), prefer
  a *solo* working file (.pln derived from Teamwork) — script against
  that, then have a human merge changes back via the UI

This isn't AWARE giving up; this is ArchiCAD's Teamwork being the
sharded-database-style worksharing model it is. Until Graphisoft
exposes transactional Teamwork primitives, the architect-in-the-loop
gate is unavoidable.
