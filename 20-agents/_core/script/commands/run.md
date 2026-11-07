# `script.run` — host-free JavaScript over a workflow's data

Stateless, `mode: read`, built in. Runs a JavaScript function body in embedded
QuickJS-ng, in-process, and returns its `return` value as the node's `result`. Use it
for the data and report steps of a workflow that touch no host program: compare two
exports, total a schedule, shape rows, build an HTML report string. A step that
really reads or writes a host's model stays on that host's own runner
(`tekla/exec`, `revit/exec`, `blender/exec`).

## Inputs

| Field | Type | Default | Description |
|---|---|---|---|
| `language` | enum (`javascript`) | required | The only language today. |
| `code` | string, **literal** | required | The function body. Written out in the workflow; any template here is refused (`E_APP_INPUT_MUST_BE_LITERAL`) at validate, compile and run. |
| `args` | object | `{}` | Templatable data, handed over as a frozen deep copy `args`. |
| `files` | object, **path-map** | `{}` | Up to 16 `name: <absolute path>` pairs. Literal absolute paths only: a template anywhere in it is `E_APP_INPUT_MUST_BE_LITERAL`; a relative path (`inputs.x` included), a `.`/`..` step, a device path or a bad name is `E_APP_FILE_PATH_NOT_LITERAL`. |
| `timeout-seconds` | number | `60` | Deadline for compile, run and result conversion; at most 600. |
| `memory-mb` | integer | `1024` | 16 to 4096. |
| `max-input-mb` | integer | `128` | Total of `args` plus the files; 1 to 512. |
| `max-output-mb` | integer | `32` | Largest serialised result; 1 to 128. |

A limit out of range is refused, never clamped. The four limits are **literal**: a template in
any of them is refused (`E_APP_INPUT_MUST_BE_LITERAL`), so no upstream value can move a limit the
person approved. Reaching the memory or stack limit fails the node even when the script catches
the error QuickJS raises for it: how far a script gets before either limit differs between
platforms, so a caught limit would otherwise return a platform-dependent answer. A built-in that
never yields to the deadline check (a search over two very large strings, say) fails the node at
the deadline plus a few seconds; that operation is then left to finish in the background, and
while two such are still running in one AWARE process, a new script is refused.

## What the script sees

| Global | What it is |
|---|---|
| `args` | The node's `args`, deep frozen. |
| `files.<name>` | `{ text, json, path, bytes, sha256 }` per named file. `path` is the path as the workflow wrote it, never where a link resolved. `text` is the UTF-8 contents (a BOM is kept in `text`, skipped by `json`); `json` parses on first access and names the file when it is not valid JSON. |
| `run` | `{ startedAt, startedAtMs }`: the run's recorded start, the only clock there is. |
| `console` | `log`, `info`, `warn`, `error` into `logs` (capped at 64 KB or 4096 lines). |

Not there, by design: `eval`, `Function` and every function constructor, `Date`,
`performance`, `Intl`, `WeakRef`, `FinalizationRegistry`, any file system or network.
`Math.random` and the locale methods (`toLocaleString`, `localeCompare`,
`toLocaleUpperCase`, `toLocaleLowerCase`) throw. `Error.prepareStackTrace` is a read-only
`undefined`: AWARE keeps the engine's hook to see a stack overflow or out-of-memory error
the script catches.

## Outputs (single)

```yaml
ok:     true
result: <the return value, strict JSON>
logs:   [string]                         # only when the script logged
files:  [{name, path, resolvedPath, bytes, sha256}]  # a receipt per file read, never its contents
run:    {startedAt, startedAtMs}
engine: {name: QuickJS-ng, version, rquickjs, aware-cli}
```

The result must be strict JSON: no `undefined`, functions, symbols, `NaN`/`Infinity`,
BigInt, cycles, accessors, `toJSON`, or non-plain objects. A problem is named by its
path (`result.rows[3].total`).

## Failure

Any failure fails the node, so nothing downstream runs on a broken answer: a throw
(`threw TypeError: … (at line L, column C)`), a syntax error, the deadline, the memory
limit, too deep recursion, a file that cannot be read or is not UTF-8, a budget
exceeded, or a result that is not strict JSON. In a streaming run the failure is
reported against the script node and the next event is handled as usual.

## Determinism

The output is a function of the code, `args`, the named files' bytes and the recorded
run start, **for a given AWARE CLI version** (the engine is pinned per CLI release and
recorded in `engine`).

## Example

```yaml
nodes:
  - id: compare
    agent: script
    command: run
    inputs:
      language: javascript
      files:
        before: C:/exports/rev-a.json
        after:  C:/exports/rev-b.json
      code: |
        // Lists the marks whose weight changed between the two exports.
        const a = new Map(files.before.json.map(m => [m.mark, m.weight]));
        return files.after.json
          .filter(m => a.has(m.mark) && a.get(m.mark) !== m.weight)
          .map(m => ({ mark: m.mark, was: a.get(m.mark), now: m.weight }));
```

See [writing-scripts.md](../skills/writing-scripts.md) for when to use it and more
worked recipes.
