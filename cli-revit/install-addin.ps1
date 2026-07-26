# Installs AwareRevit.dll + AwareRevit.addin into the per-user Revit add-in
# folder. Rewrites the manifest's <Assembly> entry to the absolute deployed
# path so Revit can find the DLL regardless of cwd.
#
# Runs unchanged in both places the add-in ever lives:
#   * the installed bridge - `aware sidecar install revit` unpacks the release
#     zip into ~/.aware/bridges/, where the add-in payload sits in an
#     AwareRevit\ subfolder next to this script; nothing else is needed
#   * a repo clone - the payload is the project's own build output
#
# Usage:
#   pwsh "$HOME\.aware\bridges\install-addin.ps1"           # installed bridge
#   pwsh .\cli-revit\install-addin.ps1                      # repo, debug build
#   pwsh .\cli-revit\install-addin.ps1 -Configuration Release
#   pwsh .\cli-revit\install-addin.ps1 -RevitVersion 2025   # default 2026

param(
    [string]$Configuration = "Debug",
    [string]$RevitVersion  = "2026"
)

$ErrorActionPreference = "Stop"

$repoRoot = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
$buildOut = Join-Path $repoRoot "cli-revit\AwareRevit\bin\$Configuration\net8.0-windows"

# Where the add-in payload lives, in order of preference. Whichever hit wins,
# its directory holds the add-in AND everything it loads at run time (Roslyn
# scripting, the shared frame types, the localized resource folders), because
# the whole directory is what gets deployed below.
#   1. The AwareRevit\ subfolder of the release zip - the installed-bridge case.
#      It is a subfolder and not a loose DLL on purpose: the zip extracts flat
#      into the shared ~/.aware/bridges/, so a flat payload would both collide
#      with the other bridges' assemblies and get swept into Revit wholesale.
#   2. The repo build output - developers running from a clone.
$payloadDirs = @(
    (Join-Path $PSScriptRoot "AwareRevit"),
    $buildOut
)
$sourceDir = $payloadDirs | Where-Object { Test-Path (Join-Path $_ "AwareRevit.dll") } | Select-Object -First 1

if (-not $sourceDir) {
    # Older releases shipped no DLL at all and told people to drop a self-built
    # one right beside this script. Deploying from here would copy the entire
    # bridges directory into Revit, so name the one-line fix instead.
    if (Test-Path (Join-Path $PSScriptRoot "AwareRevit.dll")) {
        Write-Error (@"
Found AwareRevit.dll beside this script, but the add-in must live in its own
folder - deploying from here would sweep every other bridge into Revit too.
Move it, with its dependencies, into:
       $(Join-Path $PSScriptRoot 'AwareRevit')
Or simply re-install the bridge, which now ships the add-in for you:
       aware sidecar install revit
"@)
    }
    Write-Error (@"
AwareRevit.dll not found. Looked in:
$($payloadDirs | ForEach-Object { "  $_" } | Out-String)
  A) Installed bridge: re-run ``aware sidecar install revit`` - the release zip
     ships the add-in, so this script needs nothing else.
  B) Repo clone: build it first (no local Revit install required - the Revit API
     comes from Autodesk's reference assemblies on nuget.org when Revit is absent):
       dotnet build cli-revit\AwareRevit\AwareRevit.csproj -c $Configuration
"@)
}

# The manifest is built into the payload directory alongside the DLL; the loose
# copy beside the script is a fallback for zips that predate that.
$manifestCandidates = @(
    (Join-Path $sourceDir    "AwareRevit.addin"),
    (Join-Path $PSScriptRoot "AwareRevit.addin")
)
$srcManifest = $manifestCandidates | Where-Object { Test-Path $_ } | Select-Object -First 1

if (-not $srcManifest) {
    Write-Error "AwareRevit.addin not found next to $sourceDir\AwareRevit.dll. Build first: dotnet build cli-revit\AwareRevit\AwareRevit.csproj -c $Configuration"
}

$addinDir = Join-Path $env:APPDATA "Autodesk\Revit\Addins\$RevitVersion"
if (-not (Test-Path $addinDir)) {
    New-Item -ItemType Directory -Path $addinDir -Force | Out-Null
}

# Deploy DLL + its dependencies (Roslyn etc.) next to the manifest. We put
# them all in a subfolder so they don't pollute the addin root. $sourceDir is
# the payload directory resolved above, and holds nothing but the add-in.
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
