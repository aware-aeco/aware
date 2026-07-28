# bake-scene

Materialize the member records in a canonical scene as Rhino Breps in the
active Rhino 8 document.

```powershell
Get-Content scene-request.json -Raw |
  aware-rhino bake-scene --json-stdin
```

The request carries the unchanged host-neutral scene:

```json
{
  "verb": "bake-scene",
  "rhino_id": "rhinocode_remotepipe_75029",
  "host_pid": 31204,
  "scene": {
    "meta": {
      "name": "Frame A",
      "units": "mm",
      "sourceId": "floless:drop:frame-a",
      "sceneHash": "sha256-of-canonical-scene"
    },
    "elements": [
      {
        "id": "M1",
        "kind": "member",
        "from": [0, 0, 0],
        "to": [6000, 0, 0],
        "section": { "w": 165, "d": 310 },
        "meta": { "profile": "W12X26" }
      }
    ],
    "operations": [],
    "referenceSystems": []
  }
}
```

## Units and geometry

The scene is always interpreted in millimetres. The materializer reads
`RhinoDoc.ActiveDoc.ModelUnitSystem` and converts every coordinate and section
dimension with RhinoCommon's millimetre-to-document-unit conversion. It does not
change the document unit system.

Each member becomes a capped rectangular Brep extruded along its exact `from` to
`to` axis. The rectangle preserves the authored section width and depth. The v1
sink does not infer flange, web or wall thickness from a designation; a missing
or non-positive section is a failed member, never a guessed shape.

All objects are placed on the `AWARE` layer. Rhino has no family lookup for this
operation, so the receipt does not distinguish "native" from "fallback".

## Ownership and replacement

Every Brep carries four namespaced object user-text values:

- source id;
- record id;
- scene hash;
- a versioned materialization marker.

`bake-scene` is retire-and-replace by source id. It constructs every incoming
Brep in memory, stages the complete replacement set, and reads back all ownership
text before deleting any prior object. It then retires only objects carrying all
four valid ownership values for the same non-empty source id. Other sources,
partially stamped objects and ordinary user geometry are not touched.

The write is grouped into one Rhino Undo record. If materialization or retirement
fails, the materializer deletes staged objects, undeletes any prior objects it
already retired, and verifies the restored state. `rolledBack` is true only when
that verification succeeds. An uncertain cleanup adds a
`commit-state-uncertain` warning and the safe recovery is to repeat the same
scene with the same source id.

## Target selection

Writes never use rhinocode's implicit default session:

- `rhino_id` selects an exact Script Server pipe;
- `host_pid` selects a process only when it maps to one live pipe;
- supplying both requires them to identify the same session;
- a version prefix must have one match;
- with no selector, exactly one Script Server session must be live.

Zero matches and ambiguous matches fail before the script is dispatched.
Any selector that is supplied with the wrong JSON type, null, or a blank value
also fails closed; it is never downgraded to an implicit single-session write.

## Long-running writes

The sidecar waits up to 15 minutes by default (`AWARE_BAKE_TIMEOUT_MS` can
override this). RhinoCode dispatch is fire-and-forget, so reaching that deadline
does not prove the Rhino script stopped. The sidecar therefore preserves the
late result channel and returns `commit_state: "uncertain"` with recovery
guidance, rather than claiming the operation rolled back or deleting its result
path. Wait for Rhino to finish, then repeat the same scene with the same
`sourceId` to reconcile ownership before making another edit.

## Receipt

The outer sidecar envelope identifies the actual target:

```json
{
  "ok": true,
  "host": "rhino",
  "host_version": "8.31",
  "host_pid": 31204,
  "rhino_id": "rhinocode_remotepipe_75029",
  "verb": "bake-scene",
  "result": {
    "ok": true,
    "sourceId": "floless:drop:frame-a",
    "sceneHash": "sha256-of-canonical-scene",
    "materializationHash": "host-bound-sha256",
    "attemptId": "unique-attempt-id",
    "created": 1,
    "retired": 0,
    "emitted": [
      {
        "id": "M1",
        "kind": "member",
        "status": "emitted",
        "nativeGuid": "Rhino-object-guid"
      }
    ],
    "failed": [],
    "unsupported": [],
    "warnings": [],
    "rolledBack": false
  }
}
```

Every authored scene record appears exactly once in `emitted`, `failed`, or
`unsupported`. Nested plate holes and bolt hole effects use the shared
`opening` receipt kind. A nested `result.ok:false` also makes the outer command
fail, so a refused or rolled-back bake cannot be mistaken for success.
