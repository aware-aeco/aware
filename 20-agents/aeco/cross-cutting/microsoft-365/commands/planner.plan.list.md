# `microsoft-365.planner.plan.list`

List the Planner plans Microsoft reports as shared with the signed-in person.

## When to use

Find a plan's id before reading its tasks with `planner.task.list`.

Read-only. Work or school accounts only. Covers basic plans; premium (Project) plans are not included.

**Not a complete inventory.** Graph's `/me/planner/plans` returns plans shared with the person, and
Microsoft may leave out plans made in the new Planner even when they sit in a group the person
belongs to (seen live on #709: a group plan with one task was readable by id while this list came
back empty). An empty list is not proof that there are no plans. When a plan is missing, take its id
from the plan's web address — `planner.cloud.microsoft/.../plan/<plan-id>/...` — and pass it to
`planner.task.list`.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `page-size` | int | no | Plans to return, 1 to 200 (default 100). |

## Output

```yaml
plans:
  - id: "xqQg5FS2LkCp935s-FIFm2QAFkHM"
    title: "Acme Tower - coordination"
    container: { type: "group", id: "ebf3b108-5234-4e22-b93d-656d7dae5874" }
    created-at: "2026-09-01T08:00:00Z"
more-available: false   # Graph has another page, or entries were cut at page-size
omitted: 0              # plans dropped because Graph sent a malformed id
```

`container` is the plan's typed container (a group, a roster, …) — never the deprecated `owner`.

## Implementation note

Runs in-process through AWARE's code-owned Graph read (#709): `GET /me/planner/plans`. Planner
documents no query options, so nothing is sent with it and the page is bounded by AWARE. A
`@odata.nextLink` is reported, never followed. Needs `User.Read` and `Tasks.Read` (the code also
accepts `Tasks.ReadWrite`, `Group.Read.All` or `Group.ReadWrite.All`; AWARE never asks for the
`Group.*` grants, which need tenant admin consent); connect with
`aware connect microsoft-365 --oauth --scopes User.Read,Tasks.Read`.
