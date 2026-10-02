# `google-workspace.account.userinfo`

Read which Google account is signed in: its email address and its stable
account id (`sub`), from the OpenID Connect userinfo endpoint
`GET https://openidconnect.googleapis.com/v1/userinfo`.

## When to use

This is the agent's connection probe — `aware agent probe google-workspace`
runs it to prove the stored credential reaches the real account, and to say
which account that is. It needs only the `openid` and `userinfo.email` scopes
the consent already grants; no new scope.

**READ-mode.** No input.

## Inputs

None.

## Output

The REST transport's response envelope:

```yaml
status: 200
headers: { … }
body:
  sub:            string   # stable Google account id
  email:          string
  email_verified: boolean
```

## Probe

```yaml
probe:
  command: account.userinfo
  kind: account
  rest: { origin: https://openidconnect.googleapis.com }
  reports:
    summary:   /body/email
    identity:  /body/email
    stable-id: /body/sub
```

The origin is pinned twice: the probe refuses any URL not on
`https://openidconnect.googleapis.com`, and AWARE sends the google-workspace
credential only to origins in its code-owned allowlist for that integration.
No redirect is followed, and only the email and `sub` leave AWARE — never the
response body or the token.
