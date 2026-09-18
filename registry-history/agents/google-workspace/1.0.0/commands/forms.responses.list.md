# `google-workspace.forms.responses.list`

Pull responses from a Google Form.

## When to use

The "what did people submit since last Friday" verb — pairs with the BIM-manager Monday audit flow. Feed new site-survey or snagging submissions into a tracker or digest.

**READ-mode**.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `form-id` | string | yes | |
| `since` | string | no | Optional ISO 8601 cutoff. |

## Output

```yaml
responses:
  - response-id:  string
    submitted-at: string
    respondent:   string
    answers:      object
```

## Worked example

```yaml
- id: weekend-submissions
  agent: google-workspace
  command: forms.responses.list
  inputs:
    form-id: "{{ secrets.snagging-form-id }}"
    since: "2026-05-16T00:00:00Z"
```

## Implementation note

Calls `GET forms.responses.list` (Forms API). `since` maps to the `filter` parameter (`timestamp > ...`); answers are returned keyed by question id. Read scope: `forms.responses.readonly`. Back off on HTTP 429 / 403 `rateLimitExceeded`.

## See also

- [`forms.list`](./forms.list.md)
- [`sheets.row.append`](./sheets.row.append.md)
- [`tasks.create`](./tasks.create.md)
