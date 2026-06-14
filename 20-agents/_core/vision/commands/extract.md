# `vision.extract` — runtime model extraction (the one carve-out)

`vision.extract` reads an image/PDF and returns **structured JSON conforming to a fixed
schema**, by calling a pinned multimodal model **at run time**. It is the single, fenced
exception to decalog #9's "no model in the run path" rule (RFC #223). It **extracts, never
decides** — every node downstream of it stays the reviewed, deterministic plan.

## When to use it

Only when **both** hold: the input genuinely needs *vision* (a deterministic parser can't read
it — a photo, a handwritten markup, an unstructured schedule) **and** the source drawing changes
per run (so compose-time extraction would mean a re-compile each swap). If a parser can read it,
use a read-only `exec` node (B1). If the drawing rarely changes, extract at compose time (bake).

## Inputs

| input | type | notes |
|---|---|---|
| `file` | bytes | image/PDF bytes on the edge, e.g. `{{ inputs.drawing }}` |
| `schema` | object | the FIXED output JSON schema the result must conform to |
| `prompt` | string | the extraction instruction (lock-pinned; part of the cache key) |
| `model` | string | the pinned model id (e.g. `claude-…`); a swap re-invalidates approval |

Output: `{ result: <JSON conforming to schema> }`.

## Worked example

```yaml
inputs:
  drawing: { type: image, widget: file, read-strategy: vision }
nodes:
  - id: extract
    agent: vision
    command: extract
    config:
      file:   "{{ inputs.drawing }}"
      model:  "claude-sonnet-4-6"
      prompt: "Extract each connection-schedule row as {member, boltCount, boltDia}."
      schema:
        type: object
        properties:
          rows:
            type: array
            items:
              type: object
              properties:
                member:    { type: string }
                boltCount: { type: integer }
                boltDia:   { type: string }
  - id: write
    agent: tekla
    command: save-attributes
    mode: write
    config:
      rows: "{{ extract.result.rows }}"
    approve:                      # human confirms the extraction before any host write
      summary: "Re-extracted {{ extract.result.rows | length }} rows — review before writing."
```

## The fence (why this is allowed)

- **Curated + capability-flagged.** The carve-out is admitted by the validator ONLY for a
  `category: curated` command whose agent declares `capabilities.runtime-model-extraction: true`.
  Any other command carrying `model-extraction: true` is rejected `E_APP_RUNTIME_MODEL_FORBIDDEN`.
- **Schema-bound.** The output is a fixed schema, not free text a downstream node `eval`s.
- **Content-hash cached.** Key = `sha256(file ‖ prompt ‖ schema ‖ model)`. A hit returns the
  stored JSON with no model call → deterministic per distinct input, replayable from the cache.
- **Approve-gated.** The first downstream write sits behind `approve:`; the human eyeballs the
  extraction. It **cannot** be an `assert:` evaluator and **cannot** branch control flow.

## Model provider

AWARE runs inside an AI terminal that already has an authenticated, subscription-billed CLI on
`PATH`, so by **default `vision.extract` needs no API key** — a cache miss shells out to the
local CLI. The provider is chosen from the **optional** `~/.aware/credentials/vision-model.json`:

| `vision-model.json` | Provider | Notes |
|---|---|---|
| *absent* (the default) | local **`claude`** CLI, else **`codex`** | uses your existing subscription; no key, no metered API cost |
| `{ "provider": "claude" }` | local **`claude`** CLI | reads the artifact with Claude's Read tool (images **and** PDFs); honors the pinned `model` id via `--model` |
| `{ "provider": "codex" }` | local **`codex`** CLI | `codex exec -i`; runs codex's own configured model, so the pinned Anthropic `model` id is informational |
| `{ "api_key": "…", "base_url"?: "…" }` | **Anthropic API** | the metered fallback (for CI / headless hosts with no logged-in CLI); also `{ "provider": "anthropic", "api_key": "…" }` |

The credential is never written into the app file or the lock. The fence is unchanged — provider
is a runtime credential choice, not a validation concern (no validator/lock change): the cache,
schema-binding, and `approve:` gate hold for every provider.
