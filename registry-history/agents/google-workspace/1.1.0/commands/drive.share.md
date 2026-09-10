# `google-workspace.drive.share`

Share a Drive file or folder with one or more users.

## When to use

End of an issue cycle — share the tender PDFs folder with consultants. Default `send-email: false` because most apps already notified via Chat / Mail earlier in the flow.

**WRITE-mode**.

## Inputs

| Field | Type | Required | Default | Description |
|---|---|---|---|---|
| `file-id` | string | yes | — | Drive file or folder id. |
| `emails` | array<string> | yes | — | Recipients. |
| `role` | enum | no | `reader` | `reader` / `writer` / `commenter`. |
| `send-email` | boolean | no | `false` | If `true`, Drive sends a notification email. |

## Worked example

```yaml
- id: tender-folder
  agent: google-workspace
  command: drive.folder.create
  inputs:
    name: "Tender — {{ project.name }} — {{ run.date }}"
    parent-folder-id: "{{ secrets.project-folder-id }}"
- id: share-with-consultants
  agent: google-workspace
  command: drive.share
  inputs:
    file-id: "{{ tender-folder.folder-id }}"
    emails:
      - struct@consultant.com
      - mep@consultant.com
    role: reader
  safety:
    transaction-group: share
    snapshot: false
```

## Implementation note

Calls `permissions.create` per email. Permission `type` is set to `user`; `transferOwnership` is always `false`.
