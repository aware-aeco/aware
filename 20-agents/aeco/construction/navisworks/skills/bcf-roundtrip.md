# BCF round-trip

Navisworks ↔ ACC Issues ↔ BIMcollab/Solibri/Revizto all speak BCF.
The persona audit ranked this as the #4 cross-tool primitive.

## The round-trip

1. **Navisworks → BCF**: `clash.export --format bcf` produces a
   `.bcfzip` with one topic per clash, including:
   - Title (clash name)
   - Status (mapped from ClashStatus)
   - Viewpoint (camera + selection)
   - Comment (clash distance / clearance value)

2. **BCF → ACC Issues**: pipe the `.bcfzip` to the v0.15
   `acc-docs` agent (or v0.14 `bcf-file` agent for low-level
   manipulation).

3. **ACC Issues updates → BCF**: as consultants resolve issues,
   pull updated `.bcfzip` from ACC.

4. **BCF → Navisworks**: import via the `bcf-file.import` flow into
   a NW saved-viewpoints folder; clashes auto-link by GUID.

## GUID stability

Each clash gets a stable `guid` in the BCF (matches Navisworks
internal clash result id). A round-trip preserves this — meaning
"clash #4327 was approved by MEP on 2026-05-10" survives across
ACC ↔ Navisworks ↔ BIMcollab.

If a clash's geometry changes (Revit move), Navisworks may emit a
**new** clash with a new GUID — the resolved one becomes "resolved"
on its own. This is desirable: closed coordination decisions stay
closed; new geometry produces new conversations.

## BCF 3.0 vs 2.1

| Feature | 2.1 | 3.0 |
|---|---|---|
| Topics, viewpoints, comments | ✓ | ✓ |
| Markup styling (color, opacity) | partial | ✓ |
| Components grouping | flat | hierarchical |
| Extension schemas | limited | extensible |

The Navisworks 2026 transport emits 2.1 by default for compatibility.
Set `--bcf-version 3.0` if downstream consumers all support it.
