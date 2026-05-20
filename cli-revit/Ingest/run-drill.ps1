# AWARE v0.33 - revit.exec 20-prompt drill harness.
#
# Prerequisites:
#   1. Revit 2026 running with a model open.
#   2. AwareRevit.dll add-in installed (run install-addin.ps1).
#   3. aware-revit.exe built (Debug or Release).
#
# Output: cli-revit/Ingest/Output/drill-summary.md (PASS/FAIL + receipt JSONs).
#
# Compatible with Windows PowerShell 5.1 and PowerShell 7+.

param(
    [string]$AwareRevit = $null,
    [string]$FixturesDir = (Join-Path $PSScriptRoot "Output"),
    [string]$SummaryPath = (Join-Path $PSScriptRoot "Output\drill-summary.md")
)

$ErrorActionPreference = "Stop"

if (-not $AwareRevit) {
    $candidates = @(
        (Join-Path $PSScriptRoot "..\Sidecar\bin\Release\net8.0-windows\win-x64\publish\aware-revit.exe"),
        (Join-Path $PSScriptRoot "..\Sidecar\bin\Release\net8.0-windows\aware-revit.exe"),
        (Join-Path $PSScriptRoot "..\Sidecar\bin\Debug\net8.0-windows\aware-revit.exe")
    )
    foreach ($c in $candidates) {
        if (Test-Path $c) { $AwareRevit = $c; break }
    }
}
if (-not $AwareRevit -or -not (Test-Path $AwareRevit)) {
    Write-Error "aware-revit.exe not found. Run 'dotnet publish cli-revit/Sidecar/cli-revit.csproj -c Release -r win-x64' first, or pass -AwareRevit <path>."
}

$fixtures = Get-ChildItem -Path $FixturesDir -Filter "prompt-*.json" | Sort-Object Name
if ($fixtures.Count -eq 0) { Write-Error "No prompt-*.json fixtures found in $FixturesDir" }

Write-Host "Drill: $($fixtures.Count) prompts against $AwareRevit"
Write-Host ""

$results = @()
$pass = 0; $fail = 0
foreach ($fix in $fixtures) {
    $name = $fix.BaseName
    Write-Host -NoNewline ("  {0} ... " -f $name)
    $payload = Get-Content $fix.FullName -Raw
    $rawOutput = $payload | & $AwareRevit --json-stdin 2>&1
    $exit = $LASTEXITCODE
    if ($rawOutput -is [array]) { $rawOutput = ($rawOutput -join "`n") }
    $rawOutput = [string]$rawOutput

    $receipt = $null
    try { $receipt = $rawOutput | ConvertFrom-Json } catch { }

    $ok = $false
    if ($receipt -and ($receipt.ok -eq $true -or $receipt.status -eq "ok")) {
        $ok = $true
    }
    $status = if ($ok) { "PASS" } else { "FAIL" }
    if ($ok) { $pass++ } else { $fail++ }

    $results += [pscustomobject]@{
        Name      = $name
        Status    = $status
        ExitCode  = $exit
        Receipt   = $rawOutput
    }

    Write-Host $status
}

Write-Host ""
Write-Host "Summary: $pass PASS, $fail FAIL of $($fixtures.Count)"

$lines = @()
$lines += "# revit.exec drill - $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')"
$lines += ""
$lines += "**Sidecar:** $AwareRevit"
$lines += "**Fixtures:** $FixturesDir"
$lines += "**Result:** $pass PASS, $fail FAIL of $($fixtures.Count)"
$lines += ""
$lines += "| # | Status | Exit | Result snippet |"
$lines += "|---|---|---|---|"
foreach ($r in $results) {
    $clean = ($r.Receipt -replace "[`r`n]+", " ")
    $maxLen = [Math]::Min(120, $clean.Length)
    $snippet = $clean.Substring(0, $maxLen)
    $lines += "| $($r.Name) | **$($r.Status)** | $($r.ExitCode) | ``$snippet`` |"
}
$lines += ""
$lines += "## Full receipts"
$lines += ""
foreach ($r in $results) {
    $lines += "### $($r.Name) - $($r.Status)"
    $lines += ""
    $lines += '```json'
    $lines += $r.Receipt
    $lines += '```'
    $lines += ""
}
$lines -join "`n" | Out-File -FilePath $SummaryPath -Encoding utf8
Write-Host "Wrote summary -> $SummaryPath"

exit $fail
