# Agent Builder Skill · Extract from a NuGet package

**Source type: `nuget`. Fetch a public NuGet package, extract DLLs + XML docs, run the same reflection pipeline as [extract-from-dlls](./extract-from-dlls.md). Optional ILSpy decompile pass when XML docs are missing.**

The point: you don't need the software installed locally. If the vendor publishes a NuGet package (most do), you can build an agent from your laptop in minutes.

## Inputs

- Package name (e.g. `Autodesk.Forge.DesignAutomation`, `Bentley.OpenBridge.Modeler`)
- Optional version (default = latest)
- Optional `--decompile` flag (off by default; license-checked)

## Pipeline

```
1. Resolve version          (latest from api.nuget.org if not pinned)
2. Download .nupkg          (curl, ~few MB typically)
3. Unzip                    (.nupkg is a zip)
4. Pick TFM                 (prefer netstandard2.0 → net6.0 → net48)
5. Extract DLLs + XML docs  (from lib/<tfm>/*.dll and lib/<tfm>/*.xml)
6. Check XML doc coverage   (% of public members with <summary>)
7. If coverage < threshold AND --decompile AND license permits:
     → ILSpy pass, extract method bodies, summarize with AI
8. Run normal DLL pipeline  (see extract-from-dlls.md)
9. Tag manifest provenance  (provenance.source.type = "nuget")
```

## Concrete: fetching

```bash
TEKLA_VERSION="2025.0.0"

curl -L -s -o /tmp/Tekla.Structures.Model.nupkg \
  "https://api.nuget.org/v3-flatcontainer/tekla.structures.model/$TEKLA_VERSION/tekla.structures.model.$TEKLA_VERSION.nupkg"

unzip -o /tmp/Tekla.Structures.Model.nupkg -d /tmp/tekla-model/
# → /tmp/tekla-model/lib/netstandard2.0/Tekla.Structures.Model.dll
# → /tmp/tekla-model/lib/netstandard2.0/Tekla.Structures.Model.xml  (sometimes)
```

## TFM selection

Prefer in this order:

1. `netstandard2.0` — most portable, works with reflection on any host
2. `net6.0` / `net8.0` — modern
3. `net48` — falls back for older packages
4. Newest available — last resort

The agent declares which TFM was used in `provenance.source.tfm`. If a user later targets a different TFM and signatures differ, regenerate.

## XML doc coverage check

After reflection, measure % of public members that have `<summary>`. Thresholds:

| Coverage | Builder behavior |
|---|---|
| ≥ 70% | Use XML docs directly. Tier 1 entries have prose descriptions. |
| 30–70% | Mixed. Members with docs get prose; members without get signature-only. Builder logs a warning. |
| < 30% | If `--decompile` is set and license permits: run ILSpy pass. Otherwise emit signatures-only and prompt the contributor to refine. |

## Decompile pass (when invoked)

Only runs if **all** of:
- User passed `--decompile`
- Package license is in the permissive whitelist (MIT, BSD, Apache 2.0, MPL 2.0, ISC, Unlicense). See [license-aware-extraction.md](./license-aware-extraction.md).
- The user explicitly confirms the license at the prompt (one-time per package)

```bash
ilspycmd Tekla.Structures.Model.dll -o /tmp/decompiled/ --project
```

The decompiled `.cs` is then chunked and fed to the AI with prompts like:

> *Summarize what this method does in one sentence, focusing on the side effects on Tekla model state. The summary will appear in an AWARE agent skill as if it were an XML doc comment.*

The AI's summaries become the prose in the skill file. **Decompiled code itself is never written to the agent folder** — only the AI's summary of what it does. This sidesteps redistribution concerns.

## Output

Same shape as [extract-from-dlls](./extract-from-dlls.md), with two additions:

- `provenance.source.type: nuget`
- `provenance.source.url: https://api.nuget.org/v3-flatcontainer/...`
- `provenance.source.tfm: netstandard2.0`
- `provenance.source.coverage: 0.84` (XML doc coverage)

## Why this matters strategically

Without this skill, AWARE only works for software the contributor has installed locally. With this skill, **anyone can build an agent for any publicly-distributed AECO library on NuGet** — even without owning a license to the underlying software.

Examples already feasible:
- `Autodesk.Forge.DesignAutomation` (APS cloud)
- `Bentley.MicroStation.SDK`
- `IfcOpenShell.Net`
- `Tekla.Structures.Model` (yes, even though you'd typically have it installed)
