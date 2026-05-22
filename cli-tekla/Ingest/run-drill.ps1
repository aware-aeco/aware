# AWARE v0.31 - tekla.exec 20-prompt drill harness.
#
# Prerequisites:
#   1. Tekla Structures 2026 running with a model open.
#   2. aware-tekla.exe built (Debug or Release).
#
# Output: cli-tekla/Ingest/Output/drill-summary.md (PASS/FAIL + receipt JSONs).
#
# Compatible with Windows PowerShell 5.1 and PowerShell 7+.

param(
    [string]$AwareTekla = $null,
    [string]$FixturesDir = (Join-Path $PSScriptRoot "Output"),
    [string]$SummaryPath = (Join-Path $PSScriptRoot "Output\drill-summary.md")
)

$ErrorActionPreference = "Stop"

if (-not $AwareTekla) {
    $candidates = @(
        (Join-Path $PSScriptRoot "..\bin\Release\net48\publish\aware-tekla.exe"),
        (Join-Path $PSScriptRoot "..\bin\Release\net48\aware-tekla.exe"),
        (Join-Path $PSScriptRoot "..\bin\Debug\net48\aware-tekla.exe")
    )
    foreach ($c in $candidates) {
        if (Test-Path $c) { $AwareTekla = $c; break }
    }
}
if (-not $AwareTekla -or -not (Test-Path $AwareTekla)) {
    Write-Error "aware-tekla.exe not found. Run 'dotnet publish cli-tekla/cli-tekla.csproj -c Release' first, or pass -AwareTekla <path>."
}

$fixtures = Get-ChildItem -Path $FixturesDir -Filter "prompt-*.json" | Sort-Object Name
if ($fixtures.Count -eq 0) { Write-Error "No prompt-*.json fixtures found in $FixturesDir" }

Write-Host "Drill: $($fixtures.Count) prompts against $AwareTekla"
Write-Host ""

$results = @()
$pass = 0; $fail = 0
foreach ($fix in $fixtures) {
    $name = $fix.BaseName
    Write-Host -NoNewline ("  {0} ... " -f $name)
    # Drive stdin via Process API, writing raw UTF-8 bytes to BaseStream so
    # PowerShell's pipe OutputEncoding (which prepends a BOM on 5.1/net48) is
    # never in the path. File.ReadAllBytes + manual BOM strip = byte-perfect.
    $payloadBytes = [System.IO.File]::ReadAllBytes($fix.FullName)
    # Strip UTF-8 BOM (EF BB BF) if present.
    if ($payloadBytes.Length -ge 3 -and
        $payloadBytes[0] -eq 0xEF -and $payloadBytes[1] -eq 0xBB -and $payloadBytes[2] -eq 0xBF) {
        $payloadBytes = $payloadBytes[3..($payloadBytes.Length - 1)]
    }
    $psi = New-Object System.Diagnostics.ProcessStartInfo
    $psi.FileName               = $AwareTekla
    $psi.Arguments              = "--json-stdin"
    $psi.RedirectStandardInput  = $true
    $psi.RedirectStandardOutput = $true
    $psi.RedirectStandardError  = $true
    $psi.UseShellExecute        = $false
    $proc = [System.Diagnostics.Process]::Start($psi)
    $proc.StandardInput.BaseStream.Write($payloadBytes, 0, $payloadBytes.Length)
    $proc.StandardInput.Close()
    $stdout = $proc.StandardOutput.ReadToEnd()
    $stderr = $proc.StandardError.ReadToEnd()
    $proc.WaitForExit()
    $exit      = $proc.ExitCode
    $rawOutput = if ($stdout) { $stdout } else { $stderr }
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
$lines += "# tekla.exec drill - $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')"
$lines += ""
$lines += "**Sidecar:** $AwareTekla"
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
