# Host Coverage IR Schema

JSON Schema describing the uniform intermediate representation that
per-vendor extractors produce and the generator consumes.

## Usage

```bash
# Validate an IR file against the schema
jq -e '.' < Output/tekla-2026.0.ir.json    # syntax check
# Or via cli-sidecar:
cli-sidecar.exe coverage-validate Output/tekla-2026.0.ir.json
```

## Required fields

Every type must have `name`, `namespace`, `kind`, `summary`, `doc_url`.
Every method must have `name`, `signature`, `summary`, `params`, `returns`, `doc_url`.
Every event must have `name`, `delegate`, `signature`, `summary`, `doc_url`, `handler_thread`, `interacts_with`.
Every property must have `name`, `type`, `getter`, `setter`, `summary`, `doc_url`.

`Method.returns` is either `null` (for void methods) or an object with
required `type` and `doc` strings. `Method.throws[]` items are objects
with required `type` and `when` strings.

`Event.handler_thread` is one of `main` | `worker` | `unknown` — use
`unknown` as the explicit escape hatch when the vendor docs don't say.
`Event.interacts_with` may be an empty array but must be present.

Anything not in the vendor docs is `null` (not empty string). Codex
reviewer requires `summary` and `doc_url` to be present for every
member — these are the absolute minimum.

## Closed schema (no unknown fields)

The schema sets `additionalProperties: false` at every object level
(top-level, `source`, `metadata`, and every `$defs.*` definition).
A typo like `summery` instead of `summary` is rejected outright rather
than silently ignored. This forces all 14 downstream extractors to use
the exact field names defined here.
