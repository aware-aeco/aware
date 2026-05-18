# verify-types.ps1 — sample 50 types from an IR + spot-check each doc_url.
#
# Usage: ./verify-types.ps1 <ir.json> [<count>=50]
#
# Sampling is deterministic — seeded by the SHA-256 hash of the IR file itself
# so the same IR + count always produces the same sample list.

param(
    [Parameter(Mandatory)] [string] $IrPath,
    [int] $Count = 50
)

$json = Get-Content $IrPath -Raw | ConvertFrom-Json
$types = $json.types

# Deterministic seed from the IR file's SHA-256.
$hashBytes = (Get-FileHash $IrPath -Algorithm SHA256).Hash
$seed = [int]([Convert]::ToInt64($hashBytes.Substring(0,8), 16) -band 0x7FFFFFFF)
$rng = New-Object System.Random($seed)

# Fisher-Yates shuffle, take first $Count
$indices = 0..($types.Count - 1)
$arr = [int[]]$indices
for ($i = $arr.Count - 1; $i -gt 0; $i--) {
    $j = $rng.Next($i + 1)
    $tmp = $arr[$i]; $arr[$i] = $arr[$j]; $arr[$j] = $tmp
}
$sampleIdx = $arr[0..($Count - 1)]

Write-Host "Sampled $Count types from $($types.Count) (deterministic seed = $seed):"
Write-Host ""

$pass = 0
$fail = 0
$results = @()
foreach ($i in $sampleIdx) {
    $t = $types[$i]
    $url = $t.doc_url
    try {
        $r = Invoke-WebRequest -Uri $url -UseBasicParsing -UserAgent "AWARE-coverage-verify/0.30" -ErrorAction Stop
        $html = $r.Content
        # Spot-check: the type name should appear in the rendered HTML and the kind word.
        $kindWord = switch ($t.kind) {
            "class" { "Class" }
            "struct" { "Structure" }
            "interface" { "Interface" }
            "enum" { "Enumeration" }
            "delegate" { "Delegate" }
            default { "" }
        }
        $namePresent = $html.Contains($t.name)
        $kindPresent = $html.Contains("$($t.name) $kindWord")
        if ($namePresent -and $kindPresent) {
            $pass += 1
            $results += [pscustomobject]@{ idx = $i; name = $t.name; ns = $t.namespace; kind = $t.kind; result = "PASS"; url = $url }
        } else {
            $fail += 1
            $results += [pscustomobject]@{ idx = $i; name = $t.name; ns = $t.namespace; kind = $t.kind; result = "FAIL_name=$namePresent kind=$kindPresent"; url = $url }
        }
    } catch {
        $fail += 1
        $results += [pscustomobject]@{ idx = $i; name = $t.name; ns = $t.namespace; kind = $t.kind; result = "FAIL_$($_.Exception.Message)"; url = $url }
    }
}
$results | Format-Table -AutoSize | Out-String -Width 4096
Write-Host ""
Write-Host "Verification summary: $pass pass / $fail fail / $Count total"
