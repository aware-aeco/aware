# extract-grasshopper-with-restart.ps1 — run the Grasshopper extractor with auto-restart on stall.
#
# Mirrors the Tekla + Rhino babysitter wrappers. Snapshot every 200 enrichments lets a killed
# process resume from disk. The github raw CDN serves Grasshopper docs from the same repo as
# Rhino; throttle behaviour is the same.

param(
    [Parameter(Mandatory)] [string] $Version,
    [int] $StallTimeoutSec = 240,
    [int] $MaxRestarts = 50
)

$Repo = "C:\Users\bimst\source\repos\aware"
$OutIr = Join-Path $Repo "cli-sidecar\Ingest\Output\grasshopper-$Version.ir.json"
$Exe = Join-Path $Repo "cli-sidecar\bin\Release\net10.0\win-x64\publish\aware-sidecar.exe"
$VYear = $Version.Split('.')[0]
$LogErr = Join-Path $Repo "cli-sidecar\Ingest\Output\grasshopper-$VYear-extraction.log"
$LogOut = Join-Path $Repo "cli-sidecar\Ingest\Output\grasshopper-$VYear-stdout.log"
$ReqFile = Join-Path $Repo "cli-sidecar\Ingest\Output\grasshopper-$VYear-req.json"

$req = '{"op":"coverage-extract","args":{"vendor":"grasshopper","version":"' + $Version + '","out_path":"' + $OutIr.Replace('\','\\') + '"}}'
[System.IO.File]::WriteAllText($ReqFile, $req)

$restartCount = 0
while ($restartCount -lt $MaxRestarts) {
    $restartCount++
    Write-Host "==> [$(Get-Date)] starting grasshopper sidecar (attempt $restartCount) for $Version..."

    $cmdLine = "/c `"`"$Exe`" < `"$ReqFile`" >> `"$LogOut`" 2>> `"$LogErr`"`""
    $proc = Start-Process -FilePath "cmd.exe" -ArgumentList $cmdLine -PassThru -WindowStyle Hidden

    $lastSize = if (Test-Path $LogErr) { (Get-Item $LogErr).Length } else { 0 }
    $lastChange = Get-Date

    while (-not $proc.HasExited) {
        Start-Sleep 10
        if (-not (Test-Path $LogErr)) { continue }
        $size = (Get-Item $LogErr).Length
        if ($size -ne $lastSize) {
            $lastSize = $size
            $lastChange = Get-Date
            $tailLine = (Get-Content $LogErr -Tail 5) -join " "
            if ($tailLine.Contains("[grasshopper-extract] done:")) {
                Write-Host "==> [$(Get-Date)] extraction complete."
                $proc.WaitForExit(60000) | Out-Null
                return
            }
        } else {
            $idle = (Get-Date) - $lastChange
            if ($idle.TotalSeconds -gt $StallTimeoutSec) {
                Write-Host "==> [$(Get-Date)] stalled for $($idle.TotalSeconds)s, killing and restarting..."
                try { $proc.Kill() } catch {}
                Get-Process aware-sidecar -ErrorAction SilentlyContinue | Stop-Process -Force -ErrorAction SilentlyContinue
                Start-Sleep 5
                break
            }
        }
    }
    if ($proc.HasExited) {
        Write-Host "==> [$(Get-Date)] sidecar exited with code $($proc.ExitCode)"
        $tail = (Get-Content $LogErr -Tail 10) -join " "
        if ($tail.Contains("[grasshopper-extract] done:")) {
            Write-Host "==> SUCCESS"
            return
        }
        Write-Host "==> sidecar died without completing, restarting (snapshot will resume)"
        Start-Sleep 5
    }
}

Write-Host "==> giving up after $MaxRestarts restarts."
exit 1
