# Breaking changes

## 0.3.0 — list-folder becomes runnable; the `aware-m365` transport is removed (not breaking)

The 0.2.0 manifest named an `aware-m365` executable that was never shipped, so none
of its 24 commands ran. 0.3.0 removes that CLI transport and runs on REST:

- `list-folder` lists one page of a OneDrive / SharePoint folder's children,
  read-only, through AWARE's code-owned Graph read (#667, #709). It needs AWARE
  0.166.0 or newer (`status: requires-runtime`). `drive-id: me` lists the
  signed-in person's own OneDrive; `folder-id: root` is a drive's root. Connect
  with `aware connect microsoft-365 --device-code --scopes User.Read,Files.Read.All`.
  The output gains `drive-id`, `mime-type`, `modified-time`, `more-available` and
  `omitted`; it is one page — a next page is reported, not fetched.
- Every other command is now explicitly `status: planned`.
- Every command declares the delegated Microsoft permissions to request under
  `permissions:` (`scopes`, `admin-consent`).
