# `aware-agent-builder.build` — generate an AWARE agent from a source

The one command this agent exposes. Generates a full AWARE agent folder (manifest + skills + commands) from a single source.

## Lifecycle

`single` — one call, one response

## CLI surface

```bash
aware agent build --from-dlls    "C:/Program Files/Autodesk/AutoCAD 2025/*.dll"
aware agent build --from-nuget   Bentley.OpenBridge.Modeler
aware agent build --from-nuget   Tekla.Structures.Model@2025.0.0 --decompile
aware agent build --from-openapi https://docs.procore.com/openapi.yaml
aware agent build --from-com     "AutoCAD.Application"
aware agent build --from-cli     git
aware agent build --from-headers "C:/SDKs/Foo/include/*.h"
aware agent build --from-python  rhino3dm
```

Optional flags:

| Flag | Default | Effect |
|---|---|---|
| `--output <name>` | derived from source | Override the generated agent id |
| `--output-dir <path>` | `~/.aware/agents/` | Override the output directory |
| `--decompile` | off | Run ILSpy on DLLs missing XML docs. License-checked. |
| `--decompile-anyway` | off | Force decompile for non-permissive licenses. User assumes responsibility. |
| `--tier-strategy <auto\|all-tier-1\|all-tier-2>` | auto | Skill organization (see [tier-strategy.md](../skills/tier-strategy.md)) |
| `--dry-run` | off | Print what would be written without writing |
| `--no-validate` | off | Skip `aware agent validate` on output (discouraged) |

## Outputs

```yaml
agent-id:      string         # e.g. "autocad", derived from source or override
output-dir:    string         # absolute path; typically ~/.aware/agents/<id>/
files:         array          # list of files written, with relative paths
skill-count:   int
command-count: int
warnings:      array          # non-fatal issues (TFM fallback, low XML coverage, etc.)
```

## What gets generated

```
~/.aware/agents/<id>/
├── manifest.yaml
├── skills/
│   ├── <namespace-1>.md        ← one per top-level namespace (or per OpenAPI tag)
│   ├── <namespace-2>.md
│   └── ...
├── commands/
│   ├── <command-1>.md          ← one per detected entry point
│   └── ...
└── README.md                   ← brief summary with provenance info
```

The `manifest.yaml`'s `provenance` block records exactly how the agent was generated, with timestamps and the user identity. See [license-aware-extraction.md](../skills/license-aware-extraction.md) for the audit-trail spec.

## Composition examples

### As a one-shot CLI invocation

The typical usage. The user runs this directly in the terminal:

```bash
$ aware agent build --from-nuget Bentley.OpenBridge.Modeler

✓ Resolved Bentley.OpenBridge.Modeler@10.16.0 from nuget.org
✓ Downloaded (4.2 MB) and unpacked
✓ Picked TFM netstandard2.0 (28 DLLs)
✓ XML doc coverage: 73% (good)
✓ Reflected 432 public types across 6 namespaces
✓ Tier 1: 156 types (full detail) · Tier 2: 276 types (compact)
✓ Generated 6 skill files, 24 commands
✓ Validated output
→ Wrote ~/.aware/agents/bentley-openbridge-modeler/

Next: refine the skills with project-specific gotchas, then commit.
```

### As a node in an automation app

Less common but useful for "auto-onboard new SDK versions" pipelines:

```yaml
nodes:
  - id: watch-vendor-releases
    agent: rss
    command: watch
    config: { feed: "https://www.nuget.org/.../Tekla.Structures.Model/atom.xml" }

  - id: build
    agent: aware-agent-builder
    command: build
    config:
      from: nuget
      source: "Tekla.Structures.Model@{{ watch.version }}"

  - id: open-pr
    agent: github
    command: open-pull-request
    config:
      title: "chore: regen tekla agent for {{ watch.version }}"
      branch: "agent/tekla/{{ watch.version }}"
      files: "{{ build.files }}"
```

That's an *agent-of-agents* app: a watcher fires a builder, which proposes a PR to the AWARE repo. The substrate growing itself.

## Failure modes

| Error | Cause | Recovery |
|---|---|---|
| `builder.source-not-found` | Path or URL invalid | Check the source argument |
| `builder.tfm-not-supported` | Package has no DLLs in a TFM the builder knows | Specify `--tfm <name>` if you know which to use |
| `builder.no-xml-docs-and-no-decompile` | < 30% XML doc coverage and decompile is off | Pass `--decompile` (if license permits) or accept signatures-only output |
| `builder.license-decompile-blocked` | Package has a non-permissive license | Refine the agent by hand using the signatures-only output |
| `builder.validation-failed` | Generated agent doesn't pass `aware agent validate` | Pass `--no-validate` to see partial output and debug |

## See also

- [extract-from-dlls.md](../skills/extract-from-dlls.md) — the DLL reflection pipeline
- [extract-from-nuget.md](../skills/extract-from-nuget.md) — fetching from public packages
- [extract-from-openapi.md](../skills/extract-from-openapi.md) — REST API extraction
- [tier-strategy.md](../skills/tier-strategy.md) — how the output is organized
- [license-aware-extraction.md](../skills/license-aware-extraction.md) — when decompile is allowed
