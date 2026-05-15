---
name: tekla-drawing-identity
description: This skill should be used when authoring AWARE compositions or code that looks up, persists, or compares Tekla drawings, assemblies, or connection parts. Encodes the critical gotcha that identity in Tekla is `Mark` (unique per model) and NEVER `Name` (which can repeat). Apply when querying drawings by identifier, building lookup dictionaries, emitting events with model-object references, or matching objects across runs for idempotency.
---

# Tekla drawing identity

**Identity is `Mark`, not `Name`.** Drawing.Name can repeat across different drawings — it is a label, not a key. Drawing.Mark is unique within a model. The same applies to assemblies and connection parts: always key by Mark.

## Why this matters

The Tekla Open API exposes both `Name` and `Mark` as string properties on the same `ModelObject`. The API issues no warning when the wrong one is used. Code that compiles, runs, and looks correct silently breaks the first time two objects share a `Name`:

```csharp
// WRONG — matches the first drawing that happens to share this name
var drawing = drawings.First(d => d.Name == "A-104");

// RIGHT — Mark is unique
var drawing = drawings.First(d => d.GetMark() == "A-104");
```

## How to apply

- Key every lookup dictionary by `Mark`.
- Persist by `Mark` (state files, CSV exports, registry entries).
- Include `Mark` in every event payload emitted downstream — never just `Name`.
- When matching across runs for idempotency, compare on `Mark`.

## What `Name` is good for

Display in UIs. Nothing else. Do not store it as a key. Do not compare with it.

## Anti-pattern signal

A downstream node consuming both `Name` and `Mark` from the same payload usually indicates copy-paste from a Tekla API tutorial that ignored this distinction. Replace the `Name` usage with `Mark`.

## Source

Verified against `developer.tekla.com` per release. Stable since Tekla 21.x; not expected to change.
