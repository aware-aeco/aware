<#
.SYNOPSIS
  The Authenticode policy every Windows binary AWARE ships must meet, applied three ways.

.DESCRIPTION
  release.yml signs AWARE's Windows binaries with Azure Artifact Signing and refuses to
  publish any that fail this policy. One predicate (Get-SignatureProblem) decides both what
  gets signed and what is accepted, so the two cannot drift apart:

    -Mode Catalog   list every binary under -Root that the policy rejects, i.e. what to sign
    -Mode Verify    fail if any binary under -Root is rejected, a -Require'd name is missing,
                    or there are no binaries at all (a vacuous pass is a failed check)
    -Mode SelfTest  the negative control: build binaries the policy must reject (unsigned,
                    tampered, a third party's signature on an AWARE name) and fail unless
                    each is rejected; -SignedSample adds the same cases cut from a real
                    signed release binary
    -Mode Expand    unpack every .zip and .msi under -Root into -Destination,
                    so Verify can see the binaries inside what the release publishes

  A binary is accepted when its embedded Authenticode signature is Valid, carries an RFC3161
  timestamp, and its signer certificate has the code-signing EKU. AWARE's own binaries
  (aware*, AwareRevit*) must also be signed by our organisation (O=BIMSTUDIO, the same
  rotation-proof identity floless.app pins). A third-party binary we redistribute, such as the
  .NET runtime DLLs in aware-roslyn/, may keep its publisher's signature if that signature
  meets the rest of the policy; if it does not, Catalog lists it and release.yml signs it.

  Signatures from a Windows catalog (SignatureType=Catalog) are refused: they only verify on
  a machine that has the catalog installed, so they say nothing about the file a user downloads.
#>
[CmdletBinding()]
param(
  [Parameter(Mandatory)][ValidateSet('Catalog', 'Verify', 'SelfTest', 'Expand')][string]$Mode,
  [string]$Root,
  [string]$ListPath,
  [string]$Destination,
  [string[]]$Require = @(),
  [string]$SignedSample
)

Set-StrictMode -Version Latest
$ErrorActionPreference = 'Stop'

$CodeSigningEku = '1.3.6.1.5.5.7.3.3'
$TimeStampingEku = '1.3.6.1.5.5.7.3.8'
$OurOrg = '(?:^|,\s*)O="?BIMSTUDIO\b'
$OurBinary = '^(?:aware|AwareRevit)'
$BinaryExtensions = @('.exe', '.dll', '.msi', '.sys', '.node', '.ocx', '.cpl', '.scr')

function Test-WindowsBinary([IO.FileInfo]$File) {
  if ($BinaryExtensions -contains $File.Extension.ToLowerInvariant()) { return $true }
  # A PE image under an unusual extension is still a binary the user runs; catch it by its
  # MZ header rather than trusting the name.
  if ($File.Length -lt 2) { return $false }
  $stream = [IO.File]::OpenRead($File.FullName)
  try { return ($stream.ReadByte() -eq 0x4D -and $stream.ReadByte() -eq 0x5A) } finally { $stream.Dispose() }
}

function Get-WindowsBinary([string]$Directory) {
  @(Get-ChildItem -LiteralPath $Directory -File -Recurse -Force | Where-Object { Test-WindowsBinary $_ })
}

function Get-Eku($Certificate) {
  @($Certificate.Extensions | Where-Object { $_.Oid.Value -eq '2.5.29.37' } | ForEach-Object {
      $_.EnhancedKeyUsages | ForEach-Object { $_.Value }
    })
}

# Returns the reasons $Path fails the policy; an empty result means it is accepted.
function Get-SignatureProblem([string]$Path) {
  $signature = Get-AuthenticodeSignature -LiteralPath $Path
  $problems = [Collections.Generic.List[string]]::new()
  if ($signature.Status -ne 'Valid') { $problems.Add("status is $($signature.Status), not Valid") }
  if ($signature.SignatureType -ne 'Authenticode') { $problems.Add("signature type is $($signature.SignatureType), not an embedded Authenticode signature") }
  if ($null -eq $signature.SignerCertificate) {
    $problems.Add('no signer certificate')
  } else {
    if ((Get-Eku $signature.SignerCertificate) -notcontains $CodeSigningEku) { $problems.Add('signer certificate lacks the code-signing EKU') }
    if ((Split-Path -Leaf $Path) -match $OurBinary -and $signature.SignerCertificate.Subject -notmatch $OurOrg) {
      $problems.Add("an AWARE binary signed by someone else: $($signature.SignerCertificate.Subject)")
    }
  }
  if ($null -eq $signature.TimeStamperCertificate) {
    $problems.Add('no RFC3161 timestamp')
  } elseif ((Get-Eku $signature.TimeStamperCertificate) -notcontains $TimeStampingEku) {
    $problems.Add('timestamp certificate lacks the time-stamping EKU')
  }
  , $problems.ToArray()
}

function Get-RelativePath([string]$Base, [string]$Path) { [IO.Path]::GetRelativePath($Base, $Path).Replace('\', '/') }

# Verify's whole verdict as data, so SelfTest can assert on it without catching exceptions.
function Get-VerifyFailure([string]$Directory, [string[]]$Required) {
  $failures = [Collections.Generic.List[string]]::new()
  $binaries = @(Get-WindowsBinary $Directory)
  if ($binaries.Count -eq 0) { $failures.Add("no Windows binaries under $Directory") }
  foreach ($file in $binaries) {
    $relative = Get-RelativePath $Directory $file.FullName
    $problems = Get-SignatureProblem $file.FullName
    if ($problems.Count -gt 0) {
      $failures.Add("${relative}: $($problems -join '; ')")
    } else {
      $signature = Get-AuthenticodeSignature -LiteralPath $file.FullName
      Write-Host "signed OK: $relative"
      Write-Host "  signer:    $($signature.SignerCertificate.Subject)"
      Write-Host "  timestamp: $($signature.TimeStamperCertificate.Subject)"
    }
  }
  foreach ($pattern in $Required) {
    if (-not ($binaries | Where-Object { $_.Name -like $pattern })) { $failures.Add("required binary missing: $pattern") }
  }
  , $failures.ToArray()
}

function Invoke-Catalog {
  if (-not $Root -or -not $ListPath) { throw 'Catalog needs -Root and -ListPath' }
  $rootPath = (Resolve-Path -LiteralPath $Root).Path
  $binaries = @(Get-WindowsBinary $rootPath)
  if ($binaries.Count -eq 0) { throw "no Windows binaries under $rootPath" }
  $toSign = @()
  foreach ($file in $binaries) {
    $problems = Get-SignatureProblem $file.FullName
    $relative = Get-RelativePath $rootPath $file.FullName
    if ($problems.Count -gt 0) {
      Write-Host "to sign: $relative ($($problems -join '; '))"
      $toSign += $file.FullName
    } else {
      Write-Host "keeps its signature: $relative ($((Get-AuthenticodeSignature -LiteralPath $file.FullName).SignerCertificate.Subject))"
    }
  }
  [IO.File]::WriteAllLines($ListPath, [string[]]$toSign, [Text.UTF8Encoding]::new($false))
  Write-Host "$($toSign.Count) of $($binaries.Count) Windows binaries need signing; list written to $ListPath"
}

function Invoke-Verify {
  if (-not $Root) { throw 'Verify needs -Root' }
  $rootPath = (Resolve-Path -LiteralPath $Root).Path
  $failures = Get-VerifyFailure $rootPath $Require
  if ($failures.Count -gt 0) {
    $failures | ForEach-Object { Write-Host "::error::unsigned or wrongly signed: $_" }
    throw "$($failures.Count) Windows binary signature check(s) failed under $rootPath"
  }
  Write-Host "All $(@(Get-WindowsBinary $rootPath).Count) Windows binaries under $rootPath meet the signing policy."
}

function Find-SignTool {
  $onPath = Get-Command signtool.exe -ErrorAction SilentlyContinue
  if ($onPath) { return $onPath.Source }
  $kits = Join-Path ${env:ProgramFiles(x86)} 'Windows Kits\10\bin'
  $found = @(Get-ChildItem -LiteralPath $kits -Filter signtool.exe -File -Recurse -ErrorAction SilentlyContinue |
      Where-Object { $_.Directory.Name -eq 'x64' } | Sort-Object { $_.Directory.Parent.Name })
  if ($found.Count -eq 0) { throw 'signtool.exe not found; the negative control needs it to strip a signature' }
  $found[-1].FullName
}

function Remove-Signature([string]$SignTool, [string]$Path) {
  $output = & $SignTool remove /s $Path 2>&1
  if ($LASTEXITCODE -ne 0) { throw "signtool remove failed on ${Path}: $output" }
}

function Invoke-Expand {
  if (-not $Root -or -not $Destination) { throw 'Expand needs -Root and -Destination' }
  $rootPath = (Resolve-Path -LiteralPath $Root).Path
  New-Item -ItemType Directory -Path $Destination -Force | Out-Null
  $destination = (Resolve-Path -LiteralPath $Destination).Path
  $packages = @(Get-ChildItem -LiteralPath $rootPath -File -Recurse | Where-Object { $_.Extension -in '.zip', '.msi' })
  if ($packages.Count -eq 0) { throw "no .zip or .msi packages under $rootPath" }
  foreach ($package in $packages) {
    $target = Join-Path $destination $package.Name
    if ($package.Extension -eq '.zip') {
      Expand-Archive -LiteralPath $package.FullName -DestinationPath $target
    } else {
      # An administrative install unpacks the MSI's files without installing anything,
      # so the binaries a user's install would lay down are checked as they come out of
      # the cabinet, not as they went in.
      $process = Start-Process msiexec.exe -ArgumentList '/a', "`"$($package.FullName)`"", '/qn', "TARGETDIR=`"$target`"" -Wait -PassThru
      if ($process.ExitCode -ne 0) { throw "msiexec /a failed on $($package.Name) (exit $($process.ExitCode))" }
      # The admin image carries its own rewritten copy of the database, which is not
      # what users download (that copy's signature no longer matches); the shipped
      # MSI itself is verified where it was downloaded.
      Get-ChildItem -LiteralPath $target -Filter *.msi -File | Remove-Item -Force
    }
    Write-Host "expanded $($package.Name) -> $target"
  }
}

function Set-FlippedByte([string]$Path) {
  # Flip one byte in the middle of the image, well inside the signed range, so the
  # signature still parses but no longer matches the bytes.
  $bytes = [IO.File]::ReadAllBytes($Path)
  $bytes[[int]($bytes.Length / 2)] = $bytes[[int]($bytes.Length / 2)] -bxor 0xFF
  [IO.File]::WriteAllBytes($Path, $bytes)
}

function Invoke-SelfTest {
  $signTool = Find-SignTool
  # A binary with a Valid, timestamped embedded signature from a publisher that is not us.
  $thirdParty = (Get-Process -Id $PID).Path
  if ((Get-SignatureProblem $thirdParty).Count -ne 0) { throw "self-test fixture $thirdParty is not itself validly signed: $((Get-SignatureProblem $thirdParty) -join '; ')" }

  $work = Join-Path ([IO.Path]::GetTempPath()) ("aware-signature-selftest-" + [Guid]::NewGuid().ToString('N'))
  New-Item -ItemType Directory -Path $work | Out-Null
  $results = [Collections.Generic.List[string]]::new()
  function Assert-Verdict([string]$Name, [scriptblock]$Arrange, [bool]$ExpectAccepted, [string[]]$Required = @()) {
    $case = Join-Path $work $Name
    New-Item -ItemType Directory -Path $case | Out-Null
    & $Arrange $case
    $failures = Get-VerifyFailure $case $Required
    $accepted = $failures.Count -eq 0
    $verdict = if ($accepted) { 'accepted' } else { "rejected ($($failures -join ' | '))" }
    if ($accepted -ne $ExpectAccepted) {
      $results.Add("FAIL ${Name}: expected $(if ($ExpectAccepted) { 'accepted' } else { 'rejected' }), got $verdict")
    } else {
      $results.Add("ok   ${Name}: $verdict")
    }
  }

  try {
    Assert-Verdict 'third-party-signature-on-its-own-name-is-accepted' {
      param($d) Copy-Item $thirdParty (Join-Path $d 'pwsh.exe')
    } $true
    Assert-Verdict 'unsigned-binary-is-rejected' {
      param($d) $f = Join-Path $d 'pwsh.exe'; Copy-Item $thirdParty $f; Remove-Signature $signTool $f
    } $false
    Assert-Verdict 'tampered-binary-is-rejected' {
      param($d) $f = Join-Path $d 'pwsh.exe'; Copy-Item $thirdParty $f; Set-FlippedByte $f
    } $false
    Assert-Verdict 'third-party-signature-on-an-aware-name-is-rejected' {
      param($d) Copy-Item $thirdParty (Join-Path $d 'aware.exe')
    } $false
    Assert-Verdict 'third-party-signature-on-an-AwareRevit-dll-is-rejected' {
      param($d) Copy-Item $thirdParty (Join-Path $d 'AwareRevit.dll')
    } $false
    Assert-Verdict 'unsigned-binary-in-a-nested-folder-is-rejected' {
      param($d) $n = Join-Path $d 'aware-roslyn/runtimes/win-x64'; New-Item -ItemType Directory -Path $n | Out-Null
      Copy-Item $thirdParty (Join-Path $d 'pwsh.exe')
      $f = Join-Path $n 'native.dll'; Copy-Item $thirdParty $f; Remove-Signature $signTool $f
    } $false
    Assert-Verdict 'unsigned-PE-under-a-non-binary-extension-is-rejected' {
      param($d) $f = Join-Path $d 'payload.bin'; Copy-Item $thirdParty $f; Remove-Signature $signTool $f
    } $false
    Assert-Verdict 'a-folder-with-no-binaries-is-rejected' {
      param($d) Set-Content -LiteralPath (Join-Path $d 'README.md') -Value 'no binaries here'
    } $false
    Assert-Verdict 'a-missing-required-binary-is-rejected' {
      param($d) Copy-Item $thirdParty (Join-Path $d 'pwsh.exe')
    } $false @('aware.exe')

    if ($SignedSample) {
      $sample = (Resolve-Path -LiteralPath $SignedSample).Path
      $leaf = Split-Path -Leaf $sample
      Assert-Verdict 'the-signed-release-sample-is-accepted' {
        param($d) Copy-Item $sample (Join-Path $d $leaf)
      } $true @($leaf)
      Assert-Verdict 'the-release-sample-with-its-signature-stripped-is-rejected' {
        param($d) $f = Join-Path $d $leaf; Copy-Item $sample $f; Remove-Signature $signTool $f
      } $false
      Assert-Verdict 'the-release-sample-tampered-after-signing-is-rejected' {
        param($d) $f = Join-Path $d $leaf; Copy-Item $sample $f; Set-FlippedByte $f
      } $false
    }
  } finally {
    Remove-Item -LiteralPath $work -Recurse -Force -ErrorAction SilentlyContinue
  }

  $results | ForEach-Object { Write-Host $_ }
  $failed = @($results | Where-Object { $_.StartsWith('FAIL') })
  if ($failed.Count -gt 0) { throw "signature policy self-test failed: $($failed.Count) case(s) got the wrong verdict" }
  Write-Host "Signature policy self-test passed: $($results.Count) cases."
}

switch ($Mode) {
  'Catalog' { Invoke-Catalog }
  'Verify' { Invoke-Verify }
  'SelfTest' { Invoke-SelfTest }
  'Expand' { Invoke-Expand }
}
