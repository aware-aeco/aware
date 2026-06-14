# `vision.extract` — local-CLI model provider (claude / codex headless)

**Date:** 2026-06-14 · **Status:** approved design → building · Follow-up to RFC #223
(`2026-06-13-vision-extract-rfc.md`).

## Problem

`vision.extract` (shipped 0.68.0) calls the Anthropic Messages API directly via `ureq`,
reading an **API key** from `~/.aware/credentials/vision-model.json`. RFC §8 flagged this
honestly: "it introduces a model dependency + a credential + network egress."

But AWARE runs **inside an AI terminal** (Claude Code / Codex). That terminal already has an
authenticated, subscription-billed CLI on `PATH`. Forcing a *separate, metered* API key for
the one model call AWARE makes is redundant and ~10× the marginal cost of the subscription the
operator is already paying for. The operator's ask: **default `vision.extract` to the local
`claude` / `codex` CLI in headless mode; keep the API key as an explicit opt-in.**

## Design: a model **provider** behind `call_vision_model`

`vision.extract`'s cache/approve/schema fence (RFC §5) is unchanged — the provider is *only*
how a cache **miss** reaches a model. We dispatch on a provider resolved from the (now
**optional**) credential file:

| Provider | Selected when | How it calls the model |
|---|---|---|
| **`claude` CLI** (default) | no credential, or `provider: claude` | `claude -p <instruction> --allowedTools Read --permission-mode acceptEdits --output-format text --model <pinned>`; the image/PDF is written to a temp file and read via Claude's Read tool (handles **both** images and PDFs) |
| **`codex` CLI** | `provider: codex` | `codex exec -i <img> -o <out> --skip-git-repo-check`; native `-i` image attach, the schema-constrained prompt fed on **stdin** (a multi-line arg can't be escaped for codex's Windows `.cmd` shim), the result read from `-o`. We do *not* use codex's `--output-schema` — it requires a strict JSON Schema (`additionalProperties:false` + all keys `required`) that arbitrary vision schemas won't have; the schema is a textual constraint, as in the other paths |
| **Anthropic API** | `provider: anthropic`, or an `api_key` is present with no `provider` (back-compat) | the existing `ureq` POST to `{base}/v1/messages` |

### Resolution (in `resolve_vision_provider`)

```
cred = read ~/.aware/credentials/vision-model.json   (optional)
match cred.provider:
  "anthropic"|"api"|"anthropic-api" → Anthropic { api_key (required), base_url? }
  "claude"|"claude-cli"|"claude-code" → ClaudeCli
  "codex"|"codex-cli" → CodexCli
  (absent):
     api_key present?  → Anthropic            (back-compat: an explicit key = use the API)
     else claude on PATH? → ClaudeCli         (zero-config default)
     else codex on PATH?  → CodexCli
     else → error: install `claude` (recommended, uses your subscription, no key)
            or `codex`, or set vision-model.json {"api_key":"…"}
```

This **preserves back-compat** (an existing `api_key` credential keeps using the API exactly as
before) while making the **zero-credential default** the local `claude` CLI — the cheap path the
operator is already authenticated for.

### Why `claude` is the default (not `codex`)

- It is a **single native binary** (`claude.exe` on Windows) — `Command::new("claude")` resolves
  it directly; `codex` ships only `.cmd`/`.ps1` shims (CreateProcess won't append those), so it
  needs PATH+PATHEXT resolution.
- Its `--model` accepts the **pinned Anthropic model id** the lock already carries
  (`claude-sonnet-4-6`), so the cache key's `model` matches the model actually used. `codex` runs
  its own (GPT) model regardless — for `provider: codex` the pinned id is informational; this is
  documented.
- Claude's Read tool reads **PDFs** as well as images; `codex -i` is image-oriented.

### Determinism is untouched

The §5.4 content-hash cache wraps `call_vision_model`, so a hit still replays stored JSON with
**no** subprocess at all. The provider only matters on first sight of a new `(bytes, prompt,
schema, model)`. Approve-gate, schema-binding, and the validator carve-out are unchanged
(provider is a *runtime credential* choice, not a *manifest/validation* concern — no validator
change, no agent-count change).

## Cross-platform CLI invocation

`find_on_path(name)` resolves a spawnable executable: on Windows it tries `name + {.exe,.cmd,
.bat,.com}` across `PATH` (skipping the extensionless/`.ps1` shims CreateProcess can't run);
on Unix it returns the PATH-resolved name. `claude.exe` (native) and `codex.cmd` (a shim) both
resolve. Because Rust's `Command` refuses to pass un-escapable args to a `.cmd`/`.bat` shim, the
`codex` prompt is fed on **stdin** rather than argv (claude is a native exe, so its prompt stays
in argv — verified). Each CLI call runs under a watchdog timeout (stdout/stderr drained on
threads) so a hung or chatty model can't deadlock or hang the run.

## Out of scope

- No change to the fence, cache, schema-binding, approve-gate, or validator (RFC §5 stands).
- `provider: codex` does not honor the pinned Anthropic model id (codex uses its own config);
  documented, not engineered around.

## Verification

- Unit: `resolve_vision_provider` over credential variants (none / api_key / each `provider`);
  `find_on_path`.
- Real E2E: `aware app run` a minimal `image → vision.extract → output` app with **no
  credential**, asserting the extraction JSON comes back via the local `claude` CLI; repeat with
  `provider: codex`.
