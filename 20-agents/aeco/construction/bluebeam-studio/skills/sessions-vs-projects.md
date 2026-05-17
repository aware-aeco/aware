# Sessions vs Projects — the two halves of Studio

Bluebeam Studio gives you two different containers for PDFs. They behave very differently — using the wrong one is a Friday-evening problem.

## Studio Session

- **Real-time collaboration.** Multiple users open the same PDF and see each other's markups live.
- **Time-bounded.** Designed for a *comment cycle* — opens, gets reviewed, closes.
- **Markup ownership.** Each markup is attributable to one user; their disposition status is tracked separately by each user.
- **Lock semantics.** No exclusive lock; everyone edits the same canvas.
- **Lifecycle.** Created → invitees added → reviewed → archived. The archive flattens all markups into the PDF and produces a record file.

Use Sessions when:
- You need a tight review window (e.g. owner mark-up of IFC drawings, one week)
- You want a flattened-with-comments record at the end
- You want disposition tracking per-markup (accepted/rejected by user)

## Studio Project

- **Persistent file storage.** No comment-cycle semantics; it's a Bluebeam-flavoured Dropbox.
- **Markup permanence.** Markups added to a Project PDF stay with the file.
- **Versioning.** Optional version history per file.
- **Lock semantics.** Files can be checked-out, blocking edits by others.

Use Projects when:
- You need permanent PDF storage (the "issued for construction" set, the record drawings)
- You don't want real-time collaboration — just a place where files live
- Your team workflow is download → mark up → upload, not live editing

## In AWARE

This agent's verbs split cleanly:

| Verb prefix | Container |
|---|---|
| `session.*` | Sessions |
| `markups.*` | Sessions (markups live in Sessions) |
| `users.*` | Sessions (invites only make sense in Sessions) |
| `file.*` | Projects |

There is no `file.markups` because Project markups are read via the PDF itself (download + parse), not via the Studio API. If you need to read markups from a Project file, download it and use a PDF-side agent (future v0.15.x) — Bluebeam intentionally separates these flows.

## Authentication scope

`bluebeam-studio` requires the `studio:read` and `studio:write` OAuth scopes. The `aware connect bluebeam-studio` flow asks for both by default. Restrict to read-only if your app only pulls markups: `aware connect bluebeam-studio --scopes "studio:read"`.
