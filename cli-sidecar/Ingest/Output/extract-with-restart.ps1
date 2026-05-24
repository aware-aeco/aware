# extract-with-restart.ps1 - run the Tekla extractor with auto-restart on CDN throttle.
#
# The Tekla CDN throttles requests aggressively after the first ~2-3000 fetches per
# IP/session. The extractor snapshots its progress every 200 enrichments, so killing
# and restarting picks up where the previous run left off (skipping already-enriched
# members). This wrapper babysits the process and restarts it whenever it stalls for
# more than StallTimeoutSec seconds without writing a new progress line.
#
# Stops once the extractor logs '[tekla-extract] done:' successfully.
#
# Tuning notes for B2-B14 implementers:
#   - StallTimeoutSec = 240 (4 minutes) is calibrated against the Tekla CDN's typical
#     pause-after-throttle window. Measured against tekla-2025 run (4 restarts needed
#     to complete 12k member fetches): each throttle event paused 60-180s, comfortably
#     under the 240s budget. Going lower (e.g. 120s) caused false-positive restarts
#     during a real throttle pause — wasted work. Going higher (e.g. 600s) burned
#     wall-clock when the CDN had genuinely killed the connection.
#   - Other vendors WILL need tuning per-CDN. Symptoms to measure on first run:
#       - Slow-DOM-rendering vendor pages (rare, but possible for JS-heavy sites
#         under HttpScraper's pre-rendered HTML capture): may need 360-600s.
#       - Low-throughput / always-on stalls (vendor genuinely returning slow per-page):
#         may need 120-180s.
#       - High-burst vendors that throttle aggressively at the second-of-day boundary:
#         keep 240s and tune MaxRestarts up to 100+ instead.
#   - Document the chosen value + reasoning in the vendor's EXTRACTION-NOTES.md
#     under the "Concurrency tuning" section.

param(
    [Parameter(Mandatory)] [string] $Version,
    [int] $StallTimeoutSec = 240,
    [int] $MaxRestarts = 50
)

$Repo = "C:\Users\bimst\source\repos\aware"
$OutIr = Join-Path $Repo "cli-sidecar\Ingest\Output\tekla-$Version.ir.json"
$Exe = Join-Path $Repo "cli-sidecar\bin\Release\net10.0\win-x64\publish\aware-sidecar.exe"
$VYear = $Version.Split('.')[0]
$LogErr = Join-Path $Repo "cli-sidecar\Ingest\Output\tekla-$VYear-extraction.log"
$LogOut = Join-Path $Repo "cli-sidecar\Ingest\Output\tekla-$VYear-stdout.log"
$ReqFile = Join-Path $Repo "cli-sidecar\Ingest\Output\tekla-$VYear-req.json"

$req = '{"op":"coverage-extract","args":{"vendor":"tekla","version":"' + $Version + '","out_path":"' + $OutIr.Replace('\','\\') + '"}}'
[System.IO.File]::WriteAllText($ReqFile, $req)

$restartCount = 0
while ($restartCount -lt $MaxRestarts) {
    $restartCount++
    Write-Host "==> [$(Get-Date)] starting sidecar (attempt $restartCount)..."

    # Build cmd line with explicit redirection. We invoke cmd.exe with /c and pass the
    # whole pipeline as one argument; cmd handles the < > >> 2>> redirections itself.
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
            if ($tailLine.Contains("[tekla-extract] done:")) {
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
        if ($tail.Contains("[tekla-extract] done:")) {
            Write-Host "==> SUCCESS"
            return
        }
        Write-Host "==> sidecar died without completing, restarting (snapshot will resume)"
        Start-Sleep 5
    }
}

Write-Host "==> giving up after $MaxRestarts restarts."
exit 1
