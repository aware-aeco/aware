---
name: idea-connection-design-flow
description: This skill should be used when composing snippets that work with IDEA StatiCa Connection — the industry-standard tool for designing + checking steel connections. Encodes the workflow shape (load model → assign material/plates/bolts/welds → analyze → consume report), the difference between CBFEM (Component-Based FEM) and traditional code-check approaches, the BCM (Bolted-Connection-Model) interchange format, and the gotcha that connections from Tekla/Revit imports often need post-import "make sense" pass before they're code-checkable.
---

# IDEA StatiCa Connection design flow

IDEA StatiCa Connection is the de-facto standard for steel connection design across the EU + UK + AU markets (US is shifting too). Its differentiator vs traditional connection-design tools (RAM Connection, Tekla Design Suite, RISA Connection) is **Component-Based FEM (CBFEM)** — every plate, bolt, weld, and contact gets a true FEA model, not a closed-form formula approximation.

## The workflow shape

```
1. Import connection geometry         (from Tekla Open API / Revit / IFC / manual)
2. Assign materials                   (steel grade, bolt class, weld electrode)
3. Refine plate / weld / bolt sizes   (UI dialog or scripted via Connection API)
4. Set load combinations              (forces + moments at the connection node)
5. Analyze                            (CBFEM solver runs in background, ~30s-2min)
6. Consume report                     (PDF / XML / per-component utilization)
7. (optional) Round-trip changes back to Tekla / Revit
```

## CBFEM vs traditional

Traditional connection design = closed-form equations from EN 1993-1-8 / AISC 360-22 (or similar). Fast but limited to standardized geometries.

CBFEM = FEA the connection. Slower (~30s per analysis vs <1s for closed-form) but handles arbitrary geometry: stiffened, asymmetric, complex weld patterns, mixed-material.

**For AWARE workflows, CBFEM is the only viable approach** when:
- The connection is not standard (e.g. asymmetric T-stub, gusseted, knee with stiffener)
- You need per-component utilization (which weld is at 95%, which bolt at 12%)
- You need the deformation pattern for stiffness export (e.g. to model the connection's rotational stiffness in a global frame analysis)

## The BCM interchange

IDEA's `.ideaCon` (per-connection) and `.ideaXml` (project-wide) are XML-based interchange formats. The Connection API exposes them via REST:

```
GET  /api/connection/{id}/export-bcm        — XML model
POST /api/connection/import-bcm             — load XML model
```

For multi-vendor round-trips, BCM is the lingua franca:
- Tekla Open API → IDEA BCM → IDEA analyze → BCM update → Tekla update
- Revit BIM Link → IDEA → results → Revit consume

## Project structure

| File | What |
|---|---|
| `.ideaCon` | Single-connection project |
| `.ideaPrj` | Multi-connection project; one connection per node |
| `.ideaXml` | The text-XML serialization of either above |
| `.idea` | Legacy format; auto-converts on open |

A typical building project has ONE `.ideaPrj` with dozens of `.ideaCon` connections inside. Each connection is independently analyzable.

## Per-component utilization

The CBFEM output reports utilization (load / capacity ratio) per component:

```
Component        | Utilization | Status |
-----------------|-------------|--------|
Bolt M16 (1)     |   0.42      | OK     |
Bolt M16 (2)     |   0.61      | OK     |
Weld throat 5mm  |   0.89      | OK     |  ← high, but under 1.0
Plate t=12mm     |   0.95      | OK     |  ← marginal; consider refining
End plate stiff. |   1.03      | FAIL   |  ← redesign needed
```

A connection is CODE-PASSING when ALL components are <= 1.0 utilization. AWARE workflows that auto-analyze for review should flag any component >= 0.9 as "verify before issuing" — the engineer's judgment is needed.

## Common gotchas

- **Imported connections often need "make-sense" review.** Tekla / Revit may export a connection with the right geometry but missing bolts, missing welds, or implausible plate offsets. Pre-analyze visual check is non-skippable.
- **Material auto-mapping has version-specific quirks.** EN 10025-2 S355 ↔ ASTM A572 Gr.50 mapping in IDEA's table changes between versions. Always verify post-import that materials match what the engineer intended.
- **Load combinations are NOT auto-detected from import.** You must specify them explicitly via the Connection API or in the UI.
- **CBFEM convergence failures** look like exit code 0 but report `Status: Not converged`. Always check the status field, not just the exit code.
- **Per-component utilization is reported AT analysis time.** Changing a plate after analysis doesn't auto-recompute — you must re-trigger analysis.

## Connection API quickref

| Endpoint | Purpose |
|---|---|
| `POST /connection/create` | New connection from BCM XML |
| `POST /connection/{id}/analyze` | Trigger CBFEM analysis |
| `GET /connection/{id}/status` | Poll for completion |
| `GET /connection/{id}/utilization` | Per-component utilization |
| `GET /connection/{id}/report?format=pdf` | Generate PDF report |
| `POST /connection/{id}/update-bcm` | Push geometry change back |

## See also

- `idea-statica-revit-tekla-integration.md` (forthcoming) — round-trip details
- `ideastatica-connectionapi-api.md` — auto-generated REST API reference
- `ideastatica-connectionapi-model.md` — auto-generated DTO reference
