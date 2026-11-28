param(
    [Parameter(Mandatory)][string]$Stages,
    [Parameter(Mandatory)][string]$RuntimeHome,
    [Parameter(Mandatory)][string]$PythonExecutable
)
$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$node = (Get-Command node).Source
$wheelhouse = New-Item -ItemType Directory -Force (Join-Path $Stages wheels)
$wheelLock = Join-Path $PSScriptRoot 'win-amd64-cp312.requirements.lock'

function Invoke-Checked([scriptblock]$Action) {
    & $Action
    if ($LASTEXITCODE -ne 0) { throw "Command failed ($LASTEXITCODE): $Action" }
}

function Build-Test([string]$Target, [string]$Name, [string[]]$Features) {
    $lines = & cargo test --locked --no-run --message-format=json $Target @Features
    if ($LASTEXITCODE -ne 0) { throw "Could not build $Name tests." }
    $artifacts = @($lines | ForEach-Object { $_ | ConvertFrom-Json } |
        Where-Object { $_.reason -eq 'compiler-artifact' -and $_.target.name -eq $Name -and $_.profile.test -and $_.executable })
    if ($artifacts.Count -ne 1) { throw "Expected one $Name test binary." }
    $artifacts[0].executable
}

Invoke-Checked { & $PythonExecutable -m pip download --only-binary=:all: --no-deps --require-hashes -r $wheelLock -d $wheelhouse.FullName }
Invoke-Checked { npm --prefix (Join-Path $root 'cli-connection-reader') ci }

Push-Location (Join-Path $root cli)
try {
    $release = New-Item -ItemType Directory -Force (Join-Path $Stages release)
    $releaseTests = Build-Test '--test=geometry_exec' 'geometry_exec' @()
    Copy-Item -LiteralPath 'target/debug/aware.exe' -Destination $release.FullName
    @{ wire = $releaseTests } | ConvertTo-Json | Set-Content (Join-Path $release.FullName 'rust-tests.json')

    $features = @('--features=geometry-dev')
    $wire = Build-Test '--test=geometry_exec' 'geometry_exec' $features
    $supervision = Build-Test '--bin=aware' 'aware' $features
    $normal = Join-Path $Stages normal
    $stage = Join-Path $PSScriptRoot 'stage-development.ps1'
    & $stage -AwareExecutable 'target/debug/aware.exe' -NodeExecutable $node -StageDirectory $normal
    @{ wire = $wire; supervision = $supervision } | ConvertTo-Json | Set-Content (Join-Path $normal 'rust-tests.json')

    Invoke-Checked { cargo build --locked --features geometry-test-faults }
    & $stage -AwareExecutable 'target/debug/aware.exe' -NodeExecutable $node -StageDirectory (Join-Path $Stages fault)
} finally {
    Pop-Location
}

$env:AWARE_HOME = $RuntimeHome
Invoke-Checked { & (Join-Path $Stages normal/aware.exe) geometry install --occt --json --python $PythonExecutable --wheelhouse $wheelhouse.FullName }
