# When to reach for `vision.extract` (and when not to)

`vision.extract` is the **only** way to run a model in AWARE's run path, and it is fenced on
purpose (RFC #223). Reach for it sparingly — it trades a sliver of the fully-unattended,
fully-deterministic promise for the ability to read a *new* image each run without recompiling.

## Decision

```
Need to turn an image/PDF into data?
├─ Can a deterministic parser read it (vector PDF text, clean table, barcode, dimensions)?
│   └─ YES → read-only `exec` node (PdfPig / OCR / decoder). No model. Always prefer this (B1).
└─ Needs vision (photo, handwriting, unstructured)?
    ├─ Does the drawing change per run?
    │   ├─ NO  → extract at COMPOSE time and bake the values into config (the bake pattern).
    │   └─ YES → `vision.extract` (this), behind an `approve:` gate.   ← the carve-out
```

## Rules

- **Extracts, never decides.** It produces typed JSON against a fixed `schema`. It must not pick
  which nodes run next or synthesize an action — downstream stays deterministic.
- **Not an `assert:` evaluator.** Validators stay LLM-free (decalog #9). `vision.extract` is a
  data-producing leaf only.
- **Pin the model.** The `model` id is part of the lock + the cache key; a swap re-invalidates the
  approval, like a source-hash change.
- **Gate the first write.** Put `approve:` on the first write-mode node so a human confirms the
  extraction before anything is written to a host.
- **Cache is determinism.** Same `(file, prompt, schema, model)` → same JSON, replayed from
  `~/.aware/apps/<app>/cache/vision/` with no model call. Non-determinism is bounded to first
  sight of a brand-new input.

If you find yourself wanting free-form prompts, control-flow decisions, or a general "ask the
model" node — that's out of scope by design. Keep the extraction narrow, or move the judgment to
compose time where a human reviews the result.

## After you extract: self-verify

A schema-valid extraction can still be **spatially wrong** — a region silently under-extracted or
an element placed off its target. Whenever the read places things in space (members on a plan,
boxes on a schedule), close the loop: render the extraction back over the source and read the
overlay with vision until the two agree. See [self-verify-overlay.md](self-verify-overlay.md).
