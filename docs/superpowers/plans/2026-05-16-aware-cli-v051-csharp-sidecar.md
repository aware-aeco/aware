# AWARE CLI v0.5.1 (C# NativeAOT Sidecar) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development.

**Goal:** Replace v0.5's tier-C stubs (`--from-dlls`, `--decompile`) with real implementations powered by a C# NativeAOT sidecar binary that the Rust CLI invokes over stdin/stdout JSON. `--from-com` and `--from-headers` stay stubbed for v0.5.2+ (COM is Windows-only API surface; headers want libclang — both worth their own design phase).

**Architecture:** A new sub-project at `cli-sidecar/` is a self-contained .NET 9 console app with NativeAOT enabled (`<PublishAot>true</PublishAot>`). Builds to a single `aware-sidecar.exe` (or `aware-sidecar` on Linux/macOS) that lives alongside `aware.exe` in releases. The Rust CLI spawns the sidecar as a child process; one JSON request goes in on stdin, one JSON response comes back on stdout. The sidecar uses `System.Reflection.MetadataLoadContext` for DLL reflection (load-without-execute, cross-platform) and shells to `ilspycmd` (a separate dotnet global tool) for decompile.

**Why this architecture:** The v0.1 Rust-validation pass flagged DLL reflection as the place Rust would bleed (PowerShell-by-subprocess is fragile, `clr` bindings don't exist for AOT-friendly Rust). A separate sidecar:
1. Keeps the Rust CLI 100% pure-Rust and small
2. Uses the *right* tool for .NET work (the .NET runtime itself)
3. NativeAOT means no .NET runtime required at user-install time — single binary that boots fast
4. The IPC contract isolates the two languages cleanly

---

## Open design decisions (resolved here)

| Decision | Choice | Why | Alternative |
|---|---|---|---|
| **Project location** | `cli-sidecar/` at repo root (sibling of `cli/`) | Mirrors `cli/`; both are CLI-stack components | Under `cli/sidecar/` — makes Rust + C# tooling overlap in one dir |
| **.NET version** | .NET 9 (stable, latest LTS-adjacent; user has SDK installed) | NativeAOT support is mature; smallest binaries | .NET 8 (also good); .NET 10 (RC, premature) |
| **NativeAOT** | `<PublishAot>true</PublishAot>` — yes | No .NET runtime at install time; ~15 MB binary; fast boot | Self-contained non-AOT (~70 MB; .NET runtime bundled) |
| **IPC shape** | One-shot per process invocation: JSON line in on stdin, JSON line out on stdout, process exits 0/non-zero. Re-spawn per `aware build agent` call | Simple; no IPC framing; no long-running daemon to manage | Persistent JSON-RPC server (more complex; faster for batch ops; not needed yet) |
| **Request/response schema** | `{"op": "<op>", "args": { ... }}` → `{"ok": true, "data": { ... }}` or `{"ok": false, "error": "..."}`. `op` is one of `reflect-dlls`, `decompile`. Args + data shapes documented per op | Mirrors v0.1's response envelope; serde-friendly | Verb-as-CLI-arg (`aware-sidecar reflect-dlls --path foo`) — works too but JSON-in is more flexible for nested args |
| **Sidecar binary discovery** | Try, in order: (1) `AWARE_SIDECAR` env var, (2) sibling of `aware.exe` (alongside the running binary), (3) `aware-sidecar` on PATH. Error with clear "install the sidecar from https://github.com/aware-aeco/aware/releases" if missing | Standard pattern for CLI-with-helper binaries (git uses this for git-lfs, az uses for az-cli-iot, etc.) | Bundle the sidecar in the same binary (impossible — different runtime) |
| **Tier-C coverage in v0.5.1** | `--from-dlls` (full), `--decompile` (full). `--from-com`, `--from-headers` remain stubbed | DLL reflection covers the original v0.1 use case (Tekla agent generation); decompile is the closely-related decompile-as-fallback for closed-source NuGet packages. COM is Windows-only and a different .NET API; headers need libclang. Both are worth their own phases | Do everything in v0.5.1 (too big; ~5+ days) |
| **`ilspycmd` dependency** | `dotnet tool install -g ilspycmd` required on the user's machine; sidecar shells to it. Sidecar errors clearly if not present, pointing at the install command | ilspycmd is .NET-native, actively maintained, MIT-licensed. Embedding ILSpy as a library is non-trivial with NativeAOT due to reflection | Embed ILSpy as a library (heavyweight); embed dnlib (smaller but different API) |
| **License-aware decompile** | Reads the package's nuspec license (already extracted by v0.5 nuget builder); blocks if non-permissive unless `--accept-license` is passed | Matches the cli-spec.md "license-checked" qualifier; consistent with v0.5 nuget license gate | Always allow (security/legal risk); always block (rules out legitimate use) |
| **Cross-platform** | v0.5.1 ships Windows-x64 NativeAOT binary built in CI. Linux + macOS deferred to v0.5.2 (NativeAOT supports them but each platform needs its own runner) | Aligns with user's Windows-primary dev environment; CI matrix for cross-platform is its own work | Cross-platform in v0.5.1 (extends scope significantly) |
| **Test strategy** | C#: xUnit tests inside `cli-sidecar/` for DLL reflection logic (against a tiny fixture assembly built at test-time). Rust: integration test that spawns the *actual* built sidecar binary against the same fixture assembly | Catches both layers; the IPC contract is exercised end-to-end | Mock the sidecar in Rust tests (loses the integration value) |
| **Versioning** | Sidecar version-string in its response envelope: `{"version": "0.5.1", "ok": true, ...}`. Rust CLI checks the major.minor matches the CLI version and warns on mismatch | Catches sidecar/CLI version skew during dev | Skip versioning (works until it doesn't) |
| **Build pipeline** | Add `cli-sidecar/` build steps to a new `.github/workflows/build-sidecar.yml`. Triggered on tags; produces `aware-sidecar-<platform>.zip` artifacts | Standard release artifact pattern | Bake into `cli/` build — couples release cycles unnecessarily |

---

## Scope discipline

**v0.5.1 user-facing surfaces:** none new. The four tier-C stubs in v0.5 stay as the same CLI surface; their internal implementation flips from "return NotYetImplemented" to "spawn sidecar". So `aware build agent --from-dlls foo.dll` becomes a working command instead of an error.

**v0.5.1 internal surfaces:**
- `cli-sidecar/` — new .NET 9 NativeAOT project
- `cli-sidecar/Program.cs` — main entry, JSON-in/out dispatch
- `cli-sidecar/Reflection/` — DLL reflection handler using `MetadataLoadContext`
- `cli-sidecar/Decompile/` — ilspycmd shell-out + license check
- `cli-sidecar/Protocol/` — Request/Response DTOs (System.Text.Json)
- `cli/src/sidecar.rs` (new) — Rust client: discover sidecar binary, JSON IPC
- `cli/src/builder/stubs.rs` — rewire `build_from_dlls` and `build_with_decompile` to call the sidecar; `build_from_com` and `build_from_headers` keep their existing NotYetImplemented errors

**Out of scope** — stays stubbed:
- `--from-com` (Windows-only; needs COM `ITypeLib` interop in the sidecar; v0.5.2)
- `--from-headers` (needs libclang via P/Invoke or shell to clang.exe; v0.5.3)
- Linux + macOS sidecar builds (CI matrix; v0.5.2)

---

## File Structure

### New files

| Path | Responsibility |
|---|---|
| `cli-sidecar/cli-sidecar.csproj` | .NET 9 NativeAOT project file. |
| `cli-sidecar/Program.cs` | Entry point: read JSON from stdin, dispatch, write JSON to stdout. |
| `cli-sidecar/Protocol/Request.cs` | Polymorphic request DTOs (`ReflectDllsRequest`, `DecompileRequest`). |
| `cli-sidecar/Protocol/Response.cs` | Response DTOs (typed `data` payload per op). |
| `cli-sidecar/Reflection/DllReflector.cs` | `MetadataLoadContext`-based DLL reflection → list of types + members + XML doc. |
| `cli-sidecar/Decompile/IlspyShell.cs` | Wraps `ilspycmd`; verifies it's installed; runs against a DLL; AI-summarize-deferred (returns raw decompiled per type). |
| `cli-sidecar/README.md` | What this is + how to build + how the IPC contract works. |
| `cli-sidecar/Tests/SidecarTests.csproj` | xUnit project for sidecar logic (DLL reflection against a fixture). |
| `cli-sidecar/Tests/DllReflectorTests.cs` | xUnit tests for DLL reflection. |
| `cli-sidecar/Tests/FixtureAssembly/FixtureAssembly.csproj` | A tiny test-only assembly the reflection tests load. |
| `cli-sidecar/Tests/FixtureAssembly/Types.cs` | Test-only types with XML doc comments. |
| `cli/src/sidecar.rs` | Rust client for the IPC protocol. |
| `cli/tests/sidecar_integration.rs` | End-to-end test: spawn real sidecar binary, reflect the fixture assembly, verify the agent shape. |
| `.github/workflows/build-sidecar.yml` | CI to build + publish sidecar artifacts on tags. |

### Modified files

| Path | Change |
|---|---|
| `cli/src/builder/stubs.rs` | `build_from_dlls` + `build_with_decompile` now call into `crate::sidecar`. The two `build_from_com` and `build_from_headers` stubs keep their existing NotYetImplemented (different message — "v0.5.2+ separate phase"). |
| `cli/src/main.rs` | Add `mod sidecar;`. |
| `cli/README.md` | New "Sidecar" section explaining the binary, install location, and `ilspycmd` dependency. |
| `.gitignore` | Add `cli-sidecar/bin/`, `cli-sidecar/obj/`, `cli-sidecar/Tests/bin/`, `cli-sidecar/Tests/obj/`. |

### Key IPC schema

**Request** (one JSON object per line on stdin):

```json
{ "op": "reflect-dlls", "args": { "globs": ["C:/foo/*.dll"], "agent_id": "optional-override" } }
```

```json
{ "op": "decompile",    "args": { "package_path": "C:/foo.nupkg", "version": "1.0", "agent_id": "optional", "accept_license": false } }
```

**Response** (one JSON object on stdout, process exits 0):

```json
{ "ok": true, "version": "0.5.1", "op": "reflect-dlls", "data": {
    "agent": {
      "id": "tekla", "version": "0.1.0", "description": "...", "license": "...",
      "commands": [ { "name": "...", "description": "...", "lifecycle": "single" }, ... ],
      "skills":   [ { "filename": "...", "body": "..." }, ... ]
    }
} }
```

```json
{ "ok": false, "version": "0.5.1", "error": "ilspycmd not found; install with: dotnet tool install -g ilspycmd" }
```

---

## Tasks

### Task 0: Branch + baseline (done)

### Task 1: Scaffold `cli-sidecar/` C# project

- Create `cli-sidecar/cli-sidecar.csproj` targeting .NET 9 with NativeAOT enabled
- Minimal `Program.cs` that just reads a JSON line, echoes it back wrapped in `{"ok": true, "echo": <line>}`, exits
- `dotnet build` succeeds; `dotnet publish -c Release -r win-x64` produces a single `aware-sidecar.exe`
- `.gitignore` entries

Commit: `feat(cli-sidecar): scaffold .NET 9 NativeAOT project`

### Task 2: Define Protocol DTOs (C# side)

```csharp
public abstract record Request(string Op);
public sealed record ReflectDllsRequest(string[] Globs, string? AgentId) : Request("reflect-dlls");
public sealed record DecompileRequest(string PackagePath, string Version, string? AgentId, bool AcceptLicense) : Request("decompile");

public sealed record Response<T>(bool Ok, string Version, string Op, T? Data, string? Error);

public sealed record GeneratedAgent(
    string Id, string Version, string Description, string License,
    GeneratedCommand[] Commands, GeneratedSkill[] Skills
);
public sealed record GeneratedCommand(string Name, string Lifecycle, string Description);
public sealed record GeneratedSkill(string Filename, string Body);
```

Use `System.Text.Json` with source-generated serialization (NativeAOT-compatible). Unit tests for serialization round-trips.

Commit: `feat(cli-sidecar): protocol DTOs + source-generated JSON`

### Task 3: DLL reflection via `MetadataLoadContext`

`Reflection/DllReflector.cs`:

```csharp
public static GeneratedAgent Reflect(string[] globs, string? agentIdOverride)
{
    // Resolve globs to file paths
    // Build a MetadataLoadContext with the dll directories as search paths
    // For each assembly: enumerate public types, public methods on each
    // Produce one command per <namespace>.<type>.<method> (or grouped — needs design)
    // Look for sibling .xml doc files; extract per-member <summary>
    // Return GeneratedAgent with commands + skills (skills = per-namespace API reference markdown)
}
```

Tests: against the fixture assembly. Verify expected types appear, expected methods become commands, XML doc summaries appear in descriptions.

Commit: `feat(cli-sidecar): DLL reflection via MetadataLoadContext`

### Task 4: Decompile via `ilspycmd` shell-out

`Decompile/IlspyShell.cs`:

```csharp
public static GeneratedAgent Decompile(string packagePath, string version, ...)
{
    // Check `where ilspycmd` (Windows) / `which ilspycmd` (Unix); error if missing
    // Run `ilspycmd <path>/<dll>` for each DLL inside the nupkg
    // Parse the C# output, extract public types + summaries
    // (For v0.5.1: lightweight — just emit one skill per decompiled file with the C# wrapped in a code block)
    // Return GeneratedAgent
}
```

For v0.5.1 ship a *lightweight* version: ilspycmd produces raw C#; we emit it as skill content without further AI summarization. Future v0.5.2 adds AI-driven type-by-type extraction.

Commit: `feat(cli-sidecar): decompile via ilspycmd`

### Task 5: Wire `Program.cs` dispatch

Read one JSON line from stdin → deserialize into `Request` (polymorphic by `op` field) → call the appropriate handler → serialize `Response<T>` → write to stdout → exit with 0 (success) or 1 (error).

Commit: `feat(cli-sidecar): wire stdin/stdout JSON dispatch`

### Task 6: NativeAOT publish + verify single-file binary

`dotnet publish -c Release -r win-x64 -p:PublishAot=true` produces `cli-sidecar/bin/Release/net9.0/win-x64/publish/aware-sidecar.exe`.

Verify:
- Binary runs without .NET runtime installed
- Binary size is reasonable (~15-25 MB)
- A simple echo request round-trips correctly

Commit: `chore(cli-sidecar): NativeAOT publish + smoke verify`

### Task 7: Rust client — `cli::sidecar`

```rust
//! Discover + invoke the C# sidecar binary.

pub fn discover() -> Result<PathBuf, AwareError> { /* env var → sibling → PATH */ }
pub fn invoke<T: Serialize, U: DeserializeOwned>(op: &str, args: &T) -> Result<U, AwareError> {
    // Spawn child; write JSON to stdin; close stdin; read stdout; parse response
}
```

Unit tests: discovery logic (env var override → file existence checks). End-to-end test deferred to Task 9 (needs the built sidecar binary on disk).

Commit: `feat(cli): Rust client for the sidecar IPC`

### Task 8: Rewire `--from-dlls` and `--decompile` to call the sidecar

Update `cli/src/builder/stubs.rs`:
- `build_from_dlls` now: serialize args → `sidecar::invoke("reflect-dlls", ...)` → deserialize → convert to `GeneratedAgent`
- `build_with_decompile` similar
- `build_from_com` and `build_from_headers` keep their existing stub (update message: "v0.5.2+ — separate phase, tracked")

Commit: `feat(cli): rewire --from-dlls + --decompile to sidecar`

### Task 9: End-to-end integration test

Add `cli/tests/sidecar_integration.rs`:
- Skip if sidecar binary isn't on disk (CI gate)
- Build the C# fixture assembly at test time (`dotnet build cli-sidecar/Tests/FixtureAssembly/...`)
- Run `aware build agent --from-dlls <fixture-dll-path>` end-to-end
- Assert: agent folder is generated under `<AWARE_HOME>/agents/<id>/` with manifest + commands

Commit: `feat(cli): sidecar end-to-end integration test`

### Task 10: CI workflow — `.github/workflows/build-sidecar.yml`

GitHub Actions job that:
- Runs on tag pushes matching `v0.5.1*`
- Sets up .NET 9
- Runs `dotnet publish -c Release -r win-x64 -p:PublishAot=true`
- Uploads `aware-sidecar.exe` as a release artifact

Commit: `ci: build + publish aware-sidecar on tags`

### Task 11: README + final verification

Update `cli/README.md`:
- New "Sidecar (v0.5.1)" section explaining the binary, install location, and ilspycmd dep
- Update Status to "v0.5.1 — C# sidecar lifts the four tier-C stubs to two working (DLLs, decompile) and two pending-design (COM, headers)"
- Document where the sidecar binary should live (`alongside aware.exe`, or via `AWARE_SIDECAR=<path>`)

Run all checks:
- `cargo fmt --all -- --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test --all-targets`
- `dotnet test cli-sidecar/`
- `cargo build --release`
- Manual smoke: spawn the sidecar via `aware build agent --from-dlls cli-sidecar/Tests/FixtureAssembly/bin/Debug/net9.0/FixtureAssembly.dll`

Commit: `docs(cli): README + sidecar v0.5.1 docs`

---

## Self-review

**Spec coverage (v0.5.1 internal goals):**
- v0.5's `--from-dlls` and `--decompile` stubs become real → Tasks 1-8
- IPC contract documented + tested → Task 5 + 9
- NativeAOT single-binary build → Task 6
- CI artifact pipeline → Task 10

**Out of scope (acknowledged):**
- `--from-com` (Windows-only COM `ITypeLib`; v0.5.2)
- `--from-headers` (libclang or clang.exe shell; v0.5.3)
- Linux + macOS sidecar builds (CI matrix; v0.5.2)
- AI-driven type-by-type decompile summarization (v0.5.2)

**Realistic effort:** 2-3 days. The sidecar itself is ~600 LOC of C#; the Rust client is ~150 LOC; the rest is integration.
