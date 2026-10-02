# model-info

**Lifecycle:** single
**Category:** curated
**Mode:** read
**Stability:** stable across Tekla 2025 / 2026

Read the name and folder of the model open in Tekla Structures. This is the
tekla agent's connection probe: `aware agent probe tekla` runs it.

## What it does

Constructs the Open API `Tekla.Structures.Model.Model`, asks
`GetConnectionStatus()`, and reads `GetInfo().ModelName` / `ModelPath`.
Nothing else — no `CommitChanges`, no `Operation.*`, no write of any kind. It
is safe to run at any time against a model someone is working in.

The instance is chosen by the same rule `exec` uses: exactly one reachable
Tekla instance, or one instance of a single version. The Open API binds
`new Model()` by version, not by process, so several same-version instances
(or one this sidecar cannot inspect) are refused instead of guessed at.

## Inputs

None. (`{}` on stdin is accepted and ignored.)

## Outputs (receipt)

```json
{
  "status": "ok",
  "host": "tekla",
  "host_version": "2026.0",
  "host_pid": 25068,
  "host_session_id": "tekla-25068",
  "verb": "model-info",
  "model_name": "Aware Tests.db1",
  "model_path": "C:\\TeklaStructuresModels\\Aware Tests",
  "delivered_at": "2026-10-02T16:34:36.3046665Z"
}
```

## Refusals

A refusal is a receipt with `status: "err"`, a kebab-case `code` and the
number of Tekla processes seen (`instance_count`), and a non-zero exit:

| `code` | exit | Meaning |
|---|---|---|
| `host-not-running` | 1 | No Tekla Structures instance this sidecar can reach. |
| `host-ambiguous` | 4 | Several same-version instances, or one that could not be inspected. Close all but one. |
| `host-not-connected` | 2 | Tekla is running but the Open API connection did not attach. |
| `model-closed` | 2 | Tekla is running with no model open. |
| `model-read-failed` | 2 | The Open API call itself failed. |
| `host-enumeration-failed` | 2 | The running instances could not be listed. |

`aware agent probe` reports only the `code` and the count
(`E_HOST_UNAVAILABLE` with `details.hostCode` / `details.instanceCount`);
no message text a vendor assembly prints leaves AWARE.
