# Cross-cutting atoms

Named, reusable, typed building blocks for `.flo` apps. Replaces the
inline JavaScript glue (`code: | e => e.type == "Welded"`) that the
2026-05-17 persona audit flagged as "not no-code."

## When to write an atom vs inline glue

| Use atom | Use inline |
|---|---|
| The predicate / map / reduce is referenced more than once | One-off; used only in this app |
| It encodes a project / firm standard ("is this drawing ready for issue") | A trivial filter inline-readable in 3 lines |
| It needs typed inputs / outputs the app composer can rely on | The shape is genuinely free-form |
| You want it auditable across apps in the registry | The logic is local to one .flo |

## Folder structure

- `atoms/` (this folder) — cross-cutting atoms; referenced as `atom://generic/<id>`
- `20-agents/<agent>/atoms/` — agent-specific atoms; referenced as `atom://<agent>/<id>`
- App-local atoms live in the app's folder under `atoms/`; referenced as `atom://app/<id>` (resolved relative to the running app)

## Atom file shape

Each atom is a single YAML file:

```yaml
id: is-newer-than
kind: predicate          # predicate | map | reduce
description: |
  Returns true if the input's timestamp is more recent than the
  threshold (ISO 8601). Both inputs are required.
inputs:
  - name: item
    type: object
    description: Must carry a `timestamp` field (ISO 8601 string).
  - name: threshold
    type: string
    description: ISO 8601 cutoff.
output:
  type: boolean
code: |
  (item, threshold) => new Date(item.timestamp).getTime() > new Date(threshold).getTime()
```

The `code:` block uses the same expression engine as inline glue —
the difference is that the atom file is **versioned, named, typed,
and reusable**. App composers reference it without re-writing:

```yaml
- id: recent
  inline:
    kind: predicate
    description: New since last Friday
    atom: 'atom://generic/is-newer-than'
    inputs:
      item: '{{ upstream }}'
      threshold: '{{ last-friday.iso }}'
```

## Audit trail

When an app references an atom, the provenance log records the
atom's content-hash. A future audit can replay an app at any point
and verify the atom's behavior hasn't drifted.
