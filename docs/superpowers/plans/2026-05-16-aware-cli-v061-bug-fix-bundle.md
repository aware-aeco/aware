# AWARE CLI v0.6.1 (Bug Fix Bundle from First End-User Dogfood) Plan

> **Context:** Written after the v0.6.0 release was published and the binaries
> were installed + run as an end user against real Tekla 2025 + Revit 2026
> install paths. Three real bugs surfaced. This document captures the fixes.

## Bugs found during end-user dogfooding

1. **Sidecar reflection can't follow dependencies.** Both Tekla
   (`Tekla.Structures.Model.dll`) and Revit (`RevitAPI.dll`) failed with
   "Could not find assembly …" errors:
   - Tekla wanted `Tekla.Structures.dll` (a sibling DLL in the same directory)
   - Revit wanted `System.Drawing.Common, Version=7.0.0.0` (a reference
     assembly from an older .NET SDK that the .NET 9 runtime dir doesn't
     contain)
   - The original `PathAssemblyResolver` only knew about input paths +
     `typeof(object).Assembly.Location`'s directory. That covers almost
     nothing real.

2. **PowerShell installer choked on 8.3 short-name TEMP path.** On a machine
   where `$env:TEMP` resolves to `C:\Users\<u>\AppData\Local\TEMP_~1` (the 8.3
   short form), `New-Item` created the directory at the short-name path but
   subsequent `Expand-Archive` / `Copy-Item` operations resolved it through
   long-form, producing path mismatch errors.

3. **Windows zip layout was inconsistent with Linux/macOS tar.gz.** The
   release workflow used `Compress-Archive -Path "$STAGE_DIR\*"` (contents
   only) while the Unix tar archives the directory itself. The installer
   expected the latter shape on all platforms, so the Windows zip download
   was always broken.

## Fixes applied

### Fix 1: `cli-sidecar/Reflection/DllReflector.cs`

Replaced the single resolver path list with a priority-ordered 4-tier
strategy:

1. Input DLLs (highest priority — exact matches preferred)
2. All `*.dll` siblings in each input's directory (catches the Tekla case)
3. Current runtime dir (existing fallback)
4. All installed .NET reference packs under
   `Program Files\dotnet\packs\*\*\ref\*\*.dll` (catches the Revit case —
   older System.Drawing.Common etc. ship as ref-pack DLLs)

Added `GetRefPackRoots()` helper, platform-aware (Windows / Linux / macOS),
respects `DOTNET_ROOT` env var.

Critical deduplication: filename-based `seenNames` dict so
`PathAssemblyResolver` never receives two paths with the same assembly
name (previously caused `FileLoadException: already loaded` on
overlapping ref packs).

### Fix 2: `scripts/install.ps1`

```powershell
$tmp = New-Item -ItemType Directory -Path (Join-Path $env:TEMP "aware-install-...") -Force
$tmp = (Get-Item $tmp.FullName).FullName   # ← canonicalize 8.3 short names
```

### Fix 3: `.github/workflows/release.yml`

```diff
- Compress-Archive -Path "${{ env.STAGE_DIR }}\*" -DestinationPath "${{ env.STAGE_DIR }}.zip"
+ Compress-Archive -Path "${{ env.STAGE_DIR }}"   -DestinationPath "${{ env.STAGE_DIR }}.zip"
```

Now Windows zip and Unix tar.gz both contain the staging directory as the
top-level entry. The installers' `$srcDir = Join-Path $tmp "aware-<v>-<rid>"`
expectation is consistent across platforms.

## Verification — live re-test against the real DLLs that originally failed

```
=== Tekla 2025 ===
$ aware build agent --from-dlls "C:\Program Files\Tekla Structures\2025.0\bin\Tekla.Structures.Model.dll" --output tekla-2025-model
✓ generated tekla-2025-model (2848 commands, 16 skills)

=== Revit 2026 ===
$ aware build agent --from-dlls "C:\Program Files\Autodesk\Revit 2026\RevitAPI.dll" --output revit-2026-api
✓ generated revit-2026-api (6445 commands, 24 skills)
```

Both previously failing assemblies now reflect successfully. The output is
honest raw API surface (no filtering, no curation) — 2,848 / 6,445
commands is the actual public method count on each input DLL. Curation
happens later via `aware skill modify`.

## Tests

- C# sidecar: 13 → **15 passed, 0 failed** (added two resolver-augmentation tests)
- Rust CLI: all 151 unit + integration tests pass unchanged

## Not in this PR

The user has separately directed v0.7 toward dual distribution (MSI + npm).
That's a much larger change and lives in its own plan + PR.
