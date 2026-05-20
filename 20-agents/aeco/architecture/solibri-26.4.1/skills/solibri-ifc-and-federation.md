---
name: solibri-ifc-and-federation
description: This skill should be used when composing snippets that work with Solibri's IFC import / federation model — loading multiple discipline-specific IFC submodels into one federated coordination model, handling IFC schema versions (2x3 vs 4 vs 4x3 ADD2), managing GUID-based identity across reloads, and tagging submodels by discipline for rule-set scoping. Encodes the GUID-as-identity invariant, the partial-update vs full-reload tradeoff, and the gotcha that re-exporting IFC from Revit can change GUIDs.
---

# Solibri IFC and federation

Solibri is fundamentally a **federation-of-IFC** tool. Every coordination workflow starts with "load the IFC files my disciplines exported." Get the federation semantics right and you get a coherent model. Get them wrong and you get a million ghosting components.

## The IFC versions Solibri reads

| Version | Released | Common in | Solibri 26.4 status |
|---|---|---|---|
| IFC 2x3 | 2007 | Legacy Revit exports, AutoCAD-derived | Reads, writes |
| IFC 4 | 2013 | Modern Revit / Archicad / Tekla | Reads, writes |
| IFC 4x3 ADD2 | 2024 | Infrastructure, new federation pipelines | Reads (write limited) |

Solibri preserves the version metadata. Loading a 2x3 model alongside a 4 model federates fine — rules run cross-version.

## Loading IFC

```
POST /solibri/v1/models/import
Content-Type: multipart/form-data
file=@C:\path\to\architectural.ifc
discipline=architectural    # optional tag for federation organization
```

The response includes a `modelId` for that submodel + a count of components. Components are immediately available for rule-checking.

## Federation = sum of submodels

A Solibri PROJECT (.smcproject) holds one or more loaded IFC submodels. Each submodel keeps its own component identity (IFC GUIDs are unique per file). Rules run against the union — so a rule like "Doors must not clash with walls" picks up doors from the architectural submodel + walls from the structural submodel.

To remove a submodel:

```
DELETE /solibri/v1/models/{modelId}
```

The components disappear from the project; rule results referencing them are orphaned (visible but with `[component-removed]` flag).

## Partial update vs full reload

When a discipline re-exports their IFC:

- **Full reload** (`POST /models/import` with same discipline) — replaces the whole submodel. Loses any "Accepted" marks on individual components.
- **Partial update** (`POST /models/{modelId}/partial-update`) — diff-merges: keeps Accepted state for components whose GUIDs match, adds new components, marks removed ones.

For frequently-iterating disciplines (architect making weekly micro-edits), partial update is the right choice. For full discipline handoffs ("here's the structural deliverable for review"), full reload is cleaner.

## The GUID-as-identity invariant

Solibri identifies EVERY component by its IFC GlobalId (22-char base64 GUID). This drives:
- Cross-submodel relationships (a door "in" a wall is linked by GUIDs)
- BCF round-trip (topics reference componentGuids)
- Partial-update merge
- Accepted-state persistence

If a re-export changes a component's GUID, Solibri treats it as a new component. The old one becomes orphaned. The architect loses their Accepted mark on it.

## Revit IFC export GUID stability — the critical setting

Revit's IFC exporter has a setting that controls GUID stability:

| Setting | Effect on GUIDs |
|---|---|
| "Export linked files as separate IFCs" + "Use family and type name for reference" | UNSTABLE — GUIDs change between exports |
| "Use active view when creating geometry" | UNSTABLE — view changes alter GUIDs |
| Default + "Store the IFC GUID in an element parameter after export" | STABLE — GUIDs persist in Revit element parameter, re-exports honor them |

**The third option is the only one that round-trips cleanly with Solibri.** Confirm this setting before assuming GUID stability.

## Empty federation gotcha

If you import ONLY one IFC file and run a "federation clash" rule set, the rule set finds nothing — there's no second submodel to clash against. Federation rules require AT LEAST TWO submodels to be meaningful.

For pre-checking single-discipline models, use single-discipline rule sets (e.g. "Architectural Pre-Checking") instead.

## Standard pattern (via `aware-solibri model.import-ifc`)

```python
import requests

base = "http://localhost:10876"
ifc_path = args["ifc-path"]
discipline = args.get("discipline", "other")

with open(ifc_path, "rb") as f:
    resp = requests.post(
        f"{base}/models/import",
        files={"file": f},
        data={"discipline": discipline},
    ).json()

return {
    "model_id":        resp["modelId"],
    "component_count": resp.get("componentCount", 0),
    "load_warnings":   resp.get("warnings", []),
}
```

## See also

- [solibri-rule-sets-and-checks](./solibri-rule-sets-and-checks.md) — what runs against the federation
- [solibri-bcf-export-import](./solibri-bcf-export-import.md) — issues that reference GUIDs
- [`model.import-ifc` curated verb](../commands/model.import-ifc.md) (forthcoming)
