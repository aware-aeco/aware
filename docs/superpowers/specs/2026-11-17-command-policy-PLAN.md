# Command policy, checked at operation dispatch — PLAN

Date: 2026-11-17. Context: floless.app R6.8 slice 4 (epic pawellisowski/floless.app#2465 row 10) wants
"read Teams channel messages" to be a per-account opt-in that is OFF by default. A host can gate its own
reads, but a workflow node, a routine or a long-lived watcher is dispatched by AWARE, which loads the
credential at each operation. So "off" can only be enforced inside AWARE, at the moment of the read.
The substrate change is generic: nothing provider-specific.

## 1. The file

`<AWARE_HOME>/command-policy.json` (next to `config.yaml`; one file, so a host's write is one atomic
replace). Absent file = no policy = every command allowed, exactly as today.

```json
{
  "schema": "aware.command-policy/v1",
  "rules": [
    {
      "id": "host-teams-channel-reads",
      "agent": "microsoft-365",
      "command": "teams.channel.list-messages",
      "accounts": "*",
      "except": [{ "slot": "microsoft-365.work", "binding": "<bindingId uuid>" }],
      "note": "optional free text for the host, never interpreted"
    }
  ]
}
```

Every rule is a DENY. Fields:

- `id` — required, unique, `[a-z0-9][a-z0-9._-]{0,127}`; what `remove` and the error name.
- `agent` — required, exact agent id. `command` — required, exact command name or `"*"` (all commands of
  that agent). The brief said "integration + command"; command names are only unique within an agent,
  and the integration is the base of the credential slot, which the account selector names.
- `accounts` — `"*"` (every account of the agent's credential) or a non-empty list of selectors.
- `except` — optional list of selectors exempt from this rule. Each MUST carry a `binding`.
- Selector: `{ "slot": "<integration>[.<alias>]" | "<custom handle>[.<alias>]", "binding"?: "<uuid>" }`.
  `slot` is the exact credential slot the dispatch resolves (manifest `auth.secret`, `--as`, a baked
  `<integration>.<alias>`, or a bound call's alias). `binding` is AWARE's existing
  `binding_id(integration, slot, sub, tenant)` — the same `bindingId` `agent call-capabilities` reports
  and `agent call` requests carry — i.e. slot + verified principal + (Microsoft) tenant.

## 2. Which selector is safe, and why

A slot can be re-signed to another account; an account can be connected under another slot.

- **Deny by slot** survives a re-sign (the new account is still denied: over-deny, the safe direction).
  It does not follow an account into a new slot, so a host that wants "off by default" writes
  `accounts: "*"` and lists opted-in accounts under `except`, never per-slot denies.
- **Exempt by slot alone would be unsafe**: re-signing the slot to someone else would silently inherit
  the opt-in. So an exception must name a `binding` (slot + principal + tenant). Re-sign → new binding →
  the exception no longer matches → denied.
- **Deny by binding** (slot + binding in `accounts`) denies exactly that account in that slot.
- **A binding needs a verified principal.** Where the dispatch cannot verify identity, the decision is
  made fail-safe: a binding-qualified deny counts as matching, and a binding exception does not exempt.
  Where it can (a reviewed operation of a provider with an identity read), AWARE reads the identity
  (the existing `verify_principal`, the same `/me` / userinfo / `/users/me` read a bound call makes —
  never the denied command) and decides exactly. That identity read happens only when a matching rule's
  outcome depends on it.

Evaluation for `(agent, command, slot, identity)` is three-valued per rule (rev. after Codex round 1):
for each rule with `agent` equal and `command` equal or `*`:
`applies` = Yes when `accounts` is `*` or a selector matches by slot with no binding, or by slot and the
verified binding; Unknown when a selector matches by slot, carries a binding, and identity is unverified;
No otherwise. `exempt` = Yes when an `except` selector has the slot and the verified binding; Unknown when
one has the slot and identity is unverified; No otherwise. Then: `applies` No or `exempt` Yes → the rule is
silent; `applies` Yes and `exempt` No → **Deny** at once (that rule's id); any other combination → the rule
is undecided. After all rules: any undecided → **NeedsIdentity**, else **Allow**. So a definite deny by
any rule wins over another rule that is only undecided, and with a verified identity nothing is undecided.

## 3. Where it is checked (every operation, re-read each time)

The policy is read from disk at every check (small file, size-capped), so a long-running watcher picks up a
new deny on its next event with no restart, and removing it re-allows on the next event.

The policy governs dispatches that carry an AWARE-resolved credential slot. An agent that declares no
credential has no account and is not consulted. A command marked `no-auth` of an agent that DOES declare
one is governed anyway (rev. after Codex round 1): the pinned flows (`gmail.send`, Trimble Connect
upload/download/ops/BCF) resolve the manifest credential themselves whatever `no-auth` says, so the flag
must not be a way out; governing a public command too errs on the safe side. Every such path:

1. **`aware agent call @req`** (`agent_call::call_within`): after the operation is resolved, before any
   credential is read: Deny → refuse; NeedsIdentity → decided again after the bound call's own
   `verify_principal`, before the data request.
2. **Reviewed operation from a workflow node or `aware agent invoke`** (`agent_call::run_reviewed`):
   before the credential: Deny → refuse; NeedsIdentity → after `load_slot`, verify the principal (needs
   the identity scopes, else refuse as unverified), then decide, before `read_data`. Again right before
   `read_data` is not needed beyond this single post-identity decision.
3. **Every other REST dispatch** (`RestInvoker::invoke_inner`, which `aware agent invoke` and every
   workflow REST node — one-shot, chained, per-event downstream of a watcher, nested exposed app — go
   through; including the pinned flows `gmail.send`, Trimble Connect upload/download/ops/BCF): at the
   top, before any credential resolution or network; for the generic request only (not the pinned
   flows, which return before it — see Revocation semantics) again immediately before the HTTP request,
   after credential resolution/refresh: slot = `--as` handle or manifest `auth.secret` (none only when
   the manifest has no `auth`). Deny or NeedsIdentity → refuse (no identity read exists there).
   Reviewed operations are delegated to (2), which decides with identity.
4. **`aware agent probe`** (`probe::probe_agent`): the declared probe command, after the credential plan
   is known and before the credential is loaded. NeedsIdentity → refuse.
5. **Builtin `vision.extract`** (rev. after Codex round 1): the one builtin that sends an AWARE-held
   credential (the Anthropic API key in `credentials/vision-model.json`). Governed as agent `vision`,
   command `extract`, slot `vision-model`, checked before the API request; the local `claude`/`codex` CLI
   providers use no AWARE credential and are not governed.

Revocation semantics (rev. after Codex round 2): a rule takes effect for every operation ADMITTED after it
is written — that is the guarantee. In addition, three paths re-read it after the credential is resolved
and immediately before the data request, for exactly the slot just resolved: the bound call, the reviewed
executor, and the generic REST request (its re-check sits inside the credential-injection block and uses
the slot of that same manifest snapshot, so a manifest swapped mid-dispatch cannot make the two checks
look at different slots). The pinned flows (`gmail.send`, Trimble Connect upload/download/ops/BCF), the
probe and `vision.extract` are admission-checked only: a rule written during their own credential refresh
does not stop that one in-flight operation; the next one is refused. Per-event downstream nodes of a
watcher are separate operations, so the next event is refused.

Trust model: the policy stops AWARE from dispatching what a host denied. It is not a defence against a
local process that can write `AWARE_HOME` — that process can rewrite the policy itself. Hardening against
link swaps and manifest swaps is there so a mistake fails closed, not as a security boundary.

The identity verification added for NeedsIdentity reuses `verify_principal` unchanged: the pinned identity
endpoint (`/me`, userinfo, `/users/me`), for Microsoft the `/organization` tenant read and possibly the
pinned refresh-token grant, under the same remaining-budget callback and generation checks. It needs the
identity scopes; without them the dispatch is refused as `account-unverified` (the command is not sent,
though the slot's token refresh may already have happened). Those extra
requests are made only when a rule's answer depends on the account.

Not covered by design: `agent call-capabilities` (reads identity only, runs no command); app-templated
`{{ secrets.x }}` values (an app author's own secret, not an agent credential slot); CLI-transport agents
(the manifest `auth` block is only ever resolved by the REST transport, the reviewed executor and probe).

## 4. Refusal

Stable code **`E_COMMAND_DENIED_BY_POLICY`**, exit 5 (permission denied), plain message
"This command is turned off for this account by the command policy, so the command was not sent." with
details `reason` (`denied` | `account-unverified` | `policy-unreadable`) and `rule` (the rule id, when one).
On `agent call` and `agent probe` it is the verb's own failure (`CallFailure`/`ProbeFailure`, same JSON
envelope as every call/probe error, exit 5). On invoke / workflow it is a structured
`AwareError::AgentStructured { code: E_COMMAND_DENIED_BY_POLICY, phase: dispatch, details: {reason, rule,
agent, command} }` (rev. after Codex round 1), whose exit status is special-cased to 5, so a run record's
`node-error.structured` carries the fields and `aware agent invoke` prints them on stderr like every other
invoke error. A watcher keeps running; only that event's node fails (existing long-running semantics).

## 5. Fail-safe

- Absent means exactly: opening the path reports NotFound. That alone allows. The open never follows a
  link (Unix `O_NOFOLLOW | O_NONBLOCK`, Windows `FILE_FLAG_OPEN_REPARSE_POINT`), and the opened handle's
  own metadata must be a regular, non-reparse file within the cap; a symlink (dangling or not), a
  junction, a directory, a FIFO, a permission error are unreadable. The bytes are one bounded snapshot
  from that one handle, for dispatch and writers alike (rev. after Codex rounds 1 and 2).
- A writer never produces a file a reader would refuse: the serialized document (pretty-printed, with
  its newline) is checked against the cap before the replace, and an oversize `set`/`deny` is refused
  with the previous policy left in place.
- Present but unreadable, oversized (>256 KiB), not JSON, wrong `schema` →
  **every credential-bearing dispatch is refused** with `reason: policy-unreadable`. Credential-less
  commands are untouched. Rationale: AWARE cannot tell which agents the host meant to deny, and a silent
  allow is the one outcome the file exists to prevent.
- JSON and schema valid, but an individual rule invalid → if the rule's `agent` is a readable string,
  that agent's every command is refused (`policy-unreadable`, rule id if readable); the rest of the
  policy applies normally. Only a rule with no readable `agent` widens to the whole file. So a bad rule
  for one integration never bricks the others.
- Writers use the CLI, which validates strictly and replaces atomically (temp file in the same directory,
  fsync, `crate::fs::replace_file`), so a torn file cannot be produced by AWARE itself. Every writer holds
  one persistent sidecar lock, `<AWARE_HOME>/command-policy.json.lock` (exclusive `fs2`), from BEFORE it
  reads the current policy through the replace, so `deny`/`remove` never lose each other's updates; `set`
  is a whole-document replace, last writer wins. Readers (dispatch) take no lock: they see the old or the
  new file, never a mix.
- `aware policy show` reports an unreadable file as such (with the parse error), and `aware policy set`
  can replace it; `deny`/`remove` refuse to merge into an unreadable file (they would discard rules
  they cannot read) and point at `set`.

## 6. CLI

- `aware policy show [--json]` — the parsed policy (or the unreadable state).
- `aware policy set @<file>` — validate a whole policy document and replace atomically. What a host that
  owns the whole policy (FloLess) calls; idempotent.
- `aware policy deny <id> --agent A --command C [--account SLOT[=BINDING]]... [--except SLOT=BINDING]...
  [--note TEXT]` — upsert one rule by id (no `--account` = `*`).
- `aware policy remove <id>` — delete one rule (absent id is not an error, reported as `removed: false`).

With `--json` each verb prints the standard success envelope (`ok`, `data`, `meta`); a validation
failure prints `error: … [E_COMMAND_POLICY_INVALID|E_COMMAND_POLICY_UNREADABLE] …` on stderr with exit 3,
and a filesystem failure (cannot create the lock, the temp file, the replace) is an ordinary I/O error,
exit 1, as for other non-call verbs.

Documented in `10-core/cli-spec.md` (new `aware policy` section + filesystem layout line) and a new
contract `10-core/command-policy.md`.

## 7. Tests

Unit: evaluation table (slot / binding / `*` / except / NeedsIdentity / wildcard command), fail-safe
cases (malformed, wrong schema, oversized, one bad rule scoped to its agent, unknown field), atomic
write + upsert/remove. Dispatch: bound call refused before any request reaches the fixture; reviewed
workflow path refused, exempted with a matching verified binding (identity read happens, data read
happens), not exempted with another binding; REST `invoke_inner` refused before the fixture sees a
request and allowed after removal; per-dispatch re-read (deny written between two calls on the same
invoker). Integration (`assert_cmd`): `aware policy` verbs round-trip.

## 8. E2E (temp AWARE_HOME, no real account)

A local REST agent (`auth: bearer`, secret `local-test`) against a local HTTP server, a token via
`aware credential put local-test --from-env`, and a tiny app whose node calls the command:
allowed → `policy deny` → refused with the code → `policy remove` → allowed, through `agent invoke`
and `app run`; a long-running app whose CLI stream source emits periodic events, with the deny written
mid-run, showing the next event's REST node refused and the watcher still alive. `agent call` cannot
reach a real provider without a real account; it is shown refused by policy (before any credential)
versus reaching the credential stage (`E_CREDENTIAL_MISSING`) without the rule, and covered end to end
by the unit fixtures.
