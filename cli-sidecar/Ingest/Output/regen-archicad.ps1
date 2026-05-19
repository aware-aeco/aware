# regen-archicad.ps1 — re-generate ArchiCAD agent from IR + run strict 50-type verify.
#
# Mirrors the Rhino / Grasshopper regen scripts: vendor `graphisoft`, vertical `architecture`.
# Copies manifest.yaml + skills/ + catalog/ into 20-agents/aeco/architecture/archicad-<v>/.
#
# Usage: ./regen-archicad.ps1 <version> [<count>=50]   # version is "28.0" or "29.0"

param(
    [Parameter(Mandatory)] [string] $Version,
    [int] $Count = 50
)

$RepoRoot = "C:\Users\bimst\source\repos\aware"
$VMajor = $Version.Split('.')[0]
$Ir = Join-Path $RepoRoot "cli-sidecar\Ingest\Output\archicad-$Version.ir.json"
$Aware = Join-Path $RepoRoot "cli\target\release\aware.exe"
$Sidecar = Join-Path $RepoRoot "cli-sidecar\bin\Release\net9.0\win-x64\publish\aware-sidecar.exe"

$env:AWARE_SIDECAR = $Sidecar

if (-not (Test-Path $Ir)) {
    Write-Error "IR file not found: $Ir"
    exit 1
}

Write-Host "==> Step 1: regenerate agent from IR"
& $Aware coverage generate archicad $Version --from-ir $Ir --vendor graphisoft --vertical architecture
if ($LASTEXITCODE -ne 0) { Write-Error "coverage generate failed with exit $LASTEXITCODE"; exit 1 }

Write-Host ""
Write-Host "==> Step 2: copy artefacts into 20-agents/aeco/architecture/archicad-$VMajor/"
$src = "$HOME\.aware\agents\archicad-$Version"
$dst = Join-Path $RepoRoot "20-agents\aeco\architecture\archicad-$VMajor"
if (-not (Test-Path $src)) { Write-Error "Generated agent not found at $src"; exit 1 }
if (-not (Test-Path $dst)) { New-Item -ItemType Directory -Path $dst -Force | Out-Null }
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
