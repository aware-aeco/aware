# AWARE v0.34 - sketchup.exec 20-prompt drill harness.
#
# Prerequisites:
#   1. SketchUp 2026 installed.
#   2. aware-sketchup.exe built (Debug or Release).
#   3. aware_sketchup_bridge.rb installed into Plugins/ via:
#         aware-sketchup --install-bridge
#   4. SketchUp restarted so the bridge auto-loads.
#   5. (Recommended) A SketchUp document open with a few entities + selection
#      so write/ops prompts have something to act on. New-doc default works
#      for read-side prompts.
#
# Output: cli-sketchup/Ingest/Output/drill-summary.md (PASS/FAIL table + receipt JSONs).
#
# Compatible with Windows PowerShell 5.1 and PowerShell 7+.

param(
    [string]$AwareSketchup = $null,
    [string]$FixturesDir   = (Join-Path $PSScriptRoot "Output"),
    [string]$SummaryPath   = (Join-Path $PSScriptRoot "Output\drill-summary.md"),
    [switch]$NoDismissWelcome
)

$ErrorActionPreference = "Stop"

# Auto-discover aware-sketchup.exe if not supplied. Try win-x64 release first.
if (-not $AwareSketchup) {
    $candidates = @(
        (Join-Path $PSScriptRoot "..\bin\Release\net10.0-windows\win-x64\publish\aware-sketchup.exe"),
        (Join-Path $PSScriptRoot "..\bin\Release\net10.0-windows\win-x64\aware-sketchup.exe"),
        (Join-Path $PSScriptRoot "..\bin\Release\net10.0-windows\aware-sketchup.exe"),
        (Join-Path $PSScriptRoot "..\bin\Debug\net10.0-windows\aware-sketchup.exe")
    )
    foreach ($c in $candidates) {
        if (Test-Path $c) { $AwareSketchup = $c; break }
    }
}
if (-not $AwareSketchup -or -not (Test-Path $AwareSketchup)) {
    Write-Error "aware-sketchup.exe not found. Run ``dotnet publish -c Release -r win-x64`` first, or pass -AwareSketchup <path>."
}

# Auto-dismiss the SketchUp 2026 "Welcome to SketchUp" CEF modal if it's
# blocking Plugins/ load. See dismiss-welcome.ps1 for the technique. Skip
# with -NoDismissWelcome if you've already dismissed it manually.
if (-not $NoDismissWelcome) {
    $dismissScript = Join-Path $PSScriptRoot "dismiss-welcome.ps1"
    if (Test-Path $dismissScript) {
        $sketchup = Get-Process SketchUp -ErrorAction SilentlyContinue
        if ($sketchup) {
            Write-Host "Checking for SketchUp Welcome dialog..."
            # Suppress non-zero exit so a "no welcome to dismiss" / harmless
            # timeout doesn't kill the drill.
            try { & $dismissScript -TimeoutSec 5 | Out-Host } catch { Write-Host "  (dismiss-welcome reported: $_; continuing)" }
        }
    }
}

$fixtures = Get-ChildItem -Path $FixturesDir -Filter "prompt-*.json" | Sort-Object Name
if ($fixtures.Count -eq 0) {
    Write-Error "No prompt-*.json fixtures found in $FixturesDir"
}

Write-Host "Drill: $($fixtures.Count) prompts against $AwareSketchup"
Write-Host ""

$results = @()
$pass = 0; $fail = 0
foreach ($fix in $fixtures) {
    $name = $fix.BaseName
    Write-Host -NoNewline ("  {0} ... " -f $name)
    # Use cmd's stdin redirection instead of PowerShell's pipe. PS5/PS7 pipes
    # re-encode native-cmd stdin through the console output encoding which adds
    # a UTF-16 BOM in PS5; the sidecar's JSON parser then rejects the leading
    # 0xEF byte. cmd's '<' is a byte-faithful redirect.
    $rawOutput = cmd /c "`"$AwareSketchup`" --json-stdin < `"$($fix.FullName)`"" 2>&1
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

# Write markdown summary
$lines = @()
$lines += "# sketchup.exec drill - $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')"
$lines += ""
$lines += "**Sidecar:** $AwareSketchup"
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
