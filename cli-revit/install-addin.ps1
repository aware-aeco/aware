# Installs AwareRevit.dll + AwareRevit.addin into the per-user Revit add-in
# folder. Rewrites the manifest's <Assembly> entry to the absolute deployed
# path so Revit can find the DLL regardless of cwd.
#
# Usage:
#   pwsh .\cli-revit\install-addin.ps1                      # debug build
#   pwsh .\cli-revit\install-addin.ps1 -Configuration Release
#   pwsh .\cli-revit\install-addin.ps1 -RevitVersion 2025   # default 2026

param(
    [string]$Configuration = "Debug",
    [string]$RevitVersion  = "2026"
)

$ErrorActionPreference = "Stop"

$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
$buildOut = Join-Path $repoRoot "cli-revit\AwareRevit\bin\$Configuration\net8.0-windows"

# DLL search order:
#   1. Alongside this script — covers the `aware sidecar install revit` zip case
#      where the user has manually placed a pre-built AwareRevit.dll next to the
#      script after building from source (AwareRevit.dll cannot be built without
#      RevitAPI.dll, which ships with a local Revit install and is not in the
#      GitHub release zip).
#   2. Standard repo build output — covers developers running from a clone.
$dllCandidates = @(
    (Join-Path $PSScriptRoot "AwareRevit.dll"),
    (Join-Path $buildOut     "AwareRevit.dll")
)
$dllPath = $dllCandidates | Where-Object { Test-Path $_ } | Select-Object -First 1

$manifestCandidates = @(
    (Join-Path $PSScriptRoot "AwareRevit.addin"),
    (Join-Path $buildOut     "AwareRevit.addin")
)
$srcManifest = $manifestCandidates | Where-Object { Test-Path $_ } | Select-Object -First 1

if (-not $dllPath) {
    Write-Error (@"
AwareRevit.dll not found. Two options:
  A) Build from the repo (requires RevitAPI.dll from a local Revit $RevitVersion install):
       dotnet build cli-revit\AwareRevit\AwareRevit.csproj -c $Configuration
     Then copy AwareRevit.dll next to this script and re-run.
  B) Place a pre-built AwareRevit.dll next to this script:
       $(Join-Path $PSScriptRoot 'AwareRevit.dll')
"@)
}
if (-not $srcManifest) {
    Write-Error "AwareRevit.addin not found. Build first: dotnet build cli-revit\AwareRevit\AwareRevit.csproj -c $Configuration"
}

$addinDir = Join-Path $env:APPDATA "Autodesk\Revit\Addins\$RevitVersion"
if (-not (Test-Path $addinDir)) {
    New-Item -ItemType Directory -Path $addinDir -Force | Out-Null
}

# Deploy DLL + its dependencies (Roslyn etc.) next to the manifest. We put
# them all in a subfolder so they don't pollute the addin root.
# Source dir = wherever we found the DLL (repo build output or zip dir).
$sourceDir = Split-Path $dllPath -Parent
$deployDir = Join-Path $addinDir "AwareRevit"
if (Test-Path $deployDir) {
    Get-ChildItem $deployDir -File -Recurse | Remove-Item -Force
} else {
    New-Item -ItemType Directory -Path $deployDir -Force | Out-Null
}
Get-ChildItem $sourceDir -File -Recurse | ForEach-Object {
    $rel = $_.FullName.Substring($sourceDir.Length).TrimStart('\','/')
    $dest = Join-Path $deployDir $rel
    $destDir = Split-Path $dest -Parent
    if (-not (Test-Path $destDir)) { New-Item -ItemType Directory -Path $destDir -Force | Out-Null }
    Copy-Item $_.FullName $dest -Force
}

# Rewrite manifest's <Assembly> to absolute deployed path.
$deployedDll = Join-Path $deployDir "AwareRevit.dll"
$destManifest = Join-Path $addinDir "AwareRevit.addin"
$xml = [xml](Get-Content $srcManifest -Raw)
# Use SelectSingleNode + InnerText: setting .Assembly = string fails on element
# nodes (PowerShell XML accessor returns an XmlElement, not a string property).
$assemblyNode = $xml.SelectSingleNode("/RevitAddIns/AddIn/Assembly")
if ($assemblyNode -eq $null) { Write-Error "Could not find /RevitAddIns/AddIn/Assembly node in $srcManifest" }
$assemblyNode.InnerText = $deployedDll
$xml.Save($destManifest)

Write-Host "Deployed:"
Write-Host "  Manifest: $destManifest"
Write-Host "  Assembly: $deployedDll"
Write-Host ""
Write-Host "Restart Revit $RevitVersion. The add-in should appear under Add-Ins > External Tools."
Write-Host "If Revit prompts about trust, allow 'Always Load'."
