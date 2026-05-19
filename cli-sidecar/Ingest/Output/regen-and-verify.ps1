# regen-and-verify.ps1 — re-generate Tekla agent from an IR + run strict 50-type verify.
#
# Usage: ./regen-and-verify.ps1 <version> [<count>=50]
#
# This script is a convenience wrapper for the post-extraction steps that have to run for
# each Tekla version after the IR is produced. It:
#
#   1. Calls `aware coverage generate` with the right --vendor / --vertical flags.
#   2. Copies manifest.yaml + skills/ + catalog/ from ~/.aware/agents/tekla-<v>/ into
#      20-agents/aeco/engineering/tekla-<vYear>/.
#   3. Runs the strict 50-type self-verify (verify-types-strict.ps1).

param(
    [Parameter(Mandatory)] [string] $Version,   # "2025.0" or "2026.0"
    [int] $Count = 50
)

$RepoRoot = "C:\Users\bimst\source\repos\aware"
$VYear = $Version.Split('.')[0]
$Ir = Join-Path $RepoRoot "cli-sidecar\Ingest\Output\tekla-$Version.ir.json"
$Aware = Join-Path $RepoRoot "cli\target\release\aware.exe"
$Sidecar = Join-Path $RepoRoot "cli-sidecar\bin\Release\net9.0\win-x64\publish\aware-sidecar.exe"

$env:AWARE_SIDECAR = $Sidecar

if (-not (Test-Path $Ir)) {
    Write-Error "IR file not found: $Ir"
    exit 1
}

Write-Host "==> Step 1: regenerate agent from IR"
& $Aware coverage generate tekla $Version --from-ir $Ir --vendor trimble --vertical engineering
if ($LASTEXITCODE -ne 0) { Write-Error "coverage generate failed with exit $LASTEXITCODE"; exit 1 }

Write-Host ""
Write-Host "==> Step 2: copy artefacts into 20-agents/aeco/engineering/tekla-$VYear/"
$src = "$HOME\.aware\agents\tekla-$Version"
$dst = Join-Path $RepoRoot "20-agents\aeco\engineering\tekla-$VYear"
if (-not (Test-Path $src)) { Write-Error "Generated agent not found at $src"; exit 1 }
if (-not (Test-Path $dst)) { New-Item -ItemType Directory $dst | Out-Null }
# Wipe old manifest/skills/catalog (preserve EXTRACTION-NOTES.md).
foreach ($n in @("manifest.yaml")) {
    $p = Join-Path $dst $n
    if (Test-Path $p) { Remove-Item -Force $p }
}
foreach ($d in @("skills", "catalog", "commands")) {
    $p = Join-Path $dst $d
    if (Test-Path $p) { Remove-Item -Recurse -Force $p }
}
Copy-Item (Join-Path $src "manifest.yaml") $dst
foreach ($d in @("skills", "catalog")) {
    $sub = Join-Path $src $d
    if (Test-Path $sub) { Copy-Item -Recurse $sub $dst }
}

Write-Host ""
Write-Host "==> Step 3: strict 50-type verify"
$script = Join-Path $RepoRoot "cli-sidecar\Ingest\Output\verify-types-strict.ps1"
& $script -IrPath $Ir -Count $Count
