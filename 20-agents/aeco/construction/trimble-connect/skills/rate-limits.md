# Trimble Connect Skill · Rate limits + retry

**Trimble Connect rate-limits at the project level. Exponential backoff on `5xx` and `429`. Max 3 attempts. The agent handles this automatically — your composition rarely needs to think about it.**

## The limits (as published)

| Endpoint class | Rate (per project) |
|---|---|
| Read (GET) | 1000 / min |
| Write (POST/PUT/DELETE) | 300 / min |
| Upload bytes | 10 / sec sustained, bursts to 30 / sec |

These are higher than they sound — a typical Tekla→TC pipeline maxes at ~5 uploads / minute. But:

- **Concurrent apps** sharing a project share the budget. A fab pipeline + a daily sync job + a manual user upload can collectively pinch.
- **Backfills** are the danger. *"Upload every drawing from this 3-year-old model"* will hit the limit within seconds.

## The agent's behavior

On `5xx` or `429`, the agent retries with exponential backoff:

```
attempt 1 → immediate
attempt 2 → wait 1s, retry
attempt 3 → wait 2s, retry
fail      → emit error.rate-limit-exceeded with `retry-after` from the response header
```

If Trimble returns `Retry-After`, the agent honors that value instead of its own backoff.

After 3 attempts the error surfaces to your composition. **Don't catch it and retry yourself** — the agent already did the right thing. If you're hitting this, you need to throttle upstream.

## Throttling pattern for backfills

If you know you're going to upload thousands of files, use an inline `throttle` glue node:

```yaml
- id: tekla-list
  agent: tekla
  command: list-drawings

- id: throttle
  inline:
    kind: throttle
    rate: 4/min                # well under TC's write limit
    description: Spread uploads across time for the backfill.

- id: upload
  agent: trimble-connect
  command: upload
  config:
    file: "{{ throttle.file-bytes }}"
    ...
```

The throttle node is visible in the topology; reviewers see "this app paces itself" without reading code.

## What surfaces vs. what doesn't

| Status | Agent action | App sees |
|---|---|---|
| `2xx` | proceed | success |
| `429`, `5xx` (1st time) | retry after 1s | nothing yet |
| `429`, `5xx` (2nd time) | retry after 2s | nothing yet |
| `429`, `5xx` (3rd time, still failing) | give up | `error.rate-limit-exceeded` |
| `4xx` (other) | no retry | error surfaces immediately |
| Network error | retry like a 5xx | same as above |

## Source

Trimble Connect API documentation (`developer.trimble.com/connect`) publishes the rate values. Retry semantics are AWARE's convention, modeled on the AWS SDK approach.
