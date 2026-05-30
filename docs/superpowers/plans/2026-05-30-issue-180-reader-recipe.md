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

**Tech Stack:** Rust 2024 (`cli/`), C# `net10.0` NativeAOT (`cli-sidecar`), C# `net10.0` JIT (`cli-roslyn`),
`AwareReader` class lib **`net10.0`** (consumed only by the two net10.0 exes; no netstandard shim needed),
`System.Reflection.Metadata`, Roslyn (`Microsoft.CodeAnalysis` + `…Workspaces.MSBuild`), xUnit, `assert_cmd`.

**Spec:** `docs/superpowers/specs/2026-05-30-issue-180-reader-recipe-design.md`

**Delivery:** 4 sequential Codex-reviewed PRs (A→B→C→D); D carries `Closes #180`; one release after D.

**Repo facts that constrain this plan:**
- No `.sln`; CI builds each project via `dotnet publish <project>` (a `ProjectReference` to `AwareReader`
  builds transitively). `release.yml:81` publishes `cli-sidecar` with `-p:PublishAot=true`.
- Sidecar enforces `TreatWarningsAsErrors=true` + `PublishAot=true` → `AwareReader` must stay AOT/trim-clean.
  **Gate every .NET PR with a real AOT publish, not just `dotnet test`** (trim/AOT warnings don't surface in
  `dotnet test`): `dotnet publish cli-sidecar -c Release -r <host-rid> -p:PublishAot=true`.
- Existing reader records + `MetadataReflector` are consumed by 7 files (Navisworks/TeklaSdk extractors) —
  these get mechanical `using` updates when the records move. **`MetadataReflector` also depends on
  `XmlDocLoader.ShortName`** (`Ingest/Extractors/Common/XmlDocLoader.cs`) — that dependency must move/resolve
  with it in PR-A (verify `XmlDocLoader`'s own deps first; prefer moving `XmlDocLoader` into `AwareReader`,
  else extract just `ShortName` into a shared `DocId` helper).
- Moving `GeneratedAgent`/`Command`/`Skill` out of `Protocol` affects **every verb that emits an agent**
  (`reflect-dlls`, `decompile`, `from-com`, `from-headers`) and the `SidecarJsonContext` source-gen +
  `ResponseData.Agent`. PR-A must keep `[JsonSerializable(typeof(GeneratedAgent))]` (+ new types) valid and
  test all four verbs still round-trip. It is an API-shape PR, not "pure refactor."
- `[StructuresField]` field types are `double`/`int`/`string` only (vendor-enforced). Plugin↔data join is the
  plugin's **public constructor whose single parameter type carries `[StructuresField]` fields** (not merely
  "first ctor"). `Component.SetAttribute(name, double|int|string)` + `ComponentInput` +
  `Insert()`/`CommitComponentInput` is the in-model equivalent the recipe documents.
- `release.yml` stages single binaries (`aware`, `aware-sidecar`); the **WiX MSI** (`packaging/wix/aware.wxs`)
  installs only `aware.exe` + `aware-sidecar.exe`. Shipping `aware-roslyn` means a self-contained
  **single-file** publish + adding it to BOTH the release staging and the MSI component list.

> **Codex plan review (2026-05-30) incorporated below.** Key changes from review: AOT-publish gate; lib TFM
> `net10.0`; AttributeArg/default values are string literals (no `object`); `.csproj`/`.sln` is the
> recipe-quality source path with `--reference-dir` for bare `.cs`; `aware-roslyn` single-file + WiX; ctor
> selection by `[StructuresField]`-bearing param; `XmlDocLoader` moves with the reader; base-chain walk via
> `TypeIndex`; real-namespace Tekla fixture; YAML-safe Rust input rendering.

---

## Chunk A — Shared `AwareReader` lib + IR attribute/field extraction (PR-A: `feat/180a-reader-ir`)

Foundation only: introduce the shared lib and capture custom attributes in the IR. **No `--from-dlls`
behavior change yet** (synthesizer lands in B) — keeps PR-A low-risk and independently shippable.

**File structure:**
- Create `cli-reader/AwareReader.csproj` — **`net10.0`** lib; `<PackageReference System.Reflection.Metadata>`;
  `Nullable=enable`, `TreatWarningsAsErrors=true`; AOT/trim-clean (`<IsAotCompatible>true</IsAotCompatible>`).
- Move into `cli-reader/` (new namespace `AwareReader`): the records + `SigTypeProvider` + `GenericContext`
  from `cli-sidecar/Ingest/Extractors/Common/MetadataReflector.cs`, `MetadataReflector` itself, **and
  `XmlDocLoader`** (or, if its other deps are heavy, just the `ShortName` helper into a shared `DocId.cs`).
- Move `GeneratedAgent`/`GeneratedCommand`/`GeneratedSkill` from `cli-sidecar/Protocol/Agent.cs` → `AwareReader`.
- Modify `cli-sidecar/cli-sidecar.csproj` — add `<ProjectReference Include="..\cli-reader\AwareReader.csproj" />`.
- Modify the 7 extractor files + `cli-sidecar/Program.cs` (`SidecarJsonContext` + `EmitOk`) +
  `cli-sidecar/Protocol/Response.cs` (`ResponseData.Agent` type) `using`s.
- Test: `cli-sidecar/Tests/FixtureAssembly/PublicApi.cs` (add attributed types), new
  `cli-sidecar/Tests/AttributeReaderTests.cs`.

- [ ] **A1. Create the lib + move records (incl. `XmlDocLoader`).** Create `cli-reader/AwareReader.csproj`
  (`net10.0`, `Nullable=enable`, `TreatWarningsAsErrors=true`, `IsAotCompatible=true`,
  `System.Reflection.Metadata` 9.0.0). Move the record types + `SigTypeProvider`/`GenericContext` +
  `MetadataReflector` + `XmlDocLoader` into `cli-reader/` under `namespace AwareReader;`. Move
  `GeneratedAgent`/`Command`/`Skill` into `cli-reader/GeneratedAgent.cs`. (First verify `XmlDocLoader`'s
  dependencies — if it pulls in extractor-only types, move only `ShortName`.)
- [ ] **A2. Re-wire the sidecar.** Add the `ProjectReference`. Delete the moved files from `cli-sidecar`,
  add `using AwareReader;` to the consumer files + `Program.cs` + `Protocol/Response.cs`. Keep
  `[JsonSerializable(typeof(GeneratedAgent))]` valid (type now in `AwareReader`). Build:
  `dotnet build cli-sidecar` succeeds (warnings = errors).
- [ ] **A3. Verify nothing regressed — incl. AOT publish + all agent-emitting verbs.** Run
  `dotnet test cli-sidecar/Tests` (Navisworks/TeklaSdk/Dll reflector + a new round-trip assertion for
  `decompile`/`from-com`/`from-headers` emitting `GeneratedAgent`), **then**
  `dotnet publish cli-sidecar -c Release -r <host-rid> -p:PublishAot=true` succeeds with no IL warnings.
  Commit: `refactor(reader): extract AwareReader shared lib (#180)`.
- [ ] **A4. Add the attribute IR (failing test first).** Add to `FixtureAssembly`:
  ```csharp
  [AttributeUsage(AttributeTargets.Class | AttributeTargets.Field)] public sealed class DemoMarkerAttribute : Attribute {
      public DemoMarkerAttribute(string name) { Name = name; } public string Name { get; }
      public int Order { get; set; }
  }
  public enum DemoKind { A = 0, B = 1 }
  [DemoMarker("widget", Order = 3)] public class MarkedWidget { [DemoMarker("field-marker")] public double Size; }
  ```
  Write `AttributeReaderTests.AttributesDecodedWithPositionalAndNamedArgs()` asserting the `TypeRecord` for
  `MarkedWidget` has an `AttributeRecord` `TypeName="…DemoMarkerAttribute"`, `FixedArguments=["widget"]`,
  `NamedArguments=[{Order, "3"}]`, and the field `Size` carries `[DemoMarker("field-marker")]`. Run → FAIL.
- [ ] **A5. Implement attribute decoding (bounded + fault-tolerant).** Add `AttributeRecord`/`AttributeArg`/
  `NamedArg` where **`Value` is `string?` (a literal form), never `object`** (AOT/source-gen safety). Add
  `IReadOnlyList<AttributeRecord> Attributes` to `TypeRecord`/`MethodRecord`/`FieldRecord`/`PropertyRecord`/
  `ParameterRecord`. Add `AttributeReader.Read(...)` that per attribute: resolves the ctor's declaring-type
  full name (generalize the constructor-walk from `DllReflector.ReadAssemblyDescription`), then **inside a
  try/catch** `ca.DecodeValue(sigProvider)` → map `FixedArguments`/`NamedArguments` to literal strings
  (enum → integer literal, `typeof` → serialized type name, arrays → bracketed literal); on decode failure,
  record the attribute `TypeName` with empty args rather than throwing. Flesh out
  `SigTypeProvider.GetUnderlyingEnumType`/`GetTypeFromSerializedName`. Call it for
  types/methods/fields/properties/params. Sort attributes by `TypeName` for determinism. Run → PASS.
- [ ] **A6. Decode edge-case tests.** Add tests: enum-valued arg, `typeof(X)` arg, named + positional mix,
  and a malformed/unresolved attribute ctor (asserts graceful skip, no throw). Run → PASS.
- [ ] **A7. Commit + ship PR-A.** `dotnet test cli-sidecar/Tests` green + AOT publish green; `cargo`
  untouched. Commit `feat(reader): extract type/method/field custom attributes into the IR (#180)`. Codex
  review → address → push → PR → merge (approval) → resync main.

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
- Modify `.github/workflows/release.yml` (publish + stage `aware-roslyn`) **and `packaging/wix/aware.wxs`**
  (add the `aware-roslyn` component so the MSI installs it next to `aware.exe`/`aware-sidecar.exe`).

**Recipe-quality vs best-effort source (Codex finding #4):** `.csproj`/`.sln` (with references resolved via
MSBuild) is the **recipe-quality** path — only there do `PluginBase`/`[Plugin]`/`[StructuresField]` full
names resolve so the Tekla recipe can match. Bare `.cs` is **best-effort** structure only; add an optional
`--reference-dir <dir>` (repeatable) so users can point bare `.cs` at the SDK DLLs. When full names don't
resolve, the recipe simply doesn't fire (default synthesis instead) — documented, not an error.

- [ ] **C1. `RoslynReader` (failing test).** `RoslynReaderTests.ReflectsInMemorySourceToIR()`: parse a `.cs`
  string declaring `MarkedWidget` (same shape as A4) via `CSharpSyntaxTree`+`CSharpCompilation`, assert the
  produced `AssemblyRecord`/`TypeRecord` matches the metadata reader's output (attributes, fields). Run → FAIL.
- [ ] **C2. Implement `RoslynReader`.** Map `INamedTypeSymbol`→`TypeRecord`, `IMethodSymbol`→`MethodRecord`,
  `IFieldSymbol`→`FieldRecord`, `AttributeData`→`AttributeRecord` (ctor args = positional, named args = named),
  XML doc via `GetDocumentationCommentXml`. `ReflectPaths(paths, references)`: `.sln`/`.csproj`→
  `MSBuildWorkspace` (`MSBuildLocator.RegisterDefaults()` once) → all project `Compilation`s;
  `.cs`/dir/glob→`CSharpCompilation.Create` over parsed trees + framework refs + any `--reference-dir` DLLs.
  Returns a `ReflectedSet` whose `TypeIndex` spans the project graph. Run → PASS.
- [ ] **C3. `aware-roslyn` Program.** stdin/stdout JSON envelope (mirror `cli-sidecar/Program.cs`); op
  `reflect-csharp` (args `{paths:string[], references:string[], agent_id?}`) → `RoslynReader.ReflectPaths` →
  `AgentSynthesizer.Synthesize` → emit `{ok, version, op, data:{agent}}`. `.sln`/`.csproj` with no MSBuild SDK
  on host → actionable error naming the `--reference-dir` fallback. Add `cli-roslyn/Tests` xUnit asserting
  envelope round-trip on a temp `.cs`, plus a test that unresolved SDK refs → recipe non-match (default synth).
- [ ] **C4. Rust wiring (failing test).** Add `--from-csharp/--from-csproj/--from-sln` + repeatable
  `--reference-dir` to `BuildAgentArgs`; dispatch to `builder::roslyn::build_from_source(paths, refs, id)`.
  `builder/roslyn.rs` mirrors `sidecar.rs`: `discover()` (`AWARE_ROSLYN` env → sibling of `aware` → PATH),
  `invoke("reflect-csharp", args)` → `to_local_agent`. `assert_cmd` test: `aware build agent --from-csharp
  <tmp.cs>` with `AWARE_ROSLYN` set to a built binary → manifest written. Unit test: missing binary →
  `NotFound` with install hint. Run → PASS.
- [ ] **C5. Release + packaging (Codex finding #5).** Publish `aware-roslyn` **self-contained single-file**
  (`-p:PublishSingleFile=true -p:SelfContained=true -p:IncludeNativeLibrariesForSelfExtract=true`) per OS;
  add stage step to `release.yml` mirroring `aware-sidecar`; add the binary as a WiX component in
  `packaging/wix/aware.wxs` (+ verify the harvested file list). Document the host MSBuild-SDK requirement for
  `.csproj`/`.sln`. Update `cli-spec.md` build surface (`--from-csharp/-csproj/-sln`, `--reference-dir`).
  Commit `feat(build): C# source reader via aware-roslyn (#180)`.
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
- [ ] **D2. Tekla fixture under the REAL namespace (Codex finding #9), two-assembly graph.** In a new
  `FixtureDataAssembly` declare *fake* types under `namespace Tekla.Structures.Plugins`: `PluginBase`
  (abstract), `PluginAttribute(string name)`, `StructuresFieldAttribute(string attributeName)` — so the
  recipe's **default** config (real full names) is exercised without proprietary DLLs. Also put the data
  class there: `public class DemoData { [StructuresField("Length")] public double Length; [StructuresField("Name")]
  public string Name; [StructuresField("Count")] public int Count; }`. In `FixtureAssembly` (separate DLL):
  `[Plugin("Demo Plugin")] public class DemoPlugin : PluginBase { public DemoPlugin(DemoData d){} ... }`. Keep
  recipe full names **configurable** too (so a `*Stub` test can validate the mechanism independently), but the
  default config = the real names this fixture uses.
- [ ] **D3. `TeklaPluginRecipe` (failing test).** `Matches` = a public concrete class whose **base chain**
  (walked via `TypeIndex`, stopping gracefully at unresolved external bases — Codex finding #10) reaches the
  configured `PluginBase` full name AND carries `[Plugin]`. `Apply`: read plugin name from `[Plugin]`
  positional arg; find the data type = the **public ctor with exactly one parameter whose resolved type
  carries `[StructuresField]` fields** (Codex finding #6; if 0 match → fall back to fields on the plugin
  itself; if >1 → deterministic pick + stderr warning); collect that type's public `[StructuresField]` fields
  (double/int/string → AWARE `number`/`integer`/`string`); emit one command `insert-<kebab(plugin)>`
  (lifecycle `single`, mode `write`, inputs = field set + `input-points`) and **remove** that type's
  `*-run`/`*-define-input` commands. Test asserts the emitted command + typed inputs + absence of
  `demo-plugin-run`. Run → FAIL.
- [ ] **D4. Base-chain + ctor edge-case tests.** Tests for: direct `PluginBase` base; one in-assembly
  intermediate base; an unresolved external intermediate base (no crash, no match); multiple ctors (correct
  one chosen); no data ctor (fields-on-plugin fallback). Run → implement → PASS.
- [ ] **D5. Implement recipe + wire into synthesizer.** Implement `TeklaPluginRecipe`; in
  `AgentSynthesizer.Synthesize`, after the default projection, `RecipeRegistry.Detect(set)?.Apply(set, agent)`.
  Both `aware-sidecar` and `aware-roslyn` inherit it (shared lib). Run → PASS.
- [ ] **D6. Carry inputs to the manifest (AOT-safe + YAML-safe — Codex findings #3, #8).** Add `Inputs`
  (`List<{name:string, type:string, optional:bool, default:string?}>` — `default` is a **literal string**, not
  `object`) + `Mode` to `GeneratedCommand`; register every new type in `SidecarJsonContext`. Mirror in
  `cli/src/sidecar.rs::SidecarCommand` (add `inputs: Vec<SidecarInput>`, `mode: Option<String>`); in
  `to_local_agent`, render the inputs into `inputs_yaml` via **`serde_yaml`** (not string concat) so
  untrusted plugin field names/defaults are quoted/escaped. Verify `write_agent` emits `inputs:`/`mode:`.
  Rust unit test on the mapping incl. a field name containing `:`/quotes.
- [ ] **D7. End-to-end.** `assert_cmd`: build `FixtureAssembly`+`FixtureDataAssembly`, run
  `aware build agent --from-dlls <bin>` → manifest has `insert-demo-plugin` with 3 typed inputs + input-points,
  no `*-plugin-run`. Repeat for `--from-csharp` over the equivalent `.cs` (with `--reference-dir` to the built
  fixture DLLs so full names resolve). Update `cli-spec.md`/`agent-spec.md` if the command contract changed
  (recipe `category`/`mode`).
- [ ] **D8. Commit, Codex review, PR (`Closes #180`), merge, resync.** Then run the release step (separate).

## Release (after D merges)

Follow `aware-issue/references/release-playbook.md`: bump `cli/Cargo.toml`, `cargo build` to refresh
`Cargo.lock`, `chore(release): vX.Y.Z`, push main (approval), tag `vX.Y.Z`, `gh run watch`, verify
`npm view @aware-aeco/cli version`. Confirm the new `aware-roslyn` artifact is in the GitHub Release.

## Verification gates (every PR)

- Rust: `cd cli && cargo fmt --all && cargo clippy --all-targets -- -D warnings && cargo test`.
- .NET: `dotnet test cli-sidecar/Tests` (+ `cli-roslyn/Tests` from C on), **and a real AOT publish**
  `dotnet publish cli-sidecar -c Release -r <host-rid> -p:PublishAot=true` (catches trim/AOT warnings that
  `dotnet test` misses — Codex finding #1). From C on, also `dotnet publish cli-roslyn` self-contained.
- Re-run the issue repro (fixture build → `--from-dlls` → assert `insert-*`, not `*-plugin-run`).
- Delete `tmpclaude-*` before committing.
