# Shell open — scoped, declarative "open this path"

`shell.open` does exactly one thing: hand a local path to the OS default handler, as
if the user double-clicked it. That narrowness is the point. An `exec` sandbox (e.g.
`tekla.exec`) deliberately blocks arbitrary process launches, which is correct — but it
also means a composition can compute "the folder the user cares about" and then have no
safe way to *show* it. `shell.open` is that safe last step: reveal a folder, pop a
generated report in its viewer, hand off to the browser — without re-opening the door to
running chosen programs with arguments.

## What it is — and is not

- **It is:** "open this *path* with whatever the OS already associates with it." One input,
  a path. No program name, no argument list, no shell string.
- **It is not:** a way to run a command. There is no `shell.exec`, no `shell.run`. If a
  workflow needs to invoke a specific tool, that is an agent/command with a typed contract,
  not a generic shell escape. Keeping `open` argument-less is what lets the runtime validate
  and sandbox it (only `open` semantics, only local paths).

## When to use it

Reach for `shell.open` as a **terminal, user-facing convenience node** — the natural
follow-up after a run produces something a person should look at:

- open the folder a `file.write` / report step just populated,
- open a generated PDF/HTML artifact in its default app,
- reveal an export directory after a batch job.

It is a side-effecting leaf, not a data source: its output is just `{ path, opened }`, so
nothing downstream depends on its result. Don't use it to "launch an app and then drive
it" — opening returns as soon as the handler is launched, not when the app is ready.

## Behaviour notes

- **Headless / no desktop session:** there may be no GUI or no registered handler (a CI
  runner, a service account). `open` then fails with `error.no-handler` rather than hanging —
  treat it as best-effort UX, never as a load-bearing step.
- **Cross-platform:** the runtime maps `open` to the platform default (Windows
  `ShellExecute`/Explorer, macOS `open`, Linux `xdg-open`). Author once; the path is what
  matters.
- **Path must exist and be in scope:** the agent declares `filesystem: read: any`; the
  concrete path scope is granted at install per the agent permission model. A missing path
  fails fast (`error.path-not-found`).
