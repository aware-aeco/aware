# `google-workspace.drive.folder.create`

Create a Drive folder. Optional parent (defaults to root).

## When to use

Stand up a new project folder structure — an issue-cycle folder, a discipline subfolder. Pair with `upload-file` to populate it and `drive.share` to hand it to consultants.

**WRITE-mode**.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `name` | string | yes | Folder name. |
| `parent-folder-id` | string | no | Optional; defaults to My Drive root. |

## Output

```yaml
folder-id: string
web-url:   string
```

## Worked example

```yaml
- id: tender-folder
  agent: google-workspace
  command: drive.folder.create
  inputs:
    name: "Tender — {{ project.name }} — {{ run.date }}"
    parent-folder-id: "{{ secrets.project-folder-id }}"
  safety:
    transaction-group: publish
    snapshot: false
```

## Implementation note

Calls `POST files.create` (Drive v3) with `mimeType: application/vnd.google-apps.folder` and `parents` set when a parent is supplied. Write scope: `drive` (folder creation is outside the narrower `drive.file` scope's reach for arbitrary parents). Back off on HTTP 429 / 403 `rateLimitExceeded`.

## See also

- [`drive.share`](./drive.share.md)
- [`upload-file`](./upload-file.md)
- [`list-files`](./list-files.md)
