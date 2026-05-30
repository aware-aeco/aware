# #180 Reader/Recipe Split — Implementation Plan

> **For Claude:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or
> superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make `aware build --from-dlls/--from-nuget/--from-csharp` synthesize a *correct* agent from
attribute-driven, multi-assembly .NET SDKs (Tekla/Revit/Rhino plugins) instead of useless method-per-type
commands.

**Architecture:** A shared `AwareReader` lib (netstandard2.0, AOT-safe) holds the normalized IR (now with
custom attributes + fields), a synthesizer (IR→`GeneratedAgent`), and a recipe registry. Two readers feed
it: `MetadataReflector` (compiled, in the AOT `aware-sidecar`) and a new `RoslynReader` (source, in a new
non-AOT `aware-roslyn` tool). One synthesizer + the Tekla recipe run for both.

**Tech Stack:** Rust 2024 (`cli/`), C# `net10.0` NativeAOT (`cli-sidecar`), C# `net10.0` JIT (`cli-roslyn`,
`AwareReader` netstandard2.0), `System.Reflection.Metadata`, Roslyn (`Microsoft.CodeAnalysis` +
`…Workspaces.MSBuild`), xUnit, `assert_cmd`.

**Spec:** `docs/superpowers/specs/2026-05-30-issue-180-reader-recipe-design.md`

**Delivery:** 4 sequential Codex-reviewed PRs (A→B→C→D); D carries `Closes #180`; one release after D.

**Repo facts that constrain this plan:**
- No `.sln`; CI builds each project via `dotnet publish <project>` (a `ProjectReference` to `AwareReader`
  builds transitively). `release.yml:81` publishes `cli-sidecar` with `-p:PublishAot=true`.
- Sidecar enforces `TreatWarningsAsErrors=true` + `PublishAot=true` → `AwareReader` must stay AOT/trim-clean.
- Existing reader records + `MetadataReflector` are consumed by 7 files (Navisworks/TeklaSdk extractors) —
  these get mechanical `using` updates when the records move.
- `[StructuresField]` field types are `double`/`int`/`string` only (vendor-enforced). Plugin↔data join is the
  plugin's **public constructor parameter type**. `Component.SetAttribute(name, double|int|string)` +
  `ComponentInput` + `Insert()`/`CommitComponentInput` is the in-model equivalent the recipe documents.

---

## Chunk A — Shared `AwareReader` lib + IR attribute/field extraction (PR-A: `feat/180a-reader-ir`)

Foundation only: introduce the shared lib and capture custom attributes in the IR. **No `--from-dlls`
behavior change yet** (synthesizer lands in B) — keeps PR-A low-risk and independently shippable.

**File structure:**
- Create `cli-reader/AwareReader.csproj` — netstandard2.0 lib; `<PackageReference System.Reflection.Metadata>`.
- Move into `cli-reader/` (new namespace `AwareReader`): the records + `SigTypeProvider` + `GenericContext`
  from `cli-sidecar/Ingest/Extractors/Common/MetadataReflector.cs`, and `MetadataReflector` itself.
- Move `GeneratedAgent`/`GeneratedCommand`/`GeneratedSkill` from `cli-sidecar/Protocol/Agent.cs` → `AwareReader`.
- Modify `cli-sidecar/cli-sidecar.csproj` — add `<ProjectReference Include="..\cli-reader\AwareReader.csproj" />`.
- Modify 7 extractor files + `cli-sidecar/Program.cs` `SidecarJsonContext` `using`s.
- Test: `cli-sidecar/Tests/FixtureAssembly/PublicApi.cs` (add attributed types), new
  `cli-reader/Tests/AttributeReaderTests.cs` (or extend `cli-sidecar/Tests/DllReflectorTests.cs`).

- [ ] **A1. Create the lib + move records.** Create `cli-reader/AwareReader.csproj` (netstandard2.0,
  `Nullable=enable`, `TreatWarningsAsErrors=true`, `System.Reflection.Metadata` 9.0.0). Move the record
  types + `SigTypeProvider`/`GenericContext` + `MetadataReflector` into `cli-reader/MetadataReflector.cs`
  under `namespace AwareReader;`. Move `GeneratedAgent`/`Command`/`Skill` into `cli-reader/GeneratedAgent.cs`.
- [ ] **A2. Re-wire the sidecar.** Add the `ProjectReference`. Replace the moved code in
  `cli-sidecar/Ingest/Extractors/Common/MetadataReflector.cs` (delete the file) and add
  `using AwareReader;` to the 7 consumer files + `Program.cs`. Build: `dotnet build cli-sidecar` succeeds
  (warnings = errors).
- [ ] **A3. Verify nothing regressed.** Run `dotnet test cli-sidecar/Tests` — existing Navisworks/TeklaSdk/
  Dll reflector tests pass unchanged. Commit: `refactor(reader): extract AwareReader shared lib (#180)`.
- [ ] **A4. Add the attribute IR (failing test first).** Add to `FixtureAssembly`:
  ```csharp
  [AttributeUsage(AttributeTargets.Class)] public sealed class DemoMarkerAttribute : Attribute {
      public DemoMarkerAttribute(string name) { Name = name; } public string Name { get; }
      public int Order { get; set; }
  }
  [DemoMarker("widget", Order = 3)] public class MarkedWidget { [DemoMarker("field-marker")] public double Size; }
  ```
  Write `AttributeReaderTests.AttributesDecodedWithPositionalAndNamedArgs()` asserting the `TypeRecord` for
  `MarkedWidget` has an `AttributeRecord` `TypeName="…DemoMarkerAttribute"`, `FixedArguments=["widget"]`,
  `NamedArguments=[{Order, 3}]`, and the field `Size` carries `[DemoMarker("field-marker")]`. Run → FAIL
  (no `Attributes` member).
- [ ] **A5. Implement attribute decoding.** Add `AttributeRecord`/`AttributeArg`/`NamedArg`; add
  `IReadOnlyList<AttributeRecord> Attributes` to `TypeRecord`/`MethodRecord`/`FieldRecord`/`PropertyRecord`/
  `ParameterRecord`. Add `AttributeReader.Read(MetadataReader, CustomAttributeHandleCollection, SigTypeProvider)`
  that, per attribute: resolves the ctor's declaring-type full name (generalize the constructor-walk from the
  old `DllReflector.ReadAssemblyDescription`), then `ca.DecodeValue(sigProvider)` → map `FixedArguments`
  (type+literal) and `NamedArguments` (name+type+literal). Flesh out `SigTypeProvider.GetUnderlyingEnumType`
  (return Int32) and `GetTypeFromSerializedName`. Call it for types/methods/fields/properties/params in
  `MetadataReflector.ReflectType`/`ReflectMethod`. Sort attributes by `TypeName` for determinism. Run → PASS.
- [ ] **A6. Commit + ship PR-A.** `dotnet test cli-sidecar/Tests` green; `cargo` untouched. Commit
  `feat(reader): extract type/method/field custom attributes into the IR (#180)`. Codex review → address →
  push → PR → merge (approval) → resync main.

## Chunk B — Multi-assembly context + synthesizer; rewire `--from-dlls` (PR-B: `feat/180b-multi-assembly`)

**File structure:**
- Create `cli-reader/AgentSynthesizer.cs` (`AwareReader.AgentSynthesizer`).
- Modify `cli-reader/MetadataReflector.cs` (add `ReflectSet` + `ReflectedSet` record + `TypeIndex`).
- Modify `cli-sidecar/Reflection/DllReflector.cs` (delegate to `ReflectSet` + `AgentSynthesizer`).
- Test: `cli-reader/Tests/SynthesizerTests.cs`, update `cli-sidecar/Tests/DllReflectorTests.cs`.

- [ ] **B1. `ReflectSet` (failing test).** Add a 2nd fixture assembly `FixtureDataAssembly` (project ref'd by
  `FixtureAssembly`) whose type `WidgetData` is referenced by a `FixtureAssembly` type's ctor. Test
  `ReflectSetBuildsCrossAssemblyIndex()` asserts `ReflectSet([fixture, dataFixture]).TypeIndex` contains both
  `…WidgetData` and the consumer type. Run → FAIL.
- [ ] **B2. Implement `ReflectSet`.** Add `record ReflectedSet(IReadOnlyList<AssemblyRecord> Assemblies,
  IReadOnlyDictionary<string,TypeRecord> TypeIndex)` and `MetadataReflector.ReflectSet(IReadOnlyList<string>)`
  that reflects each DLL (reusing `Reflect`) and folds all `TypeRecord`s into a `FullName→TypeRecord` map
  (first-wins on dup names, stderr note on collision). Run → PASS.
- [ ] **B3. `AgentSynthesizer` (failing test).** `SynthesizerProducesCoherentIdAcrossSet()`: given a
  `ReflectedSet` of two assemblies, the synthesized `GeneratedAgent` id = primary (most public types) or
  override, description aggregates, commands union across the set (per-method default), skills per namespace.
  Run → FAIL.
- [ ] **B4. Implement `AgentSynthesizer.Synthesize(ReflectedSet, SynthesisOptions)`.** Move the per-type/
  per-method command + per-namespace skill projection out of `DllReflector.ReflectCore` into the synthesizer,
  operating over the whole set with the id heuristic (override → recipe(later) → primary-by-type-count →
  `reflected-agent`). Run → PASS.
- [ ] **B5. Rewire `DllReflector`.** `DllReflector.Reflect(globs, idOverride)` → `ResolveGlobs` →
  `MetadataReflector.ReflectSet` → `AgentSynthesizer.Synthesize`. Keep the large-stack worker thread + glob
  resolver. Update `DllReflectorTests` expectations (still has `greeter-greet` etc., now coherent across set).
  Run all sidecar tests → PASS. Commit `feat(reader): multi-assembly reflection + agent synthesizer (#180)`.
- [ ] **B6. Codex review → address → PR → merge → resync.**

## Chunk C — `aware-roslyn` source reader + Rust flags + release (PR-C: `feat/180c-roslyn-source`)

**File structure:**
- Create `cli-roslyn/cli-roslyn.csproj` (net10.0, **not** AOT; refs `AwareReader`, `Microsoft.CodeAnalysis.CSharp`,
  `Microsoft.CodeAnalysis.Workspaces.MSBuild`, `Microsoft.Build.Locator`), `cli-roslyn/Program.cs`,
  `cli-roslyn/RoslynReader.cs`, `cli-roslyn/Tests/RoslynReaderTests.cs`.
- Create `cli/src/builder/roslyn.rs`; modify `cli/src/commands/build.rs`, `cli/src/builder/mod.rs`,
  `cli/src/sidecar.rs` (add `AWARE_ROSLYN` discovery or a small shared discovery helper).
- Modify `.github/workflows/release.yml` (publish `aware-roslyn` per OS).

- [ ] **C1. `RoslynReader` (failing test).** `RoslynReaderTests.ReflectsInMemorySourceToIR()`: parse a `.cs`
  string declaring `MarkedWidget` (same shape as A4) via `CSharpSyntaxTree`+`CSharpCompilation`, assert the
  produced `AssemblyRecord`/`TypeRecord` matches the metadata reader's output (attributes, fields). Run → FAIL.
- [ ] **C2. Implement `RoslynReader`.** Map `INamedTypeSymbol`→`TypeRecord`, `IMethodSymbol`→`MethodRecord`,
  `IFieldSymbol`→`FieldRecord`, `AttributeData`→`AttributeRecord` (ctor args = positional, named args = named),
  XML doc via `GetDocumentationCommentXml`. `ReflectPaths(paths)`: `.sln`/`.csproj`→`MSBuildWorkspace`
  (`MSBuildLocator.RegisterDefaults()` once) → all project `Compilation`s; `.cs`/dir/glob→`CSharpCompilation.Create`
  over parsed trees + best-effort `MetadataReference` to the running framework. Returns a `ReflectedSet`. Run → PASS.
- [ ] **C3. `aware-roslyn` Program.** stdin/stdout JSON envelope (mirror `cli-sidecar/Program.cs`); op
  `reflect-csharp` (args `{paths:string[], agent_id?}`) → `RoslynReader.ReflectPaths` →
  `AgentSynthesizer.Synthesize` → emit `{ok, version, op, data:{agent}}`. `.sln`/`.csproj` with no MSBuild →
  actionable error. Add `cli-roslyn/Tests` xUnit asserting envelope round-trip on a temp `.cs`.
- [ ] **C4. Rust wiring (failing test).** Add `--from-csharp/--from-csproj/--from-sln` to `BuildAgentArgs`;
  dispatch to `builder::roslyn::build_from_source(paths, id)`. `builder/roslyn.rs` mirrors `sidecar.rs`:
  `discover()` (`AWARE_ROSLYN` env → sibling of `aware` → PATH), `invoke("reflect-csharp", args)` →
  `to_local_agent`. `assert_cmd` test: `aware build agent --from-csharp <tmp.cs>` with `AWARE_ROSLYN` set to a
  built binary → manifest written. Unit test: missing binary → `NotFound`. Run → PASS.
- [ ] **C5. Release.** Add an `aware-roslyn` publish + stage step to `release.yml` for each OS matrix entry
  (self-contained, framework-dependent acceptable; not AOT). Update `cli-spec.md` build surface
  (`--from-csharp/-csproj/-sln`). Commit `feat(build): C# source reader via aware-roslyn (#180)`.
- [ ] **C6. Codex review → address → PR → merge → resync.**

## Chunk D — Tekla recipe (PR-D: `feat/180d-tekla-recipe`, `Closes #180`)

**File structure:**
- Create `cli-reader/Recipes/IRecipe.cs`, `cli-reader/Recipes/RecipeRegistry.cs`,
  `cli-reader/Recipes/TeklaPluginRecipe.cs`.
- Modify `cli-reader/AgentSynthesizer.cs` (run recipe pass), `cli-reader/GeneratedAgent.cs`
  (`GeneratedCommand.Inputs` + `Mode`), `cli/src/sidecar.rs` + `cli/src/builder/mod.rs` (carry structured
  inputs into `inputs_yaml`/`mode`).
- Test: `cli-sidecar/Tests/FixtureAssembly` + `FixtureDataAssembly` Tekla-shaped stubs; `cli-reader/Tests/
  TeklaRecipeTests.cs`; `cli/tests/` end-to-end.

- [ ] **D1. Recipe abstraction (failing test).** `IRecipe { string Name; bool Matches(ReflectedSet);
  GeneratedAgent Apply(ReflectedSet set, GeneratedAgent baseline); }` + `RecipeRegistry.Detect(set)` returning
  the first match. Test with a no-op recipe → registry returns it. Run → FAIL → implement → PASS.
- [ ] **D2. Tekla fixture (two-assembly graph).** In `FixtureDataAssembly`: `PluginBaseStub` (abstract),
  `[PluginStub("name")]`, `[StructuresFieldStub("attr")]`. In `FixtureAssembly`:
  `public class DemoPlugin : PluginBaseStub { public DemoPlugin(DemoData d){} }` and (in the *data* assembly)
  `public class DemoData { [StructuresFieldStub("Length")] public double Length; [StructuresFieldStub("Name")]
  public string Name; [StructuresFieldStub("Count")] public int Count; }`. Recipe matches on configurable
  full names (so the real `Tekla.Structures.Plugins.*` names are a one-line config, the stub validates the
  mechanism).
- [ ] **D3. `TeklaPluginRecipe` (failing test).** `Matches` = a public concrete class whose base chain reaches
  the configured `PluginBase` full name AND carries the `[Plugin]` attribute. `Apply`: read plugin name from
  `[Plugin]` positional arg; find the data type = first public ctor's parameter type, resolved via
  `TypeIndex` (cross-assembly); collect its public fields with `[StructuresField]` (types double/int/string →
  AWARE `number`/`integer`/`string`); emit one command `insert-<kebab(plugin)>` (lifecycle `single`, mode
  `write`, inputs = field set + `input-points`) and **remove** that type's `*-run`/`*-define-input` commands.
  Test asserts the emitted command + inputs + absence of `demo-plugin-run`. Run → FAIL.
- [ ] **D4. Implement recipe + wire into synthesizer.** Implement `TeklaPluginRecipe`; in
  `AgentSynthesizer.Synthesize`, after the default projection, `RecipeRegistry.Detect(set)?.Apply(set, agent)`.
  Both `aware-sidecar` and `aware-roslyn` inherit it (shared lib). Run → PASS.
- [ ] **D5. Carry inputs to the manifest.** Add `Inputs` (`List<{name,type,optional,default}>`) + `Mode` to
  `GeneratedCommand`; in `cli/src/sidecar.rs::to_local_agent` map them into `GeneratedCommand.inputs_yaml`
  (build a small YAML block) + `mode`. Verify `write_agent` emits `inputs:`/`mode:` (existing plumbing).
  Rust unit test on the mapping.
- [ ] **D6. End-to-end.** `assert_cmd`: build `FixtureAssembly`+`FixtureDataAssembly`, run
  `aware build agent --from-dlls <bin>` → manifest has `insert-demo-plugin` with 3 typed inputs + input-points,
  no `*-plugin-run`. Repeat for `--from-csharp` over the equivalent `.cs`. Update `cli-spec.md`/`agent-spec.md`
  if the command contract changed (recipe `category`/`mode`).
- [ ] **D7. Commit, Codex review, PR (`Closes #180`), merge, resync.** Then run the release step (separate).

## Release (after D merges)

Follow `aware-issue/references/release-playbook.md`: bump `cli/Cargo.toml`, `cargo build` to refresh
`Cargo.lock`, `chore(release): vX.Y.Z`, push main (approval), tag `vX.Y.Z`, `gh run watch`, verify
`npm view @aware-aeco/cli version`. Confirm the new `aware-roslyn` artifact is in the GitHub Release.

## Verification gates (every PR)

`cd cli && cargo fmt --all && cargo clippy --all-targets -- -D warnings && cargo test`;
`dotnet test cli-sidecar/Tests` (+ `cli-roslyn/Tests` from C on). Re-run the issue repro. Delete `tmpclaude-*`.
