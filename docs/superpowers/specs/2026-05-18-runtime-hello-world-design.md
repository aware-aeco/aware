# v0.29 — Runtime Hello-World Design

> **Status:** Brainstormed & approved 2026-05-18. Implementation plan to follow via `superpowers:writing-plans`.
>
> **Goal:** Close the substrate's biggest credibility gap — currently AWARE *describes* 3,000+ commands but cannot execute any of them. v0.29 ships the first executable loop: a single command fires the same message into Tekla Structures 2025/2026, Microsoft Teams, and Outlook email, with a signed receipt at the end.

---

## 1 — Architecture (the gap closure)

The runtime contract already exists in [cli/src/runtime/invoker.rs](../../../cli/src/runtime/invoker.rs):

- `CliInvoker::invoke_single` spawns the binary declared in `transport.cli.binary`
- Input: JSON on stdin (with `--json-stdin` flag)
- Output: JSON on stdout
- Failure: non-zero exit code with stderr text

v0.29 fills in **two** binaries that satisfy this contract — no Rust runtime changes needed:

```
                    aware app run hello-world
                              │
                  ┌───────────┴───────────┐
                  ▼ Orchestrator (Rust)   ▼ Stamped Receipt signer
                              │
       ┌──────────────────────┼──────────────────────┐
       ▼                      ▼                      ▼
 aware-tekla.exe       aware-microsoft-365.exe (×2 verbs)
 (C# NativeAOT)        (Rust binary)
       │                      │
       ▼                      ▼
 Tekla 2025 / 2026     Microsoft Graph API
 Open API DLLs         (Teams chat + Outlook mail)
 (loaded from          (OAuth token from
  installed path)      `~/.aware/credentials/microsoft-365.json`)
```

The Tekla path proves out-of-process desktop bridging. The Microsoft 365 path proves cloud API bridging. Together they prove the substrate can execute, not just describe.

---

## 2 — Generic Desktop-Host Sidecar Contract

**New spec doc:** `10-core/desktop-host-sidecar-spec.md`. Every desktop-host sidecar (Tekla, Revit, SketchUp, ArchiCAD, AutoCAD, Rhino, Navisworks, Allplan, Solibri, Bluebeam — current and future) must satisfy this contract. `aware-tekla` is the **reference implementation**.

### 2.1 Targeting flags (mandatory)

```
--version <X.Y>            target a specific version
--versions <X.Y,X.Y,...>   target multiple (orchestrator can also fan-out)
--pid <N>                  target a specific PID (precise routing)
--all                      target all running instances of supported versions
(none) + one instance      use that one (warn)
(none) + multiple          exit 4 with options listed
```

### 2.2 Standardized pre-flight (mandatory)

1. Enumerate running processes matching the host's exe-name pattern (`TeklaStructures.exe`, `Revit.exe`, `SketchUp.exe`, `acad.exe`, `rhino.exe`, etc.)
2. Parse version from each process's `MainModule.FileName` path
3. Pre-validate target ∈ running BEFORE any API call
4. **Per-instance precise routing** via host-appropriate mechanism:
   - COM ROT moniker bind (Tekla, AutoCAD, classic Revit)
   - In-host bridge with PID-keyed socket (Revit 2024+, SketchUp Ruby)
   - PID env var (host-specific)

### 2.3 Standardized exit codes (mandatory)

| Code | Meaning |
|---|---|
| 0 | Success |
| 1 | No matching instance running (pre-flight failure) |
| 2 | API call failed (host-side error) |
| 3 | Host not installed on this machine |
| 4 | Ambiguous target (multiple matches, no `--pid`) |
| 5 | Per-instance routing failed (ROT bind / IPC connect) |
| 6 | Credentials / permissions denied |

### 2.4 Standardized stdout receipt schema (mandatory)

```json
{
  "status": "ok|partial|err",
  "host": "tekla|revit|sketchup|...",
  "host_version": "2026.0",
  "host_pid": 14872,
  "host_session_id": "<stable-identifier>",
  "verb": "send-status",
  "verb_result": { "...": "verb-specific" },
  "delivered_at": "2026-05-18T07:30:00Z"
}
```

### 2.5 Manifest hook

New optional block in agent manifests for desktop hosts:

```yaml
host:                              # NEW optional block
  exe-name: TeklaStructures.exe    # what to grep for in `ps`
  versions-supported: [2025.x, 2026.x]
  routing: com-rot                 # com-rot | in-host-bridge | pid-env
```

`aware doctor` reads this to pre-check "you've installed `tekla` but no Tekla 2025/2026 is running".

### 2.6 Shared library

For v0.29 Tekla-only, the contract lives **inline** in `cli-tekla/`. When Revit/SketchUp arrive in v0.30+, factor common code into:
- `cli-sidecar-shared/` — C# library (process enumeration, version parsing, receipt builder, exit-code constants)
- `aware-sidecar-shared` — Rust crate (for HTTP-only sidecars like microsoft-365)

---

## 3 — `aware-tekla.exe` — the reference implementation

**Project:** new `cli-tekla/` directory, sibling to existing `cli-sidecar/`. Same C# NativeAOT toolchain.

### 3.1 Version detection at startup

```
1. Read AWARE_TEKLA_HOME env var (override for test rigs)
2. Else enumerate running TeklaStructures.exe processes
3. Parse version from each process's MainModule.FileName path
4. Match against --version / --versions / --pid / --all flag
5. Outcomes:
   - Requested ∈ running    → proceed
   - Requested ∉ running    → exit 1 with "Tekla 2026.0 requested but not running (found: 2025.0)"
   - No flag, one running   → proceed against that one (verbose)
   - No flag, multiple      → exit 4 listing options
```

Loads `Tekla.Structures.Model.dll` + `Tekla.Structures.dll` from the matched installation path. Version-agnostic — the curated `tekla` agent's verbs target the **stable Open API subset**.

### 3.2 Multi-instance precise routing (ROT binding)

**Why it matters:** When two Teklas run (e.g. 2025 + 2026), `new Tekla.Structures.Model.Model()` connects to the **most-recently-activated** instance using the default COM moniker. You can't pick.

**Solution:** Running Object Table (ROT) binding:

1. Get the running ROT via `GetRunningObjectTable(0, out IRunningObjectTable rot)`
2. Enumerate moniker entries
3. For each entry, get the bound object's owning PID
4. Match PID against the target Tekla process found in pre-flight
5. Call `rot.GetObject(targetMoniker, out object teklaCom)` to bind to that specific instance
6. Wrap with Tekla's Open API model class

**Caveat:** ROT binding is a Windows COM trick, not part of Tekla's official Open API. It works today but isn't a documented contract. The test matrix below catches regressions.

**Fallback:** if ROT binding fails (exit code 5), the sidecar reports the failure clearly and the operator can close all-but-one Tekla as a workaround.

### 3.3 Protocol

```
$ aware-tekla.exe send-status --version 2026.0 --json-stdin
< {"message": "Hello AWARE"}
> {"status":"ok","host":"tekla","host_version":"2026.0","host_pid":14872,
>  "host_session_id":"<model-guid>","verb":"send-status",
>  "verb_result":{"message":"Hello AWARE"},
>  "delivered_at":"2026-05-18T07:30:00Z"}
exit 0
```

### 3.4 Verbs in v0.29

Just `send-status`. Calls `Tekla.Structures.Model.Operations.Operation.DisplayPrompt(message)` — stable across Tekla 2024/2025/2026.

### 3.5 Test matrix (acceptance bar — all must pass before ship)

| # | Running Teklas | Target flag | Expected |
|---|---|---|---|
| 1 | 2026 only | `--version 2026.0` | ✓ delivers to 2026, receipt confirms PID |
| 2 | 2026 only | `--version 2025.0` | exit 1, "Tekla 2025.0 not running" |
| 3 | 2025 + 2026 | `--version 2026.0` | ✓ delivers **only** to 2026 (verified by receipt PID + version) |
| 4 | 2025 + 2026 | `--version 2025.0` | ✓ delivers **only** to 2025 |
| 5 | 2025 + 2026 | `--all` | ✓ delivers to both, two PIDs in combined receipt |
| 6 | 2025 ×2 (two 2025s) | `--pid <PID-A>` | ✓ delivers only to that PID |
| 7 | None running | `--version 2026.0` | exit 1, "no Tekla instance running" |

Tests 3–6 are the **ROT-binding tests** — they fail if precise routing isn't working. Manual E2E (no automated harness can manage two real Teklas).

---

## 4 — `aware-microsoft-365.exe` — the cloud bridge

**Project:** new `cli-microsoft-365/` directory. **Rust** (HTTP only; no .NET interop needed; smaller binary; matches main `aware` toolchain).

### 4.1 Authentication

Reuses `aware connect microsoft-365` (already shipped in v0.4). OAuth token + refresh token live at `~/.aware/credentials/microsoft-365.json`. Sidecar reads, refreshes if expired, sends. No new auth code.

### 4.2 Verbs in v0.29 (already declared in m365 manifest)

```
$ aware-microsoft-365.exe teams.chat.post-message --json-stdin
< {"chat_id": "19:abc...@thread.v2", "message": "Hello AWARE"}
> {"status":"ok","message_id":"1622...","delivered_at":"..."}

$ aware-microsoft-365.exe outlook.mail.send --json-stdin
< {"to": ["pawel@floless.io"], "subject": "Hello AWARE", "body": "..."}
> {"status":"ok","message_id":"AAMk...","delivered_at":"..."}
```

### 4.3 Failure modes

| Code | Meaning |
|---|---|
| 0 | Success |
| 6 | Token expired and refresh failed → user runs `aware connect microsoft-365` again |
| 2 | Graph API returned 4xx/5xx (logged with error code + body) |

### 4.4 Multi-tenant note

If `~/.aware/credentials/microsoft-365.json` holds multiple tenants in the future, `--tenant <id>` flag will select. v0.29 = single-tenant default.

---

## 5 — `hello-world.app` composition

**Location:** `30-apps/_examples/hello-world/hello-world.app`

### 5.1 App manifest

```yaml
app:     hello-world
version: 0.1.0
description: |
  Smoke-test composition that fires the same message through every
  configured destination concurrently. Proves the AWARE runtime can
  drive desktop hosts AND cloud APIs in one orchestrated run, with
  a signed receipt at the end.

inputs:
  message:
    type: string
    default: "Hello AWARE — the substrate runs."
  tekla-versions:
    type: list-of-strings
    default: ["2025.0", "2026.0"]
  teams-chat-id:
    type: string
    required: true                # --set teams-chat-id=19:abc...@thread.v2
  email-to:
    type: list-of-strings
    default: ["pawel@floless.io"]

do:                                          # sequential; runs in declared order
  - id: tekla
    agent: tekla
    command: send-status
    inputs:
      versions: "{{ tekla-versions }}"       # orchestrator still fans out per version
      message:  "{{ message }}"
  - id: teams
    agent: microsoft-365
    command: teams.chat.post-message
    inputs:
      chat_id:  "{{ teams-chat-id }}"
      message:  "{{ message }}"
  - id: email
    agent: microsoft-365
    command: outlook.mail.send
    inputs:
      to:       "{{ email-to }}"
      subject:  "AWARE Hello World"
      body:     "{{ message }}"

receipt:
  sign:      true
  operator:  "{{ env.AWARE_OPERATOR_ID }}"
```

### 5.2 Run command

```
aware app run hello-world \
    --set teams-chat-id=19:abc...@thread.v2
```

### 5.3 Orchestrator behaviour

1. Pre-flight: `safety:` block validation (already exists from v0.20)
2. Resolves `tekla-versions` → fans out `tekla.send-status` once per target version, each invoking `aware-tekla.exe --version <X.Y>` sequentially (the orchestrator already handles list-valued inputs this way for `for-each` blocks; we reuse the pattern)
3. Runs the three top-level nodes (`tekla`, `teams`, `email`) **sequentially** — adding a `parallel:` primitive is deferred to v0.29.x (smoke test doesn't need concurrency for correctness, only speed)
4. Collects per-node receipts (JSONL)
5. Signs combined receipt → `~/.aware/logs/hello-world/<instance>/<run-id>.jsonl` + `.sig` (Stamped Receipt v0.27 path)

### 5.4 Receipt structure

```jsonl
{"node":"tekla.send-status","host":"tekla","host_version":"2025.0","host_pid":11234,"status":"ok","at":"..."}
{"node":"tekla.send-status","host":"tekla","host_version":"2026.0","host_pid":14872,"status":"ok","at":"..."}
{"node":"microsoft-365.teams.chat.post-message","status":"ok","message_id":"...","at":"..."}
{"node":"microsoft-365.outlook.mail.send","status":"ok","message_id":"AAMk...","at":"..."}
{"node":"_receipt","app":"hello-world","operator":"pawellisowski","signed":true,"sig_path":"....sig"}
```

---

## 6 — Curated `tekla` agent upgrade (the model citizen)

Three concrete changes to [20-agents/aeco/engineering/tekla/manifest.yaml](../../../20-agents/aeco/engineering/tekla/manifest.yaml):

### 6.1 Version-agnostic sdk-target

Replace the current single-version pin (`sdk-target: 2025.0`) with a structured multi-version field:

```yaml
agent:        tekla
version:      0.1.0
sdk-target:
  family:    tekla-structures
  versions:  [2025.x, 2026.x]
  contract:  stable-open-api    # the common subset, not version-specific quirks
```

String shorthand (`sdk-target: "2025+"`) also allowed for simpler cases. Schema change in [cli/src/manifest/agent.rs](../../../cli/src/manifest/agent.rs).

### 6.2 Add `send-status` verb

In `commands:` block of `tekla/manifest.yaml`:

```yaml
send-status:
  lifecycle: single
  description: |
    Display a transient message in Tekla's status bar. Uses
    Operation.DisplayPrompt(message) — stable across Tekla 2025 / 2026.
    Useful as a connection smoke-test and for non-blocking user feedback
    during long-running compositions.
  inputs:
    message:
      type: string
      max-length: 256
  outputs:
    type: single
    schema:
      delivered_at:  string
      tekla_pid:     integer
      tekla_version: string
```

New file `20-agents/aeco/engineering/tekla/commands/send-status.md` documenting input/output, idempotency, and the API method used.

### 6.3 Add `host:` block (per the new sidecar spec)

```yaml
host:
  exe-name: TeklaStructures.exe
  versions-supported: [2025.x, 2026.x]
  routing: com-rot
```

### 6.4 Add `status-bar-messaging.md` skill

New file `20-agents/aeco/engineering/tekla/skills/status-bar-messaging.md`:

- When to surface messages to a Tekla user via status bar vs `TaskDialog` vs silent logging
- Cross-version stability map for `Operation.DisplayPrompt`
- Idempotency expectations

This becomes the **template** for the upgrade pattern. v0.30+ applies the same shape to `revit`, `sketchup`, `autocad`, etc.

---

## 7 — Testing strategy

### 7.1 Unit tests

- `cli-tekla/Tests/` — process enumeration, version parsing, exit-code mapping (mock Tekla DLLs via test doubles)
- `cli-microsoft-365/tests/` — Graph API request shape, token refresh logic (mock HTTP server)

### 7.2 Integration tests

- `cli/tests/app_run_hello_world.rs` — runs `aware app run hello-world --dry-run`, confirms orchestrator routes to the right sidecars with the right args (sidecars stubbed)

### 7.3 Manual E2E (acceptance bar)

The 7-row test matrix from §3.5 — requires actually-running Tekla 2025/2026 instances. Captured as a checklist in `30-apps/_examples/hello-world/README.md`. No automated CI for this (CI runners can't host Tekla).

---

## 8 — Documentation deliverables

- `docs/superpowers/specs/2026-05-18-runtime-hello-world-design.md` — this doc
- `10-core/desktop-host-sidecar-spec.md` — the generic contract
- `30-apps/_examples/hello-world/README.md` — demo walkthrough + manual test checklist
- `cli-tekla/README.md` — sidecar build + install + multi-instance notes
- `cli-microsoft-365/README.md` — sidecar build + Graph auth setup
- Update `10-core/cli-roadmap.md` — mark v0.29 = "runtime substrate ignites"
- Update curated `tekla` agent skills/commands per §6
- Update [00-vision/manifesto.md](../../../00-vision/manifesto.md) — replace any "the substrate executes" language with current reality + this ship as the proof

---

## 9 — Out of scope (explicitly deferred)

| Item | Why deferred | Ship target |
|---|---|---|
| Revit sidecar | In-process bridge required (.addin + DLL inside Revit); separate work | v0.30 |
| SketchUp sidecar | In-process Ruby bridge required | v0.31 |
| AutoCAD sidecar | Out-of-process possible but not in Hello World scope | v0.32 |
| Multi-tenant Graph | Single-tenant covers Hello World | v0.29.x |
| `parallel:` substrate primitive | Sequential `do:` proves correctness; concurrency is performance, not correctness | v0.29.x |
| Other curated agents upgraded to `send-status` pattern | Tekla is the template ship; sweep follows | v0.30+ |
| Code-signed MSI | Pre-existing deferred decision (open-source projects ship unsigned) | When SignPath OSS tier or similar lands |

---

## 10 — Success criteria

**v0.29 ships when:**

1. `aware app run hello-world` executes end-to-end against a real Tekla 2025 OR 2026 OR both running, plus a real Microsoft 365 tenant, and the operator sees:
   - Status-bar messages appear in each running Tekla instance
   - Teams chat message arrives
   - Email arrives at pawel@floless.io
   - Signed receipt at `~/.aware/logs/hello-world/<instance>/<run-id>.jsonl[.sig]`
2. All 7 rows of the Tekla test matrix (§3.5) verified manually
3. `aware-tekla.exe --help` and `aware-microsoft-365.exe --help` both render help per the sidecar spec
4. `aware doctor` reports the new `host:` block status for installed desktop agents
5. Manifesto + roadmap updated to reflect "the substrate now runs"
