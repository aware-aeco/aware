param(
    [Parameter(Mandatory)][string]$AwareExecutable,
    [Parameter(Mandatory)][string]$NodeExecutable,
    [Parameter(Mandatory)][string]$StageDirectory
)

$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$reader = Join-Path $root 'cli-connection-reader'
$node = (Resolve-Path -LiteralPath $NodeExecutable).Path
$version = & $node --version
if ($LASTEXITCODE -ne 0 -or $version -ne 'v24.14.0') {
    throw 'The development stage requires Node 24.14.0.'
}
$package = Join-Path $reader 'node_modules/manifold-3d'
if ((Get-Content -Raw -LiteralPath (Join-Path $package 'package.json') | ConvertFrom-Json).version -ne '3.5.4') {
    throw 'The development stage requires the lockfile-resolved manifold-3d 3.5.4.'
}
$stage = New-Item -ItemType Directory -Path $StageDirectory -Force
$helper = Join-Path $stage.FullName 'geometry-mesh'
$modules = New-Item -ItemType Directory -Path (Join-Path $helper 'node_modules') -Force
Copy-Item -LiteralPath $AwareExecutable -Destination (Join-Path $stage.FullName 'aware.exe')
Copy-Item -LiteralPath $node -Destination (Join-Path $helper 'node.exe')
foreach ($name in 'geometry-mesh-dispatcher.mjs', 'geometry-slabs.mjs', 'mesh-solid.mjs', 'snapshot-clash.mjs', 'snapshot-clash-geometry.mjs', 'snapshot-compare.mjs', 'manifold-wasm.mjs') {
    Copy-Item -LiteralPath (Join-Path $reader $name) -Destination $helper
}
Copy-Item -LiteralPath $package -Destination $modules.FullName -Recurse -Force
