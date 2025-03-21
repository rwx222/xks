#!/bin/sh
set -e

(set -o pipefail) 2>/dev/null && set -o pipefail

OS=$(uname -s | tr '[:upper:]' '[:lower:]')
[ "$OS" = "darwin" ] && OS="macos"

ARCH=$(uname -m)

case $ARCH in
    arm64) ARCH="arm64" ;;              # arm64 macos (apple silicon)
    aarch64) ARCH="arm64" ;;            # arm64 linux
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
    echo "URL: $URL" >&2
    echo "Error: Neither 'curl' nor 'wget' is installed." >&2
    exit 1
fi

if ! command -v tar >/dev/null 2>&1; then
    echo "URL: $URL" >&2
    echo "Error: 'tar' is not installed." >&2
    exit 1
fi

DEST_DIR="/usr/local/bin"

if [ ! -w "$DEST_DIR" ]; then
    echo "URL: $URL" >&2
    echo "Error: No write permission for $DEST_DIR." >&2
    echo "Try running with 'sudo' or as root user." >&2
    exit 1
fi

echo "Downloading $URL"

if ! $DOWNLOADER "$URL" | tar -xz -C "$DEST_DIR"; then
    echo "Error: Download or extraction failed." >&2
    exit 1
else
    echo "Installation complete!"
    echo "Run 'xks help' to get started."
fi
