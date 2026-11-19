<#
  A2 SPIKE (aware#858). Runs one spike script against the Tekla that is open, through `aware-tekla exec`.
  EXPERIMENTAL. Do not run it until the spike has been granted Tekla time and the scratch model is open
  (see docs/superpowers/specs/2026-11-18-a2-tekla-profile-create-SPIKE.md).

  -Script probe  : read-only (E0).
  -Script export : writes one export file under the scratch root (E1); changes no catalogue.
  -Script import : WRITES THE CATALOGUE (E2-E4). Needs -Write and -BackupManifest (from snapshot-catalog.ps1 -Mode Snapshot).
#>
param(
    [Parameter(Mandatory)][ValidateSet('probe', 'export', 'import')][string]$Script,
    [string]$ArgsJson = '{}',
    [string]$BackupManifest = '',
    [switch]$Write,
    [string]$Version = '2026.0',
    [string]$Exe = 'aware-tekla'
)
$ErrorActionPreference = 'Stop'
$code = Get-Content -LiteralPath (Join-Path $PSScriptRoot "$Script.csx") -Raw
$argsObj = $ArgsJson | ConvertFrom-Json -AsHashtable
if ($Script -eq 'import') {
    if (-not $Write) { throw 'import writes the catalogue: pass -Write to say so.' }
    if (-not $BackupManifest) { throw 'import needs -BackupManifest.' }
    $m = Get-Content -LiteralPath $BackupManifest -Raw | ConvertFrom-Json
    $h = @{}
    foreach ($e in $m.files) { $h[$e.path] = $e.sha256 }
    $argsObj['backup_hashes'] = $h
}
$request = @{ code = $code; args = $argsObj; version = $Version } | ConvertTo-Json -Depth 8 -Compress
$request | & $Exe exec --json-stdin
