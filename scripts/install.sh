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
echo "Installed:"
echo "    $INSTALL_DIR/aware${ext}"
echo "    $INSTALL_DIR/aware-sidecar${ext}"
echo
if ! echo ":$PATH:" | grep -q ":${INSTALL_DIR}:"; then
  echo "Warning: ${INSTALL_DIR} is not on your PATH."
  echo "  Add this to your shell config:"
  echo "    export PATH=\"${INSTALL_DIR}:\$PATH\""
fi
echo
echo "Try it:"
echo "  aware --version"
echo "  aware doctor"
