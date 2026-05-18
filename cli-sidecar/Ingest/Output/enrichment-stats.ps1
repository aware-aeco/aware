param([Parameter(Mandatory)] [string] $IrPath)
$ir = Get-Content $IrPath -Raw | ConvertFrom-Json
$em = 0; $tm = 0
$ep = 0; $tp = 0
$ee = 0; $te = 0
foreach ($t in $ir.types) {
    foreach ($m in @($t.methods) + @($t.constructors)) {
        $tm++
        if ($m.signature -ne "$($m.name)(...)" -and $m.signature -ne "$($t.name)(...)") { $em++ }
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
"Methods:    $em/$tm ($([Math]::Round(100*$em/$tm,1))%)"
"Properties: $ep/$tp ($([Math]::Round(100*$ep/$tp,1))%)"
"Events:     $ee/$te ($([Math]::Round(100*$ee/$te,1))%)"
"Total members: $($tm+$tp+$te) -- enriched: $($em+$ep+$ee)"
