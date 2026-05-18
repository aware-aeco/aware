# launch

**Lifecycle:** single
**Category:** curated
**Stability:** stable across Tekla 2025 / 2026

Start a Tekla Structures instance **headless** — no setup dialog, no environment picker, no license picker, no model picker. Window opens maximized so subsequent status-bar / dialog verbs are visually verifiable.

## When to use

- Test harness setup (open Tekla before running an app composition)
- CI / integration tests that exercise real Open API verbs
- Any orchestrator that wants to recover from "Tekla isn't running" by starting it itself

**Do NOT use this for:**
- Daily-driver workflows where the user already has Tekla open — the existing window is fine
- Same-version multi-instance — Tekla refuses (host hard-limit)

## Inputs

### Minimum

```json
{ "version": "2026.0", "environment": "blank_project", "license": "Partner" }
```

The sidecar auto-generates a `Bypass.ini` under `%TEMP%\aware-tekla\Bypass-2026.0.ini`.

### With a model

```json
{
  "version":     "2026.0",
  "environment": "blank_project",
  "license":     "Partner",
  "model_path":  "C:/TeklaStructuresModels/New model 4"
}
```

### With a pre-built Bypass.ini

```json
{
  "version":    "2026.0",
  "bypass_ini": "C:/ProgramData/Trimble/Tekla Structures/2026.0/Environments/uk/Bypass.ini",
  "model_path": "C:/TeklaStructuresModels/MyProject"
}
```

When `bypass_ini` is supplied, `environment` / `license` / `role` are ignored.

## Required + optional fields

| Field | Required? | Example | Notes |
|---|---|---|---|
| `version` | **yes** | `"2026.0"` | Matches `bin/<version>/` install path |
| `environment` | conditional | `"blank_project"`, `"default"`, `"uk"` | Required if `bypass_ini` not supplied |
| `license` | conditional | `"Partner"`, `"DIAMOND"`, `"FULL"` | Required if `bypass_ini` not supplied. See [headless-startup-and-shutdown skill](../skills/headless-startup-and-shutdown.md) for full list. |
| `role` | no | `"%XSDATADIR%\\Environments\\default\\role_Engineer.ini"` | Defaults to Engineer role in default env |
| `model_path` | no | `"C:/TeklaStructuresModels/Demo"` | Opens this model on launch; if omitted, Tekla starts at the model picker (only useful for interactive sessions) |
| `bypass_ini` | no | `"C:/ProgramData/..../Bypass.ini"` | Use a pre-built Bypass.ini; mutually exclusive with environment/license/role |

## Output (receipt)

```json
{
  "status":       "ok",
  "host":         "tekla",
  "host_version": "2026.0",
  "host_pid":     28664,
  "verb":         "launch",
  "verb_result":  {
    "bypass_ini":  "C:/Users/.../Temp/aware-tekla/Bypass-2026.0.ini",
    "model_path":  "C:/TeklaStructuresModels/New model 4",
    "maximized":   true,
    "note":        "Tekla is starting; poll `send-status` until success to confirm Open API readiness (typically ~30s)"
  },
  "delivered_at": "2026-05-18T13:56:30.526Z"
}
```

## Readiness — NOT immediate

`launch` returns the moment `Process.Start` succeeds, which is **before** the Tekla Open API is reachable. The Tekla.Structures.dll Trimble.Remoting MMF channel doesn't exist until Tekla has loaded the model.

**Empirically:** ~30 seconds from `launch` exit to first successful `send-status` against a small / blank model. Production models can take 1-3 minutes. The caller must poll:

```python
# pseudocode — adapt to your orchestration language
launch_pid = aware_tekla.launch(version="2026.0", environment="blank_project", license="Partner")
deadline = now() + timedelta(seconds=180)
while now() < deadline:
    sleep(5)
    if aware_tekla.send_status(version="2026.0", message="ping") == 0:
        break  # Open API reachable
else:
    raise TimeoutError("Tekla 2026 failed to reach Open API readiness within 3 minutes")
```

## Failure modes

| Exit | Meaning | What user should do |
|---|---|---|
| 2 | Missing required `version`, or no Bypass.ini + insufficient env/license info | Read the stderr guide and resupply |
| 3 | Tekla version not installed at `C:\Program Files\Tekla Structures\<version>\` | Pick a version that IS installed; `list-instances` shows what's available |

The sidecar emits a structured guide on stderr when required fields are missing — copy-paste-ready JSON shape included.

## Example invocation (CLI)

```bash
echo '{"version":"2026.0","environment":"blank_project","license":"Partner","model_path":"C:/TeklaStructuresModels/Demo"}' \
  | aware-tekla.exe launch --json-stdin
```

## Example invocation (in an app)

```yaml
nodes:
  - id: start-tekla
    agent: tekla
    command: launch
    config:
      version:     "2026.0"
      environment: "blank_project"
      license:     "Partner"
      model_path:  "C:/TeklaStructuresModels/Demo"

  # Wait + smoke-test
  - id: wait-for-ready
    agent: tekla
    command: send-status
    config:
      message: "AWARE pipeline starting"
      version: "2026.0"
    retry:
      max-attempts: 30
      backoff:      5s
```

## Implementation notes for sidecar authors

The full Bypass.ini schema, license tiers, parameter reference, and Open API readiness diagnostics are encoded in [skills/headless-startup-and-shutdown.md](../skills/headless-startup-and-shutdown.md). Read that first.
