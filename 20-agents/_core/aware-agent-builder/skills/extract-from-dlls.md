# Agent Builder Skill · Extract from local DLLs

**Source type: `dlls`. Reflect a folder of .NET assemblies, supplement with XML doc comments, emit per-namespace skill files. This is the original pattern that proved the meta-primitive works (Tekla extractor).**

## Inputs

- Glob path to one or more `.dll` files (e.g. `"C:/Program Files/Autodesk/AutoCAD 2025/*.dll"`)
- Optional: matching `.xml` files alongside the DLLs (Visual Studio writes these for projects compiled with `/doc`)

## Pipeline

```
1. Validate environment       (powershell available, target DLLs exist)
2. Resolve XML doc paths      (look for *.xml next to *.dll, or accept --xml-dir override)
3. Reflect each DLL           (PowerShell script: enumerate types/methods/properties)
4. Merge XML doc descriptions (one <member> per reflected member)
5. Bucket types by namespace  (one bucket → one skill file)
6. Apply tier strategy        (see tier-strategy.md)
7. Generate skill files       (one per bucket, Markdown with tables)
8. Generate manifest.yaml     (commands derived from public static methods + likely entry points)
9. Validate output            (aware agent validate runs all the checks)
```

The reflection script lives at `agent-builder/runtime/reflect.ps1` (PowerShell, cross-version compatible).

## What gets extracted per type

For each public type in the assembly:

- Name, namespace, base type, abstract/sealed modifiers
- Constructors with parameter types
- Public properties (name, type, get/set)
- Public methods (name, return type, parameters with `ref`/`out`)
- Static methods/properties
- Enum names and values
- Inheritance hierarchy (used to render ASCII trees in the skill)

XML doc tags (`<summary>`, `<param>`, `<returns>`, `<remarks>`) become prose in the skill file. If XML docs are missing, see [extract-from-nuget.md](./extract-from-nuget.md) for fetching them, or fall back to signatures-only output.

## Output shape per namespace

```markdown
---
name: <agent-id>-<namespace>
description: <agent-id> API — <Namespace>. Generated from <version>.
---

# <Namespace> API Reference (v<version>)

## Critical patterns
(Hand-edited by refiner — the AI's read-this-first section. Empty in raw output; the contributor fills this in based on extracted-from-dlls.md guidance.)

## Inheritance hierarchy
ASCII tree of public class relationships.

## Core classes (Tier 1)
Top N most-important types per namespace, fully documented:
- constructors
- properties
- methods
- XML doc descriptions when present

## Enumerations
All enum values with descriptions.

## Type reference (Tier 2)
Remaining types, compact one-line-per-type table:
| Type | Base | Description |
|---|---|---|
```

## Tier selection

See [tier-strategy.md](./tier-strategy.md). The default is `auto`: top 25–35 types per namespace go to Tier 1, the rest to Tier 2.

## DLL-to-skill mapping heuristic

One skill file per **top-level namespace**. Sub-namespaces are flattened into the parent unless they have ≥ 20 public types (then they get their own file).

Example for Tekla Structures 2025.0:

| DLL | Skill file(s) | Public types |
|---|---|---|
| Tekla.Structures.dll | `tekla-structures.md` + `tekla-structures-filtering.md` | ~271 |
| Tekla.Structures.Model.dll | `tekla-structures-model.md` | ~372 |
| Tekla.Structures.Drawing.dll | `tekla-structures-drawing.md` | ~368 |

## What to skip

- Internal namespaces (`*Internal*`)
- FlatBuffers / Render namespaces (generated, not for end users)
- Non-vendor types that snuck in (e.g. references to third-party libs)
- Compiler-generated types (`<>c__DisplayClass`)

## Validation

The generator runs `aware agent validate` on output before emitting. If any of these fail, the build aborts with a clear error:

- Manifest fields complete
- At least one skill file exists
- At least one command defined
- All command input/output schemas valid YAML
- No XML escape errors in skill markdown
