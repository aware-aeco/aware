# `microsoft-365.planner.task.list`

List the tasks of one Planner plan.

## When to use

Read a project's to-do list — what is open, who it is assigned to, when it is due — to report on it
or decide the next step.

Read-only. Work or school accounts only.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `plan-id` | string | yes | The plan's id, from `planner.plan.list` or the plan's web address (`.../plan/<plan-id>/...`). |
| `page-size` | int | no | Tasks to return, 1 to 200 (default 100). |

The id is 1-256 characters of letters, digits, `!`, `_` and `-`; anything else is refused before any
request. Filtering by bucket, assignee or state is not offered yet.

## Output

```yaml
tasks:
  - id: "01gzSlKkIUSUl6DF_EilrmQAKDhh"
    title: "Issue IFC set for steel"
    bucket-id: "gcrYAaAkgU2EQUvpkNNXLGQAGTtu"
    percent-complete: 50
    due-date: "2026-11-10T17:00:00Z"
    assigned-to: ["fbab97d0-4932-4511-b675-204639209557"]
    assigned-to-omitted: 0   # assignees beyond 50, or with a malformed id
more-available: false        # Graph has another page, or entries were cut at page-size
omitted: 0                   # tasks dropped because Graph sent a malformed id
```

`assigned-to` holds Microsoft Entra user ids, not names.

## Implementation note

Runs in-process through AWARE's code-owned Graph read (#709): `GET /planner/plans/{plan-id}/tasks`.
Planner documents no query options, so nothing is sent with it and the page is bounded by AWARE. A
`@odata.nextLink` is reported, never followed. Needs `User.Read` and `Tasks.Read`; connect with
`aware connect microsoft-365 --oauth --scopes User.Read,Tasks.Read`.
