# AWARE CLI v0.5.2 (COM + Headers + Linux Sidecar) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development.

**Goal:** Lift v0.5.1's two remaining stubbed builder sources — `--from-com` (Windows COM TypeLib introspection) and `--from-headers` (C++ header parsing via clang). Add Linux x64 to the sidecar's CI build matrix. macOS sidecar builds deferred to v0.5.3.

**Architecture:** Both new builders live in the C# sidecar (`cli-sidecar/`), reusing the existing JSON-IPC contract. The Rust CLI's `cli::sidecar` client gets two new request types (`from-com` + `from-headers`) and the corresponding Rust-side wiring in `cli/src/builder/stubs.rs`. CI workflow gains a matrix dimension for `linux-x64` alongside the existing `win-x64`.

---

## Open design decisions (resolved here)

| Decision | Choice | Why | Alternative |
|---|---|---|---|
| **COM scope** | TypeLib **introspection only** — list coclasses + dispatch interfaces + methods. No live COM invocation in the sidecar. | A *build* command should be read-only against the type library; live invocation belongs at runtime via the (future) per-agent `transport.com` config | Embed COM invocation (overlaps with v0.3 runtime work; entangles concerns) |
| **COM API** | P/Invoke into `oleaut32!LoadTypeLibEx` + `ITypeLib::GetTypeInfo` + walk via `ITypeInfo::GetFuncDesc` | Stable Windows API since Win2k; standard pattern for type-lib browsers | Use `Microsoft.VisualStudio.OLE.Interop` (NuGet pkg, AOT-unfriendly) |
| **COM platform gate** | `[SupportedOSPlatform("windows")]` on the COM handler; on non-Windows the sidecar errors clearly. The Rust CLI surfaces "from-com is Windows-only" before even spawning the sidecar | Honest UX: don't pretend a Windows-only feature works elsewhere | Stub on non-Windows (more code; same UX) |
| **Headers approach** | Shell to `clang.exe -Xclang -ast-dump=json -c <header>`. Parse the JSON AST | clang is universally available; AST-as-JSON is the well-documented stable contract; lighter than bundling libclang (which is ~150 MB) | `libclang` via P/Invoke (clean API but huge dep); hand-rolled C++ parser (impossible to do right) |
| **Headers: what we emit** | One command per non-static public function (declared in the supplied header, not in transitively-included headers). XML doc-comment-style summaries when present. No class methods in v0.5.2 (free functions only) | Conservative scope; covers the most-common Win32 / C library headers; class methods add type tracking complexity | Full class + member coverage (way bigger scope) |
| **Headers dep on user machine** | clang on PATH required. Sidecar errors clearly if missing, pointing at install instructions per OS | Standard for build tools; matches the `ilspycmd` dep gate from v0.5.1 | Bundle clang (huge); skip if missing (silent failure) |
| **Linux sidecar build** | Add `ubuntu-latest` runner job alongside `windows-latest`. Same `dotnet publish` invocation with `-r linux-x64`. Output: `aware-sidecar` (no `.exe`) | Standard CI matrix; NativeAOT supports linux-x64 cleanly | Skip Linux entirely (incomplete promise of "cross-platform single binary") |
| **macOS sidecar build** | Deferred to v0.5.3. macOS NativeAOT works but the runners are slower + need `xcode-select` setup that has bitten people on shared runners. Better to nail Linux first | Avoid yak-shaving when the user's primary platform is Windows | Do all three at once (more upfront work; same outcome) |
| **Testing on Linux locally** | Not done in this session (no Linux env on the dev machine). Tests gated on `cfg!(target_os = "linux")` skip on Windows; CI is the only way to verify them | Pragmatic; matches the v0.5.1 keyring-test gating pattern | Spin up WSL (works but adds friction to the session) |
| **Provenance source field** | New types: `{ "type": "com", "progid": "..." }` and `{ "type": "headers", "files": [...] }` | Mirrors the existing nuget / openapi / python provenance shapes | Reuse "cli" generic type (loses specificity) |

---

## Scope discipline

**v0.5.2 user-facing surfaces:** none new. Same two CLI flags from v0.5 (`--from-com`, `--from-headers`) flip from `NotYetImplemented` to working. After this PR:
- `aware build agent --from-com <progid>` — works on Windows
- `aware build agent --from-headers <path-or-glob>` — works wherever clang.exe is on PATH

**v0.5.2 internal surfaces:**
- `cli-sidecar/Com/ComReflector.cs` — new C# module for TypeLib introspection
- `cli-sidecar/Headers/HeaderParser.cs` — new C# module for clang AST parsing
- Updated `cli-sidecar/Program.cs` — two new `op` cases
- Updated `cli-sidecar/Protocol/Request.cs` — `FromComArgs`, `FromHeadersArgs`
- Updated `cli-sidecar/Tests/` — xUnit tests for both
- Updated `cli/src/sidecar.rs` — `from_com()` + `from_headers()` Rust wrappers
- Updated `cli/src/builder/stubs.rs` — wire `build_from_com` + `build_from_headers` through the sidecar (today they return NotYetImplemented)
- Updated `.github/workflows/build-sidecar.yml` — add `ubuntu-latest` job

**Out of scope** — stays stubbed:
- macOS NativeAOT sidecar builds (v0.5.3)
- Live COM invocation at runtime (separate concern; agent runtime, not build)
- Class methods in C++ headers (free functions only for v0.5.2)
- C++ template / macro expansion (clang handles macros; templates need instantiation we don't do)

---

## File Structure

### New files

| Path | Responsibility |
|---|---|
| `cli-sidecar/Com/ComReflector.cs` | P/Invoke + `ITypeLib`/`ITypeInfo` walker → `GeneratedAgent`. Windows-only. |
| `cli-sidecar/Headers/HeaderParser.cs` | Shell to `clang -Xclang -ast-dump=json -c <file>`; parse the JSON; emit commands. |
| `cli-sidecar/Tests/ComReflectorTests.cs` | xUnit tests (gated on `cfg!(windows)` equivalent; uses a well-known ProgID like `WScript.Shell`). |
| `cli-sidecar/Tests/HeaderParserTests.cs` | xUnit tests against a tiny fixture `.h` file. Skips if clang isn't on PATH. |
| `cli-sidecar/Tests/fixtures/sample.h` | Fixture C header: a couple of free functions with doc comments. |

### Modified files

| Path | Change |
|---|---|
| `cli-sidecar/cli-sidecar.csproj` | No new NuGet deps (P/Invoke is built-in). Maybe drop the unused `MetadataLoadContext` ref — no, keep it; reflection still uses it. |
| `cli-sidecar/Protocol/Request.cs` | Add `FromComArgs` + `FromHeadersArgs` DTOs. |
| `cli-sidecar/Program.cs` | Add `case "from-com"` and `case "from-headers"` to the dispatch. |
| `cli/src/sidecar.rs` | Add `FromComArgs`, `FromHeadersArgs`, `from_com()`, `from_headers()`. |
| `cli/src/builder/stubs.rs` | Rewire `build_from_com` + `build_from_headers` through the sidecar. |
| `cli/tests/build_agent.rs` | Update the existing tests for these two flags from `NotYetImplemented` to "fails clearly if sidecar missing" or "succeeds if sidecar present" (skip gracefully). |
| `cli/tests/basic.rs` | Update the smoke-test sentinel since `--from-com` is no longer stubbed. Pick the next still-stubbed surface. **There isn't one** after v0.5.2 — all 8 sources work. Switch the sentinel to a different not-yet-implemented path: maybe `--decompile` without `--accept-license` on a non-permissive package, or `agent publish` (still stubbed). |
| `.github/workflows/build-sidecar.yml` | Matrix job: add `ubuntu-latest` runner with `dotnet publish -r linux-x64`. |
| `cli/README.md` | Status bumps to v0.5.2. Document clang dep + COM Windows-only constraint. |

---

## Tasks

### Task 0: Branch + baseline (done)

### Task 1: Sidecar — `Com/ComReflector.cs`

Use P/Invoke into `oleaut32.dll`. Resolve a ProgID → CLSID → TypeLib. Walk:
- For each `ITypeInfo` in the TypeLib
- If it's a coclass: emit a skill summarizing it
- If it's a dispatch interface: emit one command per `FuncDesc` (skip property getters/setters; mark them as inputs)

Implementation skeleton:

```csharp
using System.Runtime.InteropServices;
using System.Runtime.InteropServices.ComTypes;
using System.Runtime.Versioning;
using AwareSidecar.Protocol;

namespace AwareSidecar.Com;

[SupportedOSPlatform("windows")]
public static class ComReflector
{
    public static GeneratedAgent Reflect(string progId, string? agentIdOverride)
    {
        // Resolve ProgID → CLSID
        var hr = CLSIDFromProgID(progId, out var clsid);
        if (hr < 0) throw new InvalidOperationException($"unknown ProgID: {progId} (hr=0x{hr:X8})");

        // Open TypeLib associated with the class
        // Several strategies; the simplest: instantiate the object, query IDispatch, get its TypeInfo
        // OR: read the TypeLib path from registry (HKCR\CLSID\<clsid>\TypeLib)
        // For v0.5.2 use the simpler registry approach.

        // ... read TypeLib path from registry ...
        // ... LoadTypeLib(path) → ITypeLib ...
        // ... walk types ...

        return new GeneratedAgent { /* ... */ };
    }

    [DllImport("ole32.dll", CharSet = CharSet.Unicode)]
    private static extern int CLSIDFromProgID(string lpszProgID, out Guid clsid);

    [DllImport("oleaut32.dll", CharSet = CharSet.Unicode)]
    private static extern int LoadTypeLibEx(string szFile, RegKind regkind, out ITypeLib pptlib);

    private enum RegKind { Default = 0, Register = 1, None = 2 }
}
```

**Note for implementer:** the COM TypeLib walking is genuinely non-trivial. If you hit walls (HRESULT errors, ITypeInfo lifecycle issues, registry permission denials on certain ProgIDs), document the error and proceed to a *minimal viable* version:
- Just emit a single skill listing the discovered ProgIDs and CLSIDs
- One placeholder command named after the dispatch interface
- Provenance correctly populated

That's still useful (the user knows the COM object can be resolved + has the basics surfaced). Full method introspection can be v0.5.3.

Tests:
- Reflect against `WScript.Shell` (a universally-available COM object on Windows)
- Expect at least one command emitted; expect the agent id to default to a sensible kebab-case form of the ProgID
- Skip the test on non-Windows

Commit: `feat(cli-sidecar): COM TypeLib reflection (Windows)`

### Task 2: Wire `--from-com` end to end

Add to `cli-sidecar/Protocol/Request.cs`:

```csharp
public sealed class FromComArgs
{
    [JsonPropertyName("progid")] public string ProgId { get; set; } = "";
    [JsonPropertyName("agent_id")] public string? AgentId { get; set; }
}
```

Add to `Program.cs` dispatch:

```csharp
case "from-com":
{
    if (!OperatingSystem.IsWindows())
    {
        EmitError(envelope.Op, "--from-com is Windows-only");
        return 1;
    }
    var argsObj = envelope.Args.Deserialize(SidecarJsonContext.Default.FromComArgs);
    if (argsObj is null) { EmitError(envelope.Op, "missing args"); return 2; }
    try
    {
        var agent = AwareSidecar.Com.ComReflector.Reflect(argsObj.ProgId, argsObj.AgentId);
        EmitOk(envelope.Op, new ResponseData { Agent = agent });
        return 0;
    }
    catch (Exception ex) { EmitError(envelope.Op, ex.Message); return 1; }
}
```

Add to `cli/src/sidecar.rs`:

```rust
#[derive(Serialize)]
pub struct FromComArgs {
    pub progid: String,
    pub agent_id: Option<String>,
}

pub fn from_com(args: FromComArgs) -> Result<GeneratedAgent, AwareError> {
    let agent = invoke("from-com", &args)?;
    Ok(to_local_agent(agent, "com"))
}
```

Rewire `cli/src/builder/stubs.rs::build_from_com`:

```rust
pub fn build_from_com(progid: &str, agent_id: Option<&str>) -> Result<GeneratedAgent, AwareError> {
    if !cfg!(windows) {
        return Err(AwareError::NotYetImplemented("--from-com is Windows-only"));
    }
    let args = crate::sidecar::FromComArgs {
        progid: progid.to_string(),
        agent_id: agent_id.map(String::from),
    };
    crate::sidecar::from_com(args)
}
```

Commit: `feat(cli): wire --from-com through sidecar (Windows)`

### Task 3: Sidecar — `Headers/HeaderParser.cs`

Shell to clang, parse the AST JSON, emit one command per `FunctionDecl`.

```csharp
public static class HeaderParser
{
    public static GeneratedAgent Parse(string[] files, string? agentIdOverride)
    {
        if (!IsClangAvailable())
            throw new InvalidOperationException("clang not found on PATH. Install LLVM from https://releases.llvm.org/ or via your package manager.");

        var commands = new List<GeneratedCommand>();
        foreach (var file in ResolveFiles(files))
        {
            // Run: clang -Xclang -ast-dump=json -fsyntax-only <file>
            var psi = new ProcessStartInfo("clang", $"-Xclang -ast-dump=json -fsyntax-only \"{file}\"")
            {
                RedirectStandardOutput = true,
                RedirectStandardError = true,
                UseShellExecute = false,
            };
            using var p = Process.Start(psi);
            // ... parse JSON output looking for "kind": "FunctionDecl" entries that come from THIS file (not transitive includes)
            // ... emit one command per function
        }
        // ...
    }
}
```

For v0.5.2 simplest: handle only top-level function declarations. Skip class methods, templates, macros. Document this in the README.

Tests:
- Build a tiny `sample.h` with a couple of `extern` declarations
- Run the parser
- Expect commands for each declared function
- Skip if clang isn't on PATH (CI installs it; dev machines may not have it)

Commit: `feat(cli-sidecar): C++ header parsing via clang shell`

### Task 4: Wire `--from-headers` end to end

Same pattern as Task 2: add `FromHeadersArgs`, add `case "from-headers"` to dispatch, add Rust wrapper, rewire stubs.

Commit: `feat(cli): wire --from-headers through sidecar`

### Task 5: Update `cli/tests/build_agent.rs` + `tests/basic.rs`

After v0.5.2 all eight build-source flags work. The previous "tier C is stubbed" tests need updating:
- The `--from-com` test now expects success (Windows) or "Windows-only" error (non-Windows). Skip if sidecar missing.
- The `--from-headers` test now expects success if clang is on PATH, or a "clang missing" error otherwise. Skip if sidecar missing.
- The `basic.rs` smoke test currently uses `--from-com` as its NotYetImplemented sentinel. Switch to `agent publish` (still stubbed for v0.2+):

```rust
#[test]
fn unimplemented_subcommand_exits_nonzero_with_message() {
    Command::cargo_bin("aware")
        .unwrap()
        .args(["agent", "publish", "C:/some/path"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("not yet implemented"));
}
```

Commit: `test(cli): update build_agent + basic tests for v0.5.2`

### Task 6: CI matrix — add Linux

Update `.github/workflows/build-sidecar.yml`:

```yaml
name: Build sidecar

on:
  push:
    tags:
      - 'v0.5.*'
      - 'v0.6*'
      - 'v1.*'
  workflow_dispatch: {}

jobs:
  build:
    strategy:
      fail-fast: false
      matrix:
        include:
          - os: windows-latest
            rid: win-x64
            artifact: aware-sidecar-win-x64
            binary: aware-sidecar.exe
          - os: ubuntu-latest
            rid: linux-x64
            artifact: aware-sidecar-linux-x64
            binary: aware-sidecar
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-dotnet@v4
        with:
          dotnet-version: '9.0.x'
      - name: Install clang on Linux
        if: matrix.os == 'ubuntu-latest'
        run: sudo apt-get update && sudo apt-get install -y clang
      - name: Publish NativeAOT
        run: dotnet publish cli-sidecar -c Release -r ${{ matrix.rid }} -p:PublishAot=true
      - name: Smoke test (Windows only)
        if: matrix.os == 'windows-latest'
        shell: pwsh
        run: |
          dotnet build cli-sidecar/Tests/FixtureAssembly
          $bin = "cli-sidecar/bin/Release/net9.0/${{ matrix.rid }}/publish/${{ matrix.binary }}"
          $fixture = "cli-sidecar/Tests/FixtureAssembly/bin/Debug/net9.0/FixtureAssembly.dll"
          $req = ('{{"op":"reflect-dlls","args":{{"globs":["{0}"],"agent_id":"ci-smoke"}}}}' -f $fixture.Replace("\","/"))
          $resp = $req | & $bin
          if (-not $resp.Contains('"ok":true')) {
            Write-Error "sidecar smoke failed: $resp"
            exit 1
          }
      - name: Upload artifact
        uses: actions/upload-artifact@v4
        with:
          name: ${{ matrix.artifact }}
          path: cli-sidecar/bin/Release/net9.0/${{ matrix.rid }}/publish/${{ matrix.binary }}
```

Commit: `ci: build sidecar for Linux x64 in addition to Windows`

### Task 7: README + final verification

Update `cli/README.md`:
- Status bumps to v0.5.2
- Document the clang dep for `--from-headers` (link to LLVM downloads)
- Document COM as Windows-only
- Update the "tracked for v0.5.x" callout

Run:
- `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test --all-targets`, `cargo build --release`
- `dotnet test cli-sidecar/`, `dotnet publish cli-sidecar -c Release -r win-x64 -p:PublishAot=true`

Manual smoke (if clang available): `aware build agent --from-headers .\some.h`.
Manual smoke (always on Windows): `aware build agent --from-com WScript.Shell`.

Commit: `docs(cli): README + v0.5.2 surfaces`

---

## Self-review

**Spec coverage (v0.5.2 goals):**
- `--from-com` lifted from stub to working (Windows) → Tasks 1-2
- `--from-headers` lifted from stub to working (where clang is available) → Tasks 3-4
- Linux sidecar CI → Task 6

**Honest scope honesty:**
- COM: TypeLib introspection only. If full method walking is harder than expected, fall back to a minimal "discovered the ProgID, here's a skeleton" implementation. Better to ship a partial v0.5.2 than to over-promise.
- Headers: free functions only. Class methods, templates, macros are future work.
- macOS sidecar build deferred to v0.5.3.

**Realistic effort:** 1-2 days. The COM piece carries the most risk (Win32 P/Invoke can surprise). The headers piece is mostly process plumbing + JSON parsing. CI is config.
