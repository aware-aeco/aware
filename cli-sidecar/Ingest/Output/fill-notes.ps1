# fill-notes.ps1 — fill in the IR / catalog counts section of an EXTRACTION-NOTES.md.
# Usage: ./fill-notes.ps1 <vendor> <version>  → updates the notes inline.

param(
    [Parameter(Mandatory)] [ValidateSet("rhino", "grasshopper")] [string] $Vendor,
    [Parameter(Mandatory)] [string] $Version
)

$RepoRoot = "C:\Users\bimst\source\repos\aware"
$VMajor = $Version.Split('.')[0]
$Ir = Join-Path $RepoRoot "cli-sidecar\Ingest\Output\$Vendor-$Version.ir.json"
$Notes = Join-Path $RepoRoot "20-agents\aeco\architecture\$Vendor-$VMajor\EXTRACTION-NOTES.md"

if (-not (Test-Path $Ir)) { Write-Error "IR not found: $Ir"; exit 1 }
if (-not (Test-Path $Notes)) { Write-Error "Notes not found: $Notes"; exit 1 }

$json = Get-Content $Ir -Raw | ConvertFrom-Json
$types = $json.types
$pageCount = $json.metadata.page_count
$typeCount = $json.metadata.type_count

# Enrichment stats
$em = 0; $tm = 0; $ep = 0; $tp = 0; $ee = 0; $te = 0; $ec = 0; $tc = 0
foreach ($t in $types) {
    foreach ($m in $t.methods) {
        $tm++
        if ($m.signature -ne "$($m.name)(...)" -and $m.signature -ne "$($t.name)(...)") { $em++ }
    }
    foreach ($c in $t.constructors) {
        $tc++
        if ($c.signature -ne "$($c.name)(...)" -and $c.signature -ne "$($t.name)(...)") { $ec++ }
    }
    foreach ($p in $t.properties) {
        $tp++
        if ($p.type -ne "object") { $ep++ }
    }
    foreach ($e in $t.events) {
        $te++
        if ($e.delegate -ne "EventHandler") { $ee++ }
    }
}

$ic = [System.Globalization.CultureInfo]::InvariantCulture
$mPct = if ($tm) { ([Math]::Round(100.0 * $em / $tm, 1)).ToString($ic) + '%' } else { "n/a" }
$cPct = if ($tc) { ([Math]::Round(100.0 * $ec / $tc, 1)).ToString($ic) + '%' } else { "n/a" }
$pPct = if ($tp) { ([Math]::Round(100.0 * $ep / $tp, 1)).ToString($ic) + '%' } else { "n/a" }
$ePct = if ($te) { ([Math]::Round(100.0 * $ee / $te, 1)).ToString($ic) + '%' } else { "n/a" }

$total = $tm + $tc + $tp + $te
$enriched = $em + $ec + $ep + $ee
$totalPct = if ($total) { ([Math]::Round(100.0 * $enriched / $total, 1)).ToString($ic) + '%' } else { "n/a" }

$extractedAt = $json.source.extracted_at

$counts = @"
## IR / catalog counts

- **Vendor-published namespaces (from sidebar or root):** _see Pass 1 log_
- **Types in IR:** $typeCount
- **Types in generated catalog:** $typeCount (1:1)
- **Page count (HTTP fetches):** $pageCount

## Member-page enrichment results

| field | filled / total | % |
|-------|---------------:|--:|
| Methods with real signature | $em / $tm | $mPct |
| Constructors with real signature | $ec / $tc | $cPct |
| Properties with non-``object`` type | $ep / $tp | $pPct |
| Events with non-``EventHandler`` delegate | $ee / $te | $ePct |
| **Total members enriched** | **$enriched / $total** | **$totalPct** |

Extracted at: $extractedAt
"@

# Replace the placeholder section. Look for "## IR / catalog counts" and replace through to
# the next "## " heading.
$content = Get-Content $Notes -Raw
$pattern = '(?s)## IR / catalog counts\n.*?(?=\n## )'
if ($content -match $pattern) {
    $content = [regex]::Replace($content, $pattern, $counts + "`n")
} else {
    # Append at end
    $content = $content.TrimEnd() + "`n`n" + $counts + "`n"
}
$content | Set-Content $Notes -NoNewline -Encoding UTF8

Write-Host "Updated ${Notes}:"
Write-Host "  types=$typeCount, page_count=$pageCount"
Write-Host "  enrichment: methods $mPct, ctors $cPct, props $pPct, events $ePct"
Write-Host "  total: $enriched/$total ($totalPct)"
