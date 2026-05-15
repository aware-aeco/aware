# `tekla.save-attributes` — persist current dialog values

Stateless command. Runs the Akit (Application Kit) save-as macro to write the currently-set dialog values for a connection joint to a `.j<id>` attribute file. This is how you snapshot a connection's configured parameters for reuse across drawings, models, or projects.

## Lifecycle

`single` — one call, one response

## Inputs

| Field | Type | Description |
|---|---|---|
| `joint-id` | string | Tekla connection ID (e.g. `"144"` for End plate 144, `"27"` for Welded gusset 27). |
| `filename` | string | Name of the attribute file (no extension; the agent adds `.j<id>` automatically). |

## Outputs

```yaml
path: string        # absolute path where the file was written
```

The file lands at `<model-folder>/attributes/<filename>.j<joint-id>`. Tekla writes this synchronously when the macro completes; the agent verifies the file exists before emitting success.

## Generated code

```csharp
// Tekla Akit pattern — must call Connect() before any macro work,
// otherwise the macro silent-no-ops (see floless memory: feedback_tekla_connect_before_macro)
if (!TeklaStructures.Connect())
    return Error("tekla.not-connected");

ValueChange($"joint_{jointId}", "saveas_file", filename);
PushButton("attrib_saveas", $"joint_{jointId}");

// Verify the file landed
var expectedPath = Path.Combine(modelFolder, "attributes", $"{filename}.j{jointId}");
if (!File.Exists(expectedPath))
    return Error("tekla.save-failed", $"expected file not found: {expectedPath}");

return Ok(new { path = expectedPath });
```

## Why this pattern (and not a "save attributes API call")

The Tekla Open API doesn't expose a direct method to save the current dialog values. The Akit macro path is the supported workaround: drive the same UI controls a human would (`ValueChange` to fill the filename field, `PushButton` to click Save As). The synchronous file write is reliable; verifying via the filesystem catches the edge case where the dialog isn't open.

## Compositional use — connection-mapper pattern

The original use case is the Tekla Connection Variable Mapper / X-Ray: snapshot a connection's dialog values into a named file, then walk all its variables programmatically.

```yaml
nodes:
  - id: slider
    agent: tekla-ui
    command: pick-joint
    config: { connection-id: "144" }

  - id: save
    agent: tekla
    command: save-attributes
    config:
      joint-id: "{{ slider.connection-id }}"
      filename: "current-{{ slider.assembly-mark }}"

  - id: walk
    agent: tekla-dialog-walker
    command: discover
    config:
      attribute-file: "{{ save.path }}"
```

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `tekla.not-connected` | `TeklaStructures.Connect()` returned false | Confirm Tekla is running and the model is open. |
| `tekla.dialog-not-open` | The joint's connection dialog isn't open | Open it via `tekla-ui.open-dialog` first (separate agent). |
| `tekla.save-failed` | The macro ran but the expected file didn't appear | Check `<model>/attributes/` permissions. Often a stale Tekla process holds the dir. |
| `tekla.invalid-joint-id` | The joint-id doesn't match an installed Tekla connection | Run `tekla.list-joints` to see what's available. |

## See also

- The Connection X-Ray reference app — uses this command + the dialog walker for full variable discovery
- Tekla MFC dialog walker pattern (see `feedback_tekla_dialog_uses_win32.md` in floless memory) — for going deeper than this command exposes
