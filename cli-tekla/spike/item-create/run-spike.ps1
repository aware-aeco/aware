<#
  A5 SPIKE (aware#859). Runs one spike script against the Tekla that is open, through `aware-tekla exec`.
  EXPERIMENTAL. Do not run it until the spike has been granted Tekla time and the fresh scratch model is open
  (see docs/superpowers/specs/2026-11-19-a5-tekla-item-SPIKE.md). It never starts, attaches to or closes Tekla.

  -Script probe        : read-only (E0).
  -Script readback     : read-only (E7).
  -Script shape-create : WRITES THE MODEL'S SHAPE CATALOGUE (E1-E2, E5-E8). Needs -Write and -BackupManifest
                         (from snapshot-shapes.ps1 -Mode Snapshot, taken just before this call).
  -Script item-place   : WRITES ONE ITEM INTO THE OPEN SCRATCH MODEL (E3-E4). Needs -Write.
  scratch_root defaults to C:\A5SPIKE\ and is passed to every script, which refuse a model outside it.
#>
param(
    [Parameter(Mandatory)][ValidateSet('probe', 'readback', 'shape-create', 'item-place')][string]$Script,
    [string]$ArgsJson = '{}',
    [string]$BackupManifest = '',
    [string]$ScratchRoot = 'C:\A5SPIKE\',
    [switch]$Write,
    [string]$Version = '2026.0',
    [string]$Exe = 'aware-tekla',
    # Only for a throwaway model that is not named A5SPIKE* (run 1 used a copy named A2SPIKE-*): the scripts' leaf-name guard
    # prefix is swapped in the text sent to Tekla. The scratch-root guard still applies. The scripts on disk are not changed.
    [string]$ModelPrefix = ''
)
$ErrorActionPreference = 'Stop'
$code = Get-Content -LiteralPath (Join-Path $PSScriptRoot "$Script.csx") -Raw
if ($ModelPrefix) { $code = $code.Replace('StartsWith("A5SPIKE"', 'StartsWith("' + $ModelPrefix + '"') }
$argsObj = $ArgsJson | ConvertFrom-Json -AsHashtable
$argsObj['scratch_root'] = $ScratchRoot
if ($Script -eq 'shape-create' -or $Script -eq 'item-place') {
    if (-not $Write) { throw "$Script writes: pass -Write to say so." }
}
if ($Script -eq 'shape-create') {
    if (-not $BackupManifest) { throw 'shape-create needs -BackupManifest.' }
    $m = Get-Content -LiteralPath $BackupManifest -Raw | ConvertFrom-Json
    $h = @{}
    foreach ($e in @($m.files)) { $h[$e.path] = $e.sha256 }
    $argsObj['backup_hashes'] = $h
    $argsObj['backup_id'] = [string]$m.taken_utc
}
$request = @{ code = $code; args = $argsObj; version = $Version } | ConvertTo-Json -Depth 8 -Compress
$request | & $Exe exec --json-stdin
