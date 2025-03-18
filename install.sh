#!/bin/sh
set -e
set -o pipefail

OS=$(uname -s | tr '[:upper:]' '[:lower:]')

ARCH=$(uname -m)
case $ARCH in
    arm64) ARCH="arm64" ;;              # arm64 macos (apple silicon)
    aarch64) ARCH="aarch64" ;;          # arm64 linux
    x86_64) ARCH="x86_64" ;;            # 64-bit (linux or macos)
    i386|i486|i586|i686) ARCH="i686" ;; # 32-bit linux
    *) echo "Unsupported architecture: $ARCH" >&2; exit 1 ;;
esac

VERSION="v1.0.0"
FILENAME="xks-${VERSION}-${OS}-${ARCH}.tar.gz"
URL="https://github.com/rwx222/xks/releases/download/$VERSION/$FILENAME"

if command -v curl >/dev/null 2>&1; then
    DOWNLOADER="curl -fsSL"
elif command -v wget >/dev/null 2>&1; then
    DOWNLOADER="wget -qO-"
else
    echo "URL: $URL\nError: Neither 'curl' nor 'wget' is installed." >&2
    exit 1
fi

if ! command -v tar >/dev/null 2>&1; then
    echo "URL: $URL\nError: 'tar' is not installed." >&2
    exit 1
fi

DEST_DIR="/usr/local/bin"

if [ ! -w "$DEST_DIR" ]; then
    echo "URL: $URL\nError: No write permission for $DEST_DIR.\nTry running with 'sudo' or as root user." >&2
    exit 1
fi

echo "Downloading $URL"
if ! $DOWNLOADER "$URL" | tar -xz -C "$DEST_DIR"; then
    echo "Error: Download or extraction failed." >&2
    exit 1
fi

echo "Installation complete!\nRun 'xks help' to get started."
