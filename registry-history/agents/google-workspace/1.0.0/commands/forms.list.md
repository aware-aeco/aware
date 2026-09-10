# `google-workspace.forms.list`

List Google Forms the user has access to.

## When to use

The "what intake forms are live" verb — RFI intake, site-survey forms, snagging-checklist forms. Discover a form id before pulling its responses.

**READ-mode**.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `query` | string | no | Optional Drive search filter (e.g. `name contains 'RFI'`). |

## Output

```yaml
forms:
  - form-id: string
    title:   string
    web-url: string
```

## Worked example

```yaml
- id: list-intake-forms
  agent: google-workspace
  command: forms.list
  inputs:
    query: "name contains 'RFI'"
```

## Implementation note

Forms has no native list endpoint, so discovery runs through Drive: `GET files.list` with `q` filtered to `mimeType='application/vnd.google-apps.form'` (combined with any caller `query`). Per-form titles come from `forms.get`. Read scopes: `drive.readonly` + `forms.body.readonly`. Back off on HTTP 429 / 403 `rateLimitExceeded`.

## See also

- [`forms.responses.list`](./forms.responses.list.md)
- [`list-files`](./list-files.md)
- [`sheets.row.append`](./sheets.row.append.md)
