---
name: bcf-round-trip-interop
description: This skill should be used when moving BCF issues between coordination tools (Navisworks, Solibri, Revit, ACC, BIMcollab, Trimble Connect) — understanding what survives a round-trip, why topic GUIDs are stable but component anchors are not, and how an IFC re-export can silently break a viewpoint. Encodes the interop rules behind merge, diff, and the read verbs.
---

# BCF round-trip interoperability

BCF exists so coordination issues move *between* tools. The whole value proposition is the round-trip: Navisworks → BCF → BIMcollab → BCF → ACC Issues. This skill encodes what survives that trip and what quietly doesn't.

## What is stable: the topic GUID

A topic's `Guid` is **stable across tools**. A topic created in Solibri, imported into Navisworks, edited, exported, re-imported into Solibri keeps the same GUID, and its comment chain appends incrementally. This is why:

- [`merge`](../commands/merge.md) can dedupe by GUID (same issue from two sources collapses to one).
- [`diff`](../commands/diff.md) can compare two BCFs by GUID to report added / removed / status-changed.
- A weekly round-trip accumulates one coherent comment history per issue instead of duplicating.

**Build all cross-BCF logic on the topic GUID, never on the title or the line order.**

## What is NOT stable: the component anchor

A viewpoint's selected/coloured components are referenced by **IFC GUID**, not BCF GUID. The viewpoint stays meaningful only as long as those IFC GUIDs are stable across model re-exports — and that is **not guaranteed**:

- Some Revit operations (group edit, copy/paste, certain element regenerations) **change** an element's IFC GUID on the next IFC export.
- When that happens, the BCF topic survives (its own GUID is fine) but its viewpoint's component selection no longer resolves — it points at a GUID that no longer exists. The camera still flies to the right place; the highlight is just empty.

This failure is **silent**. Nothing errors. The reviewer opens the issue, sees the right view, but the clashing wall isn't highlighted. Mitigation: use IFC Pset-based identification as a fallback anchor, and test the GUID stability of your specific authoring-tool IFC export settings before assuming the loop holds.

## What does NOT round-trip: tool-specific state

- **Vendor extension markup** (Solibri rule-set-id, tool-private attributes) is dropped when the BCF passes through a tool that doesn't understand it. The *core* topic + viewpoint + comment chain survives; the proprietary trimmings do not. (`bcf-file` itself preserves unknown fields verbatim — it's the *other* tools in the chain that drop them.)
- **Reviewer-accepted state** is often tool-only. Solibri's "Accepted" is a Solibri concept; the exported BCF just carries the status + comment. The upstream tool must interpret status changes manually.

## The canonical Solibri ↔ Revit dance

```
1. Revit exports IFC                         (stable GUIDs — the critical assumption)
2. Solibri loads IFC, runs rules, exports BCF
3. BCF imports into Revit (BCFier / plugin)
4. Revit user fixes the issue, marks topic Closed
5. Revit re-exports IFC                       (same GUIDs — or the loop breaks here)
6. Solibri loads new IFC + updated BCF, re-runs rules, fixed issues drop out
```

If step 5's GUIDs aren't stable, the loop breaks at step 6: Solibri can't match the fixed components and re-raises the issue as new.

## How this shapes the verbs

- **`removed` in [`diff`](../commands/diff.md) is ambiguous.** A topic gone from the newer file might be *resolved-and-archived* or *lost in a bad export*. The diff can't tell you which — your pipeline's conventions decide. Don't treat `removed` as "fixed" automatically.
- **`merge` precedence matters.** Order `input-paths` so the most-current source is last (later wins on conflicting topic fields; comments union regardless). See [`merge`](../commands/merge.md).
- **Snapshots are frozen.** They reflect the model at viewpoint-creation time, not now (see [bcf-viewpoints-and-snapshots](./bcf-viewpoints-and-snapshots.md)).

## See also

- [`merge`](../commands/merge.md) · [`diff`](../commands/diff.md) — the verbs that depend on GUID stability
- [bcf-viewpoints-and-snapshots](./bcf-viewpoints-and-snapshots.md) — the IFC-GUID component anchor in detail
- The [solibri](../../../architecture/solibri-26.4.1/skills/solibri-bcf-export-import.md) agent's BCF export/import skill — the other half of the dance
