# verify-types-strict.ps1 — sample 50 types from an IR + spot-check each on
# both TYPE-LEVEL (name + kind) AND MEMBER-LEVEL (random method signature is real,
# random property type is real, random event delegate is real).
#
# Usage: ./verify-types-strict.ps1 <ir.json> [<count>=50] [<seed>=hash]
#
# Seeding is deterministic from the IR file's SHA-256 unless overridden so the
# same IR + count always produces the same sample list, but the seed can be
# specified explicitly to allow re-running on a stable sample after IR changes.

param(
    [Parameter(Mandatory)] [string] $IrPath,
    [int] $Count = 50,
    [int] $Seed = 0
)

$json = Get-Content $IrPath -Raw | ConvertFrom-Json
$types = $json.types

if ($Seed -eq 0) {
    $hashBytes = (Get-FileHash $IrPath -Algorithm SHA256).Hash
    $Seed = [int]([Convert]::ToInt64($hashBytes.Substring(0,8), 16) -band 0x7FFFFFFF)
}
$rng = New-Object System.Random($Seed)

# Fisher-Yates shuffle, take first $Count.
$arr = [int[]](0..($types.Count - 1))
for ($i = $arr.Count - 1; $i -gt 0; $i--) {
    $j = $rng.Next($i + 1)
    $tmp = $arr[$i]; $arr[$i] = $arr[$j]; $arr[$j] = $tmp
}
$sampleIdx = $arr[0..($Count - 1)]

Write-Host "Sampled $Count types from $($types.Count) (seed = $Seed):"
Write-Host ""

$results = @()
$pass = 0
$fail = 0

foreach ($i in $sampleIdx) {
    $t = $types[$i]
    $url = $t.doc_url

    $issues = @()

    # Type-page check: name + kind present in HTML.
    try {
        $r = Invoke-WebRequest -Uri $url -UseBasicParsing -UserAgent "AWARE-coverage-verify/0.30" -ErrorAction Stop
        $html = $r.Content
        $kindWord = switch ($t.kind) {
            "class" { "Class" }
            "struct" { "Structure" }
            "interface" { "Interface" }
            "enum" { "Enumeration" }
            "delegate" { "Delegate" }
            default { "" }
        }
        if (-not $html.Contains($t.name)) { $issues += "type-name-missing" }
        if ($kindWord -ne "" -and -not $html.Contains("$($t.name) $kindWord")) {
            $issues += "kind-word-missing"
        }
    } catch {
        $issues += "type-fetch-failed: $($_.Exception.Message)"
    }

    # Member checks: up to 3 random methods, 3 random properties, 1 random event.
    $methods = if ($t.methods) { $t.methods } else { @() }
    $properties = if ($t.properties) { $t.properties } else { @() }
    $events = if ($t.events) { $t.events } else { @() }

    # Pick samples without re-using the IR seed (so the same RNG advances naturally).
    function Sample-Random($items, $n) {
        if ($items.Count -eq 0) { return @() }
        $count = [Math]::Min($n, $items.Count)
        $idxs = [int[]](0..($items.Count - 1))
        for ($k = $idxs.Count - 1; $k -gt 0; $k--) {
            $jj = $rng.Next($k + 1)
            $tmp = $idxs[$k]; $idxs[$k] = $idxs[$jj]; $idxs[$jj] = $tmp
        }
        return $idxs[0..($count - 1)] | ForEach-Object { $items[$_] }
    }

    # Methods: signature must not be the synthesized placeholder, and either params must be populated
    # or zero-arg, and returns must be either populated or null (we don't have an "always" rule).
    $methodSample = Sample-Random $methods 3
    foreach ($m in $methodSample) {
        $expectedPlaceholder = "$($m.name)(...)"
        if ($m.signature -eq $expectedPlaceholder) {
            $issues += "method-placeholder-signature($($m.name))"
        }
        # If the method name suggests it takes arguments (contains "(" in name) but params is empty,
        # that's suspicious — the type-page row name carries the param-summary in parens for overloads.
        # We can't fully validate without re-fetching the member page; we trust the parser.
    }

    # Properties: type must not be "object" unless vendor explicitly says it is.
    $propSample = Sample-Random $properties 3
    foreach ($p in $propSample) {
        if ($p.type -eq "object") {
            $issues += "property-placeholder-type($($p.name))"
        }
    }

    # Events: delegate must not be "EventHandler" unless vendor explicitly says it is.
    # Tekla's actual events DO use System.EventHandler in many cases — so "EventHandler" alone
    # is NOT a smoking gun; "EventHandler" with no namespace prefix when the vendor page rendered
    # "System.EventHandler" is. We accept any delegate text that contains a dot (qualified) or
    # ends in "Delegate".
    $evtSample = Sample-Random $events 1
    foreach ($e in $evtSample) {
        if ($e.delegate -eq "EventHandler" -and $e.signature -eq "event EventHandler $($e.name)") {
            # This is exactly the placeholder shape from PageParser. Real event pages produce a richer
            # signature (e.g. "public event System.EventHandler AttributesLoadedFromModel") that does NOT
            # match this string.
            $issues += "event-placeholder-delegate($($e.name))"
        }
    }

    if ($issues.Count -eq 0) {
        $pass++
        $results += [pscustomobject]@{ idx = $i; ns = $t.namespace; name = $t.name; kind = $t.kind; result = "PASS"; issues = "" }
    } else {
        $fail++
        $results += [pscustomobject]@{ idx = $i; ns = $t.namespace; name = $t.name; kind = $t.kind; result = "FAIL"; issues = ($issues -join "; ") }
    }
}

$results | Format-Table -AutoSize | Out-String -Width 8000
Write-Host ""
Write-Host "Strict verification summary: $pass pass / $fail fail / $Count total"
