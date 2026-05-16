# AWARE CLI v0.6 (Distribution) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development.

**Goal:** Turn "works on your machine" into "works for anyone who runs one command." Tagging `v0.6.0` triggers a GitHub Actions release workflow that cross-compiles `aware.exe` (Rust CLI) for win-x64 + linux-x64 + darwin-arm64, publishes `aware-sidecar` for the same three platforms, bundles each pair into a platform-specific archive, and attaches all of them to a GitHub Release. A `scripts/install.sh` curl-pipe installer detects the user's OS/arch and drops both binaries into `~/.local/bin/` (or `$AWARE_INSTALL_DIR`).

**Out of scope:** winget manifest, Homebrew formula, code signing. Each requires submitting to an external ecosystem repo or buying a signing cert — separate operational tasks documented as v0.6.1+.

---

## Open design decisions (resolved here)

| Decision | Choice | Why | Alternative |
|---|---|---|---|
| **Trigger** | `push` to `v0.6.*` tag → build → create Release → upload artifacts | Standard semver-tag release pattern; tag is the source of truth | Manual `workflow_dispatch` only (no version history) |
| **Platforms** | Rust: win-x64, linux-x64, darwin-arm64. Sidecar: same three (already covered by v0.5.3) | The three the sidecar already supports; matches v0.5.3 matrix | Add darwin-x64 / linux-arm64 (marginal users in 2026) |
| **Archive format** | Windows: `.zip`. Linux/macOS: `.tar.gz`. Each archive contains `aware{.exe}`, `aware-sidecar{.exe}`, `LICENSE`, `README.md`, `VERSION` | Native to each platform; no extra tooling needed at extraction time | Same format on all 3 (loses native convention) |
| **Archive name** | `aware-<version>-<rid>.{zip,tar.gz}` — e.g. `aware-0.6.0-win-x64.zip`, `aware-0.6.0-linux-x64.tar.gz`, `aware-0.6.0-osx-arm64.tar.gz` | Discoverable; matches the `dotnet publish` RID convention so users + scripts can predict the name | Plain `aware-<version>.zip` (loses platform info) |
| **Code signing** | NOT in v0.6.0. Document as v0.6.1+ requiring a Microsoft Authenticode cert (~$300/yr) and Apple Developer ID enrollment | Out of scope for this PR; needs $$$ + identity verification | Skip distribution entirely until signed (too conservative) |
| **Install script location** | `scripts/install.sh` at repo root. Curl-pipe form: `curl -fsSL https://raw.githubusercontent.com/aware-aeco/aware/main/scripts/install.sh \| bash` | Conventional; predictable URL pattern | Hosted at separate domain (more infra) |
| **Install script: OS/arch detection** | Detect via `uname -s` + `uname -m`. Map → RID. Fail clear if unsupported. Refuse to run if `--help` provided so users can read docs first | Standard portable shell pattern | PowerShell variant (separate file; deferred to v0.6.1) |
| **Install location** | Default `~/.local/bin/`. Override via `AWARE_INSTALL_DIR=<path>`. Sidecar lives alongside `aware`. The Rust client's discover() already looks for sidecar as a sibling | XDG-friendly; works without sudo; matches what gh / rustup do | `/usr/local/bin/` (requires sudo); `~/.aware/bin/` (less standard) |
| **Latest-version resolution** | `install.sh` accepts an optional `--version <ver>`; without it, queries the GitHub Releases API for the latest tag matching `v0.6.*` | Allows reproducible installs (pinned version) and convenience (latest) | Hard-code "latest" only (loses pinning) |
| **PowerShell installer** | `scripts/install.ps1` for Windows users who prefer `iex (irm <url>)`. Mirrors install.sh behavior. | Native Windows experience; gh CLI offers both | Skip PowerShell (Windows users left in WSL) |
| **Version bumping** | Bump `cli/Cargo.toml` `version` from `0.1.0-dev` → `0.6.0`. Bump `cli-sidecar/cli-sidecar.csproj` `<Version>` from `0.5.1` → `0.6.0`. (Yes, the sidecar version was stuck at 0.5.1 since that's when it was added.) | Aligns CLI + sidecar versions for releases | Independent versioning (more flexibility, more drift) |
| **Release-only CI workflow** | New `.github/workflows/release.yml`. The existing `build-sidecar.yml` still runs on every tag for safety; release.yml additionally creates the GitHub Release + bundles the Rust binary | Separation of concerns; build-sidecar is the sidecar-only smoke; release is the orchestrator | Combine into one workflow (more complex; harder to debug) |
| **Cross-compilation strategy** | Each platform runner builds for itself (windows-latest builds win-x64, ubuntu-latest builds linux-x64, macos-latest builds osx-arm64). Avoids cross-compile toolchain pain | Standard for projects with native code (here: keyring's libsecret link on Linux) | Single Linux runner cross-compiling all (cleaner CI; libsecret would need separate handling) |
| **VERSION file in archive** | Plain text file at archive root with one line: the version string (e.g. `0.6.0`). Doctor could read it later to show "you're running 0.6.0 (latest is X)" but that's a v0.6.1 feature | Trivial to write; useful for any future "am I current?" check | Skip (loses a small audit feature) |

---

## Scope discipline

**v0.6 user-facing surfaces:** none new. The existing CLI surface is unchanged. The release pipeline + installers are infrastructure.

**v0.6 internal surfaces:**
- `.github/workflows/release.yml` — orchestrates the full release
- `scripts/install.sh` — Unix install script (bash + curl)
- `scripts/install.ps1` — Windows install script (PowerShell)
- Updated `cli/Cargo.toml` — version `0.6.0`
- Updated `cli-sidecar/cli-sidecar.csproj` — version `0.6.0`
- Updated `cli/README.md` — install section + curl-pipe instructions

**Out of scope (for v0.6 — explicit follow-ups):**
- **v0.6.1**: code signing (Authenticode + Apple Developer ID)
- **v0.6.2**: winget manifest (submit to microsoft/winget-pkgs)
- **v0.6.3**: Homebrew formula (custom tap repo at aware-aeco/homebrew-aware or submit to homebrew-core)
- **v0.6.4**: ARM Linux + Intel macOS in release matrix

---

## Tasks

### Task 0: Branch + baseline (done — on `feat/cli-v06-distribution`)

### Task 1: Bump versions

`cli/Cargo.toml`: change `version = "0.1.0-dev"` to `version = "0.6.0"`.
`cli-sidecar/cli-sidecar.csproj`: change `<Version>0.5.1</Version>` to `<Version>0.6.0</Version>`.

Build + test to ensure no version-string-dependent code breaks:
```powershell
$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"; cd cli; cargo build; cargo test
cd C:\Users\bimst\source\repos\aware\cli-sidecar; dotnet test
```

Both should pass. Some tests may check `env!("CARGO_PKG_VERSION")` and expect `0.1.0-dev`; update those to `0.6.0` or to a regex pattern.

Specifically check:
- `cli/tests/basic.rs::version_prints_dev_version` — likely needs adjustment. Update to assert `0.6.0` (or just assert it contains a digit and a dot, looser).

Commit: `chore(cli): bump cli + sidecar to 0.6.0`

### Task 2: GitHub Actions release workflow

Create `.github/workflows/release.yml`:

```yaml
name: Release

on:
  push:
    tags:
      - 'v0.6.*'
      - 'v0.7.*'
      - 'v1.*'
  workflow_dispatch: {}

jobs:
  build:
    strategy:
      fail-fast: false
      matrix:
        include:
          - os: windows-latest
            rid: win-x64
            ext: .exe
            archive: zip
            archive_ext: .zip
          - os: ubuntu-latest
            rid: linux-x64
            ext: ""
            archive: tar
            archive_ext: .tar.gz
          - os: macos-latest
            rid: osx-arm64
            ext: ""
            archive: tar
            archive_ext: .tar.gz

    runs-on: ${{ matrix.os }}

    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install .NET 9
        uses: actions/setup-dotnet@v4
        with:
          dotnet-version: '9.0.x'

      - name: Install clang on Linux
        if: matrix.os == 'ubuntu-latest'
        run: sudo apt-get update && sudo apt-get install -y clang libsecret-1-dev

      - name: Build Rust CLI (release)
        working-directory: cli
        run: cargo build --release

      - name: Publish C# sidecar (NativeAOT)
        run: dotnet publish cli-sidecar -c Release -r ${{ matrix.rid }} -p:PublishAot=true

      - name: Stage artifacts
        shell: bash
        run: |
          set -e
          version="${GITHUB_REF_NAME#v}"
          if [ -z "$version" ] || [[ ! "$version" =~ ^[0-9] ]]; then
            version="0.0.0-dev"
          fi
          stage="aware-${version}-${{ matrix.rid }}"
          mkdir -p "$stage"
          cp "cli/target/release/aware${{ matrix.ext }}" "$stage/"
          cp "cli-sidecar/bin/Release/net9.0/${{ matrix.rid }}/publish/aware-sidecar${{ matrix.ext }}" "$stage/"
          cp LICENSE "$stage/" 2>/dev/null || echo "(no LICENSE file)" > "$stage/LICENSE"
          cp cli/README.md "$stage/README.md"
          echo "$version" > "$stage/VERSION"
          ls -la "$stage/"
          echo "STAGE_DIR=$stage" >> $GITHUB_ENV
          echo "VERSION=$version" >> $GITHUB_ENV

      - name: Archive (Linux/macOS — tar.gz)
        if: matrix.archive == 'tar'
        shell: bash
        run: |
          tar -czf "${{ env.STAGE_DIR }}.tar.gz" "${{ env.STAGE_DIR }}"
          ls -la *.tar.gz
          echo "ARTIFACT=${{ env.STAGE_DIR }}.tar.gz" >> $GITHUB_ENV

      - name: Archive (Windows — zip)
        if: matrix.archive == 'zip'
        shell: pwsh
        run: |
          Compress-Archive -Path "${{ env.STAGE_DIR }}\*" -DestinationPath "${{ env.STAGE_DIR }}.zip"
          Get-ChildItem *.zip
          "ARTIFACT=${{ env.STAGE_DIR }}.zip" | Out-File -FilePath $env:GITHUB_ENV -Append

      - name: Upload artifact
        uses: actions/upload-artifact@v4
        with:
          name: aware-${{ env.VERSION }}-${{ matrix.rid }}
          path: ${{ env.ARTIFACT }}

  release:
    needs: build
    runs-on: ubuntu-latest
    if: startsWith(github.ref, 'refs/tags/v')
    permissions:
      contents: write
    steps:
      - uses: actions/checkout@v4

      - name: Download all artifacts
        uses: actions/download-artifact@v4
        with:
          path: dist
          merge-multiple: true

      - name: List downloaded artifacts
        run: ls -la dist/

      - name: Create GitHub Release
        uses: softprops/action-gh-release@v2
        with:
          files: dist/*
          generate_release_notes: true
          fail_on_unmatched_files: false
          draft: false
          prerelease: ${{ contains(github.ref_name, '-') }}
```

Commit: `ci: release workflow that builds + publishes per-platform archives`

### Task 3: Unix install script

Create `scripts/install.sh`:

```bash
#!/usr/bin/env bash
#
# AWARE CLI installer
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/aware-aeco/aware/main/scripts/install.sh | bash
#   curl -fsSL https://raw.githubusercontent.com/aware-aeco/aware/main/scripts/install.sh | bash -s -- --version 0.6.0
#
# Env vars:
#   AWARE_INSTALL_DIR   destination for binaries (default: ~/.local/bin)
#   AWARE_VERSION       version tag to install (default: latest v0.6.*)

set -euo pipefail

REPO="aware-aeco/aware"
INSTALL_DIR="${AWARE_INSTALL_DIR:-$HOME/.local/bin}"

# --- Parse flags ---
VERSION=""
while [ $# -gt 0 ]; do
  case "$1" in
    --version)
      VERSION="$2"
      shift 2
      ;;
    --help|-h)
      echo "Usage: install.sh [--version <ver>]"
      echo "Env vars: AWARE_INSTALL_DIR (default: ~/.local/bin)"
      exit 0
      ;;
    *)
      echo "unknown arg: $1" >&2
      exit 2
      ;;
  esac
done

# --- Detect platform ---
os="$(uname -s | tr '[:upper:]' '[:lower:]')"
arch="$(uname -m)"
case "$os/$arch" in
  linux/x86_64)
    rid="linux-x64"
    ext=""
    archive_ext="tar.gz"
    ;;
  darwin/arm64)
    rid="osx-arm64"
    ext=""
    archive_ext="tar.gz"
    ;;
  darwin/x86_64)
    echo "warning: Intel macOS isn't in the release matrix yet (tracked for v0.6.4)." >&2
    echo "If you're on a Mac with Rosetta, try the osx-arm64 build:" >&2
    rid="osx-arm64"
    ext=""
    archive_ext="tar.gz"
    ;;
  *)
    echo "unsupported platform: $os/$arch" >&2
    echo "supported: linux/x86_64, darwin/arm64, windows/x86_64 (use install.ps1)" >&2
    exit 1
    ;;
esac

# --- Resolve version ---
if [ -z "$VERSION" ]; then
  VERSION="${AWARE_VERSION:-}"
fi
if [ -z "$VERSION" ]; then
  echo "Resolving latest v0.6.* release..."
  if command -v curl >/dev/null 2>&1; then
    VERSION=$(curl -fsSL "https://api.github.com/repos/${REPO}/releases" \
      | grep '"tag_name":' \
      | sed -E 's/.*"tag_name": *"v([0-9.]+)".*/\1/' \
      | grep -E '^0\.6\.' \
      | head -1)
  fi
  if [ -z "$VERSION" ]; then
    echo "could not resolve latest version. Pass --version <ver> or set AWARE_VERSION." >&2
    exit 1
  fi
fi

echo "Installing aware ${VERSION} for ${rid}..."

# --- Download + extract ---
url="https://github.com/${REPO}/releases/download/v${VERSION}/aware-${VERSION}-${rid}.${archive_ext}"
echo "  source: ${url}"

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

archive="$tmp/aware.${archive_ext}"
curl -fsSL -o "$archive" "$url"

tar -xzf "$archive" -C "$tmp"

# --- Install ---
mkdir -p "$INSTALL_DIR"
src_dir="$tmp/aware-${VERSION}-${rid}"
cp "$src_dir/aware${ext}" "$INSTALL_DIR/"
cp "$src_dir/aware-sidecar${ext}" "$INSTALL_DIR/"
chmod +x "$INSTALL_DIR/aware${ext}" "$INSTALL_DIR/aware-sidecar${ext}"

echo
echo "✓ Installed:"
echo "    $INSTALL_DIR/aware${ext}"
echo "    $INSTALL_DIR/aware-sidecar${ext}"
echo
if ! echo ":$PATH:" | grep -q ":${INSTALL_DIR}:"; then
  echo "⚠ ${INSTALL_DIR} is not on your PATH."
  echo "  Add this to your shell config:"
  echo "    export PATH=\"${INSTALL_DIR}:\$PATH\""
fi
echo
echo "Try it:"
echo "  aware --version"
echo "  aware doctor"
```

Make executable:
```bash
chmod +x scripts/install.sh
```

Commit: `feat(install): scripts/install.sh — curl-pipe Unix installer`

### Task 4: PowerShell install script

Create `scripts/install.ps1`:

```powershell
# AWARE CLI installer for Windows.
#
# Usage:
#   iex (irm https://raw.githubusercontent.com/aware-aeco/aware/main/scripts/install.ps1)
#   $env:AWARE_VERSION = "0.6.0"; iex (irm https://raw.githubusercontent.com/aware-aeco/aware/main/scripts/install.ps1)
#
# Env vars:
#   AWARE_INSTALL_DIR   destination for binaries (default: ~\.local\bin or %LOCALAPPDATA%\aware\bin)
#   AWARE_VERSION       version tag to install (default: latest v0.6.*)

$ErrorActionPreference = 'Stop'

$Repo = "aware-aeco/aware"
$InstallDir = $env:AWARE_INSTALL_DIR
if (-not $InstallDir) {
  $InstallDir = Join-Path $env:LOCALAPPDATA "aware\bin"
}

$Version = $env:AWARE_VERSION
if (-not $Version) {
  Write-Host "Resolving latest v0.6.* release..."
  $releases = Invoke-RestMethod -Uri "https://api.github.com/repos/$Repo/releases"
  $latest = $releases | Where-Object { $_.tag_name -match '^v0\.6\.' } | Select-Object -First 1
  if (-not $latest) {
    Write-Error "could not resolve latest version. Set `$env:AWARE_VERSION."
    exit 1
  }
  $Version = $latest.tag_name -replace '^v', ''
}

Write-Host "Installing aware $Version for win-x64..."

$url = "https://github.com/$Repo/releases/download/v$Version/aware-$Version-win-x64.zip"
$tmp = New-Item -ItemType Directory -Path (Join-Path $env:TEMP ("aware-install-" + [Guid]::NewGuid().ToString('N'))) -Force
try {
  $archive = Join-Path $tmp "aware.zip"
  Invoke-WebRequest -Uri $url -OutFile $archive

  Expand-Archive -Path $archive -DestinationPath $tmp -Force
  $srcDir = Join-Path $tmp "aware-$Version-win-x64"

  if (-not (Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
  }
  Copy-Item (Join-Path $srcDir "aware.exe") $InstallDir -Force
  Copy-Item (Join-Path $srcDir "aware-sidecar.exe") $InstallDir -Force

  Write-Host ""
  Write-Host "✓ Installed:" -ForegroundColor Green
  Write-Host "    $InstallDir\aware.exe"
  Write-Host "    $InstallDir\aware-sidecar.exe"
  Write-Host ""

  # Check PATH
  $pathEntries = $env:PATH -split ';'
  if ($pathEntries -notcontains $InstallDir) {
    Write-Host "⚠ $InstallDir is not on your PATH." -ForegroundColor Yellow
    Write-Host "  Add it via System Properties → Environment Variables, OR run:"
    Write-Host "    [Environment]::SetEnvironmentVariable('Path', `$env:Path + ';$InstallDir', 'User')"
  }
  Write-Host ""
  Write-Host "Try it:"
  Write-Host "  aware --version"
  Write-Host "  aware doctor"
} finally {
  Remove-Item -Recurse -Force $tmp -ErrorAction SilentlyContinue
}
```

Commit: `feat(install): scripts/install.ps1 — curl-pipe Windows installer`

### Task 5: README — install section

Update `cli/README.md` Status section to v0.6.0; add a new "Install" section near the top:

```markdown
## Install

**Linux / macOS (curl + bash):**

```bash
curl -fsSL https://raw.githubusercontent.com/aware-aeco/aware/main/scripts/install.sh | bash
```

Drops `aware` and `aware-sidecar` into `~/.local/bin/`. Override with
`AWARE_INSTALL_DIR=<path>`.

**Windows (PowerShell):**

```powershell
iex (irm https://raw.githubusercontent.com/aware-aeco/aware/main/scripts/install.ps1)
```

Drops both binaries into `%LOCALAPPDATA%\aware\bin\`. Override with
`$env:AWARE_INSTALL_DIR`.

**Pinned version:**

```bash
curl -fsSL .../install.sh | bash -s -- --version 0.6.0
# PowerShell
$env:AWARE_VERSION = "0.6.0"; iex (irm .../install.ps1)
```

**From source:**

```bash
git clone https://github.com/aware-aeco/aware
cd aware
cargo build --release -p aware --manifest-path cli/Cargo.toml
dotnet publish cli-sidecar -c Release -r <rid> -p:PublishAot=true
```

**Tracked for v0.6.1+:**
- Code-signed binaries (Authenticode + Apple Developer ID)
- winget package (`winget install aware-aeco.aware`)
- Homebrew formula (`brew install aware-aeco/tap/aware`)
- ARM Linux + Intel macOS in the release matrix
```

Commit: `docs(cli): README install section for v0.6 release`

### Task 6: Final verification

Run:
```powershell
$env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"; cd cli; cargo fmt --all -- --check; cargo clippy --all-targets -- -D warnings; cargo test --all-targets; cargo build --release
cd C:\Users\bimst\source\repos\aware\cli-sidecar; dotnet test
```

All green. Workflow files are syntactically YAML-valid (no local CI run; relies on GitHub Actions on first tag).

If any test mentions a version string, fix it to match `0.6.0`.

Commit: `chore(cli): post-v0.6 verification` (only if anything actually changed; otherwise skip)

---

## Self-review

**Spec coverage:**
- Release pipeline → Task 2
- Curl-pipe installers (Unix + Windows) → Tasks 3 + 4
- Versions bumped → Task 1
- Install docs → Task 5

**Out of scope (acknowledged):**
- Code signing
- winget / Homebrew submissions
- ARM Linux / Intel macOS

**Realistic effort:** 1-2 hours of subagent work. Most of the workflow is YAML config; the scripts are ~80 lines each. No deep logic.
