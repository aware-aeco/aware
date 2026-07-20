# `lookup`

Deterministic clause lookup with **no LLM in the loop**. Given an exact rule id, returns
the Eurocode / P358 clause, value, units and citation as typed JSON — or an explicit miss
(`found: false`) so the caller refuses rather than guesses.

**Status:** `available`. The backing binary (`aware-steel-detailer-uk`) ships in the aware
release archive and MSI from **v0.98.0**, next to `aware`, and the CLI resolves it there.
On an older CLI it is absent. The rules it reads are installed separately by
`aware agent install steel-detailer-uk` — the binary carries no data of its own.

## Why deterministic

A clause lookup must be reproducible and auditable: the same id must always return the
same clause + value, with the value coming from the verified store, never from a model — the
hard-provenance complement to the skills' compose-time grounding.

## Inputs

| name | type | description |
|---|---|---|
| `rule` | string | **Exact rule id**, e.g. `bolt.edge.min.e1`, `partial.gamma.M2.connectors`. Not a free-text description. |

Ids are exact. A phrase like `"minimum edge distance e2"` returns `found: false` — by
design, since guessing at a near match is the failure this command exists to prevent.
Discover ids with `lookup --list`, or list one category with `lookup --category bolts`.

Categories: `bolts`, `welds`, `materials`, `partial-factors`.

## Outputs

| field | type | description |
|---|---|---|
| `id` | string | The id looked up (echoed, including on a miss). |
| `category` | string | Rule category. |
| `rule` | string | The matched rule. |
| `value` | string | The value/limit (e.g. `1.2·d0`). |
| `units` | string | Units (mm, × d0, …). |
| `citation` | string | e.g. `BS EN 1993-1-8:2005 Table 3.3` or `SCI P358 Check 4 (EN 1993-1-8 cl. 3.9.1)`. |
| `source_quote` | string | Short verbatim excerpt for verification. |
| `found` | boolean | `false` → no verified rule; the caller must refuse, not guess. |

## Example

Real output from the shipped v0.98.0 binary:

```console
$ aware-steel-detailer-uk lookup --rule bolt.edge.min.e1
{
  "category": "bolts",
  "citation": "BS EN 1993-1-8:2005 Table 3.3 §3.5 (corroborated: steelconstruction.info + eurocodeapplied.com)",
  "found": true,
  "id": "bolt.edge.min.e1",
  "rule": "Minimum end distance e1 (parallel to load direction)",
  "source_quote": "End distance e1 ≥ 1.2d0 (Table 3.3 minimum)",
  "units": "× hole diameter d0",
  "value": "1.2·d0"
}
```

**Exit codes:** `0` found (or listing), `1` not found, `2` error.

`aware agent invoke` does **not** work here — it is builtin-only by design and rejects
`cli` transports. Either run the binary directly as above, or compose the agent as a node
in an app and let the CLI dispatch it; that path resolves the binary on every install
method, including npm, where it is not on PATH.

UK National Annex values only — never mix with US/AISC (`steel-detailer-us`) or the EN
recommended values (`steel-detailer-eu`).
