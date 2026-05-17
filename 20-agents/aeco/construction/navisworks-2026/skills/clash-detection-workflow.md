# Clash-detection workflow

The Monday-morning clash pass:

```yaml
nodes:
  - id: fed
    agent: navisworks-2026
    command: federation.open
    inputs:
      path: '\\office\projects\acme\fed\acme-master.nwf'

  - id: clashes
    agent: navisworks-2026
    command: clash.run-all
    inputs:
      document-id: '{{ fed.document-id }}'
    safety:
      transaction-group: clash-pass
      snapshot: false   # clash results are model-side, not source-model writes

  - id: export
    agent: navisworks-2026
    command: clash.export
    inputs:
      document-id: '{{ fed.document-id }}'
      format: bcf
      output-path: '{{ run.tmp-dir }}/acme-clashes-{{ run.date }}.bcfzip'

  - id: post
    agent: microsoft-365
    command: teams.channel.post-with-card
    inputs:
      team-id: "Project Acme"
      channel-id: "coordination"
      title: 'Monday clash digest — {{ run.date }}'
      rows:
        - test: '{{ clashes.tests[0].test-name }}'
          new: '{{ clashes.tests[0].results.new }}'
          active: '{{ clashes.tests[0].results.active }}'
```

## ClashTest types

The audit's BIM-manager flagged these as the realistic mental units:

| Type | What it catches |
|---|---|
| **Hard** | Solid-vs-solid geometric intersection |
| **Hard (Conservative)** | Hard + nearly-touching (tolerance you set) |
| **Clearance** | Within N mm of another item (e.g. duct above ceiling) |
| **Duplicate** | Same item modelled twice (federation bug) |
| **TimeBased** | Schedule clash — items occupying space at same time |

`clash.list-tests` exposes `test-type` so you can route results by
risk class (Hard → MEP-vs-Structure ball-in-court, Clearance →
accessibility review, etc.).

## Status lifecycle

```
new → active → reviewed → approved → resolved
                       ↘ disregard
```

`clash.run-all` overwrites `new` on every run but preserves the
others. So a downstream BCF round-trip can map "approved" clashes to
"closed" issues in ACC, "active" to "open", etc.

## Common pitfalls

- **Test name with spaces:** `clash.run-test` matches the test name
  case-sensitively, including trailing whitespace. Use `clash.list-tests`
  first if the name was hand-typed.
- **BCF export size:** a federation with 5,000+ clashes produces a
  BCF that crashes some consumers. Use `test-names:` to scope to a
  subset before exporting.
- **Viewpoint generation cost:** if `clash.export` produces BCF with
  viewpoints, expect ~2s per clash. For 500 clashes that's 15 minutes.
  Schedule via v0.19's overnight cron.
