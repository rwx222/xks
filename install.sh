#!/bin/sh
set -e

if command -v curl >/dev/null 2>&1; then
    DOWNLOADER="curl -fsSL"
elif command -v wget >/dev/null 2>&1; then
    DOWNLOADER="wget -qO-"
else
    echo "Error: Neither 'curl' nor 'wget' is installed." >&2
    exit 1
fi

if ! command -v tar >/dev/null 2>&1; then
    echo "Error: 'tar' is not installed." >&2
    exit 1
fi

OS=$(uname -s | tr '[:upper:]' '[:lower:]')

ARCH=$(uname -m)
case $ARCH in
    arm64) ARCH="arm64" ;;              # apple silicon macos
    aarch64) ARCH="aarch64" ;;          # arm64 linux
    x86_64) ARCH="x86_64" ;;            # 64-bit (linux or macos)
    i386|i486|i586|i686) ARCH="i686" ;; # 32-bit linux
    *) echo "Unsupported architecture: $ARCH"; exit 1 ;;
esac

VERSION="v1.0.0"
FILENAME="xks-${VERSION}-${OS}-${ARCH}.tar.gz"
URL="https://github.com/rwx222/xks/releases/download/$VERSION/$FILENAME"

DEST_DIR="/usr/local/bin"

if [ ! -w "$DEST_DIR" ]; then
    echo "Error: No write permission for $DEST_DIR. Try running with 'sudo'." >&2
    exit 1
fi

echo "Downloading $URL"
$DOWNLOADER "$URL" | tar -xz -C "$DEST_DIR"

echo "Installation complete. Run 'xks help' to get started!"
