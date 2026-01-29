#!/bin/sh
set -e

get_arch() {
  case "$(uname -m)" in
    x86_64) echo "x64" ;;
    arm64|aarch64) echo "arm64" ;;
    *) echo "unsupported architecture" >&2; exit 1 ;;
  esac
}

get_os() {
  case "$(uname -s)" in
    Darwin) echo "macos" ;;
    Linux) echo "linux" ;;
    *) echo "unsupported os" >&2; exit 1 ;;
  esac
}

OS=$(get_os)
ARCH=$(get_arch)
NAME="f-${OS}-${ARCH}"
URL="https://github.com/crush/find/releases/latest/download/${NAME}"
DIR="${HOME}/.local/bin"

mkdir -p "$DIR"
curl -fsSL "$URL" -o "${DIR}/f"
chmod +x "${DIR}/f"

echo "installed to ${DIR}/f"
echo "add to path: export PATH=\"\$HOME/.local/bin:\$PATH\""
