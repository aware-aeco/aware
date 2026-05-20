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

- **Preferred:** regenerate the agent from a current spec — `aware build agent --from-openapi https://api.slack.com/specs/openapi/v2/slack_web.json` — so `files.getUploadURLExternal` / `files.completeUploadExternal` land in the catalog, then use the 3-step flow.
- **Interim:** don't attach the binary; **post a link instead**. Upload the artifact to where it already lives (Trimble Connect, ACC Docs, SharePoint) and `chat.postMessage` a Block Kit link to it. For AECO this is usually *better* anyway — the file stays in the document-control system of record, and Slack carries the notification + link, not a stray copy.

## What still works

The non-upload file verbs reflect fine and are current: `files.list`, `files.info`, `files.delete`, `files.revokePublicUrl`, and the `files.remote.*` family (register an externally-hosted file as a Slack "remote file" and share its reference). `files.remote.add` + `files.remote.share` is a clean way to surface an ACC/Trimble-hosted document in a channel without copying bytes into Slack.

## See also

- [slack-messaging-and-block-kit](./slack-messaging-and-block-kit.md) — the "post a link, not the bytes" pattern
- [slack-auth-and-scopes](./slack-auth-and-scopes.md) — `files:write` scope
- The `aware build agent --from-openapi` flow — regenerate to pick up the new upload methods
