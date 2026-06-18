# `lookup` (planned — roadmap Phase 4)

Deterministic clause lookup with **no LLM in the loop**. Given a rule selector, returns the
exact AISC/RCSC clause, value, units and citation as typed JSON — or an explicit miss
(`found: false`) so the caller refuses rather than guesses.

**Status:** `planned`. The backing binary (`aware-steel-detailer-us`) is not shipped yet.
Until then, the agent's value is its cited skills, read by the composing AI at compose/prompt
time (see `skills/_grounding.md`).

## Why deterministic

A clause lookup must be reproducible and auditable: the same selector must always return the
same clause + value, with the value coming from the verified store, never from a model. This
makes the answer lock-able and receipt-able (engineering envelope) — the hard-provenance
complement to the skills' compose-time grounding.

## Inputs

| name | type | description |
|---|---|---|
| `rule` | string | Rule selector, e.g. `"bolt min edge distance, M20, sheared edge"`. |

## Outputs

| field | type | description |
|---|---|---|
| `rule` | string | The matched rule. |
| `value` | string | The value/limit. |
| `units` | string | Units (e.g. mm, in, × bolt diameter). |
| `citation` | string | e.g. `AISC 360-22 §J3.4, Table J3.4`. |
| `source_quote` | string | Short verbatim excerpt for verification. |
| `found` | boolean | `false` → no verified rule; the caller must refuse, not guess. |

## Example (intended shape)

```
aware agent invoke steel-detailer-us lookup --inputs '{"rule":"minimum bolt spacing"}'
→ {
    "rule": "minimum spacing between bolt centers",
    "value": "<the limit, from the verified store>",
    "units": "× bolt diameter",
    "citation": "AISC 360-22 §J3.3",
    "source_quote": "<short verbatim excerpt>",
    "found": true
  }
```

(The concrete values live in the verified skills, not in this doc.)
