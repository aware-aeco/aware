# `lookup` (planned — roadmap Phase 4)

Deterministic clause lookup with **no LLM in the loop**. Given a rule selector, returns the
exact Eurocode / P358 clause, value, units and citation as typed JSON — or an explicit miss
(`found: false`) so the caller refuses rather than guesses.

**Status:** `planned`. The backing binary (`aware-steel-detailer-uk`) is not shipped yet.
Until then, the agent's value is its cited skills, read by the composing AI at compose/prompt
time (see `skills/_grounding.md`).

## Why deterministic

A clause lookup must be reproducible and auditable: the same selector must always return the
same clause + value, with the value coming from the verified store, never from a model — the
hard-provenance complement to the skills' compose-time grounding.

## Inputs / Outputs

| field | type | description |
|---|---|---|
| `rule` (in) | string | Rule selector, e.g. `"minimum edge distance e2"`, `"slip resistance"`. |
| `rule` (out) | string | The matched rule. |
| `value` | string | The value/limit (e.g. `1.2·d0`). |
| `units` | string | Units (mm, × d0, …). |
| `citation` | string | e.g. `BS EN 1993-1-8:2005 Table 3.3` or `SCI P358 Check 4 (EN 1993-1-8 cl. 3.9.1)`. |
| `source_quote` | string | Short verbatim excerpt for verification. |
| `found` | boolean | `false` → no verified rule; the caller must refuse, not guess. |

(The concrete values live in the verified skills, not in this doc.)
