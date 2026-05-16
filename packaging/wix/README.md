# Windows MSI installer

`packaging/wix/aware.wxs` is the source of the Windows MSI installer for the AWARE CLI. Built with [WiX 6](https://wixtoolset.org/) and packaged on every tag push by the `Build MSI installer (Windows)` step in [`../../.github/workflows/release.yml`](../../.github/workflows/release.yml). (WiX 7+ added an "Open Source Maintenance Fee" EULA requiring per-invocation acceptance — we pin WiX 6 to avoid that.)

## What ends up in the MSI

```
C:\Program Files\AWARE\
├── aware.exe
├── aware-sidecar.exe
├── LICENSE
├── README.md
└── VERSION
```

Plus a `Software\aware-aeco\AWARE` registry key (HKLM) and a system `PATH` entry pointing at `C:\Program Files\AWARE\`, so `aware` works in any shell after install. Add/Remove Programs entry links back to the GitHub repo + issue tracker.

## Why MSI

- **`winget install aware-aeco`** requires either MSI or signed EXE per Microsoft's [winget-pkgs manifest schema](https://github.com/microsoft/winget-pkgs). MSI is the simpler route.
- Clean uninstall via Apps & Features / `msiexec /x`.
- Standard Windows package format. IT departments can deploy via Group Policy / SCCM / Intune.

## How it gets built

The release workflow stages `aware.exe`, `aware-sidecar.exe`, `LICENSE`, `README.md`, `VERSION` into a directory named `aware-<version>-win-x64/` and then runs:

```pwsh
dotnet tool install -g wix --version 6.0.0
wix extension add -g WixToolset.UI.wixext
wix build packaging/wix/aware.wxs `
  -d Version=<version> `
  -d StageDir=aware-<version>-win-x64 `
  -arch x64 `
  -o aware-<version>-win-x64.msi
```

Output `aware-<version>-win-x64.msi` is uploaded as a GitHub Release asset alongside the zip / tar.gz archives.

## Building locally

```pwsh
# Install WiX once
dotnet tool install -g wix --version 6.0.0

# From repo root, after a release build of both binaries
$version = "0.8.0-dev"
$stage = "aware-$version-win-x64"
mkdir $stage
copy cli\target\release\aware.exe $stage\
copy cli-sidecar\bin\Release\net9.0\win-x64\publish\aware-sidecar.exe $stage\
copy LICENSE $stage\
copy cli\README.md $stage\README.md
"$version" | Out-File -Encoding ASCII $stage\VERSION

wix build packaging\wix\aware.wxs -d Version=$version -d StageDir=$stage -arch x64 -o "aware-$version-win-x64.msi"

# Install (admin shell)
msiexec /i "aware-$version-win-x64.msi" /qn
aware --version

# Uninstall
msiexec /x "aware-$version-win-x64.msi" /qn
```

## Signing

The MSI is currently **unsigned**. SmartScreen will warn "Windows protected your PC" with "publisher unknown" on first run. Users can click *More info → Run anyway*. Acceptable for v0.8.

Code-signing involves:
- Purchasing an OV or EV cert (~$200–500/yr from Sectigo / DigiCert / GlobalSign)
- Storing it in Azure Key Vault (no longer permitted to ship as a `.pfx` file since 2023)
- Adding `signtool` (or `AzureSignTool`) invocation after `wix build`
- Re-running the build pipeline

Tracked as a follow-up. EV cert is required for SmartScreen reputation bypass, but adds ~$200/yr and a hardware token complication. Decide based on user complaints + adoption signal.

## UpgradeCode

`FEDFDE94-1035-4AAE-900F-BD3DE69B1F10` — permanent. Do not change. This GUID is how Windows recognizes an installed AWARE CLI when a newer MSI is run; changing it means the new MSI installs alongside the old one instead of replacing it.

## winget submission (separate operational step)

After an MSI is signed (or after we decide unsigned is fine), submit a manifest to [`microsoft/winget-pkgs`](https://github.com/microsoft/winget-pkgs):

```
manifests/a/aware-aeco/aware/<version>/
  aware-aeco.aware.installer.yaml
  aware-aeco.aware.locale.en-US.yaml
  aware-aeco.aware.yaml
```

Microsoft validates the PR, runs a sandbox install, and (typically within a few days) merges it. After merge, `winget install aware-aeco` works.
