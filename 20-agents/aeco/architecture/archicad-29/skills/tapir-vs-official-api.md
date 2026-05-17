# Tapir vs the Graphisoft official API

The same ArchiCAD instance exposes two scriptable surfaces. AWARE's
transport prefers Tapir when present and falls back to the official
Graphisoft API otherwise.

## Official Graphisoft API

- **Built into ArchiCAD 23+** — no extra install
- **Free** — no NDA / partner program gate
- Server-style JSON/HTTP on `localhost:19723` (default)
- Activated under *Options → Preferences → Json Reporting*
- API key configured in the same dialog
- **Limitations** as of ArchiCAD 29:
  - No bulk element-by-classification filter (you must walk all elements + filter client-side)
  - No GDL object introspection
  - Limited Teamwork ops (status only; no targeted reserve/release at the API level)
  - Sparse documentation for newer 29-only commands

Docs: https://archicadapi.graphisoft.com/

## Tapir (community extension)

- **MIT licensed** — fully open source
- **Free** — no gate at all
- Source: https://github.com/ENZYME-APD/tapir-archicad-automation
- Active maintenance — 32 releases as of 2026-05-17
- Supports ArchiCAD 25-29 on Windows + macOS
- Installs as both an ArchiCAD Add-On + a Grasshopper plugin

What Tapir adds beyond the official API:

| Capability | Official | Tapir |
|---|---|---|
| `elements.list` filter by classification | ❌ | ✓ |
| GDL object introspection | ❌ | ✓ |
| Library part inventory | ❌ | ✓ |
| Bulk property-pattern queries | partial | ✓ |
| Element-by-attribute (line type / fill / pen) | ❌ | ✓ |
| View-set bulk re-publish | partial | ✓ |
| Story / floor-plan-level introspection | partial | ✓ |
| Grasshopper bridge | ❌ | ✓ |

## Which one does the transport use?

The `aware-archicad-29` transport probes both endpoints at startup:

1. Check if Tapir is installed (`GET /tapir/version` against
   `localhost:19724` — Tapir's separate port)
2. Check if the official API is enabled (`POST /` to `localhost:19723`
   with an `API.GetProductInfo` body)

If both: Tapir wins.
If only one: use it.
If neither: refuse to start with a clear setup-required error.

The error message links to setup instructions for both paths so the
user can install whichever fits their workflow.

## When you'd choose only-official

- IT department mandates no third-party add-ons
- You're on Windows + Teamwork, and the official API is the only
  Graphisoft-supported path for tickets
- You're scripting against a frozen ArchiCAD 23 install (Tapir's
  earliest supported version is 25)

## When you'd choose Tapir

- Default for most workflows
- You need any of the rows in the table above
- You're already using Tapir from Grasshopper / Python

## Auth

Both APIs require an API key. Set it in the ArchiCAD preferences UI,
then `aware connect archicad-29` will store it in the keychain:

```bash
aware connect archicad-29
# Paste the API key from ArchiCAD prefs
```
