---
name: archicad-hotlinks-and-teamwork
description: This skill should be used when composing Archicad automation that runs in a multi-user (BIMcloud / Teamwork) environment OR touches hotlinked modules. Encodes the Teamwork architecture (BIMcloud server, reservation-based locking, Send/Receive sync cycle), the hotlink-module model (linked .mod / .pln modules vs the embedded library vs linked libraries), why ReserveElements is mandatory before any write, and the gap in Tapir's API where hotlink-reload is NOT exposed (so headless hotlink refresh is currently impossible).
---

# Archicad hotlinks and teamwork

Two related but distinct mechanisms for sharing model content across files / users / firms:

| Mechanism | What it does | Storage |
|---|---|---|
| **Teamwork** | Multi-user concurrent editing of ONE project, with reservation-based locking | BIMcloud server (Graphisoft) or BIMcloud Basic |
| **Hotlink** | Embed another .mod / .pln file's geometry as a referenced placeholder; updates pulled from the source on demand | Local filesystem path OR Teamwork-shared module |
| **Linked library** | A library of GDL Objects/Doors/Windows/Lamps referenced from a folder or LCF outside the project | Local filesystem path OR BIMcloud library |
| **Embedded library** | A library of GDL objects living INSIDE the .pln file | The .pln itself |

Each has different sync semantics and a different set of automation verbs.

## Teamwork architecture

In a Teamwork project:

1. Multiple users open the SAME project from a BIMcloud server simultaneously.
2. Each user **reserves** elements they want to edit. The reservation is a server-side lock.
3. Local edits accumulate in the user's local cache (the .bimproject5 file).
4. **TeamworkSend** uploads the user's reserved changes back to the server.
5. **TeamworkReceive** downloads other users' sent changes into the local cache.

This is the BIM equivalent of git: local edits, explicit push/pull, never auto-sync. Both Send AND Receive are required to round-trip.

### Tapir Teamwork verbs

```
TeamworkSend       — push local changes to server
TeamworkReceive    — pull server changes to local
ReserveElements    — lock elements for editing
ReleaseElements    — unlock elements
```

`TeamworkSend` and `TeamworkReceive` take no parameters — they sync the whole project state.

### Reservation — the read/write gate

Before ANY mutating call in a Teamwork project, the orchestrator MUST reserve the target elements. Otherwise the write fails with a per-element `executionResult.success: false, error: "Element is not reserved"`.

```http
POST http://127.0.0.1:19723/
Content-Type: application/json

{
  "command": "API.ExecuteAddOnCommand",
  "parameters": {
    "addOnCommandId": {
      "commandNamespace": "TapirCommand",
      "commandName":      "ReserveElements"
    },
    "addOnCommandParameters": {
      "elements": [
        { "elementId": { "guid": "AAAAAAAA-BBBB-CCCC-DDDD-EEEEEEEEEEEE" } }
      ]
    }
  }
}
```

Reservation can fail if another user holds the element. The response `conflicts[]` lists conflicting users. Best practice for AI orchestration:

1. `TeamworkReceive` (latest server state)
2. `ReserveElements` for the planned write set
3. Check the conflicts list; abort or wait if any element is held by another user
4. Perform the mutation (`SetDetailsOfElements`, `Create*s`, `Delete*`, …)
5. `ReleaseElements`
6. `TeamworkSend` (push the change)

Skipping `TeamworkReceive` produces "missing-element"-style errors on stale local cache. Skipping `TeamworkSend` leaves the change purely local and invisible to teammates.

### Detecting whether you're in Teamwork

`ProjectCommands.GetProjectInfo` returns `isTeamwork: true|false`. Always probe before composing — the right verb set differs between solo and Teamwork.

## Hotlinks — linked modules

A hotlink is a reference from THIS project to a chunk of geometry/elements stored in ANOTHER file:

```
This project (Main Building.pln)
└── Hotlink "Wing A"  →  ./Wing A.mod
    └── (1,247 walls / slabs / windows from Wing A.mod, ghosted, non-editable here)
```

Common uses:
- The same MEP riser referenced from every floor's plan
- A repeated apartment unit type used 50 times
- The site context model (terrain, neighbours) shared across all projects
- Coordination with a consultant's separate Archicad project

### Hotlink data model

Each hotlink has:
- **Location** — filesystem path or BIMcloud reference
- **Children** — nested hotlinks (modules within modules)
- **Status** — current / out-of-date / missing
- **Transform** — placement (position, rotation, scale)

The Hotlink element type is `Hotlink` in the ElementType enum. Hotlink instances appear in `GetAllElements` results.

### Hotlink read verbs (available)

- `ProjectCommands.GetHotlinks` — list every hotlink in the project, with their nested children.

```http
POST http://127.0.0.1:19723/
Content-Type: application/json

{
  "command": "API.ExecuteAddOnCommand",
  "parameters": {
    "addOnCommandId": {
      "commandNamespace": "TapirCommand",
      "commandName":      "GetHotlinks"
    },
    "addOnCommandParameters": {}
  }
}
```

### Hotlink write/refresh verbs (NOT available)

**There is NO `ReloadHotlinks`, no `UpdateHotlinks`, no `RefreshHotlinks` verb in Tapir or the Graphisoft built-in surface.** Hotlink update is currently manual:

- User opens Navigator → File → External Content → Hotlink Manager → "Update All" — manual GUI action.
- OR, in Teamwork, `TeamworkReceive` catches hotlink updates published by other users — but only if the hotlink source is itself a Teamwork module that was sent.

This is a known gap. The closest workflow-level automation is library reload (`LibraryCommands.ReloadLibraries`) — which is DIFFERENT (it refreshes GDL library content, not hotlinked modules), but is the only "reload" primitive Tapir exposes.

**Implication for curated verbs:** the `hotlink.reload-all` workflow verb was deliberately dropped from this agent's curated set because no underlying primitive exists. Use `library.reload-all` for library content; document the hotlink-refresh limitation in any app that requires it.

## Linked libraries vs embedded library

Two places GDL objects can live:

| Type | Source | Updates when source changes? | When to use |
|---|---|---|---|
| **Linked library** | A folder OR LCF file outside the .pln | Yes, on `ReloadLibraries` | Firm-wide standard library (chairs / sinks / etc.); ARCHICAD Library 29 system library |
| **Embedded library** | Inside the .pln file | No source — they ARE the source | Project-specific custom objects; safe-to-ship for handover |

`LibraryCommands.GetLibraries` returns both, with a `type` field. Only linked libraries respond to `ReloadLibraries`.

### When to `ReloadLibraries`

- After a server-side library push (BIMcloud library distribution)
- After a manual library swap (firm rolled out v3 of standard library)
- After dropping a new GDL into a linked folder
- NOT routinely on every script run — reload is expensive (re-indexes thousands of objects)

## Teamwork + hotlinks + libraries — the full sync cycle for headless automation

For a multi-user / multi-firm project, the safe-write order is:

```
1. GetProjectInfo                  ← detect isTeamwork
2. TeamworkReceive                 ← pull latest server state (Teamwork only)
3. ReloadLibraries                 ← refresh GDL content (if relevant)
4. GetHotlinks → log; warn if stale (no auto-refresh available)
5. ReserveElements (target set)    ← lock for write (Teamwork only)
6. (Mutation calls)
7. ReleaseElements                 ← unlock (Teamwork only)
8. TeamworkSend                    ← push changes back (Teamwork only)
```

In a solo (non-Teamwork) project, steps 2 / 5 / 7 / 8 are no-ops; ReloadLibraries and GetHotlinks still apply.

## Gotchas

- **Solo project verbs that look like Teamwork verbs.** `ReserveElements` is a no-op outside Teamwork — calling it on a solo .pln succeeds trivially. Avoid relying on its return as a real lock.
- **Stale local cache silently produces stale data.** Forgetting `TeamworkReceive` is the #1 bug in Teamwork automation. Always Receive at the start of a workflow.
- **TeamworkSend can fail at the server end** even after local success — network drops, server side conflicts, etc. Check the response and retry.
- **Hotlinks are not editable in the host project.** Attempts to `SetDetailsOfElements` on a hotlink-source element succeed silently locally but never persist. Filter elements with `Hotlink` type out of mutation sets.
- **Library reload force-closes open GDL editors.** Warn the user before triggering in interactive sessions.
- **No hotlink-refresh automation exists today.** Don't design workflows that require it without surfacing the manual-step limitation.

## See also

- [archicad-scripting-context](./archicad-scripting-context.md) — Tapir + JSON-RPC transport
- [archicad-element-model](./archicad-element-model.md) — Hotlink as an ElementType
- [archicad-layouts-and-issue](./archicad-layouts-and-issue.md) — TeamworkReceive before publish
- [library.reload-all](../commands/library.reload-all.md) — the library-only reload verb
