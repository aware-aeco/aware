# verify-types-strict-archicad.ps1 — strict 50-type verify adapted for ArchiCAD.
#
# The default verify-types-strict.ps1 expects vendor doc URLs to be SERVER-RENDERED
# HTML containing the type name + kind word as literal substrings. ArchiCAD's docs
# are JS-rendered single-page apps:
#   - Tapir (https://enzyme-apd.github.io/tapir-archicad-automation/archicad-addon/)
#     loads command_definitions.js and renders the catalog client-side.
#   - Graphisoft (https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/)
#     loads content/menutree.json + per-command JSON files client-side.
#
# Fetching the bare HTML of these sites returns the JS bootstrap shell only — the
# type names are NOT in the initial HTML. So the standard strict verify always
# reports "type-name-missing" / "kind-word-missing" — but the failure is in the
# verifier's assumption, not the IR.
#
# This script verifies against the JSON-SOURCE-OF-TRUTH instead:
#   - For Tapir.AdditionalCommands and Tapir.Schema, it re-fetches
#     command_definitions.js + common_schema_definitions.js and checks that the
#     IR's `name` is present in the parsed JSON (matching by category name for
#     command-group types, or by top-level key for shared schema types).
#   - For Archicad.OfficialCommands, it re-fetches content/menutree.json and
#     checks that the IR's command-group name is present.
#
# Member-level checks are identical to the default verify: random
# methods/properties/events from the catalog must have non-placeholder shapes.
#
# Usage: ./verify-types-strict-archicad.ps1 <ir.json> [<count>=50] [<seed>=0]

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

# Fetch sources once
$tapirCommandsRaw = (Invoke-WebRequest "https://raw.githubusercontent.com/ENZYME-APD/tapir-archicad-automation/main/docs/archicad-addon/command_definitions.js" -UseBasicParsing -UserAgent "AWARE-coverage-verify/0.30").Content
$tapirSchemaRaw = (Invoke-WebRequest "https://raw.githubusercontent.com/ENZYME-APD/tapir-archicad-automation/main/docs/archicad-addon/common_schema_definitions.js" -UseBasicParsing -UserAgent "AWARE-coverage-verify/0.30").Content
$graphisoftMenu = (Invoke-WebRequest "https://archicadapi.graphisoft.com/JSONInterfaceDocumentation/content/menutree.json" -UseBasicParsing -UserAgent "AWARE-coverage-verify/0.30").Content

# Strip JS wrappers and parse
$tapirCommandsJson = ($tapirCommandsRaw -replace '^var gCommands = ', '' -replace ';\s*$', '') | ConvertFrom-Json
$tapirSchemaJson   = ($tapirSchemaRaw   -replace '^var gSchemaDefinitions = ', '' -replace ';\s*$', '') | ConvertFrom-Json
$graphisoftJson    = $graphisoftMenu | ConvertFrom-Json

# Pre-flatten Tapir command category names + Graphisoft group names
$tapirCategoryNames = @{}
foreach ($cat in $tapirCommandsJson) {
    # sanitise (strip whitespace) to match IR `name` field convention
    $key = ($cat.name -replace '\s+', '')
    $tapirCategoryNames[$key] = $true
}

$graphisoftGroupNames = @{}
function Walk-Menu($items) {
    foreach ($it in $items) {
        if ($it.commanddocumentation) { continue }
        if ($it.menuitems) {
            $key = ($it.name -replace '\s+', '')
            $graphisoftGroupNames[$key] = $true
            Walk-Menu $it.menuitems
        }
    }
}
Walk-Menu $graphisoftJson.menuitems

# Tapir schema top-level type names
$tapirSchemaNames = @{}
$tapirSchemaJson.PSObject.Properties | ForEach-Object { $tapirSchemaNames[$_.Name] = $true }

$results = @()
$pass = 0
$fail = 0

foreach ($i in $sampleIdx) {
    $t = $types[$i]
    $issues = @()
    $ns = $t.namespace
    $name = $t.name

    # Type-existence check against the source
    if ($ns -eq "Tapir.AdditionalCommands") {
        if (-not $tapirCategoryNames.ContainsKey($name)) {
            $issues += "tapir-category-missing"
        }
    } elseif ($ns -eq "Tapir.Schema") {
        if (-not $tapirSchemaNames.ContainsKey($name)) {
            $issues += "tapir-schema-missing"
        }
    } elseif ($ns -eq "Archicad.OfficialCommands") {
        if (-not $graphisoftGroupNames.ContainsKey($name)) {
            $issues += "graphisoft-group-missing"
        }
    } else {
        $issues += "unknown-namespace($ns)"
    }

    # Member checks (lightweight — IR can never have placeholder methods since
    # we always materialise from JSON Schema; we still spot-check for the
    # placeholder shapes that other extractors produce so the test catches any
    # regression of our PageParser).
    $methods = if ($t.methods) { $t.methods } else { @() }
    $properties = if ($t.properties) { $t.properties } else { @() }

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

    $methodSample = Sample-Random $methods 3
    foreach ($m in $methodSample) {
        $expectedPlaceholder = "$($m.name)(...)"
        if ($m.signature -eq $expectedPlaceholder) {
            $issues += "method-placeholder-signature($($m.name))"
        }
    }

    $propSample = Sample-Random $properties 3
    foreach ($p in $propSample) {
        # Tapir.Schema property types are valid as "string" / "integer" / etc. — not placeholders.
        # The only thing we want to catch is empty type strings.
        if ([string]::IsNullOrEmpty($p.type)) {
            $issues += "property-empty-type($($p.name))"
        }
    }

    if ($issues.Count -eq 0) {
        $pass++
        $results += [pscustomobject]@{ idx = $i; ns = $ns; name = $name; kind = $t.kind; result = "PASS"; issues = "" }
    } else {
        $fail++
        $results += [pscustomobject]@{ idx = $i; ns = $ns; name = $name; kind = $t.kind; result = "FAIL"; issues = ($issues -join "; ") }
    }
}

$results | Format-Table -AutoSize | Out-String -Width 8000
Write-Host ""
Write-Host "Strict verification summary: $pass pass / $fail fail / $Count total"
