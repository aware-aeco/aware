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
$dllPath  = Join-Path $buildOut "AwareRevit.dll"
$srcManifest = Join-Path $buildOut "AwareRevit.addin"

if (-not (Test-Path $dllPath)) {
    Write-Error "AwareRevit.dll not found at $dllPath. Build first: dotnet build cli-revit\AwareRevit\AwareRevit.csproj -c $Configuration"
}
if (-not (Test-Path $srcManifest)) {
    Write-Error "AwareRevit.addin not found at $srcManifest. Build first."
}

$addinDir = Join-Path $env:APPDATA "Autodesk\Revit\Addins\$RevitVersion"
if (-not (Test-Path $addinDir)) {
    New-Item -ItemType Directory -Path $addinDir -Force | Out-Null
}

# Deploy DLL + its dependencies (Roslyn etc.) next to the manifest. We put
# them all in a subfolder so they don't pollute the addin root.
$deployDir = Join-Path $addinDir "AwareRevit"
if (Test-Path $deployDir) {
    Get-ChildItem $deployDir -File -Recurse | Remove-Item -Force
} else {
    New-Item -ItemType Directory -Path $deployDir -Force | Out-Null
}
Get-ChildItem $buildOut -File -Recurse | ForEach-Object {
    $rel = $_.FullName.Substring($buildOut.Length).TrimStart('\','/')
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
