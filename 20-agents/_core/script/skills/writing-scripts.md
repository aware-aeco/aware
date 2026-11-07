---
name: script-writing-scripts
description: This skill should be used whenever a workflow needs a computation step that touches no host program — comparing two JSON or CSV exports, totalling or grouping a schedule, reshaping rows for a report, diffing two revisions, building an HTML report string — and so composes the built-in `script` agent's `run` command instead of reaching for a host's `exec` (`tekla/exec`, `revit/exec`, `blender/exec`) that would need the host installed and running. Use it when writing or reviewing a `script/run` node's `code`, wiring its `args` and `files`, choosing its limits, reading its `result`/`logs`/`files` receipt, or working out why it was refused with E_APP_INPUT_MUST_BE_LITERAL or E_APP_FILE_PATH_NOT_LITERAL, or why it failed at run time. Covers when to pick it over a host exec, the contract (globals, what is missing on purpose, strict-JSON result), the limits, failures, and two worked recipes.
---

# `script/run` — host-free JavaScript, pinned by the lock

`script/run` runs a JavaScript function body in QuickJS, inside the `aware` process. It
needs nothing installed, runs the same on every platform AWARE ships, and is pinned by
the lock like every other node: the `code` is written once, in the workflow, approved,
and runs unchanged on every run.

## When to use it, and when not

Pick `script/run` when the step is **pure computation over data the workflow already
has**: two exported JSON files, a schedule a reader produced, the `result` of an
upstream node. Typical jobs: compare, total, group, filter, reshape, render a string.

Pick a host's own runner (`tekla/exec`, `revit/exec`, `blender/exec`) only when the
code must **read or write that host's model** — select parts, set a parameter, render
a scene. A comparison of two exports does not need Tekla or Revit running, and putting
it in a host exec makes the whole workflow depend on a host it never uses.

Pick an `inline` map/filter node for a one-line transform of an upstream value; reach
for `script/run` when the logic is more than a line, reads files, or needs logs.

`script/run` writes nothing. To land its result, connect `file/write` (or a report
node) downstream; that node carries its own `safety:` gate.

## The contract

The `code` is the **body of a function**. It runs top to bottom; its `return` value is
the node's `result`. It is synchronous: there is nothing to `await`, and a returned
Promise is refused.

Globals:

- `args` — the node's `args`, as a frozen deep copy. This is where run-time data
  travels: templates are fine in `args` (`'{{ inputs.threshold }}'`,
  `'{{ upstream.result }}'`), because `args` is data and can never become code.
- `files.<name>` — one entry per name under `files`: `text` (UTF-8 contents), `json`
  (parsed on first access; a leading BOM is skipped; invalid JSON throws naming the
  file), `path`, `bytes`, `sha256`. Every file is read once, before the script starts.
- `run` — `{ startedAt, startedAtMs }`, the run's recorded start. It is the only clock;
  use it for any "generated at" stamp.
- `console.log/info/warn/error` — collected into `logs` (64 KB cap).

Missing on purpose, so the output depends only on its inputs: `eval`, `Function` and
the other function constructors, `Date`, `performance`, `Intl`, `WeakRef`,
`FinalizationRegistry`, file system, network. `Math.random` and the locale methods
(`toLocaleString`, `localeCompare`, `toLocaleUpperCase`, `toLocaleLowerCase`) throw;
format numbers with `toFixed` and sort strings with `<`/`>`.

Two inputs are pinned and refused at validate, compile and run when they are not
written out:

- `code` is literal: no `{{ }}` anywhere in it (`E_APP_INPUT_MUST_BE_LITERAL`). Put the
  changing value in `args` and read it from there.
- each `files` path is a literal absolute path. A template anywhere in `files` is
  refused like one in `code` (`E_APP_INPUT_MUST_BE_LITERAL`); a relative path (a bare
  `inputs.x` included), a `.`/`..` step, a device path or a name that is not a plain
  identifier is `E_APP_FILE_PATH_NOT_LITERAL`. If the
  path has to vary per run, have an upstream `file/read` read it and pass its
  `content` in `args` instead.

The result must be **strict JSON**: plain objects, arrays, strings, finite numbers,
booleans, `null`. `undefined`, functions, `NaN`, `Infinity`, BigInt, cycles, `Map`,
`Set`, class instances, getters and `toJSON` are refused, and the problem is named by
its path (`result.rows[3].total`). Convert a `Map` with `Object.fromEntries`, a `Set`
with `[...set]`.

Open the `code` with a `//` comment saying in plain English what the step does; tools
that show a workflow to people use it as the node's description.

## Limits

| Input | Default | Range | Raise it when |
|---|---|---|---|
| `timeout-seconds` | 60 | ≤ 600 | a large comparison genuinely needs longer |
| `memory-mb` | 1024 | 16–4096 | the files are large and parsed whole |
| `max-input-mb` | 128 | 1–512 | `args` plus files exceed 128 MB |
| `max-output-mb` | 32 | 1–128 | the result is a big report string |

Out-of-range values are refused, never clamped.

## Failures

Every failure fails the node, so nothing downstream runs on a broken answer: a throw
(`threw TypeError: … (at line L, column C)`), a syntax error, the deadline, the memory
limit, recursion too deep, an unreadable or non-UTF-8 file, a budget exceeded, or a
result that is not strict JSON. To report a data problem *without* failing, return it:
`return { ok: false, problems }`, and let a downstream predicate or report decide.

## Recipe 1 — compare two JSON exports

```yaml
inputs:
  tolerance-kg: { type: number, default: 0.5 }

nodes:
  - id: compare
    agent: script
    command: run
    inputs:
      language: javascript
      args:
        tolerance: '{{ inputs.tolerance-kg }}'
      files:
        before: C:/exports/rev-a.json
        after:  C:/exports/rev-b.json
      code: |
        // Compares two part exports by mark and lists what was added, removed or changed weight.
        const index = list => new Map(list.map(p => [p.mark, p]));
        const a = index(files.before.json);
        const b = index(files.after.json);
        const added = [...b.keys()].filter(k => !a.has(k)).sort();
        const removed = [...a.keys()].filter(k => !b.has(k)).sort();
        const changed = [];
        for (const [mark, now] of b) {
          const was = a.get(mark);
          if (was && Math.abs(was.weight - now.weight) > args.tolerance) {
            changed.push({ mark, was: was.weight, now: now.weight });
          }
        }
        console.log(`${added.length} added, ${removed.length} removed, ${changed.length} changed`);
        return { added, removed, changed, before: files.before.sha256, after: files.after.sha256 };
```

Recording the two `sha256` values in the result ties the answer to the exact files it
was computed from.

## Recipe 2 — build an HTML report string, then write it

```yaml
nodes:
  - id: render
    agent: script
    command: run
    inputs:
      language: javascript
      args:
        title: '{{ inputs.title }}'
        rows: '{{ compare.result.changed }}'
      code: |
        // Turns the list of changed parts into a small HTML table report.
        const esc = s => String(s).replace(/[&<>"]/g, c =>
          ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;' })[c]);
        const body = args.rows.map(r =>
          `<tr><td>${esc(r.mark)}</td><td>${r.was.toFixed(1)}</td><td>${r.now.toFixed(1)}</td></tr>`
        ).join('');
        return `<!doctype html><meta charset="utf-8"><title>${esc(args.title)}</title>
        <h1>${esc(args.title)}</h1><p>Generated ${run.startedAt}</p>
        <table><tr><th>Mark</th><th>Was (kg)</th><th>Now (kg)</th></tr>${body}</table>`;

  - id: save
    agent: file
    command: write
    inputs:
      path: '{{ inputs.output }}'
      bytes: '{{ render.result }}'

connections:
  - { from: compare, to: render }
  - { from: render, to: save }
```

Escape every value that came from data before putting it into HTML, and stamp the
report with `run.startedAt`, never a clock of your own: there is none, and that is what
keeps two runs over the same files byte-identical.
