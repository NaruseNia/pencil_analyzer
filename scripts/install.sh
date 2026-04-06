#!/usr/bin/env bash
set -euo pipefail

REPO="NaruseNia/pencil_analyzer"
BINARY="pencil_analyzer"
INSTALL_DIR="${INSTALL_DIR:-/usr/local/bin}"

# Detect platform
OS="$(uname -s)"
ARCH="$(uname -m)"

case "$OS" in
  Linux)  OS_TARGET="unknown-linux-gnu" ;;
  Darwin) OS_TARGET="apple-darwin" ;;
  *)
    echo "Error: Unsupported OS '${OS}'." >&2
    exit 1
    ;;
esac

case "$ARCH" in
  x86_64|amd64)  ARCH_TARGET="x86_64" ;;
  aarch64|arm64) ARCH_TARGET="aarch64" ;;
  *)
    echo "Error: Unsupported architecture '${ARCH}'." >&2
    exit 1
    ;;
esac

TARGET="${ARCH_TARGET}-${OS_TARGET}"

# Determine version
if [ $# -ge 1 ]; then
  VERSION="$1"
  TAG="v${VERSION}"
else
  echo "Fetching latest release..."
  TAG="$(curl -fsSL "https://api.github.com/repos/${REPO}/releases/latest" | grep '"tag_name"' | sed -E 's/.*"tag_name": *"([^"]+)".*/\1/')"
  if [ -z "$TAG" ]; then
    echo "Error: Could not determine latest release." >&2
    exit 1
  fi
fi

ARCHIVE="${BINARY}-${TAG}-${TARGET}.tar.gz"
URL="https://github.com/${REPO}/releases/download/${TAG}/${ARCHIVE}"

echo "Installing ${BINARY} ${TAG} for ${TARGET}..."

# Download to temp directory
TMPDIR="$(mktemp -d)"
trap 'rm -rf "$TMPDIR"' EXIT

echo "Downloading ${URL}..."
if ! curl -fsSL -o "${TMPDIR}/${ARCHIVE}" "$URL"; then
  echo "Error: Failed to download. Check that ${TAG} exists and has a build for ${TARGET}." >&2
  exit 1
fi

# Extract
tar xzf "${TMPDIR}/${ARCHIVE}" -C "$TMPDIR"

# Install
if [ -w "$INSTALL_DIR" ]; then
  mv "${TMPDIR}/${BINARY}" "${INSTALL_DIR}/${BINARY}"
else
  echo "Installing to ${INSTALL_DIR} (requires sudo)..."
  sudo mv "${TMPDIR}/${BINARY}" "${INSTALL_DIR}/${BINARY}"
fi

chmod +x "${INSTALL_DIR}/${BINARY}"

echo "Installed ${BINARY} to ${INSTALL_DIR}/${BINARY}"
echo ""
"${INSTALL_DIR}/${BINARY}" --help 2>/dev/null || true
