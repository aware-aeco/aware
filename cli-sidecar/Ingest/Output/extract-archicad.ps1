# extract-archicad.ps1 — drive the ArchiCAD extractor.
#
# Unlike the Tekla / Rhino / Grasshopper extractors, ArchiCAD does NOT need the
# auto-restart wrapper: the entire extraction is < 60s wall-clock with no rate
# limiting (Tapir is hosted on raw.github.com which is generous for 3 fetches;
# Graphisoft serves ~73 small JSON files via nginx with no observed throttling).
#
# Usage: ./extract-archicad.ps1 -Version 28.0    # or 29.0

param(
    [Parameter(Mandatory)] [string] $Version
)

$Repo = "C:\Users\bimst\source\repos\aware"
$OutIr = Join-Path $Repo "cli-sidecar\Ingest\Output\archicad-$Version.ir.json"
$Exe = Join-Path $Repo "cli-sidecar\bin\Release\net10.0\win-x64\publish\aware-sidecar.exe"
$VMajor = $Version.Split('.')[0]
$LogErr = Join-Path $Repo "cli-sidecar\Ingest\Output\archicad-$VMajor-extraction.log"
$LogOut = Join-Path $Repo "cli-sidecar\Ingest\Output\archicad-$VMajor-stdout.log"
$ReqFile = Join-Path $Repo "cli-sidecar\Ingest\Output\archicad-$VMajor-req.json"

$req = '{"op":"coverage-extract","args":{"vendor":"archicad","version":"' + $Version + '","out_path":"' + $OutIr.Replace('\','\\') + '"}}'
[System.IO.File]::WriteAllText($ReqFile, $req)

if (Test-Path $LogErr) { Remove-Item $LogErr -Force }
if (Test-Path $LogOut) { Remove-Item $LogOut -Force }

Write-Host "==> [$(Get-Date)] starting archicad extractor (v$Version)..."
$cmdLine = "/c `"`"$Exe`" < `"$ReqFile`" >> `"$LogOut`" 2>> `"$LogErr`"`""
$proc = Start-Process -FilePath "cmd.exe" -ArgumentList $cmdLine -PassThru -WindowStyle Hidden -Wait

if ($proc.ExitCode -eq 0) {
    Write-Host "==> [$(Get-Date)] extraction complete (exit 0)."
    if (Test-Path $LogErr) { Get-Content $LogErr -Tail 5 }
} else {
    Write-Error "Extractor exited $($proc.ExitCode). stderr tail:"
    if (Test-Path $LogErr) { Get-Content $LogErr -Tail 20 | Out-Host }
    exit 1
}
