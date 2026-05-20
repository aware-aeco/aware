---
name: rhino-scripting-context
description: This skill should be used whenever composing app nodes that invoke Rhino 8 from outside its UI — via the v0.32 `aware-rhino` sidecar, McNeel's `rhinocode` CLI, or `Rhino.Inside`. Encodes the Script Server lifecycle (StartScriptServer prerequisite, named-pipe IPC routing, the `--rhino <pipeId>` flag for multi-instance precision), the relationship between rhinocode CLI subcommands (list, command, script) and the AWARE exec verb, and the failure modes (no script server, stale pipe ID, dead Rhino process). Critical for any app that ships Roslyn snippets through `rhino.exec`.
---

# Rhino scripting context

Rhino 8 has THREE distinct out-of-process scripting bridges. They look similar but have different lifecycle and licensing implications. Compose against the wrong one and the snippet doesn't run.

## The three bridges

| Bridge | Talks to | License | Use when |
|---|---|---|---|
| **rhinocode CLI** (Rhino 8.11+) | Running Rhino instance via named pipe | Uses the user's running Rhino's license | Default. AWARE's `aware-rhino` sidecar wraps this. |
| **Rhino.Inside.NET** | Boots its own Rhino in-process | Consumes a separate license | When you need full RhinoDoc but the user has no Rhino UI open |
| **rhinoinside Python** | PyNetCore-loaded headless Rhino | Same as Rhino.Inside | Python-only paths |

AWARE's v0.32 sidecar exclusively uses **rhinocode CLI**. Every `aware-rhino exec` shell-out becomes a `rhinocode script <tempfile>` invocation against a running Rhino's Script Server.

## Script Server prerequisite

`rhinocode` connects to Rhino via a named pipe that's **NOT started by default**. The user must run `StartScriptServer` once per Rhino session at the Rhino command prompt — or `aware-rhino launch` with the default `start_script_server: true` does it via `/runscript=_StartScriptServer` at startup.

Symptom of a missing Script Server: `rhinocode` exits with `Unable to connect to Rhino` and the AWARE receipt comes back as `{ok: false, error: "rhinocode could not reach Script Server", ...}`. Recovery: ask the user to run `StartScriptServer` in Rhino, retry.

## Multi-instance routing

When the user has multiple Rhino windows open, `rhinocode` defaults to the first one McNeel's internal pipe enumerator finds — which is not always the one the user is looking at. AWARE solves this:

1. `aware-rhino list-instances` returns one row per running Rhino, including the `rhino_id` (the named-pipe id e.g. `rhinocode_remotepipe_75029`)
2. Subsequent `exec` / `send-status` / `close` calls accept `"rhino_id": "rhinocode_remotepipe_75029"` to target precisely
3. Alternatively, `"version": "8"` picks the first instance whose `processVersion` starts with `"8"` — handy when only one Rhino 8 is running

Pipe IDs are NOT stable across sessions. Re-list before targeting.

## rhinocode CLI subcommands AWARE uses

| Subcommand | AWARE verb wrapping it |
|---|---|
| `rhinocode list --json` | `aware-rhino list-instances` (reshapes McNeel's keys to snake_case) |
| `rhinocode script <file.cs>` | `aware-rhino exec` (with the wrap+sentinel pattern around the user's `code`) |
| `rhinocode command <macro>` | `aware-rhino close` (issues `_-Exit _N` to shut Rhino down cleanly) |

The full rhinocode CLI also exposes `project build` (compile plugins from script-editor projects) which AWARE doesn't wrap.

## The wrap-and-sentinel pattern

Rhino's Script Server runs `.cs` files as **C# 9 top-level statements** — there's no `Main` method. To preserve a `return X;` semantics for the user's snippet, `aware-rhino exec` wraps the body in a synthetic `static object? __AwareRun(IDictionary<string, object?> args)` local function and prints the return value between `__AWARE_RESULT_BEGIN__` / `__AWARE_RESULT_END__` markers on stdout. The sidecar reads stdout, parses the block, returns the JSON to the caller.

This means:
- `using` directives at the top of `code` work normally (they're hoisted above the local function)
- `return X;` works exactly like in cli-tekla
- Anything else on stdout (user's `Console.WriteLine` debug prints, Rhino's chatter) lands in `receipt.stdout_log` for diagnostics
- A script that throws before reaching the result sentinel surfaces as `{ok: false, error: "result sentinel not found in script output (script may have thrown)", stack: ..., stdout_log: ...}`

## Failure modes worth knowing

| Symptom | Likely cause | Fix |
|---|---|---|
| `{ok:false, error: "no running Rhino instance matches version='8'"}` | No Rhino 8 running (or `version` mismatch) | `aware-rhino launch` first |
| `{ok:false, error: "Could not find rhinocode.exe..."}` | Rhino 8.11+ not installed, OR custom install | Install standard path, OR set `RHINOCODE_EXE` env var |
| `{ok:false, error: "rhinocode list did not return a JSON array"}` | rhinocode version too old (pre-8.11) for `--json` | Update Rhino |
| `{ok:false, error: "rhinocode exited 99"}` | Script Server not running OR pipe stale | User runs `StartScriptServer`, then re-list, then retry with fresh `rhino_id` |
| `result.success == true` but doc unchanged | Script ran in wrong RhinoDoc.ActiveDoc (multi-instance race) | Pin `rhino_id` precisely |

## Composition guidance

When writing apps that depend on Rhino:

- **Always check `list-instances` first** if the user has multiple Rhinos open or you need a specific version
- **Pass `rhino_id`** when precision matters; pass `version` when "any Rhino 8 will do" is good enough
- **Never assume Script Server is running** — `launch` it yourself or document the prerequisite at app-install time
- **Treat `exec` as the only mutation surface** — even curated workflow verbs (`view.capture-bitmap`, `layer.set-by-pattern`, etc.) route through it; the verb is the recipe, exec is the engine
