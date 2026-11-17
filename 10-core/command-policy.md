# Command policy

A **command policy** is a list of deny rules a host writes into `AWARE_HOME`, naming agent commands that
must not run for given credential accounts. AWARE checks it at the moment it dispatches an operation that
carries a credential AWARE resolved, on every path: `aware agent call`, `aware agent invoke`, every
workflow node of `aware app run` (one-shot, chained, each event downstream of a long-running watcher, a
nested exposed app), `aware agent probe`, and the one builtin that sends an AWARE-held key
(`vision.extract`). A denied command is refused even when the credential is valid and holds every scope
the command needs.

Why it exists: a host can gate the operations it starts itself, but an approved workflow, a scheduled
routine or a long-lived watcher is dispatched by AWARE, which loads and checks the credential at each
operation. A per-account "this command is off" can only be enforced where the operation is dispatched.
The policy is generic — it names agents, commands and credential slots, never a provider.

## File

`<AWARE_HOME>/command-policy.json`, written by `aware policy` (see
[`cli-spec.md`](./cli-spec.md#aware-policy-showsetdenyremove)). No file means no policy: everything is
allowed, exactly as without this feature.

```json
{
  "schema": "aware.command-policy/v1",
  "rules": [
    {
      "id": "channel-reads-off",
      "agent": "microsoft-365",
      "command": "teams.channel.list-messages",
      "accounts": "*",
      "except": [
        { "slot": "microsoft-365.work", "binding": "0f8c3b8e-6d1a-5b7e-9c2d-4e5f6a7b8c9d" }
      ],
      "note": "free text for the host; never interpreted"
    }
  ]
}
```

Every rule is a deny. Unknown fields anywhere are invalid.

| Field | Required | Meaning |
|---|---|---|
| `id` | yes | `[a-z0-9][a-z0-9._-]*`, at most 128 characters, unique. Named by `aware policy remove` and by a refusal. |
| `agent` | yes | The exact agent id. |
| `command` | yes | The exact command name, or `"*"` for every command of the agent. |
| `accounts` | yes | `"*"` (every account) or a non-empty list of selectors. |
| `except` | no | Selectors exempt from this rule. Each **must** carry a `binding`. |
| `note` | no | At most 1024 characters, kept for the host. |

A **selector** is `{ "slot": <slot>, "binding"?: <binding id> }`.

- `slot` is the exact credential slot the dispatch uses: `<integration>` or `<integration>.<alias>` for a
  connected integration (the manifest's `auth.secret`, a baked `<integration>.<alias>`, `--as <alias>`, or
  a bound call's `alias`), a custom handle for an opaque credential, `vision-model` for `vision.extract`.
- `binding` is the account's binding id — the `bindingId` `aware agent call-capabilities` reports and
  `aware agent call` requests carry. It is derived from the integration, the slot, the verified principal
  and (Microsoft) the tenant, so it names *this account in this slot*.

## Which selector to use

A slot can be re-signed to another account, and an account can be connected under another slot.

- A deny by **slot** keeps denying whoever holds the slot (a re-sign is still denied). It does not follow
  an account into a different slot.
- An exception by slot alone is not accepted: re-signing the slot would hand the exception to an account
  that never opted in. An exception names a `binding`; a re-signed slot has a different binding, so the
  exception stops matching.
- For **off by default, on per account**, write `accounts: "*"` and list each opted-in account under
  `except` with its binding. A new account, a re-signed slot and an account moved to another slot are all
  denied until the host adds them.
- A deny by slot **and** binding denies exactly that account in that slot.

## Evaluation

For a dispatch of `agent`/`command` through `slot`, with the account either verified (its binding is
known) or not:

1. Absent policy: allow.
2. Unreadable policy (see Fail-safe): refuse, reason `policy-unreadable`.
3. A rule that could not be read, whose `agent` is this agent: refuse, reason `policy-unreadable`.
4. For each rule with this `agent` and this `command` (or `"*"`):
   - it **applies** if `accounts` is `"*"`, or a selector has this slot and either no binding or the
     verified binding; it **may apply** if a selector has this slot and a binding but the account is not
     verified;
   - it is **exempted** if an `except` selector has this slot and the verified binding; it **may be
     exempted** if one has this slot and the account is not verified;
   - applies and is not exempted (nor may be): refuse at once, reason `denied`, with the rule id.
5. If any rule was left undecided by an unverified account, the answer depends on the account. A path that
   can verify it does so (below) and decides again; a path that cannot refuses, reason
   `account-unverified`.
6. Otherwise allow.

**Verifying the account.** The reviewed operations (the code-owned operation table behind `aware agent
call`, which workflow nodes and `aware agent invoke` of those commands also run through) can read the
account: they reuse the bound call's own identity read — the pinned identity endpoint and, for Microsoft,
the tenant read and possibly a refresh through the pinned token endpoint, under the call's deadline. This
happens only when a rule's answer depends on the account, and needs the identity scopes (without them the
dispatch is refused as `account-unverified`). Every other path has no identity read, so an exception by
binding can never exempt there and a binding-qualified deny always applies.

## What is governed

A dispatch is governed when it uses a credential AWARE resolves: a REST agent that declares `auth` (its
commands marked `no-auth` included — the pinned flows resolve the manifest credential themselves, so the
flag is not a way out), a reviewed operation, a probe that attaches a credential, `vision.extract` when it
uses the Anthropic API key. Not governed: an agent that declares no credential, `vision.extract` through
the local `claude`/`codex` CLI, `aware agent call-capabilities` (it reads the identity and runs no
command), and `{{ secrets.x }}` values an app templates in itself.

## When it takes effect

The file is read from disk at every check, so a change takes effect without restarting anything:

- every operation **admitted** after the change is decided by it;
- the bound call, the reviewed operations and the generic REST request also decide again after the
  credential is resolved (and, when needed, the account verified), immediately before the data request —
  so a rule written during a token refresh still stops that request;
- the pinned flows (`gmail.send`, the Trimble Connect file, folder and BCF operations), the probe and
  `vision.extract` are checked at admission only;
- a long-running watcher's downstream nodes are separate operations: the next event after a deny is
  refused, the watcher keeps running, and the event after the rule is removed runs again.

## Refusal

Code **`E_COMMAND_DENIED_BY_POLICY`**, exit status 5, message *"This command is turned off for this
account by the command policy, so the command was not sent."*, with `reason` (`denied`,
`account-unverified`, `policy-unreadable`) and, when a rule caused it, `rule` (its id).

- `aware agent call` / `aware agent probe`: the verb's own failure envelope, `details.reason` and
  `details.rule`.
- `aware agent invoke` and workflow nodes: a structured agent error (`code`, `phase: dispatch`,
  `retryable: false`, `details` with `reason`, `rule`, `agent`, `command`), so a run's `node-error` event
  keeps the fields under `structured`.

## Fail-safe

- **Absent** means opening the path reports *not found*. Only that allows.
- The file is opened without following a link and read as one bounded snapshot (256 KiB) from that one
  handle, which must be a regular file. A link (even dangling), a junction, a directory, a FIFO, a
  permission error, an oversize file, bytes that are not UTF-8 JSON, a missing or other `schema`, an
  unknown top-level field or a missing `rules` array make the whole policy **unreadable**: every governed
  dispatch is refused with `policy-unreadable`. AWARE cannot tell which commands the host meant to deny,
  and a silent allow is the one outcome the policy exists to prevent. Ungoverned dispatches are not
  affected.
- One invalid rule whose `agent` is readable refuses only that agent's governed dispatches
  (`policy-unreadable`, with the rule id when it is readable); every other rule still applies. A rule with
  no readable `agent` makes the whole policy unreadable. Two rules with the same `id` are treated
  the same way: which one was meant is unknown, so every agent either names refuses as
  `policy-unreadable` with that id.
- The CLI never writes a file a reader would refuse: it validates every rule, refuses duplicate ids,
  checks the written size against the cap, and replaces the file atomically. `aware policy deny` and
  `remove` refuse to merge into an unreadable file (they would drop the rules they cannot read); `aware
  policy set` replaces it.

The policy keeps AWARE from dispatching what a host denied. It is not a defence against a local process
that can write `AWARE_HOME`, which could rewrite the policy itself; the hardening above makes mistakes fail
closed.
