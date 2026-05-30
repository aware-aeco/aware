# Design — #180: `aware build` Reader/Recipe split for attribute-driven, multi-assembly SDKs

- **Issue:** [aware-aeco/aware#180](https://github.com/aware-aeco/aware/issues/180)
- **Date:** 2026-05-30
- **Scope (confirmed with author):** Full epic — A (reader IR with attributes + fields), B (multi-assembly
  context), C (C# source reader via Roslyn), D (Tekla recipe layer).
- **Delivery:** four sequential Codex-reviewed PRs (A → B → C → D, last carries `Closes #180`), riding **one**
  release at the end.

---

## 1. Problem (verified in source @ v0.52.0 / sidecar 0.5.1)

`aware build agent --from-dlls / --from-nuget` routes to the NativeAOT sidecar
(`cli-sidecar/Reflection/DllReflector.cs`). It emits **one command per public method per public type**,
reads only `AssemblyDescriptionAttribute`, never extracts type/method/field custom attributes, has **no
cross-assembly resolution**, and takes agent id/description from the **first** assembly only. For
attribute-driven, multi-assembly SDKs (Tekla / Revit / Rhino plugins) this produces a useless agent:
`*-plugin-run`, `*-plugin-define-input`, no `[Plugin]` name, no `[StructuresField]` parameters, and no way
to join a plugin in DLL A to its data contract in DLL B.

There are **two** reflection paths today:

1. `Reflection/DllReflector.cs` — the `--from-dlls` path. Crude (method-only). **Target of this issue.**
2. `Ingest/Extractors/Common/MetadataReflector.cs` — the coverage-extractor path. Already a rich reader
   (types, methods, properties, events, **fields**, ctors, generics, params → `AssemblyRecord`) **but
   discards custom attributes** (its `ICustomAttributeTypeProvider` is an explicit stub) and is
   **single-assembly** (`Reflect(string dllPath)`).

## 2. Architecture — Reader / Recipe split

A new **`AwareReader`** class library (`cli-reader/`, `netstandard2.0`, AOT-safe — pure data transforms)
becomes the single home for the IR records (extended with attributes), the synthesizer
(`IR → GeneratedAgent`), the recipe registry, and the `GeneratedAgent`/`Command`/`Skill` output types
(moved out of `cli-sidecar/Protocol`). `MetadataReflector` moves here too (it is pure
`System.Reflection.Metadata`, already AOT-safe; the 7 coverage-extractor files that use it get a one-line
`using` change).

Two readers feed it; one synthesizer drains it:

```
 --from-dlls / --from-nuget ──► aware-sidecar (AOT)   : MetadataReflector.ReflectSet(dlls) ─┐
                                                                                             ├─► AwareReader
 --from-csharp/-csproj/-sln ──► aware-roslyn (JIT)    : RoslynReader  → same IR records  ───┘    AgentSynthesizer
                                                                                                  + Recipes(Tekla)
                                                                                                  → GeneratedAgent JSON
```

- `MetadataReflector` (compiled) and `RoslynReader` (source) converge on the **same IR records**, so the
  synthesizer + recipe are written **once** and treat compiled and source inputs identically. No drift, no
  second subprocess hop — `aware-roslyn` self-synthesizes by referencing `AwareReader`.
- Both `aware-sidecar` (AOT) and `aware-roslyn` (JIT) reference the shared lib. The sidecar stays AOT;
  Roslyn never enters the AOT binary.

### Module boundaries

| Unit | Responsibility | Depends on |
|---|---|---|
| `AwareReader` records | normalized IR: assemblies, types, members, fields, **attributes** | — |
| `AwareReader.MetadataReflector` | PE-metadata → IR (compiled), incl. `ReflectSet` + cross-assembly index | `System.Reflection.Metadata` |
| `AwareReader.AgentSynthesizer` | IR → `GeneratedAgent` (default per-type/per-method projection) | records |
| `AwareReader.Recipes` | recognize SDK idioms, override synthesis (Tekla plugin) | records, synthesizer |
| `aware-sidecar` | `reflect-dlls` verb: glob → `ReflectSet` → synthesize | `AwareReader` |
| `aware-roslyn` | `reflect-csharp` verb: source → IR → synthesize | `AwareReader`, Roslyn + Workspaces.MSBuild |
| `cli/src/builder/roslyn.rs` | discover + invoke `aware-roslyn`, parse `GeneratedAgent` | `sidecar.rs` patterns |

## 3. Part A — IR with attributes + fields

Extend the reader records with attribute carriage:

```csharp
public sealed record AttributeRecord(
    string TypeName,                               // full name, e.g. "Tekla.Structures.Plugins.StructuresFieldAttribute"
    IReadOnlyList<AttributeArg> FixedArguments,    // positional, in order
    IReadOnlyList<NamedArg> NamedArguments);       // named (properties/fields set in the attribute)
public sealed record AttributeArg(string Type, string? Value);   // Value = literal form (string/int/bool/enum-as-int/type-name)
public sealed record NamedArg(string Name, string Type, string? Value);
```

Add `IReadOnlyList<AttributeRecord> Attributes` to `TypeRecord`, `MethodRecord`, `FieldRecord`,
`PropertyRecord` (and `ParameterRecord` for completeness). Decode with
`CustomAttribute.DecodeValue(SigTypeProvider)` — the provider already implements
`ICustomAttributeTypeProvider<string>`; we flesh out `GetUnderlyingEnumType` / `GetTypeFromSerializedName`
best-effort (enum args decode to their integer literal; type-of args decode to the serialized type name).
The attribute **type name** is resolved with the constructor-walk already proven in
`DllReflector.ReadAssemblyDescription`, generalized into a shared `AttributeReader.ResolveAttributeTypeName`.

Stable ordering preserved (attributes sorted by `TypeName` then arg list) so repeated runs are identical.

## 4. Part B — multi-assembly context

`MetadataReflector.ReflectSet(IReadOnlyList<string> dllPaths)` returns:

```csharp
public sealed record ReflectedSet(
    IReadOnlyList<AssemblyRecord> Assemblies,
    IReadOnlyDictionary<string, TypeRecord> TypeIndex);  // FullName -> TypeRecord (across all assemblies)
```

The synthesizer uses `TypeIndex` to resolve a type referenced in one assembly whose definition lives in
another (the Isokorb split). Agent **id/description** chosen by heuristic, never first-assembly-wins:

1. explicit `--output` override, else
2. recipe-matched plugin name (when a recipe fires), else
3. the assembly with the most public types ("primary"), else
4. `reflected-agent`.

Description aggregates: primary assembly's `AssemblyDescriptionAttribute` if present, else a generated
"N assemblies / M types" summary. Unreadable/native DLLs are skipped with a stderr note (current behavior).

## 5. Part C — C# source reader (`aware-roslyn`)

New project `cli-roslyn/` (regular .NET `net10.0`, **not** AOT; published self-contained per-OS). stdin/stdout
JSON envelope mirroring the sidecar (`{op, args}` in, `{ok, version, op, data}` out). Op `reflect-csharp`:

- **args:** `paths: string[]` (each a `.cs` file/glob/dir, a `.csproj`, or a `.sln`), `agent_id?`.
- `.sln` / `.csproj` → `MSBuildWorkspace` (+ `MSBuildLocator.RegisterDefaults()`) loads the **project graph**,
  giving compilations with references resolved → full semantic `INamedTypeSymbol` data (base types,
  attributes with args, doc comments). This is the "project-graph resolver" role — `.csproj`/`.sln` find the
  right `.cs` + the cross-assembly set, they are not agent sources themselves.
- bare `.cs` globs → `CSharpCompilation.Create` over the parsed trees (+ best-effort framework refs) so
  attributes/base types still resolve where possible; degrades gracefully when a referenced type is absent.
- A `RoslynReader` maps `INamedTypeSymbol`/`IMethodSymbol`/`IFieldSymbol`/`AttributeData` → the **same**
  `AwareReader` IR records (`AssemblyRecord` per compilation, combined `TypeIndex`), then calls the shared
  `AgentSynthesizer`. Output is identical in shape to the compiled path.

`MSBuildLocator` requires the .NET SDK / MSBuild present on the host; if absent, `reflect-csharp` for
`.sln`/`.csproj` returns a clear error pointing at the `.cs`-glob fallback. (Documented limitation.)

## 6. Part D — Tekla recipe

A recipe is a post-pass over the IR: `bool Matches(ReflectedSet)` + `GeneratedAgent Apply(ReflectedSet, base)`.
The Tekla model-plugin recipe (grounded in `Tekla.Structures.Plugins`, see
`20-agents/aeco/engineering/tekla-2026/skills/tekla-structures-plugins.md`):

- **Match:** a public concrete class whose base-type chain reaches `Tekla.Structures.Plugins.PluginBase`
  (model plug-in). `ConnectionBase` / `CustomPartBase` / `DrawingPluginBase` are future variants; v1 targets
  `PluginBase`.
- **Plugin name:** from `[Plugin("name")]` (`PluginAttribute`, positional `string name`). Required.
- **Data contract (the cross-assembly join):** the plugin's **public constructor parameter type** is the
  StructuresData class — the classic pattern `public IsokorbPlugin(IsokorbData data) : PluginBase`. Resolve
  that type via `TypeIndex` (it may live in another assembly — the Isokorb split). Fallback: if no
  ctor-param data type, scan the plugin class itself for `[StructuresField]` fields.
- **Fields:** the data contract's **public fields** carrying `[StructuresField("attrName")]`
  (`StructuresFieldAttribute`, positional `string attributeName`), of type `double` / `int` / `string` only
  (per vendor: "the type must be double, integer or string").
- **Emit:** ONE command `insert-<plugin-kebab>` (lifecycle `single`, mode `write`) **instead of** the
  per-method `*-plugin-run` / `*-plugin-define-input` commands for that type. Inputs = the `[StructuresField]`
  set (typed `double`/`int`/`string`, mapped to AWARE input types `number`/`integer`/`string`, `default`
  from the data field's initializer when available) **plus** an `input-points` parameter (the picked points
  fed to `ComponentInput`). The command body documents the equivalent in-model action:
  `new ComponentInput()` (+ points) → `Component { Name = "<plugin>" }` → `SetAttribute("<field>", value)`
  per field → `Insert()` / `CommitComponentInput`. The `aware-tekla` bridge already runs such C# via its
  `exec` verb — **no bridge change**.

## 7. Protocol / Rust changes

- `GeneratedCommand` (C# `Protocol/Agent.cs` → `AwareReader`) gains optional `inputs` (list of
  `{name, type, optional, default}`) and `mode` so recipe commands carry their parameter contract. The Rust
  mirror (`cli/src/sidecar.rs::SidecarCommand`) and `builder::GeneratedCommand` already have
  `inputs_yaml`/`mode` plumbing into `write_agent`; we map the new structured inputs into `inputs_yaml`.
- `cli/src/commands/build.rs`: add `--from-csharp`, `--from-csproj`, `--from-sln` flags →
  `builder::roslyn::build_from_*`. `cli/src/builder/roslyn.rs` discovers + invokes `aware-roslyn`
  (discovery: `AWARE_ROSLYN` env → sibling of `aware` → PATH, mirroring `sidecar::discover`).
- `cli/src/builder/stubs.rs::build_from_dlls` is unchanged at the call site (still `reflect-dlls`); the
  richer behavior is internal to the sidecar.

## 8. Testing strategy (no proprietary Tekla DLLs needed)

- **C# unit (xUnit):** extend `FixtureAssembly` with a Tekla-shaped fixture — a `PluginBaseStub` +
  `[PluginStub("Demo")]` + `[StructuresFieldStub("Length")] public double Length;` data class in a **second**
  fixture assembly (`FixtureDataAssembly`) referenced by the first, so cross-assembly resolution + attribute
  decode + the recipe are all exercised against a real two-DLL graph. (Recipe matching is parameterized on
  the attribute/base-type **full names**, so a `*Stub` fixture validates the mechanism; a thin config maps
  the real `Tekla.Structures.Plugins.*` names.) Tests: attribute decode (positional + named), field
  extraction, `ReflectSet` cross-assembly index, synthesizer id heuristic, recipe match + emitted command.
- **Roslyn unit:** parse in-memory `.cs` source for the same fixture shape; assert identical IR + agent.
- **Rust (`assert_cmd`):** `--from-csharp` against a temp `.cs` fixture with `AWARE_ROSLYN` pointed at a
  built `aware-roslyn`; assert manifest + command files. Existing `--from-dlls` tests stay green.
- **End-to-end:** `aware build agent --from-dlls <fixture-bin>` and `--from-csharp <fixture.cs>` into a temp
  `AWARE_HOME`; diff observed vs expected (single `insert-demo` command with typed inputs, not
  `*-plugin-run`).

## 9. Release pipeline

`.github/workflows/release.yml` learns to build + publish `aware-roslyn` per-OS alongside `aware` /
`aware-sidecar` (self-contained publish; not AOT). The Rust CLI degrades gracefully when `aware-roslyn` is
absent: `--from-csharp/-csproj/-sln` returns `NotFound` with an install hint (same UX as a missing sidecar).

## 10. Risks & mitigations

- **Roslyn + MSBuildWorkspace footprint / SDK dependency** → self-contained publish; `.cs`-glob fallback
  when MSBuild is unavailable; clear error messaging.
- **Shared-lib AOT safety for the sidecar** → `AwareReader` is pure data transforms over
  `System.Reflection.Metadata`; no reflection on user types; `TreatWarningsAsErrors` + AOT publish enforced
  in CI for the sidecar.
- **Coverage-extractor churn** (7 files moving to `AwareReader` namespace) → mechanical `using` change,
  covered by existing extractor tests (Navisworks/TeklaSdk).
- **Recipe over-matching** → gated on exact `Tekla.Structures.Plugins.PluginBase` chain + `[Plugin]`
  presence; non-matching assemblies fall back to the (now richer) default per-type synthesis unchanged.

## 11. Out of scope (this epic)

- Recipes for `ConnectionBase` / `CustomPartBase` / `DrawingPluginBase`, Revit/Rhino idioms (future, on
  demand).
- Non-C# source readers (Ruby/npm already have their own readers; no new ones here).
- Any change to the `aware-tekla` runtime bridge.
