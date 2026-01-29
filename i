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

SHELL_FUNC='function f() { local dir; dir=$("$HOME/.local/bin/f" "$@"); [ -n "$dir" ] && cd "$dir"; }'

add_to_shell() {
  if [ -f "$1" ]; then
    if ! grep -q '/.local/bin/f' "$1" 2>/dev/null; then
      echo "" >> "$1"
      echo "$SHELL_FUNC" >> "$1"
    fi
  fi
}

add_to_shell "$HOME/.zshrc"
add_to_shell "$HOME/.bashrc"

if ! echo "$PATH" | grep -q "$HOME/.local/bin"; then
  for rc in "$HOME/.zshrc" "$HOME/.bashrc"; do
    if [ -f "$rc" ] && ! grep -q 'export PATH=.*\.local/bin' "$rc"; then
      echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$rc"
    fi
  done
fi

echo "done. restart shell or: source ~/.zshrc"
