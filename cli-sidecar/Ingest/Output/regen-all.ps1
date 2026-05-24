# regen-all.ps1 — regenerate all four Rhino/Grasshopper agents and run strict verify on each.
# Assumes all four IRs are present and reasonably enriched.

$RepoRoot = "C:\Users\bimst\source\repos\aware"
$Aware = Join-Path $RepoRoot "cli\target\release\aware.exe"
$Sidecar = Join-Path $RepoRoot "cli-sidecar\bin\Release\net10.0\win-x64\publish\aware-sidecar.exe"
$env:AWARE_SIDECAR = $Sidecar

$jobs = @(
    @{ Vendor = "rhino"; Version = "7.0"; VMajor = "7" },
    @{ Vendor = "rhino"; Version = "8.0"; VMajor = "8" },
    @{ Vendor = "grasshopper"; Version = "7.0"; VMajor = "7" },
    @{ Vendor = "grasshopper"; Version = "8.0"; VMajor = "8" }
)

foreach ($job in $jobs) {
    $Vendor = $job.Vendor
    $Version = $job.Version
    $VMajor = $job.VMajor
    $Ir = Join-Path $RepoRoot "cli-sidecar\Ingest\Output\$Vendor-$Version.ir.json"

    Write-Host ""
    Write-Host "================================================================================"
    Write-Host "  $Vendor-$VMajor (version $Version)"
    Write-Host "================================================================================"

    if (-not (Test-Path $Ir)) {
        Write-Warning "  IR missing: $Ir — skipping"
        continue
    }

    Write-Host "==> Step 1: regenerate agent from IR"
    & $Aware coverage generate $Vendor $Version --from-ir $Ir --vendor mcneel --vertical architecture
    if ($LASTEXITCODE -ne 0) {
        Write-Warning "  coverage generate failed for $Vendor-$Version with exit $LASTEXITCODE"
        continue
    }

    Write-Host ""
    Write-Host "==> Step 2: copy artefacts to 20-agents/aeco/architecture/$Vendor-$VMajor/"
    $src = "$HOME\.aware\agents\$Vendor-$Version"
    $dst = Join-Path $RepoRoot "20-agents\aeco\architecture\$Vendor-$VMajor"
    if (-not (Test-Path $src)) { Write-Warning "  generated agent not at $src"; continue }
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
    & $script -IrPath $Ir -Count 50 | Tee-Object -FilePath (Join-Path $RepoRoot "cli-sidecar\Ingest\Output\$Vendor-$VMajor-strict-verify.txt")
}

Write-Host ""
Write-Host "================================================================================"
Write-Host "  ALL FOUR AGENTS REGENERATED"
Write-Host "================================================================================"
