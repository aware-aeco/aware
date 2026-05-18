# send-status

**Lifecycle:** single
**Category:** curated
**Stability:** stable across Tekla 2025 / 2026

Display a transient message in Tekla's status bar.

## What it does

Wraps `Tekla.Structures.Model.Operations.Operation.DisplayPrompt(message)` —
Tekla's official non-modal user-feedback API. The message appears in the
status bar at the lower-left of the Tekla main window for a few seconds,
then fades.

Use this for:
- Confirming the AWARE ↔ Tekla connection works (smoke test)
- Reporting progress during a long composition without blocking the user
- Non-critical notifications that can be missed without consequence

**Do not use this for:**
- Errors the user must acknowledge — use `TaskDialog` (not yet curated)
- Values the user needs to copy — use clipboard or write to a file
- Audit-trail entries — write to the receipt (`aware receipt sign`)

## Inputs

```json
{
  "message": "Hello AWARE"
}
```

- `message` (string, max 256 chars) — the text to display.

## Outputs (receipt)

```json
{
  "status": "ok",
  "host": "tekla",
  "host_version": "2026.0",
  "host_pid": 14872,
  "host_session_id": "tekla-14872",
  "verb": "send-status",
  "verb_result": { "message": "Hello AWARE" },
  "delivered_at": "2026-05-18T07:30:00Z"
}
```

## Idempotency

Yes — same message twice is fine. Tekla just shows it twice. No model
state changes, no audit entries, no side effects beyond the visual blip.

## Failure modes

| Exit | Meaning |
|---|---|
| 1 | No matching Tekla instance running |
| 2 | API call failed (Tekla side error) |
| 4 | Ambiguous target — multiple Teklas running, use `--pid` |

## Implementation notes for sidecar authors

**Read [tekla-runtime-bridge-dotnet-framework.md](../skills/runtime-bridge-dotnet-framework.md)
FIRST.** The Open API DLL targets .NET Framework 4.x; sidecars must too.

The verb itself is one method call:
```csharp
Tekla.Structures.Model.Operations.Operation.DisplayPrompt(message);
```

But you cannot reach that method until you've successfully constructed a
`Tekla.Structures.Model.Model()` first — the constructor establishes the
connection-to-running-Tekla machinery that `Operation.*` static methods
silently depend on. Without an attached Model, `DisplayPrompt` throws
`FileNotFoundException` with no path.

## Example invocation (CLI)

```bash
echo '{"message":"Hello AWARE"}' | aware-tekla.exe send-status --version 2026.0 --json-stdin
```

## Example invocation (in an app)

```yaml
do:
  - id: smoke-test
    agent: tekla
    command: send-status
    inputs:
      message: "Pipeline {{ run_id }} started"
```
