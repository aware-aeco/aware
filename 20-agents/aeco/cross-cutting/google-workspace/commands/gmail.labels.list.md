# `google-workspace.gmail.labels.list`

List the signed-in person's Gmail labels.

## When to use

Turn the label a person sees ("Project 2417 / RFIs") into the label id that
[`gmail.list-message-ids`](./gmail.list-message-ids.md) and [`gmail.search`](./gmail.search.md)
take, or offer the person a list of labels to pick a Knowledge source from.

Read-only. Takes no input.

## Output

```yaml
labels:
  - id: "INBOX"
    name: "INBOX"
    type: "system"
  - id: "Label_12"
    name: "Project 2417/RFIs"
    type: "user"
omitted: 0        # labels not returned: malformed or repeated id, or past the output budget
complete: true    # omitted == 0 — the list is every label Gmail sent
```

Labels come in Gmail's order. `type` is `system` or `user`; any other value leaves `type` out
and names it in that label's `truncated`. A `name` longer than 256 characters is cut and named in
`truncated`; a missing name is named there too. Gmail allows up to 10,000 labels and does not page
this list, so AWARE keeps labels in order while the **whole** answer fits the request's
`maxOutputBytes` and counts the rest in `omitted`: `complete: false` then says the list is not the
whole account, never a silent cap. A budget too small for even an empty list is
`E_CALL_OUTPUT_TOO_LARGE` (`reason: output-too-large`).

## Failures

`E_CALL_FAILED` carries the `status` and `phase: "labels"` only — never Google's text. A rate limit
(a 429, or a 403 whose Google reason is `rateLimitExceeded` or `userRateLimitExceeded`) is
`reason: "rate-limited"`, with `retry-after-secs` when Google sent `Retry-After`. A raw answer over
8 MiB is `E_CALL_OUTPUT_TOO_LARGE` (`reason: raw-too-large`).

## Implementation note

One `GET https://gmail.googleapis.com/gmail/v1/users/me/labels`, no redirects, under the call's
deadline, for the one verified Google account (the slot is checked immediately before the
request). Needs `gmail.readonly`, opt-in:
`aware connect google-workspace --oauth --scopes https://www.googleapis.com/auth/gmail.readonly`;
without it the call is `E_CALL_SCOPE_MISSING` before any request.

## See also

- [`gmail.list-message-ids`](./gmail.list-message-ids.md)
- [`gmail.get-message`](./gmail.get-message.md)
- [`gmail.search`](./gmail.search.md)
