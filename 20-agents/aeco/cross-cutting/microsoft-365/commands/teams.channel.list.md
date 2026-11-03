# `microsoft-365.teams.channel.list`

List the channels of one Teams team.

## When to use

Find a channel's id before reading it with `teams.channel.list-messages`.

Read-only. Work or school accounts only.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `team-id` | string | yes | The team's id, a GUID, from `teams.team.list`. |
| `page-size` | int | no | Channels to return, 1 to 200 (default 100). |

Anything other than a GUID is refused before any request.

## Output

```yaml
channels:
  - id: "19:a1b2c3d4e5f6@thread.tacv2"
    name: "Steel"
    description: "Steel coordination"
    membership-type: "standard"   # or private, shared
    created-at: "2026-09-01T08:00:00Z"
more-available: false
omitted: 0              # channels dropped because Graph sent a malformed id
```

## Implementation note

Runs in-process through AWARE's code-owned Graph read (#709):
`GET /teams/{team-id}/channels?$select=id,displayName,description,membershipType,webUrl,createdDateTime`.
The page is bounded by AWARE; a `@odata.nextLink` is reported, never followed. Needs `User.Read` and
`Channel.ReadBasic.All` (the code also accepts `ChannelSettings.Read.All` or
`ChannelSettings.ReadWrite.All`).
