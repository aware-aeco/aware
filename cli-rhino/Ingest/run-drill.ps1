# AWARE v0.32 - rhino.exec 20-prompt drill harness.
#
# Prerequisites:
#   1. Rhino 8.11+ installed, with `StartScriptServer` run in the active session.
#   2. aware-rhino.exe built (Debug or Release).
#
# Output: cli-rhino/Ingest/Output/drill-summary.md (PASS/FAIL table + receipt JSONs).
#
# Compatible with Windows PowerShell 5.1 and PowerShell 7+.

param(
    [string]$AwareRhino = $null,
    [string]$FixturesDir = (Join-Path $PSScriptRoot "Output"),
    [string]$SummaryPath = (Join-Path $PSScriptRoot "Output\drill-summary.md")
)

$ErrorActionPreference = "Stop"

# Auto-discover aware-rhino.exe if not supplied: prefer Release publish, then Release, then Debug.
if (-not $AwareRhino) {
    $candidates = @(
        (Join-Path $PSScriptRoot "..\bin\Release\net10.0-windows\win-x64\publish\aware-rhino.exe"),
        (Join-Path $PSScriptRoot "..\bin\Release\net10.0-windows\aware-rhino.exe"),
        (Join-Path $PSScriptRoot "..\bin\Debug\net10.0-windows\aware-rhino.exe")
    )
    foreach ($c in $candidates) {
        if (Test-Path $c) { $AwareRhino = $c; break }
    }
}
if (-not $AwareRhino -or -not (Test-Path $AwareRhino)) {
    Write-Error "aware-rhino.exe not found. Run ``dotnet publish -c Release -r win-x64`` first, or pass -AwareRhino <path>."
}

$fixtures = Get-ChildItem -Path $FixturesDir -Filter "prompt-*.json" | Sort-Object Name
if ($fixtures.Count -eq 0) {
    Write-Error "No prompt-*.json fixtures found in $FixturesDir"
}

Write-Host "Drill: $($fixtures.Count) prompts against $AwareRhino"
Write-Host ""

$results = @()
$pass = 0; $fail = 0
foreach ($fix in $fixtures) {
    $name = $fix.BaseName
    Write-Host -NoNewline ("  {0} ... " -f $name)
    $payload = Get-Content $fix.FullName -Raw
    $rawOutput = $payload | & $AwareRhino --json-stdin 2>&1
    $exit = $LASTEXITCODE
    # Stringify in case of mixed stdout/stderr capture from 2>&1
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

# Write markdown summary
$lines = @()
$lines += "# rhino.exec drill - $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')"
$lines += ""
$lines += "**Sidecar:** $AwareRhino"
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
