<#
  A2 SPIKE (aware#858). Back up, verify and restore the files a Tekla profile-catalogue write can land in.
  EXPERIMENTAL. Run it by hand around the spike; it never starts, attaches to or drives Tekla.

  Snapshot : hashes every catalogue file in -Folders and copies each into -BackupDir, then writes -Manifest (JSON).
  Verify   : hashes the same files now and prints every file that is modified, added or removed since -Manifest.
             Exit 0 only when the tree is byte-identical to the snapshot (SHA-256 of every file).
  Restore  : puts the backed-up bytes back and deletes files that were added since, then runs Verify.
             Run it with Tekla CLOSED: a running Tekla may hold or rewrite profdb.bin.

  Files covered in each folder (top level, plus ParametricProfiles/ExportedProfiles/profil sub-folders of a model folder):
  profdb.bin, profitab.inp, profile_properties.ail, *.clb, *.uel, *.lis.
#>
param(
    [Parameter(Mandatory)][ValidateSet('Snapshot', 'Verify', 'Restore')][string]$Mode,
    [Parameter(Mandatory)][string]$Manifest,
    [string[]]$Folders = @(),
    [string]$BackupDir = ''
)
$ErrorActionPreference = 'Stop'
$patterns = @('profdb.bin', 'profitab.inp', 'profile_properties.ail', '*.clb', '*.uel', '*.lis')
$subfolders = @('', 'ParametricProfiles', 'ExportedProfiles', 'profil')

function Get-CatalogueFiles([string[]]$roots) {
    foreach ($root in $roots) {
        foreach ($sub in $subfolders) {
            $dir = if ($sub) { Join-Path $root $sub } else { $root }
            if (-not (Test-Path -LiteralPath $dir -PathType Container)) { continue }
            foreach ($pattern in $patterns) {
                Get-ChildItem -LiteralPath $dir -Filter $pattern -File -ErrorAction Stop
            }
        }
    }
}

function Get-State([string[]]$roots) {
    $state = [ordered]@{}
    foreach ($f in (Get-CatalogueFiles $roots | Sort-Object FullName -Unique)) {
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
    foreach ($e in $m.files) {
        if (-not $now.Contains($e.path)) { Write-Host "REMOVED   $($e.path)"; $bad++ }
        elseif ($now[$e.path].sha256 -ne $e.sha256) { Write-Host "MODIFIED  $($e.path)  was $($e.sha256)  now $($now[$e.path].sha256)"; $bad++ }
    }
    $known = @{}
    foreach ($e in $m.files) { $known[$e.path] = $true }
    foreach ($p in $now.Keys) { if (-not $known.ContainsKey($p)) { Write-Host "ADDED     $p"; $bad++ } }
    if ($bad -eq 0) { Write-Host "VERIFIED: $(@($m.files).Count) files are byte-identical (SHA-256) to the snapshot taken $($m.taken_utc)." }
    else { Write-Host "NOT IDENTICAL: $bad difference(s)." }
    return $bad
}

if ($Mode -eq 'Verify') { if ((Invoke-Verify) -ne 0) { exit 1 } else { exit 0 } }

if ($Mode -eq 'Restore') {
    $now = Get-State @($m.folders)
    $known = @{}
    foreach ($e in $m.files) { $known[$e.path] = $true }
    # Preflight: every backup this restore will copy back must still match its recorded SHA-256, before anything is deleted or replaced.
    foreach ($e in $m.files) {
        $isSame = $now.Contains($e.path) -and ($now[$e.path].sha256 -eq $e.sha256)
        if (-not $isSame) {
            if (-not (Test-Path -LiteralPath $e.backup)) { Write-Host "BACKUP MISSING for $($e.path): $($e.backup). Nothing was changed."; exit 1 }
            if ((Get-FileHash -LiteralPath $e.backup -Algorithm SHA256).Hash.ToLower() -ne $e.sha256.ToLower()) { Write-Host "BACKUP CORRUPT for $($e.path): $($e.backup). Nothing was changed."; exit 1 }
        }
    }
    foreach ($p in @($now.Keys)) { if (-not $known.ContainsKey($p)) { Remove-Item -LiteralPath $p -Force; Write-Host "Removed added file $p" } }
    foreach ($e in $m.files) {
        $isSame = $now.Contains($e.path) -and ($now[$e.path].sha256 -eq $e.sha256)
        if (-not $isSame) { Copy-Item -LiteralPath $e.backup -Destination $e.path -Force; Write-Host "Restored $($e.path)" }
    }
    if ((Invoke-Verify) -ne 0) { exit 1 } else { exit 0 }
}
