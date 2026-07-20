# `lookup`

Deterministic clause and section lookup with **no LLM in the loop**. Given an exact rule
id, returns the AISC/RCSC clause, value, units and citation as typed JSON — or an explicit
miss (`found: false`) so the caller refuses rather than guesses.

**Status:** `available`. The backing binary (`aware-steel-detailer-us`) ships in the aware
release archive and MSI from **v0.98.0**, next to `aware`, and the CLI resolves it there.
On an older CLI it is absent. The rules it reads are installed separately by
`aware agent install steel-detailer-us` — the binary carries no data of its own.

## Why deterministic

A clause lookup must be reproducible and auditable: the same id must always return the
same clause + value, with the value coming from the verified store, never from a model. This
makes the answer lock-able and receipt-able (engineering envelope) — the hard-provenance
complement to the skills' compose-time grounding.

## Inputs

| name | type | description |
|---|---|---|
| `rule` | string | **Exact rule id**, e.g. `bolt.spacing.min`, `section.W16X26`. Not a free-text description. |

Ids are exact. A phrase like `"minimum bolt spacing"` returns `found: false` — by design,
since guessing at a near match is the failure this command exists to prevent. Discover ids
with `lookup --list`, or list one category with `lookup --category bolts`.

Categories: `bolts`, `welds`, `connection-strength`, `materials`, `sections`.

## Outputs

| field | type | description |
|---|---|---|
| `id` | string | The id looked up (echoed, including on a miss). |
| `category` | string | Rule category. |
| `rule` | string | The matched rule. |
| `value` | string | The value/limit. |
| `units` | string | Units (e.g. in, kips, × bolt diameter). |
| `citation` | string | e.g. `AISC 360-22 §J3.4, Table J3.4`. |
| `source_quote` | string | Short verbatim excerpt for verification. |
| `found` | boolean | `false` → no verified rule; the caller must refuse, not guess. |

`sections` rules carry an extra typed `properties` object (weight, dimensions, section
properties, detailing dimensions, coating perimeters) — see `skills/lookup-usage.md`.

## Example

Real output from the shipped v0.98.0 binary:

```console
$ aware-steel-detailer-us lookup --rule bolt.spacing.min
{
  "category": "bolts",
  "citation": "AISC 360-22 §J3.4",
  "found": true,
  "id": "bolt.spacing.min",
  "rule": "Minimum bolt spacing (center-to-center)",
  "source_quote": "The distance between centers of standard, oversized, or slotted holes shall not be less than 2⅔ times the nominal diameter, d, of the fastener",
  "units": "× bolt diameter d",
  "value": "2.67d (2⅔·d); 3d preferred"
}
```

**Exit codes:** `0` found (or listing), `1` not found, `2` error.

`aware agent invoke` does **not** work here — it is builtin-only by design and rejects
`cli` transports. Either run the binary directly as above, or compose the agent as a node
in an app and let the CLI dispatch it; that path resolves the binary on every install
method, including npm, where it is not on PATH.
