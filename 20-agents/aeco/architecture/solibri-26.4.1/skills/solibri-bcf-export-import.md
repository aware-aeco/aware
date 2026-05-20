---
name: solibri-bcf-export-import
description: This skill should be used when composing snippets that move issues between Solibri and other AECO tools — exporting checking results as BCF for Revit / Navisworks / Tekla, importing BCF from upstream reviewers, or round-tripping BCF through Solibri as a "second pair of eyes" before issuing. Encodes BCF version selection (2.1 vs 3.0), the topic-mapping rules (Solibri issue → BCF topic), the markup-image conventions, and the gotcha that vendor-specific BCF extensions are dropped on round-trip.
---

# Solibri BCF export / import

BCF (BIM Collaboration Format, OASIS standard) is how coordination issues move between Solibri / Revit / Navisworks / Tekla / etc. Solibri is one of the most BCF-mature tools — but the round-trip semantics have important caveats.

## What's in a BCF .bcfzip

A BCF archive is a ZIP file with:

```
markup/                            — top-level topic metadata
  <topic-guid>/
    markup.bcf                     — XML: topic + comments + viewpoints
    viewpoint.bcfv                 — XML: camera + clipping plane + selected components
    snapshot.png                   — overlay image (the visual marker)
    additional-viewpoints/...      — extra viewpoints if needed
project.bcfp                       — optional project-level metadata
bcf.version                        — "2.1" or "3.0"
```

Solibri exports one topic per Issue. The viewpoint embeds the camera angle Solibri was at when the rule fired; the snapshot shows the affected components highlighted.

## Export from Solibri

The REST call:

```
GET /solibri/v1/bcf/export?severityFilter=Critical,Moderate&version=2.1&includeAccepted=false
```

Filter parameters:

| Param | Notes |
|---|---|
| `severityFilter` | Comma-separated subset of `Critical,Moderate,Low,Undefined` |
| `includeAccepted` | `false` (default) excludes reviewer-accepted issues; `true` includes |
| `rulesetFilter` | Limit to issues from named rule sets |
| `version` | `2.1` (wider compat) or `3.0` (richer markup) |

The response is a binary `.bcfzip`. Save to disk; downstream tools import it.

## Import into Solibri

```
POST /solibri/v1/bcf/import
Content-Type: multipart/form-data
file=@path/to/topics.bcfzip
```

Imported topics become new issues in the current project, tied to the components their viewpoint references (matched by IFC GUID).

If a referenced component isn't in the current Solibri project (because the BCF came from a different federation), the topic still imports but with empty `componentGuids` — visible in the issue list but not attached to a model.

## BCF 2.1 vs 3.0

| Feature | 2.1 | 3.0 |
|---|---|---|
| Camera + clipping plane | Yes | Yes |
| Snapshots | PNG | PNG + per-comment images |
| Markup geometry (lines, arrows) | Limited | Rich |
| Comment threading | Flat | Threaded |
| Topic relationships (parent / blocks / related-to) | No | Yes |
| Vendor extensions | App-specific tags | Same |
| Cross-vendor compat | High (everything reads 2.1) | Mixed (newer tools) |

**For Revit / Navisworks / Tekla round-trips, default to 2.1.** For Solibri ↔ Solibri or Solibri ↔ Newforma, 3.0 is the better choice.

## Round-trip caveats

- **Vendor extensions drop.** Solibri-specific markup attributes (e.g. rule-set-id) don't survive a round-trip through Revit. The CORE topic + viewpoint + comment chain does.
- **GUIDs are stable.** A BCF topic created in Solibri, imported into Navisworks, edited, exported, re-imported into Solibri — same GUID. Comment chain incrementally appends.
- **Reviewer-accepted state DOESN'T export.** "Accepted in Solibri" is a Solibri-only concept; the exported BCF topic just has the comment + status. To round-trip, the upstream tool needs to interpret status changes manually.
- **`componentGuids` mismatch is silent.** If the IFC re-export from Revit changes a wall's GUID (which it CAN do on some Revit operations), the BCF topic loses its anchor. Use IFC pset-based identification as a fallback.

## The Solibri ↔ Revit BCF dance

Standard workflow:

1. Revit exports IFC (stable GUIDs)
2. Solibri loads IFC, runs rules, exports BCF
3. BCF imports into Revit via BCFier / SOFiSTiK BIMTOOLS plugin
4. Revit user fixes the issue, marks BCF topic as Closed
5. Revit re-exports IFC (same GUIDs — critical)
6. Solibri loads new IFC, imports updated BCF, re-runs rules, sees fixed issues drop out

If step 5's GUIDs aren't stable, the loop breaks at step 6. Test the GUID stability of your specific Revit IFC export settings before assuming the loop will work.

## Standard pattern (via `aware-solibri bcf.export-checking-results`)

```python
import requests

base = "http://localhost:10876"
out = args["out-path"]
ver = args.get("bcf-version", "2.1")
sev = args.get("filter-severity", "all")

severity_filter = {
    "all":      "Critical,Moderate,Low,Undefined",
    "critical": "Critical",
    "moderate": "Critical,Moderate",
    "low":      "Critical,Moderate,Low",
}[sev]

resp = requests.get(
    f"{base}/bcf/export",
    params={"severityFilter": severity_filter, "version": ver},
)
with open(out, "wb") as f:
    f.write(resp.content)

# Parse the BCF for topic count
import zipfile
with zipfile.ZipFile(out) as z:
    topics = [n for n in z.namelist() if n.endswith("/markup.bcf")]
return {"out_path": out, "topic_count": len(topics), "bytes": len(resp.content)}
```

## See also

- [solibri-rule-sets-and-checks](./solibri-rule-sets-and-checks.md) — what produces the BCF
- [`bcf.export-checking-results` curated verb](../commands/bcf.export-checking-results.md) (forthcoming)
- Compare to Navisworks's [bcf-roundtrip](../../../construction/navisworks-2026/skills/bcf-roundtrip.md)
