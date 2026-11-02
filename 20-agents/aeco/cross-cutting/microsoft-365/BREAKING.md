# Breaking changes

## 0.5.0 — five Teams reads (not breaking)

- `teams.team.list`, `teams.channel.list`, `teams.channel.list-messages`,
  `teams.chat.list` and `teams.chat.list-messages` are new, read-only and run
  through AWARE's code-owned Graph reads (#709). They need AWARE 0.168.0 or
  newer, so the agent's floor moves from 0.167.0 to 0.168.0.
- `teams.channel.list-messages` needs `ChannelMessage.Read.All`, which a tenant
  administrator must consent to.
- Posting to Teams (`teams.*.post-*`) stays planned.

## 0.4.0 — six more reads become runnable (not breaking)

- `get-drive-item`, `outlook.mail.search`, `planner.task.list` and
  `sharepoint.list.read` run through AWARE's code-owned Graph reads (#709), and
  two discovery reads are new: `planner.plan.list` and `sharepoint.list.list`.
  They need AWARE 0.167.0 or newer (`status: requires-runtime`).
- Inputs of these four changed while they were still planned (none had ever
  run): `get-drive-item` requires `drive-id` (`me` for the person's own
  OneDrive) and no longer takes `sharing-url`; `outlook.mail.search` takes typed
  `text` and `from` instead of a KQL `query`, and `top` is now `page-size`;
  `planner.task.list` no longer takes `bucket-id`, `assigned-to` or
  `completed`; `sharepoint.list.read` no longer takes `filter`, and `top` is now
  `page-size`. `get-drive-item` no longer returns a `download-url`.
- Excel range reads and resolving a sharing link stay planned: Microsoft offers
  them only with a write grant (`Files.ReadWrite`), which needs its own consent
  decision.

## 0.3.0 — list-folder becomes runnable; the `aware-m365` transport is removed (not breaking)

The 0.2.0 manifest named an `aware-m365` executable that was never shipped, so none
of its 24 commands ran. 0.3.0 removes that CLI transport and runs on REST:

- `list-folder` lists one page of a OneDrive / SharePoint folder's children,
  read-only, through AWARE's code-owned Graph read (#667, #709). It needs AWARE
  0.166.0 or newer (`status: requires-runtime`). `drive-id: me` lists the
  signed-in person's own OneDrive; `folder-id: root` is a drive's root. Connect
  with `aware connect microsoft-365 --oauth --scopes User.Read,Files.Read.All`.
  The output gains `drive-id`, `mime-type`, `modified-time`, `more-available` and
  `omitted`; it is one page — a next page is reported, not fetched.
- Every other command is now explicitly `status: planned`.
- Every command declares the delegated Microsoft permissions to request under
  `permissions:` (`scopes`, `admin-consent`).
