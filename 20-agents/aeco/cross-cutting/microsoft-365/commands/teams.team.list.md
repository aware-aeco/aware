# `microsoft-365.teams.team.list`

List the Teams teams the signed-in person belongs to.

## When to use

Find a team's id before listing its channels with `teams.channel.list`.

Read-only. Work or school accounts only.

## Inputs

| Field | Type | Required | Description |
|---|---|---|---|
| `page-size` | int | no | Teams to return, 1 to 200 (default 100). |

## Output

```yaml
teams:
  - id: "ebf3b108-5234-4e22-b93d-656d7dae5874"
    name: "Acme Tower"
    description: "Coordination for the Acme Tower project"
more-available: false   # Graph has another page, or entries were cut at page-size
omitted: 0              # teams dropped because Graph sent an id that is not a GUID
```

## Implementation note

Runs in-process through AWARE's code-owned Graph read (#709): `GET /me/joinedTeams`. The endpoint
documents no query options, so nothing is sent with it and the page is bounded by AWARE. A
`@odata.nextLink` is reported, never followed. Needs `User.Read` and `Team.ReadBasic.All` (the code
also accepts `TeamSettings.Read.All` or `TeamSettings.ReadWrite.All`); connect with
`aware connect microsoft-365 --oauth --scopes User.Read,Team.ReadBasic.All`.
