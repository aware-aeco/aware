# resume-extract.ps1 — re-invoke a stalled extraction. Reads the existing snapshot at
# Ingest/Output/<vendor>-<version>.ir.json, re-queues all placeholder members, and lets the
# auto-restart wrapper finish the remaining work.
#
# Use this when an extraction "completes" with many errors due to CDN rate-limiting. The first
# pass populates as much as it can; subsequent passes (each spaced out from the throttle event)
# converge to 100% enrichment.

param(
    [Parameter(Mandatory)] [ValidateSet("rhino", "grasshopper")] [string] $Vendor,
    [Parameter(Mandatory)] [string] $Version,
    [int] $MaxPasses = 10
)

$Repo = "C:\Users\bimst\source\repos\aware"
$Wrapper = Join-Path $Repo "cli-sidecar\Ingest\Output\extract-$Vendor-with-restart.ps1"
$Ir = Join-Path $Repo "cli-sidecar\Ingest\Output\$Vendor-$Version.ir.json"

if (-not (Test-Path $Ir)) {
    Write-Error "IR not found: $Ir"
    exit 1
}

for ($pass = 1; $pass -le $MaxPasses; $pass++) {
    Write-Host "==> Pass $pass — invoking $Wrapper -Version $Version"
    & $Wrapper -Version $Version
    if ($LASTEXITCODE -ne 0) {
        Write-Warning "Pass $pass wrapper exited with $LASTEXITCODE — continuing"
    }

    # Read the IR and count placeholders.
    $json = Get-Content $Ir -Raw | ConvertFrom-Json
    $unenriched = 0
    foreach ($t in $json.types) {
        foreach ($m in $t.methods) {
            if ($m.signature -eq "$($m.name)(...)" -or $m.signature -eq "$($t.name)(...)") { $unenriched++ }
        }
        foreach ($c in $t.constructors) {
            if ($c.signature -eq "$($c.name)(...)" -or $c.signature -eq "$($t.name)(...)") { $unenriched++ }
        }
        foreach ($p in $t.properties) {
            if ($p.type -eq "object") { $unenriched++ }
        }
        foreach ($e in $t.events) {
            if ($e.delegate -eq "EventHandler") { $unenriched++ }
        }
    }
    Write-Host "==> Pass $pass — unenriched: $unenriched"
    if ($unenriched -lt 50) {
        Write-Host "==> DONE after $pass passes"
        return
    }
    Start-Sleep 30   # Pause between passes — let any throttle reset.
}
Write-Warning "Exhausted $MaxPasses passes; some members may remain at placeholder. Re-run resume-extract.ps1 to continue."
