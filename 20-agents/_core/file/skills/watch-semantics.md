# File watch — event-source semantics

`file.watch` is a **stateful event source**, not a poll. It opens an OS-native
filesystem subscription on a folder and streams one record per matching change
until the run stops it. Reach for it instead of a scheduled `read` loop whenever
an app should react *the moment* a file lands or changes — an inbox of uploads, a
folder of exports, a log directory. A 60-second poll both wastes work and misses
events that happen between ticks; a watch fires on the event itself.

## When to use it vs alternatives

- **Use `file.watch`** as a trigger node — the first node in a reactive app, with
  no upstream connection — when the work is "do X every time a file appears/changes".
- **Use `file.read`** (single) when you already know the path and just need the
  contents once, mid-composition.
- **Use a scheduled run** (app-level `schedule:` / cron) when the cadence is a clock,
  not a file event ("every Monday 7am"), even if the first step then reads a file.

## Continuous vs one-shot (`once`)

- `once: false` (default) — **continuous**: the watch stays open and fires downstream
  on *every* matching event for the life of the run. The natural streaming shape.
- `once: true` — **one-shot**: fire on the *first* matching event, then unsubscribe
  and exit. Use for "wait until the export shows up, act once, done".

This mirrors `tekla.watch.once` deliberately — the two event sources share a vocabulary
so an app author reasons about them the same way.

## What an event carries

Each emitted record is a single filesystem change:

| Field | Meaning |
|---|---|
| `signal` | always `"fired"` |
| `path` | absolute path of the affected file — feed this to the next node |
| `name` | the basename — convenient for naming downstream artifacts (`{{ w.name }}.pdf`) |
| `change` | `created` \| `changed` \| `deleted` \| `renamed` |
| `timestamp` | ISO-8601 instant the event was observed |

Route downstream on `change` when it matters (e.g. act on `created` only). A `deleted`
event's file no longer exists — don't chain a `read` after it.

## Gotchas to design around

- **Editors fire bursts.** A single "save" from many applications is a flurry of
  `changed` (and sometimes a create+rename) as they write to a temp file and swap it
  in. Treat events as *at-least-once*: dedupe / debounce downstream on `path` if exactly-once
  matters, and prefer matching the final name via `pattern` over reacting to temp files.
- **A file event ≠ a complete file.** `created` fires when the entry appears, which may
  be *before* the writer has finished. If the downstream node parses the file, either
  watch for the writer's atomic rename (`*.pdf`, not `*.pdf.part`) or have the consumer
  retry on a truncated read.
- **Scope with `pattern`, not downstream filtering.** `pattern: '*.pdf'` keeps temp and
  unrelated files out of the stream entirely — cheaper and clearer than emitting everything
  and discarding most of it in a branch node.
- **`include-subdirs` is opt-in.** Default watches one directory level; deep trees can
  emit a high event volume, so recursion is a deliberate choice.
