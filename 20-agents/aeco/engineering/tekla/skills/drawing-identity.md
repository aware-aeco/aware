# Tekla Skill · Drawing identity

**Identity is `Mark`, not `Name`.**

`Drawing.Name` can repeat across different drawings — it's a label, not a key. `Drawing.Mark` is unique within a model. Same applies to assemblies and connection parts: always key by `Mark`.

## Why this bites

The Tekla Open API exposes both `Name` and `Mark` as string properties on the same `ModelObject`. The API doesn't warn you when you use the wrong one. Code that compiles, runs, and looks correct will silently break the first time two objects share a `Name`:

```csharp
// WRONG — will match the first drawing that happens to share this name
var drawing = drawings.First(d => d.Name == "A-104");

// RIGHT — Mark is unique
var drawing = drawings.First(d => d.GetMark() == "A-104");
```

## What to do

- Key every lookup dictionary by `Mark`.
- Persist by `Mark` (state files, CSV exports, registry entries).
- When emitting events downstream, include `Mark` in the payload — never just `Name`.
- When matching across runs (idempotency), compare on `Mark`.

## What `Name` is good for

Display in UIs. That's it. Don't make it a key, don't store it as a key, don't compare with it.

## Cross-checks

If a downstream node uses both `Name` and `Mark` from the same payload, that's usually a sign you've copy-pasted from a Tekla API tutorial that ignored this distinction. Replace `Name` usage with `Mark`.

## Source

Verified against `developer.tekla.com` per release. The behavior has been stable since Tekla 21.x and is not expected to change.
