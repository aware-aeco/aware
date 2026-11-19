<#
  A5 SPIKE (aware#859). Back up, verify and restore the files a Tekla shape-catalogue write can land in, and prove that
  folders that must stay untouched did. Adapted from the A2 spike's snapshot-catalog.ps1 (aware#858).
  EXPERIMENTAL. Run it by hand around the spike; it never starts, attaches to or drives Tekla.

  Snapshot : hashes every covered file under -Folders and copies each into -BackupDir, then writes -Manifest (JSON).
             An empty set is valid (a fresh model has no shapes yet): the manifest then lists no files, and still carries
             the time it was taken, which run-spike.ps1 hands to shape-create.csx as proof that a snapshot exists.
  Verify   : hashes the same files now and prints every file that is modified, added or removed since -Manifest.
             Exit 0 only when the tree is byte-identical to the snapshot (SHA-256 of every file).
  Restore  : puts the backed-up bytes back and deletes files added since, then runs Verify.
             Run it with Tekla CLOSED. (For a fresh scratch model the simplest restore is deleting the model folder.)

  Covered in each folder: top level profdb.bin, profitab.inp, profile_properties.ail, *.clb, *.uel, *.lis (the profile
  catalogue, which an A5 run must NOT change) and, in the Shapes and ShapeGeometries sub-folders, every file (the model's
  own shape catalogue, which an A5 run changes).
#>
param(
    [Parameter(Mandatory)][ValidateSet('Snapshot', 'Verify', 'Restore')][string]$Mode,
    [Parameter(Mandatory)][string]$Manifest,
    [string[]]$Folders = @(),
    [string]$BackupDir = ''
)
$ErrorActionPreference = 'Stop'
$topPatterns = @('profdb.bin', 'profitab.inp', 'profile_properties.ail', '*.clb', '*.uel', '*.lis')
$shapeFolders = @('Shapes', 'ShapeGeometries')

function Get-CoveredFiles([string[]]$roots) {
    foreach ($root in $roots) {
        if (-not (Test-Path -LiteralPath $root -PathType Container)) { continue }
        foreach ($pattern in $topPatterns) {
            Get-ChildItem -LiteralPath $root -Filter $pattern -File -ErrorAction Stop
        }
        foreach ($sub in $shapeFolders) {
            $dir = Join-Path $root $sub
            if (Test-Path -LiteralPath $dir -PathType Container) { Get-ChildItem -LiteralPath $dir -File -Recurse -ErrorAction Stop }
        }
    }
}

function Get-State([string[]]$roots) {
    $state = [ordered]@{}
    foreach ($f in (Get-CoveredFiles $roots | Sort-Object FullName -Unique)) {
        $state[$f.FullName] = [pscustomobject]@{ path = $f.FullName; bytes = $f.Length; sha256 = (Get-FileHash -LiteralPath $f.FullName -Algorithm SHA256).Hash.ToLowerInvariant() }
    }
    $state
}

if ($Mode -eq 'Snapshot') {
    if (-not $Folders.Count) { throw 'Snapshot needs -Folders.' }
    if (-not $BackupDir) { throw 'Snapshot needs -BackupDir.' }
    if ((Test-Path -LiteralPath $BackupDir) -and (Get-ChildItem -LiteralPath $BackupDir -Force | Select-Object -First 1)) { throw "BackupDir $BackupDir is not empty; use a new folder so an old backup is never mixed in." }
    New-Item -ItemType Directory -Force -Path $BackupDir | Out-Null
    $state = Get-State $Folders
    $entries = @()
    $n = 0
    foreach ($e in $state.Values) {
        $n++
        $copy = Join-Path $BackupDir ('{0:D4}.bak' -f $n)
        Copy-Item -LiteralPath $e.path -Destination $copy
        $copyHash = (Get-FileHash -LiteralPath $copy -Algorithm SHA256).Hash.ToLowerInvariant()
        if ($copyHash -ne $e.sha256) { throw "Backup of $($e.path) does not match the original." }
        $entries += [pscustomobject]@{ path = $e.path; bytes = $e.bytes; sha256 = $e.sha256; backup = $copy }
    }
    [pscustomobject]@{ taken_utc = (Get-Date).ToUniversalTime().ToString('o'); folders = $Folders; files = $entries } | ConvertTo-Json -Depth 5 | Set-Content -LiteralPath $Manifest -Encoding UTF8
    Write-Host "Snapshot: $($entries.Count) files hashed and copied to $BackupDir; manifest $Manifest"
    return
}

$m = Get-Content -LiteralPath $Manifest -Raw | ConvertFrom-Json

function Invoke-Verify {
    $now = Get-State @($m.folders)
    $bad = 0
    foreach ($e in @($m.files)) {
        if (-not $now.Contains($e.path)) { Write-Host "REMOVED   $($e.path)"; $bad++ }
        elseif ($now[$e.path].sha256 -ne $e.sha256) { Write-Host "MODIFIED  $($e.path)  was $($e.sha256)  now $($now[$e.path].sha256)"; $bad++ }
    }
    $known = @{}
    foreach ($e in @($m.files)) { $known[$e.path] = $true }
    foreach ($p in $now.Keys) { if (-not $known.ContainsKey($p)) { Write-Host "ADDED     $p"; $bad++ } }
    if ($bad -eq 0) { Write-Host "VERIFIED: $(@($m.files).Count) files are byte-identical (SHA-256) to the snapshot taken $($m.taken_utc)." }
    else { Write-Host "NOT IDENTICAL: $bad difference(s)." }
    return $bad
}

if ($Mode -eq 'Verify') { if ((Invoke-Verify) -ne 0) { exit 1 } else { exit 0 } }

if ($Mode -eq 'Restore') {
    $now = Get-State @($m.folders)
    $known = @{}
    foreach ($e in @($m.files)) { $known[$e.path] = $true }
    foreach ($p in @($now.Keys)) { if (-not $known.ContainsKey($p)) { Remove-Item -LiteralPath $p -Force; Write-Host "Removed added file $p" } }
    foreach ($e in @($m.files)) {
        $isSame = $now.Contains($e.path) -and ($now[$e.path].sha256 -eq $e.sha256)
        if (-not $isSame) { Copy-Item -LiteralPath $e.backup -Destination $e.path -Force; Write-Host "Restored $($e.path)" }
    }
    if ((Invoke-Verify) -ne 0) { exit 1 } else { exit 0 }
}
