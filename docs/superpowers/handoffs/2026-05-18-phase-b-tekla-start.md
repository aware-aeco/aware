# Phase B Tekla-First Resume — 2026-05-18

> Read this in a fresh session before doing anything else. It's the minimum context needed to pick up v0.30 Phase B work cold.

## Where you are in the project

**v0.30 — host-agent 100%-coverage rework**

- Spec: [`docs/superpowers/specs/2026-05-18-v0.30-host-coverage-rework-design.md`](../specs/2026-05-18-v0.30-host-coverage-rework-design.md)
- Plan: [`docs/superpowers/plans/2026-05-18-v0.30-host-coverage-rework-plan.md`](../plans/2026-05-18-v0.30-host-coverage-rework-plan.md)
- Phase A status: **COMPLETE** (foundation built and verified)
- Phase B status: **NOT STARTED** (this is your job)

Branch: `feature/v0.29-runtime-hello-world` (v0.30 work is on the v0.29 branch — user will migrate to a proper branch later). Last commit before Phase B work: `f0ad7a26`.

## What Phase A delivered (committed `e3597bf9` → `f0ad7a26`)

Foundation infrastructure for converting vendor docs into AWARE raw agents:

1. **IR JSON Schema** at `cli-sidecar/Ingest/Schema/host-coverage.schema.json` — closed schema (`additionalProperties: false`), required collection fields, draft 2020-12
2. **C# Models + IRReader** at `cli-sidecar/Ingest/Generator/Models.cs` and `IRReader.cs` — records mirroring the schema 1:1, defensive null-checks
3. **ManifestWriter / SkillWriter / CatalogWriter** at `cli-sidecar/Ingest/Generator/*.cs` — convert IR to manifest.yaml + skills/*.md + catalog/*.json
4. **Source-generated `JsonSerializerContext`** at `cli-sidecar/Ingest/Generator/IrJsonContext.cs` — proper NativeAOT support, **no reflection workarounds**
5. **Generator orchestrator** at `cli-sidecar/Ingest/Generator/Generator.cs` — wires the 3 writers
6. **`coverage-generate` sidecar verb** in `cli-sidecar/Program.cs` — receives stdin JSON envelope `{op, args:{ir_path, out_dir, agent_id, vendor, vertical}}`
7. **Common Scraper** at `cli-sidecar/Ingest/Extractors/Common/Scraper.cs` — Playwright wrapper (AOT-friendly because Playwright shells out to Node)
8. **Common IRBuilder** at `cli-sidecar/Ingest/Extractors/Common/IRBuilder.cs` — in-memory IR accumulator
9. **Common CHM parser** at `cli-sidecar/Ingest/Extractors/Common/ChmParser.cs` — for Revit/AutoCAD/Navisworks SDK help files
10. **Rust `--from-coverage` flag** at `cli/src/builder/coverage.rs` + `cli/src/commands/build.rs`
11. **`aware coverage` command group** at `cli/src/commands/coverage.rs` — `generate`, `validate`, `review` subcommands
12. **Codex review protocol doc** at `docs/superpowers/specs/host-coverage-review-protocol.md` — what every Phase C reviewer follows
13. **Phase A E2E test** at `cli/tests/coverage_e2e.rs` — passes; verifies sidecar binary produces all 3 artefact types from a fixture IR

## Hard-earned lessons baked into the foundation (do not relitigate)

1. **NativeAOT requires source-gen JsonSerializerContext.** Don't use `DefaultJsonTypeInfoResolver` workarounds. The proper context is at `IrJsonContext.cs` covering all IR types. Reflection-based JSON deserialization is silently disabled by `PublishAot=true` at runtime.
2. **vswhere.exe is at `C:\Program Files (x86)\Microsoft Visual Studio\Installer\`** — add to PATH before running `dotnet publish -c Release -r win-x64 --self-contained`, otherwise the native link step fails with `vswhere.exe is not recognized`.
3. **`hh.exe -decompile` returns exit code 0 silently when the CHM file doesn't exist.** Vendor extractors using ChmParser must validate file existence themselves; don't trust the exit code.
4. **Playwright is AOT-compatible** because it shells out to a Node-based driver over stdio — no managed reflection on user types. The NoWarn list (`IL2026;IL2050;IL2104;IL3053`) is sufficient.
5. **Tekla Open API runtime DLLs are .NET Framework 4.x, not modern .NET.** Any code that loads `Tekla.Structures.Model.dll` at runtime must target `net48`. Tekla 2026 has a dual-runtime layout: `bin/` for .NET 8/9, `bin/Net48Runtime/` for .NET Framework. (This is a v0.29 lesson — relevant if you ever need to test extracted Tekla agents against running Tekla.)
6. **No shortcuts. No workarounds.** User-stated rule from this session. Earlier in v0.30 work I used `DefaultJsonTypeInfoResolver` as a quick fix; the user pulled me back and we did source-gen properly. Every Phase B subagent should be reminded of this principle.

## Your task: Tekla-first Phase B execution

User picked **Option 3** from the three options at session-end: do **Tekla end-to-end first as proof of pattern**, before fanning out to the other 13 vendor extractors in parallel.

**Why Tekla first:**
- It's the agent that motivated the v0.30 rework (the "couldn't find ExitTekla" moment)
- Tekla 2025 + 2026 installed locally on this machine — reality checks easy
- `developer.tekla.com` is Tier A (web scrape, well-structured docs)
- One vendor first means plan flaws surface once, not 6 times in parallel

**Concrete next move:** Dispatch a Phase B Tekla subagent per the plan's Task B1. Their deliverables:

1. `cli-sidecar/Ingest/Extractors/Tekla/Extractor.cs` — scrapes `developer.tekla.com/doc/tekla-structures/<version>` and emits IR
2. `cli-sidecar/Ingest/Extractors/Tekla/PageParser.cs` — parses a Tekla doc page into a `TypeInfo` (use AngleSharp for HTML parsing)
3. `cli-sidecar/Ingest/Output/tekla-2025.0.ir.json` — committed
4. `cli-sidecar/Ingest/Output/tekla-2026.0.ir.json` — committed
5. Regenerated raw agents at `20-agents/aeco/engineering/tekla-2025/` and `tekla-2026/` (manifest + skills + catalog)
6. `EXTRACTION-NOTES.md` per agent documenting CSS selectors, parser quirks, types skipped
7. Self-verify against 50 random types

Plan reference: lines starting at "Task B1: Tekla extractor" in [the plan](../plans/2026-05-18-v0.30-host-coverage-rework-plan.md).

After Tekla works, codex-rescue review per Phase C protocol. If review surfaces plan flaws, fix the plan before parallel Tier A fanout.

## Build + test commands (for reference)

```bash
# Build sidecar (Release, NativeAOT publish)
cd cli-sidecar
"C:\Program Files\dotnet\dotnet.exe" publish -c Release -r win-x64 --self-contained

# Run all sidecar tests
"C:\Program Files\dotnet\dotnet.exe" test Ingest/Generator/Tests

# Build aware CLI
cd ../cli
/c/Users/bimst/.cargo/bin/cargo build --release

# Run E2E (sanity check — should pass)
/c/Users/bimst/.cargo/bin/cargo test --test coverage_e2e --release

# Invoke the generator end-to-end from a generated IR
./target/release/aware.exe coverage generate tekla 2026.0 --from-ir cli-sidecar/Ingest/Output/tekla-2026.0.ir.json --vendor trimble --vertical engineering
```

## Pre-existing known issues (NOT your problem to fix)

3 cargo tests fail on the pre-existing `manifest::agent::tests::parses_real_tekla_manifest`, `tekla_commands_are_explicitly_curated`, and `plugins::claude_code::tests::generates_plugin_json_with_command_index`. They assert hardcoded counts (31 skills / 20 commands) that drifted when v0.29 added 2 verbs + 2 skills to curated `tekla`. These will be resolved when Phase B regenerates the raw `tekla-2025` / `tekla-2026` agents OR by a manual bump in the test counts. Leave for later.

## Session metrics from the Phase A build

- ~65 subagent dispatches across A1-A12 + fixes
- 14 commits on the foundation
- 39+ tests passing
- 1 NativeAOT-verified self-contained sidecar binary (8.6 MB)
- User explicitly invoked "no shortcuts, no workarounds" — honor that.
