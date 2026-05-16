# AWARE CLI v0.5.3 (macOS Sidecar + C++ Classes + Decompile UX) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development.

**Goal:** Close the last three v0.5.x gaps. Add macOS to the sidecar CI matrix. Extend `--from-headers` to walk C++ class methods (not just free functions). Improve the `--decompile` output so a user's agentic CLI can summarize it via the existing `aware skill modify` flow — without bundling AI in the CLI itself.

---

## Open design decisions (resolved here)

| Decision | Choice | Why | Alternative |
|---|---|---|---|
| **AI-decompile summarization architecture** | **Post-hoc via the existing skill flow, NOT in the CLI's pipeline.** `--decompile` continues to emit raw decompiled C# as skill content. The change in v0.5.3: each decompile skill gets a clear "Summarize this with your AI tool: `aware skill modify <agent> <skill>`" hint in a frontmatter `next-action:` field + the bottom of the skill body. The user's Claude Code / Codex / whatever does the summarization. | **Decalog #4 ("no vendor in the loop")** rules out bundling an Anthropic SDK or any built-in AI HTTP path. AWARE provides plumbing; the user's agentic CLI does the AI work. | Bundle Anthropic SDK + hardcoded API URL (violates decalog); ship `--summarize <command>` flag piping to user-specified subprocess (still possible later, more complex than needed for v0.5.3) |
| **macOS sidecar architecture** | `macos-latest` GitHub runner (Apple Silicon) + `osx-arm64` RID. NativeAOT supports this cleanly on .NET 9. clang ships with Xcode CLT (always present on macos-latest). | Apple Silicon is the dominant macOS dev arch in 2026; intel macs are vanishing | Add `osx-x64` too (double the matrix, marginal value); skip macOS again (incomplete v1.0) |
| **C++ class scope** | `CXXRecordDecl` (class/struct) walking: emit one command per `CXXMethodDecl` that is public and non-static. Free functions continue to work. Constructors / destructors / operators / private methods filtered out. | Realistic for v0.5.3; matches what users want for "wrap this C++ library". Static methods can be added cheaply if needed | Just public + static (more conservative); all methods (too noisy) |
| **C++ template scope** | Templates **not** walked in v0.5.3 — too complex (need instantiation), poor cost/benefit. Templates emit a warning to stderr and are skipped. Document as a known limit. | C++ template introspection wants full clang/libclang APIs not exposed by ast-dump=json. Better to skip than to wrong | Try to walk templates (likely buggy); silently skip (worse UX) |
| **Skill body format change** | Decompile skills now have YAML frontmatter `name`, `description`, `next-action: "aware skill modify <agent> <skill-name>"`, then the raw C# in a fenced code block under a "## Raw decompiled output" header. The `next-action` field is a new convention — documented in cli-spec.md as agent-spec.md amendment | Adds discoverability without breaking existing parsing; the existing skill loader ignores unknown frontmatter fields | Hardcoded prefix in the skill body (less structured); separate `<skill>.next-action` file (more files for no benefit) |
| **CI matrix gating** | All three runners (windows-latest, ubuntu-latest, macos-latest) build in parallel. `fail-fast: false` so a macOS hiccup doesn't kill the Linux + Windows artifacts | Robust; matches v0.5.2 pattern | `fail-fast: true` (over-strict for cross-platform); sequential (slower for no benefit) |
| **macOS smoke test** | Same JSON-IPC smoke test as Linux, just in bash on the macOS runner. Reuses the FixtureAssembly DLL (cross-platform .NET libs work the same on macOS) | Consistent with Linux; cheap | Skip the smoke test on macOS (loses verification) |
| **Tests for class methods** | Extend `cli-sidecar/Tests/fixtures/sample.h` with a small C++ class (`namespace Sample { class Greeter { public: std::string Greet(const std::string& name); int Counter(); }; }`). Tests verify the methods become commands | Reuses existing fixture infrastructure; one file edit | New fixture file (more files, no benefit) |
| **Backward compat for header parsing** | C source continues to work (free functions only). C++ classes are an additive feature: if the input is `.h` with no class declarations, behavior is identical to v0.5.2 | Zero-risk addition | Switch on .h vs .hpp extension (over-clever; users mix both) |
| **`next-action` semantics for skill loader** | The existing `manifest::Agent` deserializer doesn't reject unknown fields, so adding `next-action` doesn't break v0.1-v0.5.2 agents. New helper: `aware agent describe <agent>` could show next-actions in a "Next steps" footer — but for v0.5.3 keep it minimal: the field is present in the markdown, doctor doesn't surface it, users see it when they open the file | Avoid scope creep; the value is the hint in the file itself | Wire next-action into doctor / describe (adds surface area to test) |

---

## Scope discipline

**v0.5.3 user-facing surfaces:** none new. The same `--from-headers`, `--decompile`, and the existing release artifacts get more comprehensive. After this PR:

- `aware build agent --from-headers some.hpp` now emits commands for free functions AND public class methods
- `aware build agent --from-nuget Foo@1.0 --decompile` produces decompiled skills with clear "summarize via your AI tool" hints
- The sidecar binary is available for Windows x64, Linux x64, **and macOS arm64** on tag pushes

**v0.5.3 internal surfaces:**
- Extended `cli-sidecar/Headers/HeaderParser.cs` — `CXXRecordDecl` walking
- Extended `cli-sidecar/Decompile/IlspyShell.cs` — skill body format with `next-action` frontmatter
- Updated `cli-sidecar/Tests/fixtures/sample.h` — add a C++ class
- Updated `cli-sidecar/Tests/HeaderParserTests.cs` — assert class methods become commands
- `.github/workflows/build-sidecar.yml` — add `macos-latest` to matrix
- `cli/README.md` — document the next-action skill hint convention

**Out of scope (deferred):**
- C++ templates (too complex; `--from-headers` warns + skips)
- Built-in AI summarization in the CLI itself (decalog #4)
- New CLI commands (none needed — all surfaces are existing)
- iOS / Android sidecar builds (lol)

---

## File Structure

### Modified files

| Path | Change |
|---|---|
| `cli-sidecar/Headers/HeaderParser.cs` | Add `CXXRecordDecl` walking inside `ExtractFunctions`; emit `<class-name>-<method-name>` commands. Warn-and-skip on `ClassTemplateDecl`. |
| `cli-sidecar/Decompile/IlspyShell.cs` | Skill body format: YAML frontmatter with `next-action`, then a "## Raw decompiled output" section with the C# code block. |
| `cli-sidecar/Tests/fixtures/sample.h` | Append a C++ class under `extern "C++"` (or move the file to `sample.hpp` — simpler is to keep `sample.h` with C bits + add `sample.hpp` with C++ bits). |
| `cli-sidecar/Tests/HeaderParserTests.cs` | New test: `ParsesCppClassMethodsWhenClangPresent`. |
| `cli-sidecar/Tests/cli-sidecar.Tests.csproj` | Copy the new `sample.hpp` to output dir. |
| `.github/workflows/build-sidecar.yml` | Add `macos-latest` / `osx-arm64` row to the matrix. macOS smoke test in bash. |
| `cli/README.md` | Status bumps to v0.5.3. Document `next-action` skill convention + C++ class support in `--from-headers`. |

### New files

| Path | Responsibility |
|---|---|
| `cli-sidecar/Tests/fixtures/sample.hpp` | A tiny C++ header with a class for the new tests. |

---

## Tasks

### Task 0: Branch + baseline (done — on `feat/cli-v053-macos-cpp-classes-decompile-ux`)

### Task 1: macOS sidecar CI

Update `.github/workflows/build-sidecar.yml` matrix to add `macos-latest` / `osx-arm64`:

```yaml
- os: macos-latest
  rid: osx-arm64
  artifact: aware-sidecar-osx-arm64
  binary: aware-sidecar
```

clang ships with Xcode CLT on macos-latest, so no extra install step needed. Smoke test mirrors Linux (bash). Update the tag-trigger pattern if needed (`v0.5.*` already covers it).

Commit: `ci: add macOS arm64 sidecar build to matrix`

### Task 2: Extend HeaderParser for C++ class methods

In `cli-sidecar/Headers/HeaderParser.cs`, extend `WalkNode` to:
- On `CXXRecordDecl` (class/struct): iterate its `inner` and emit a command per public `CXXMethodDecl` (excluding constructors / destructors / static / operators)
- On `ClassTemplateDecl`: write to `Console.Error` ("skipping template <name>"); skip
- On `NamespaceDecl`: recurse normally

Test: extend `sample.hpp` with `namespace Sample { class Greeter { public: std::string Greet(...); int Counter(); }; }`. The new test asserts `greeter-greet` and `greeter-counter` appear as commands.

Implementation snippet for the C# walker:

```csharp
private static void WalkNode(JsonElement node, string headerAbs, Dictionary<string, GeneratedCommand> output)
{
    if (node.ValueKind != JsonValueKind.Object) return;

    if (node.TryGetProperty("kind", out var kindEl))
    {
        var kind = kindEl.GetString();
        switch (kind)
        {
            case "FunctionDecl":
                ExtractFunctionDecl(node, headerAbs, output);
                return; // don't recurse into function body
            case "CXXRecordDecl":
                ExtractClassMethods(node, headerAbs, output);
                return;
            case "ClassTemplateDecl":
                var tplName = node.TryGetProperty("name", out var tn) ? tn.GetString() : "<anon>";
                Console.Error.WriteLine($"[skip] class template '{tplName}' (templates are v0.5.4+)");
                return;
            default:
                // Recurse into namespaces, extern blocks, etc.
                break;
        }
    }

    if (node.TryGetProperty("inner", out var inner) && inner.ValueKind == JsonValueKind.Array)
    {
        foreach (var child in inner.EnumerateArray())
        {
            WalkNode(child, headerAbs, output);
        }
    }
}

private static void ExtractClassMethods(JsonElement classNode, string headerAbs, Dictionary<string, GeneratedCommand> output)
{
    if (!classNode.TryGetProperty("name", out var nameEl)) return;
    var className = nameEl.GetString();
    if (string.IsNullOrEmpty(className)) return;
    if (!IsInTargetFile(classNode, headerAbs)) return;

    if (!classNode.TryGetProperty("inner", out var inner) || inner.ValueKind != JsonValueKind.Array) return;

    string currentAccess = "public"; // C struct default; CXXRecordDecl with tagUsed=class would default to private, but we'll trust the AccessSpecDecl markers below

    foreach (var member in inner.EnumerateArray())
    {
        if (member.ValueKind != JsonValueKind.Object) continue;
        if (!member.TryGetProperty("kind", out var mKindEl)) continue;
        var mKind = mKindEl.GetString();

        if (mKind == "AccessSpecDecl")
        {
            if (member.TryGetProperty("access", out var accEl))
            {
                currentAccess = accEl.GetString() ?? currentAccess;
            }
            continue;
        }

        if (mKind != "CXXMethodDecl") continue;
        if (currentAccess != "public") continue;

        if (!member.TryGetProperty("name", out var methodNameEl)) continue;
        var methodName = methodNameEl.GetString();
        if (string.IsNullOrEmpty(methodName)) continue;
        // Skip constructors (named same as class), destructors (~ClassName), and operator overloads
        if (methodName == className) continue;
        if (methodName.StartsWith("~", StringComparison.Ordinal)) continue;
        if (methodName.StartsWith("operator", StringComparison.Ordinal)) continue;

        // Static methods filtered out (would need a different design for stateless invocation)
        if (member.TryGetProperty("storageClass", out var scEl) && scEl.GetString() == "static") continue;

        var commandName = ToKebab($"{className}-{methodName}");
        if (output.ContainsKey(commandName)) continue;
        output[commandName] = new GeneratedCommand
        {
            Name = commandName,
            Lifecycle = "single",
            Description = $"C++ method {className}::{methodName}",
        };
    }
}
```

Commit: `feat(cli-sidecar): walk C++ classes (CXXRecordDecl) for --from-headers`

### Task 3: Decompile skill format with `next-action` hint

In `cli-sidecar/Decompile/IlspyShell.cs`, change the skill body template from the v0.5.1 single-block form to:

```markdown
---
name: {pkg}-{stem}
description: Decompiled API surface from {filename} (license: {license})
next-action: aware skill modify {pkg} {stem}-decompiled
---

# {stem} (decompiled)

> ℹ️ This skill contains raw decompiled C#. To summarize it into structured
> AWARE-style notes, run `aware skill modify {pkg} {stem}-decompiled` from
> your agentic CLI (Claude Code / Codex). Ask your AI to read the raw
> decompiled output below and rewrite this skill as a normal AWARE skill
> (rule first, then explain) following the agent-spec.md conventions.

## Raw decompiled output

```csharp
{decompiled}
```
```

The skill loader already ignores unknown frontmatter fields, so `next-action:` is forward-compatible.

Add a unit test in `cli-sidecar/Tests/IlspyShellTests.cs`:

```csharp
[Fact]
public void SkillBodyContainsNextActionHint()
{
    // Use a tiny fake-nupkg with a permissive license (existing test pattern), trigger decompile
    // (or just check the constant string format via a refactored helper).
    // For simplicity, test the body-building helper directly if extracted.
}
```

Refactor the body-building into a small testable helper if needed.

Commit: `feat(cli-sidecar): decompile skills include next-action hint for AI summarization`

### Task 4: Update tests for new C++ fixture + class methods

Append to `cli-sidecar/Tests/fixtures/sample.hpp` (new file):

```cpp
// sample.hpp — C++ fixture for AWARE sidecar header tests

#pragma once

#include <string>

namespace Sample {

class Greeter {
public:
    std::string Greet(const std::string& name);
    int Counter();
private:
    int internal_count_;
};

} // namespace Sample

// Free function in C++ mode too
int sample_add_cpp(int a, int b);
```

Add to `cli-sidecar/Tests/cli-sidecar.Tests.csproj` (alongside the existing `sample.h` copy directive):

```xml
<None Include="fixtures\sample.hpp">
  <CopyToOutputDirectory>PreserveNewest</CopyToOutputDirectory>
</None>
```

Add test in `cli-sidecar/Tests/HeaderParserTests.cs`:

```csharp
[Fact]
public void ParsesCppClassMethodsWhenClangPresent()
{
    if (!ClangAvailable()) return;
    var here = Path.GetDirectoryName(typeof(HeaderParserTests).Assembly.Location)!;
    var fixture = Path.Combine(here, "fixtures", "sample.hpp");
    Assert.True(File.Exists(fixture), $"fixture not at {fixture}");
    var agent = HeaderParser.Parse(new[] { fixture }, agentIdOverride: "sample-cpp");
    var commandNames = agent.Commands.Select(c => c.Name).ToList();
    Assert.Contains("greeter-greet", commandNames);
    Assert.Contains("greeter-counter", commandNames);
    Assert.Contains("sample-add-cpp", commandNames); // free function still works
}
```

Commit: `test(cli-sidecar): C++ class fixture + class-method test`

### Task 5: README + final verification

Update `cli/README.md`:
- Status → v0.5.3: "macOS sidecar shipped; `--from-headers` covers C++ class methods; `--decompile` skills include a `next-action` hint"
- New short subsection on the `next-action` skill convention
- Bump the C# sidecar subsection: list macOS as supported

Run:
- `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test --all-targets`, `cargo build --release`
- `dotnet test cli-sidecar/`
- `dotnet publish cli-sidecar -c Release -r win-x64 -p:PublishAot=true`

Manual smoke: `aware build agent --from-headers cli-sidecar/Tests/fixtures/sample.hpp --output sample-cpp` (only if clang on PATH).

Commit: `docs(cli): README for v0.5.3 (macOS + C++ classes + decompile UX)`

---

## Self-review

**Spec coverage (v0.5.3 goals):**
- macOS NativeAOT sidecar build → Task 1
- C++ class methods in `--from-headers` → Tasks 2 + 4
- AI-driven decompile summarization (via the user's CLI, not bundled in AWARE) → Task 3

**Out of scope acknowledged:**
- Templates in `--from-headers` (warn-and-skip; deferred)
- Static C++ methods (filtered out; could be added cheaply later)
- iOS / Android / WASM sidecar builds (lol)
- Bundled AI in the CLI itself (decalog #4)

**Realistic effort:** 1 day. Three independent pieces; each task is small.
