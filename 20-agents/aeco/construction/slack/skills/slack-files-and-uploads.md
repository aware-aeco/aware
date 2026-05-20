---
name: slack-files-and-uploads
description: This skill should be used when attaching a file (PDF report, clash screenshot, BCF) to Slack via the slack agent — and it flags the important gap that the reflected catalog's files.upload is deprecated by Slack while the replacement upload flow is missing from this catalog. Encodes the file-sharing reality and the workaround.
---

# Slack files & uploads

Posting a deliverable to Slack — a clash-report PDF, a Navisworks screenshot, an exported BCF — is a common AECO step. There is a real catalog-vs-Slack mismatch here that will bite a composer who trusts the catalog blindly.

## The deprecation gap (read this first)

This agent was reflected from an older Slack OpenAPI spec (`sdk-target 1.7.0`). Its catalog exposes **`files.upload`** — but Slack **deprecated `files.upload` and sunset it in 2025**. Calling it against current Slack returns errors / will stop working. The replacement is a **three-step external-upload flow** that is **not present in this catalog**:

1. `files.getUploadURLExternal` → returns an `upload_url` + `file_id`
2. HTTP `POST` the file bytes to that `upload_url` (a plain multipart PUT/POST, not a Slack method)
3. `files.completeUploadExternal` → finalize and share to a `channel_id`

Because steps 1 and 3 aren't reflected here, **this agent cannot do a current-Slack upload end-to-end as shipped.**

## What to do about it

- **Preferred (and usually better anyway): post a link, not the bytes.** Upload the artifact to where it already lives (Trimble Connect, ACC Docs, SharePoint) and `chat.postMessage` a Block Kit link. The file stays in the document-control system of record; Slack carries the notification + link, not a stray copy.
- **If you must push bytes into Slack:** you need `files.getUploadURLExternal` + `files.completeUploadExternal`, which this catalog lacks. **Caution:** Slack's published OpenAPI spec lags the real API, so `aware build agent --from-openapi <slack spec>` adds these methods only *if/when the spec actually exposes them* — regeneration is **not** a guaranteed fix. The reliable route is a small hand-authored shim that calls the three-step flow directly. Don't assume re-running `--from-openapi` against the same URL fixes uploads.

## What still works

The non-upload file verbs reflect fine and are current: `files.list`, `files.info`, `files.delete`, `files.revokePublicUrl`, and the `files.remote.*` family (register an externally-hosted file as a Slack "remote file" and share its reference). `files.remote.add` + `files.remote.share` is a clean way to surface an ACC/Trimble-hosted document in a channel without copying bytes into Slack. **Scope note:** the `files.remote.*` methods require `remote_files:write` / `remote_files:share` (and `remote_files:read`) — **not** `files:write`; mixing these up yields `missing_scope`.

## See also

- [slack-messaging-and-block-kit](./slack-messaging-and-block-kit.md) — the "post a link, not the bytes" pattern
- [slack-auth-and-scopes](./slack-auth-and-scopes.md) — `files:write` scope
- `aware build agent --from-openapi` — adds the new upload methods only if Slack's spec exposes them (it may not); a hand-authored shim is the reliable route
