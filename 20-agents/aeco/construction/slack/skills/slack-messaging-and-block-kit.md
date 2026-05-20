---
name: slack-messaging-and-block-kit
description: This skill should be used when posting, updating, scheduling, or threading messages with the slack agent — chat.postMessage and friends, the message ts identity, threading rules, and Block Kit / mrkdwn formatting (which is NOT standard markdown). Encodes the messaging contract behind AECO notification workflows.
---

# Slack messaging & Block Kit

The `chat.*` family is what AECO notification apps actually call. Get the message identity and the formatting model right and the rest is mechanical.

## The core verbs

| Verb | Purpose |
|---|---|
| `chat.postMessage` | post to a conversation (`channel` = a `C…`/`D…` ID, not `#name`) |
| `chat.update` | edit a posted message (by `ts`) |
| `chat.delete` | delete a message (by `ts`) |
| `chat.postEphemeral` | message only one `user` sees in a channel (no notification, not persisted) |
| `chat.scheduleMessage` | post later (`post_at` = unix seconds) |
| `chat.getPermalink` | stable URL to a message — capture this to link back from a report |

## `ts` is the message identity

Every posted message returns a `ts` (e.g. `"1735689600.123456"`) — a string, *not* a number (don't parse it as a float; precision loss breaks lookups). The `ts` is how you `update`, `delete`, or reply-in-thread later. Persist it if the workflow follows up.

## Threading

- Pass `thread_ts` = the **parent message's `ts`** to reply inside its thread.
- Add `reply_broadcast: true` to also surface the reply in the main channel.
- This is the right shape for "one issue → a thread of status updates": post once, keep the `ts`, reply into the thread as the model/coordination state changes.

## `text` vs `blocks` — set both

- `text` is the fallback + the **notification/preview** string (what shows in the sidebar and push notification). **Always set it**, even when using `blocks` — a blocks-only message pushes a blank notification.
- `blocks` is a **Block Kit** array for rich layout: `header`, `section` (with `text`), `divider`, `context`, `actions`, `image`. Build the structured layout here.

## mrkdwn is NOT Markdown

Slack's `mrkdwn` differs from standard Markdown — a frequent source of broken-looking messages:

| Want | mrkdwn | (Markdown would be) |
|---|---|---|
| bold | `*bold*` | `**bold**` |
| italic | `_italic_` | `*italic*` |
| strike | `~strike~` | — |
| link | `<https://acc.example/issue/42\|Issue 42>` | `[Issue 42](https://…)` |
| user mention | `<@U0123>` | — |
| channel mention | `<#C0123>` | — |
| @channel | `<!channel>` | — |

No headings, no tables, no `[label](url)`. For structure, use Block Kit blocks, not Markdown syntax.

## AECO pattern — a build/coordination notification

```yaml
- id: post
  agent: slack
  command: chat-post-message
  config:
    channel: "{{ inputs.coordination-channel }}"   # a C… id
    text: "Clash check: {{ clashes.count }} open"   # notification fallback
    blocks:
      - { type: header, text: { type: plain_text, text: "Coordination — {{ run.date }}" } }
      - { type: section, text: { type: mrkdwn, text: "*{{ clashes.count }}* open clashes. <{{ report.url }}|Open report>" } }

- id: link
  agent: slack
  command: chat-get-permalink
  config: { channel: "{{ post.channel }}", message_ts: "{{ post.ts }}" }
  # capture the permalink to reference this notification from the BCF / e-mail digest
```

## See also

- [slack-conversations-model](./slack-conversations-model.md) — resolving the `channel` ID to post to
- [slack-files-and-uploads](./slack-files-and-uploads.md) — attaching a PDF/screenshot (the upload caveat)
- [slack-rate-limits-and-pagination](./slack-rate-limits-and-pagination.md) — `chat.postMessage` is ~1 msg/sec/channel
