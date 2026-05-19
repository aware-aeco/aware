# `tekla.exec` — compile + run an ad-hoc C# script against the active model

Stateless command. Receives a chunk of C# code, compiles it via Roslyn against the v0.30 catalog's typed surface (`Tekla.Structures.Model`, `Tekla.Structures.Geometry3d`, …), runs it inside Tekla's transaction discipline, returns the result as JSON.

The verb that closes the loop between the **catalog** (what types exist, what methods they have) and the **live host** (where those types come alive). The orchestrator reads `20-agents/aeco/engineering/tekla-2026/catalog/Tekla.Structures.*.json`, drafts a snippet, ships it through this verb. This is the AWARE pattern in one verb: **prompt → catalog lookup → executable C# → live host**.

## Lifecycle

`single` — one call, one response

## Category

`curated` — but unlike the other curated verbs, this one is *open-ended*. It does not encode a single Tekla workflow; it gives the orchestrator a generic execution channel.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `code` | string | **yes** | C# snippet. Optional trailing `return X;` makes the value bubble up as the receipt's `result` field. |
| `version` | string | recommended | Target Tekla version (e.g. `"2026.0"`). Used to discover the install dir and load the matching `Tekla.Structures.*.dll` set. Omit only for primitive smoke tests that never reference Tekla types. |
| `args` | object | no | Free-form input bag, exposed to the script as `IDictionary<string,object> args`. |

### Pre-imported namespaces

The script runs with the following `using` lines already in scope, so snippets stay catalog-shaped:

- `System`
- `System.Collections.Generic`
- `System.Linq`
- `Tekla.Structures` *(when Tekla DLLs resolved)*
- `Tekla.Structures.Model`
- `Tekla.Structures.Model.Operations`
- `Tekla.Structures.Geometry3d`
- `Tekla.Structures.Drawing`
- `Tekla.Structures.Datatype`

The script can add more `using` lines of its own at the top of `code` if it needs them.

### Script globals

| Name | Type | Description |
|---|---|---|
| `model` | `dynamic` | A `Tekla.Structures.Model.Model` instance constructed via the live Open API. `null` if `version` was omitted; non-null but disconnected if the version is installed but no Tekla is running. The script can call any method on it — DLR dispatch resolves at runtime against the real type. |
| `args` | `IDictionary<string, object>` | The input `args` block, normalized into a string-keyed dictionary with primitive / `List<object>` / nested-dictionary values. |

## Outputs

On success:

```json
{
  "ok": true,
  "result": <serialized return value>,
  "host": "tekla",
  "verb": "exec",
  "delivered_at": "2026-05-19T13:39:33.7Z"
}
```

The `result` field is serialized defensively:

- Primitives (`bool`, `int`, `long`, `double`, `decimal`, `string`, `DateTime`) → direct JSON.
- `System.Guid` → `"00000000-0000-0000-0000-000000000000"` string form.
- `IDictionary` → JSON object (keys stringified).
- `IEnumerable` → JSON array.
- Types with a `GUID` property (e.g. `Tekla.Structures.Identifier`) → that GUID's string form.
- Anything else → bounded-depth `System.Text.Json.JsonSerializer` (depth 6, cycle-ignored), with a final `ToString()` fallback.

On compile-error:

```json
{
  "ok": false,
  "error": "compile error: <message>",
  "stack": "<line-by-line diagnostics from Roslyn>",
  "host": "tekla",
  "verb": "exec",
  "delivered_at": "..."
}
```

On runtime exception (script threw):

```json
{
  "ok": false,
  "error": "<ExceptionType>: <message>",
  "stack": "<full stack trace>",
  "host": "tekla",
  "verb": "exec",
  "delivered_at": "..."
}
```

## Transaction semantics

The script runs **inside an implicit transaction**. The mapping to Tekla's actual API:

- **On success** the sidecar calls `model.CommitChanges()` after the script returns. This is Tekla's transaction-commit primitive — it flushes in-memory changes to the model database and makes them undoable as a single user-visible step. Skipped if `model` is `null` or `GetConnectionStatus() == false`.
- **On exception** the sidecar does **not** call `CommitChanges()`. Tekla has no rollback API, but un-committed in-memory state never reaches the database — the next `CommitChanges` from a subsequent call overwrites it, and any partial inserts the script made are effectively orphaned until then.

This is the closest the Tekla Open API gets to ACID. If you need stronger guarantees (e.g. all-or-nothing across multiple `exec` calls), gate the script with explicit checks at the top and short-circuit yourself.

## Failure modes

| Error shape | Cause | Recovery |
|---|---|---|
| `missing required field: code` | Stdin JSON didn't include `code`. | Add it. |
| `compile error: ...` | Script failed Roslyn compilation. | Re-draft. Diagnostics in `stack` field. |
| `<ExceptionType>: <msg>` runtime | Script threw at runtime. | Inspect stack; common cases: Tekla connection lost mid-call, wrong namespace, type missing in catalog. |
| `Model.CommitChanges() failed after script success` | Script ran fine but commit hit the Tekla DB. | Often means the model has un-saveable state (read-only model, sharing conflict). Inspect; do not retry blindly. |

## Example invocations

### Primitive smoke test (no Tekla required)

```bash
echo '{"verb":"exec","code":"return 1+2;","args":{}}' \
  | aware-tekla.exe exec --json-stdin
# → {"ok":true,"result":3,...}
```

### Catalog-driven insert (live Tekla)

```bash
echo '{
  "verb": "exec",
  "version": "2026.0",
  "code": "var col = new Beam { StartPoint = new Point(0,0,0), EndPoint = new Point(0,0,4000), Profile = new Profile { ProfileString = \"HEA200\" }, Material = new Material { MaterialString = \"S275\" }, Class = \"1\" }; col.Insert(); return col.Identifier.GUID;",
  "args": {}
}' | aware-tekla.exe exec --json-stdin
# → {"ok":true,"result":"00000000-0000-0000-0000-...",...}
```

### Reading model state

```bash
echo '{
  "verb": "exec",
  "version": "2026.0",
  "code": "var info = model.GetInfo(); return info.ModelName;",
  "args": {}
}' | aware-tekla.exe exec --json-stdin
```

## Implementation notes

**Read [runtime-bridge-dotnet-framework.md](../skills/runtime-bridge-dotnet-framework.md) first** — this verb extends the same `net48` discipline that `send-status` and `insert` already follow.

The Roslyn compile step runs inside the same `AppDomain` that hosts the Open API connection. References are built from the resolved DLL paths (`{host_install}/bin/Net48Runtime/` or `{host_install}/bin/`); the `AssemblyResolve` handler installed at startup picks up transitive Tekla.* deps on first reference. The `model` global is wired via Roslyn's `globalsType` mechanism using a public `ExecGlobals` class — must be public (not nested in `Program`) so the dynamically generated `Submission#N` type can access it across assembly boundaries.

The script's static type binding works because Roslyn's `MetadataReference.CreateFromFile(...)` reads the IL signatures of the Tekla DLLs at compile time. Calls on `model` go through DLR (`dynamic`) instead — that lets cli-tekla avoid any static reference to `Tekla.Structures.Model.Model` in its own assembly, which preserves the JIT-no-inlining safety pattern from the runtime-bridge skill.

## See also

- [insert.md](./insert.md) — the curated single-verb counterpart for `ConnectionPart` creation
- [send-status.md](./send-status.md) — the simplest curated verb; useful smoke-test target
- [runtime-bridge-dotnet-framework.md](../skills/runtime-bridge-dotnet-framework.md) — why everything Tekla-touching is `net48`
