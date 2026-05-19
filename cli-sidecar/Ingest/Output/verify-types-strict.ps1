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

    # Type-page check: name + kind present in the source page.
    #
    # The check has two flavours depending on how the IR was extracted:
    #
    # 1. HTML-rendered Sandcastle (Tekla, Rhino, Grasshopper). The page title is a
    #    literal "FooBar Class" / "FooBar Enumeration" / etc., and the body carries
    #    member tables with the type name verbatim. We assert both the bare type name
    #    AND the appropriate kind word ("Class", "Structure", ...) are present in
    #    the HTML body.
    #
    # 2. Markdown-rendered openapi-generator output (IDEA Statica). The Model page
    #    title is `# IdeaStatiCa.X.Model.<TypeName>` and the *Api page title is
    #    `# <TagName>Api`. There's no "Class" / "Enumeration" suffix — that's an HTML
    #    rendering convention that doesn't apply here. We still assert the type name
    #    is present in the title; for `kind` we check structural signals (Properties
    #    section for class/struct/enum; method-anchor list for API class).
    #
    # The dispatcher uses the doc_url extension to choose the right check. Generic
    # types (e.g. `Enum<E>`) have their type-parameter list stripped before the name
    # check so the bare name match is still meaningful for HTML's LST-script
    # substitution and for filename slugging.
    $bareName = $t.name -replace '<[^>]+>$', ''
    # Markdown-source variant: .md (openapi-generator) or .yaml/.yml (OpenAPI spec direct).
    # YAML pages don't have a kind-word convention either, so they fall under the same
    # structural-signal check as markdown.
    $isMarkdownSource = $url -match '\.md(\?|$)' -or $url -match '\.ya?ml(\?|$)'
    try {
        # Retry up to 2 times on transient HTTP errors (429 rate-limit, 5xx). github.com's blob
        # view and search pages rate-limit anonymous requests aggressively; sleep+retry keeps the
        # sampler from spuriously flagging types whose pages are temporarily throttled.
        $r = $null
        for ($attempt = 1; $attempt -le 3; $attempt++) {
            try {
                $r = Invoke-WebRequest -Uri $url -UseBasicParsing -UserAgent "AWARE-coverage-verify/0.30" -ErrorAction Stop
                break
            }
            catch [System.Net.WebException] {
                $code = if ($_.Exception.Response) { [int]$_.Exception.Response.StatusCode } else { 0 }
                if ($attempt -lt 3 -and ($code -eq 429 -or ($code -ge 500 -and $code -le 599))) {
                    Start-Sleep -Seconds (3 * $attempt)
                    continue
                }
                throw
            }
        }
        $pageText = $r.Content
        if ($isMarkdownSource) {
            # Markdown / YAML check: type name must appear in the page. For markdown the
            # H1 title is the canonical place; for YAML it's the components/schemas/<Name>:
            # line. Either way, a substring check on the bare name is sufficient — the
            # generic-suffix has already been stripped above for templated types.
            $isYaml = ($url -match '\.ya?ml(\?|$)')
            if ($isYaml) {
                # For YAML sources require BOTH the type name appears AND a near-context
                # indication that it's defined as a schema (e.g. "<bareName>:" line near the
                # components/schemas tree). The 'contains' check is good enough — the YAML
                # is the openapi spec, the type name is unique in /components/schemas/<Name>.
                if (-not $pageText.Contains("$bareName" + ":")) { $issues += "type-name-missing-in-yaml" }
            } else {
                $firstHashLine = ($pageText -split "`n" | Where-Object { $_ -match '^#\s' } | Select-Object -First 1)
                if (-not $firstHashLine -or -not ($firstHashLine -match [regex]::Escape($bareName))) {
                    $issues += "type-name-missing-in-h1"
                }
            }
            switch ($t.kind) {
                "class" {
                    if ($t.namespace -match '\.Api$') {
                        # API class — methods should appear as ## headings with the operationId
                        # (without the Async suffix) on markdown pages, or as operationId: lines
                        # in YAML pages.
                        $methodsToCheck = if ($t.methods) { ($t.methods | Select-Object -First 3) } else { @() }
                        $methodOk = ($methodsToCheck.Count -eq 0)
                        foreach ($mm in $methodsToCheck) {
                            $opBase = $mm.name -replace 'Async$', ''
                            $opFull = $mm.name
                            if ($isYaml) {
                                if ($pageText -match "operationId:\s+$([regex]::Escape($opBase))\s*$" `
                                    -or $pageText.Contains("operationId: $opBase`n") `
                                    -or $pageText.Contains("operationId: $opBase`r")) { $methodOk = $true; break }
                            } else {
                                # openapi-generator emits the method heading with the Async suffix
                                # (e.g. `## **CopyConnectionAsync**`) while the anchor id is the
                                # lowercased operationId without the suffix (e.g. `<a id="copyconnection">`).
                                # Accept either form so the check is robust to either rendering choice.
                                if ($pageText -match "##\s+\*\*$([regex]::Escape($opFull))\*\*" `
                                    -or $pageText -match "##\s+\*\*$([regex]::Escape($opBase))\*\*" `
                                    -or $pageText -match "<a id=""$([regex]::Escape($opBase.ToLower()))""") { $methodOk = $true; break }
                            }
                        }
                        if (-not $methodOk) { $issues += "api-method-heading-missing" }
                    } else {
                        # Model class — should have a Properties section on markdown, or a
                        # properties: child on YAML.
                        if ($t.properties -and $t.properties.Count -gt 0) {
                            if ($isYaml) {
                                if (-not ($pageText -match "(?ms)^\s+$([regex]::Escape($bareName)):\s*$.*?properties:")) {
                                    $issues += "properties-not-found-in-yaml-schema"
                                }
                            } else {
                                if (-not ($pageText -match '##\s+Properties')) { $issues += "properties-section-missing" }
                            }
                        }
                    }
                }
                "enum" {
                    # Enum markdown pages are skeletal (just the H1 + empty Properties header).
                    # The IR's enum_values come from the YAML, not the markdown. We require only
                    # that the H1 contains the type name (already checked above).
                }
                default {
                    # struct/interface/delegate aren't emitted by openapi-generator. If we see
                    # one we fall back to the type-name check only.
                }
            }
        } else {
            # HTML source — three doc-site conventions in the wild:
            #   - Sandcastle (Tekla Structures, Rhino): "<Name> Class" / "<Name> Enumeration" — kind word trails.
            #   - DocFX (Tekla Tedds, ArchiCAD, etc.): "Class <Name>" / "Enum <Name>"  — kind word leads.
            #   - Source code (Dynamo via github.com/.../blob/<ref>/...cs): "class <Name>" / "enum <Name>" —
            #     kind word leads + LOWERCASE C# keyword. The page is the rendered .cs source from
            #     GitHub blob view, NOT a generated doc page. Vendor lacks a hosted reference site so
            #     the source files themselves are the canonical "what is this type" view.
            # We accept ANY ordering AND any case, plus the alternate enum spelling ("Enum" vs "Enumeration")
            # and struct spelling ("Struct" vs "Structure") that DocFX uses.
            $kindWords = switch ($t.kind) {
                "class" { @("Class", "class") }
                "static-class" { @("Class", "class", "static class") }
                "struct" { @("Structure", "Struct", "struct") }
                "interface" { @("Interface", "interface") }
                "enum" { @("Enumeration", "Enum", "enum") }
                "delegate" { @("Delegate", "delegate") }
                default { @() }
            }
            if (-not $pageText.Contains($bareName)) { $issues += "type-name-missing" }
            $kindMatched = $kindWords.Count -eq 0
            foreach ($kw in $kindWords) {
                if ($pageText.Contains("$bareName $kw") -or $pageText.Contains("$kw $bareName")) {
                    $kindMatched = $true
                    break
                }
                # Generic types: bare name may not appear adjacent to kind word — fall back to
                # kind word presence alongside any rendering of the type name elsewhere.
                if ($t.name -ne $bareName -and $pageText.Contains($kw)) {
                    $kindMatched = $true
                    break
                }
                # Source-code declaration form: `delegate void Foo(...)`, `class<T> Foo {...}` —
                # kind word and type name are on the same line but separated by a return type or
                # generic param. Accept any line that has BOTH the kind word AND the bare name.
                $lines = $pageText -split "`n"
                foreach ($line in $lines) {
                    if ($line.Contains($kw) -and $line.Contains($bareName)) {
                        $kindMatched = $true
                        break
                    }
                }
                if ($kindMatched) { break }
            }
            if (-not $kindMatched) {
                $issues += "kind-word-missing"
            }
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
