# The official PowerFab REST API source

This agent is built against Trimble's official PowerFab developer
portal at **https://developer.tekla.com/tekla-powerfab**.

## What that gives us

- Public REST endpoint documentation (no NuGet required — PowerFab
  is a server-hosted product, not a desktop SDK)
- Free developer signup, no NDA gate (unlike SketchUp's C SDK which
  AWARE explicitly skipped per the `project_sketchup_csdk_skipped`
  memo)
- Authentication via OAuth 2.0 against Trimble Identity
  (`https://id.trimble.com/oauth2`)
- Per-endpoint rate limits documented (typical: 600 req/min per app)

## What the documented surface covers

| Domain | Endpoints |
|---|---|
| Jobs | `/jobs`, `/jobs/{id}`, `/jobs/{id}/status` |
| BOM | `/jobs/{id}/bom`, `/jobs/{id}/bom/import` |
| Production | `/jobs/{id}/pieces`, `/jobs/{id}/pieces/{mark}/advance` |
| Shipments | `/jobs/{id}/shipments` |
| Yard | `/yard/stock` |
| Costing (separate scope) | `/jobs/{id}/cost-summary` |

The 10 curated commands on this agent map to those endpoints
directly — `jobs.list` → `GET /jobs`, `production.advance` →
`POST /jobs/{id}/pieces/{mark}/advance`, etc.

## What's NOT covered

- **Plate-roll / press-brake controllers** — those are downstream of
  PowerFab; ship-floor-only. Use `peddinghaus-translator` for NC
  routing instead.
- **Painting / galvanising third-party trackers** — vendor-specific;
  most fab shops integrate via PowerFab's "External Process" hooks
  (UI-only as of v24.2). Future agent if demand surfaces.
- **Estimating** — PowerFab has an estimating module; this agent's
  scope is the post-tender production phase. A separate
  `tekla-powerfab-estimating` agent covers it (queued).

## Auth flow

```bash
aware connect tekla-powerfab --oauth
# Opens browser to https://id.trimble.com/oauth2/authorize
# Standard PKCE flow — token stored encrypted in keychain
```

For tenant-restricted PowerFab installs (some self-hosted
deployments), the device-code flow (v0.13) works too:

```bash
aware connect tekla-powerfab --device-code
```

## Note on the rename

Trimble re-branded the product from "Tekla EPM" to "Tekla PowerFab"
around 2022-23. AWARE uses the current name (`tekla-powerfab`) as the
agent id. The `keywords` field includes both strings so search hits
the agent regardless of which name the operator uses.
