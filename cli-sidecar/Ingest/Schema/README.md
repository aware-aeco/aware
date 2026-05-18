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
Every event must have `name`, `delegate`, `signature`, `summary`, `doc_url`.
Every property must have `name`, `type`, `getter`, `setter`, `summary`, `doc_url`.

Anything not in the vendor docs is `null` (not empty string). Codex
reviewer requires `summary` and `doc_url` to be present for every
member — these are the absolute minimum.
