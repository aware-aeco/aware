# Procore vs ACC (Autodesk Construction Cloud)

Why both? Different GCs run different platforms. Apps that span GCs need to know which agent to call.

| Concern | Procore | ACC |
|---|---|---|
| **Drawings** | Drawings tool (`procore.drawings.*`) | Sheets in ACC Docs (`acc-docs.sheet.*` — v0.15) |
| **Coordination clashes** | tightly bound to Bluebeam | ACC Model Coordination |
| **RFIs** | RFI tool (`procore.rfis.*`) | RFIs in ACC Build |
| **Submittals** | Submittal Builder (`procore.submittals.*`) | Submittals in ACC Build |
| **ASIs** | ASI tool (`procore.asis.*`) | Bulletin in ACC Build (different metaphor) |
| **Punch list** | Punch List tool (separate agent — `procore-field`) | Issues in ACC Build (`acc-issues` agent) |
| **Daily logs** | Daily Log (separate agent — `procore-field`) | not native; `acc-docs` files instead |
| **Cost** | Procore Financials (separate agent — `procore-cost`) | ACC Cost Management |

## When the GC uses Procore but the architect uses ACC

Common in U.S. mid-market. The architect's Revit model + design coordination lives in ACC, but the GC mandates Procore for construction-phase RFIs / submittals. AWARE bridges:

```yaml
nodes:
  - id: acc-clashes
    agent: acc-issues
    command: list-issues
    inputs:
      project-id: '{{ secrets.acc-project-id }}'

  - id: procore-rfis
    agent: procore
    command: rfis.list
    inputs:
      project-id: '{{ secrets.procore-project-id }}'

  - id: combine
    agent: html-report
    command: render
    inputs:
      template: 'cross-cde-rollup'
      data:
        acc-clashes: '{{ acc-clashes.issues }}'
        procore-rfis: '{{ procore-rfis.rfis }}'
```

## Authentication

`aware connect procore` uses Procore's OAuth 2.0 server-flow. Procore requires a registered App in their developer portal — set `AWARE_OAUTH_PROCORE_CLIENT_ID` and `AWARE_OAUTH_PROCORE_CLIENT_SECRET`.

For tenant/company-restricted environments, the device-code flow (v0.13) also works against Procore's `/oauth/authorize_device` endpoint:

```bash
aware connect procore --device-code
```

## Rate limits

Procore enforces ~3000 requests/hour per company. The transport rate-limits to 2500/hour by default to leave headroom for parallel runs. If you're processing 5000 RFIs in one run, batch via `procore.rfis.list` with pagination (transport handles this transparently) and avoid per-RFI fetches.
