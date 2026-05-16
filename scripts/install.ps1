# AWARE CLI installer for Windows.
#
# Usage:
#   iex (irm https://raw.githubusercontent.com/aware-aeco/aware/main/scripts/install.ps1)
#   $env:AWARE_VERSION = "0.6.0"; iex (irm https://raw.githubusercontent.com/aware-aeco/aware/main/scripts/install.ps1)
#
# Env vars:
#   AWARE_INSTALL_DIR   destination for binaries (default: %LOCALAPPDATA%\aware\bin)
#   AWARE_VERSION       version tag to install (default: latest v0.6.*)

$ErrorActionPreference = 'Stop'

$Repo = "aware-aeco/aware"
$InstallDir = $env:AWARE_INSTALL_DIR
if (-not $InstallDir) {
  $InstallDir = Join-Path $env:LOCALAPPDATA "aware\bin"
}

$Version = $env:AWARE_VERSION
if (-not $Version) {
  Write-Host "Resolving latest v0.6.* release..."
  $releases = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases"
  $latest = $releases | Where-Object { $_.tag_name -match '^v0\.6\.' } | Select-Object -First 1
  if (-not $latest) {
    Write-Error "could not resolve latest version. Set `$env:AWARE_VERSION."
    exit 1
  }
  $Version = $latest.tag_name -replace '^v', ''
}

Write-Host "Installing aware $Version for win-x64..."

$url = "https://github.com/$Repo/releases/download/v$Version/aware-$Version-win-x64.zip"
$tmp = New-Item -ItemType Directory -Path (Join-Path $env:TEMP ("aware-install-" + [Guid]::NewGuid().ToString('N'))) -Force
# Resolve any 8.3 short-name components in TEMP to the canonical long-form path.
# On some Windows configurations $env:TEMP is stored as a short name
# (e.g. C:\Users\bimst\AppData\Local\TEMP_~1). Expand-Archive + subsequent
# path joins then produce paths whose long-form differs from the directory
# actually created, causing Copy-Item to fail with "path not found".
$tmp = (Get-Item $tmp.FullName).FullName
try {
  $archive = Join-Path $tmp "aware.zip"
  Invoke-WebRequest -Uri $url -OutFile $archive

  Expand-Archive -Path $archive -DestinationPath $tmp -Force
  $srcDir = Join-Path $tmp "aware-$Version-win-x64"

  if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
  }
  Copy-Item (Join-Path $srcDir "aware.exe") $InstallDir -Force
  Copy-Item (Join-Path $srcDir "aware-sidecar.exe") $InstallDir -Force

  Write-Host ""
  Write-Host "Installed:" -ForegroundColor Green
  Write-Host "    $InstallDir\aware.exe"
  Write-Host "    $InstallDir\aware-sidecar.exe"
  Write-Host ""

  # Check PATH
  $pathEntries = $env:PATH -split ';'
  if ($pathEntries -notcontains $InstallDir) {
    Write-Host "Warning: $InstallDir is not on your PATH." -ForegroundColor Yellow
    Write-Host "  Add it via System Properties -> Environment Variables, OR run:"
    Write-Host "    [Environment]::SetEnvironmentVariable('Path', `$env:Path + ';$InstallDir', 'User')"
  }
  Write-Host ""
  Write-Host "Try it:"
  Write-Host "  aware --version"
  Write-Host "  aware doctor"
} finally {
  Remove-Item -Recurse -Force $tmp -ErrorAction SilentlyContinue
}
