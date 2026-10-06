# Issue #632 — host bridges declare a protocol version; an agent version can say which it works with

Part of pawellisowski/floless.app#1985 row 3b. Follow-up to #626 (versioned agent store).

## Problem

Host bridges (`aware-tekla`, `aware-revit`, `aware-rhino`, `aware-sketchup`, `aware-connection-reader`)
live in `<AWARE_HOME>/bridges/` and are versioned only by the CLI that installed them
(`bridges/<bin>.version`). Since #626 an approved workflow keeps running on an OLDER stored agent version
after the tool, or the CLI, moves on, so an old agent version can meet a bridge it was never written for.
Nothing says so: the failure appears inside the bridge, opaque. (Today's situation for anyone who updates
AWARE but not the agent — #626 added no new risk — but with migration (#628) and a longer-lived store the
gap matters more.)

## Design

### 1. A bridge protocol number

`BRIDGES` (cli/src/commands/sidecar.rs) gains `protocol: u32` per bridge: the wire/CLI contract the bridge
built in THIS release speaks (verbs, `--json-stdin` request shape, receipt/error envelope). Protocol **1**
is defined as "every bridge released up to and including the release that introduced this field". The
number is bumped, by hand, only in a release that makes an incompatible change to a bridge's contract.

* Where it is *reported from*: the asset a CLI downloads is versioned by that CLI's own release
  (`<bin>-<cli-version>-win-x64.*`), so the CLI that installed a bridge knows its protocol exactly. At install
  the CLI records it next to the version stamp: `<bin>.protocol` (decimal integer + newline). A test pins
  that each `BRIDGES` entry has `protocol >= 1`. We do NOT spawn the bridge to ask: probing would start a
  host-attached process at every dispatch/check for a value the installer already knows, and the C#/Node
  bridges stay untouched in this change (no bridge-side tests to run).
* **The marker is installer-attested, not bridge-reported**, and the docs say so. The value is trustworthy only
  as far as the install that wrote it. To keep it from outliving the bytes it describes, `sidecar install`
  deletes BOTH markers (`.protocol`, `.version`) before it touches the files, then writes `.protocol`, then
  `.version` last. An interrupted install therefore leaves "bridge present, no markers" = **unknown**, never
  a stale claim. (Hand-editing `bridges/` is out of scope, as everywhere in AWARE.)
* A managed bridge with no `.protocol` marker but WITH a `.version` marker below **0.156.0** (the first
  release that stamps the protocol) is protocol 1 — true by definition, since every earlier release is the
  baseline — reported `assumed: true`. A `.version` of 0.156.0 or later with no/garbled `.protocol`, a bridge
  with neither marker, a PATH-only legacy bridge and a binary that is not a managed bridge are **unknown**:
  AWARE makes no claim and says no more than that.
* **The verdict follows the executable dispatch selects.** Dispatch (`resolve_cli_binary` ->
  `find_bridge_by_binary`) matches `BRIDGES[].binary` EXACTLY, looks in the managed dir first
  (`find_bridge_in_dir`: flat `<dir>/<bin>.exe`, then `<dir>/<bin>/<bin>.exe`), and only then PATH. The
  protocol is read iff the declared `binary` equals a `BRIDGES` entry exactly AND `find_bridge_in_dir` finds a
  managed copy (that is then precisely what `spawn_cli` runs). Anything else — a `binary:` spelled
  `aware-tekla.exe` (which dispatch hands to PATH untouched), a bundled sibling, a PATH-only legacy copy — is
  `unknown` (`source: not-a-managed-bridge`) whatever markers sit in `bridges/`. No `.exe` normalisation is
  added (dispatch has none, and the verdict must describe dispatch, not what we wish it did). Tests: exact
  name + managed copy; `.exe`-suffixed name + managed copy -> unknown; PATH-only -> unknown.
* **A bad protocol stamp is a stale bridge, so install and repair fix it.** `bridge_is_current` (used by
  `sidecar install`'s skip check, `sidecar_status`/`repair --installed`, and `managed_bridge_is_current`)
  becomes: executable present AND `.version` == this CLI AND `.protocol` == this CLI's catalogue protocol for
  that bridge. A current `.version` with a missing, garbled or different `.protocol` is therefore `stale`
  (`repair --installed` selects it, `install` re-downloads instead of skipping). The only behavioural change
  to the older strict gates is in exactly that corrupt-stamp case (they then say "refresh the bridge"); in
  every ordinary state they decide as before. Test: `.version` current + each bad `.protocol` -> `stale`,
  install does not skip, repair selects it; ordinary current bridge -> `current`.
* **One executable, so the marker describes it.** `find_bridge_in_dir` prefers the flat copy over the sub-dir
  copy, so `sidecar install` also deletes BOTH candidate executables (flat and sub-dir) before extracting, and
  stamps `.protocol` only after re-running `find_bridge_in_dir` and finding the freshly written copy. If the
  asset produced no executable the protocol marker is not written (the version marker keeps today's
  behaviour). Test: both layouts pre-seeded with old bytes -> after install exactly the new executable exists
  and the stamp matches.
* `aware sidecar list --json` rows gain additive `installed-protocol` (u32|null) and `protocol` (the
  protocol the current CLI's catalogue entry speaks); `schema-version` stays 1 (additive).
* Uninstall removes the marker with the version marker.

### 2. A manifest declaration

`transport.cli.bridge-protocol` — optional, closed grammar, either bound optional but at least one present:

```yaml
transport:
  cli:
    binary: aware-tekla
    bridge-protocol: { min: 1, max: 2 }    # inclusive; omit a side for "no bound"
```

* Parsed from the raw YAML value (not a typed serde field) so a bad value in an already-installed manifest
  can never stop the manifest loading. PRESENCE is kept separate from value (a custom `deserialize_with`
  yields `Some(Yaml::Null)` for `bridge-protocol:` with no value, unlike a plain `Option<Value>`, which
  collapses null into absent): the parse result is `Declared(range) | Invalid(reason) | Absent`, and a
  present-but-null key is `Invalid("empty")`, never silently "no declaration".
* `validate_agent` (the one validator used by `agent validate` and both install routes) reports
  `E_AGENT_BRIDGE_PROTOCOL_INVALID` for a malformed block — non-mapping, unknown key, non-integer, < 1,
  `min > max`, or neither bound — the same way `E_PROBE_INVALID` holds the probe block to its grammar. That is
  authoring-time validation of the manifest's own contract, not a refusal to run something the person has.
  No bridge-protocol check on a manifest without a `cli` transport (the key lives under `cli`).
* Backward compatibility: no declaration → no claim → today's behaviour byte for byte. Older CLIs ignore
  the key (`TransportCli` has no `deny_unknown_fields`). Existing agents/bridges need no change; the shipped
  agents declare nothing in this change.
* The migration contract (#628) already treats `transport` as a projected top-level manifest key, so a
  changed declaration between two versions is reported as a changed executable contract (tested).

### 3. The verdict

One pure function `bridge_fit(declared, installed) -> BridgeFit`:
`NotDeclared | DeclarationInvalid{reason} | Unknown{why} | Fits{installed} | BridgeTooOld{installed, min} |
BridgeTooNew{installed, max}`. Two callers, one answer:

* **Which manifest.** Both callers read the manifest of the copy the run's resolver CHOSE — for a pinned
  approval that is the stored package, not the installed working copy (`assess_agent`'s `outcome.manifest` in
  check, the resolved catalogue's `DiscoveredAgent` in run), and the effective dispatch transport must be `cli`
  (`effective_transport`): an agent whose winning transport is `app`/`rest`/`builtin` carries no bridge verdict
  even if a `cli:` block with a declaration is also present. Nested: a backing app's leaves use that app's own
  resolution — rows with `via` in `app check`, `wrapper>leaf` keys in the run record (the existing keys).
* **`aware app check --json`** — each agent row (and backing-app leaf) whose chosen copy has a `bridge-protocol`
  key (valid or not) gains `bridge: {binary, declared:{min,max}, installed-protocol, assumed, status, detail, choices[]}`
  (`status`: `fits | bridge-too-old | bridge-too-new | unknown | declaration-invalid`). Rows without a
  declaration carry nothing (output unchanged). It does **not** change `approval-current`: a bridge mismatch
  is not an approval drift (`E_APP_LOCK_*`), it is a separate fact the front door can show as needs-you
  before anyone presses Run.
* **`aware app run`** — after the run's one resolution, for every resolved agent (nested leaves too) whose
  verdict is `BridgeTooOld`/`BridgeTooNew`: a warning on stderr that names both numbers and the choices, the
  finding recorded in the run record's `agent-resolution.<agent>.bridge`, and **the run proceeds**.
  `DeclarationInvalid` also warns ("the declaration could not be read, so no bridge check was made") and is
  recorded; `Unknown` is recorded and quiet on stderr (nothing to act on).
  `--simulate` dispatches nothing and checks nothing.

### 4. Nothing refuses, it asks (owner rule, floless.app CLAUDE.md)

The issue text says "Dispatch refuses a pinned agent whose declared range the installed bridge doesn't
satisfy". That is **replaced**: a mismatch is a warning with choices, never a hard stop. It is none of the
listed carve-outs (there is something to act on, it is not the `.flo`→`.lock` gate, it is not an
AI-governance line), and a refusal here would strand an unattended routine that might in fact work. The
choices, worded by direction:

* bridge too old (agent needs newer): `aware sidecar install <id>` (installs the bridge shipped with this
  `aware`; if the number does not move, `aware` itself is older than the agent needs — update `aware`) — or
  proceed knowing it may fail.
* bridge too new (the approved/stored tool version predates the bridge): update the tool
  (`aware agent update <agent>`) and compile the workflow again — or proceed.

Proceeding never makes the record lie: the finding is in the run record (`agent-resolution…bridge`), and
`app check` keeps reporting it. Unattended runs proceed on the default and batch the warning (stderr +
record) — they cannot ask mid-run.

Two older strict gates exist in `current_bridge_is_required` (invoker.rs) and stay exactly as they are:
`tekla.bake-scene` requires the managed `aware-tekla` stamped by THIS CLI version, and
`model-reference-reader` requires a current `aware-connection-reader`. They are CLI-version-equality
contracts for individual verbs that predate the owner rule; narrowing them is a separate decision and is not
bundled here. They coexist: the protocol verdict is evaluated and recorded first, independently, and the old
gates still refuse as before (existing tests unchanged; one new test asserts a `fits`/mismatch verdict does not
change either gate's outcome).

### 5. Docs / specs

`10-core/agent-spec.md` (the declaration, the baseline-1 definition, the verdict, the warning-not-refusal
rule), `10-core/app-spec.md` (`app check` `bridge` row + run record field), `10-core/cli-spec.md`
(`sidecar list --json` additive fields). `scripts/sync_stats.py --check` stays green (no agents added).

## Tests

Unit (pure): `bridge_fit` matrix (min only, max only, both, boundaries, assumed-baseline, unknown);
declaration parser (valid, each invalid shape → reason); `validate_agent` rejects malformed, accepts valid
and absent. Marker: install stamps `.protocol`; missing marker → assumed 1; garbled → unknown; uninstall
removes it. Integration (`cli/tests`): `app check --json` with a fixture agent declaring `min: 2` against a
managed bridge stamped 1 → `bridge-too-old`, approval-current unchanged; matching → `fits`; no declaration →
no `bridge` key; `app run` mismatch → warning on stderr AND the run proceeds AND the record carries the
finding. Contract: a changed declaration marks `transport` changed. Chosen-manifest test: installed working copy declares one range, the pinned stored package another — `app
check` and the run's record both report the PINNED copy's verdict. Nested: a leaf behind a backing app yields
a `via` row and a `wrapper>leaf` record. Presence: `bridge-protocol:` (null) -> `declaration-invalid`; absent
key -> no `bridge` object and byte-identical `app check` output for a manifest without it. Source: managed
copy absent + PATH name present -> `unknown`; interrupted install (no markers) -> `unknown`; `.version` 0.155.0
no `.protocol` -> assumed 1; `.version` 0.156.0 no `.protocol` -> `unknown`. CLI boundary (assert_cmd, real
`aware app run` against a fixture bridge stamped protocol 1 and a fixture agent declaring `min: 2`): exit 0,
the warning on stderr, the finding in the run record, the fixture bridge actually executed — while an absent
or unknown declaration adds no output. `approval-current` is asserted identical with and without the
declaration. The existing `current_bridge_is_required` tests stay green untouched.

## Out of scope (named)

Bridge-side `--protocol` verb in the C#/Node bridges (the installer-recorded value is the reporting
mechanism; revisit if bridges ever ship outside the CLI release); declaring ranges on the shipped agents
(first real use is the next incompatible bridge change); FloLess display of the verdict (follow-up filed on
floless.app after release); `tekla.bake-scene`'s strict gate.

## Amendments after code review (PR #cli)

* Codex round 1: `sidecar install` cleared the working bridge before the replacement was proven good. Now the
  asset is staged and checked (it must hold `<bin>.exe`) first; clearing and the move happen only after.
* pr-review-toolkit: the protocol stamp had leaked into `managed_bridge_is_current`, i.e. into the two older
  per-verb gates. Reverted: those gates use a version-only helper, exactly as before; the stamp counts only for
  `sidecar list`/`install`/`repair` (this replaces the plan's "narrow corrupt-stamp change to the gates"
  sentence above). A test pins that a bad protocol stamp never changes the gates.
