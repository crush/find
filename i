#!/bin/sh
set -e

get_arch() {
  case "$(uname -m)" in
    x86_64) echo "x64" ;;
    arm64|aarch64) echo "arm64" ;;
    *) exit 1 ;;
  esac
}

get_os() {
  case "$(uname -s)" in
    Darwin) echo "macos" ;;
    Linux) echo "linux" ;;
    *) exit 1 ;;
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

ZSH_HOOK='
function f() {
  local dir
  dir=$("$HOME/.local/bin/f" "$@")
  [ -n "$dir" ] && cd "$dir" && "$HOME/.local/bin/f" boost "$dir" 2>/dev/null
}
function fb() {
  local dir
  dir=$("$HOME/.local/bin/f" back "$@")
  [ -n "$dir" ] && cd "$dir"
}'

BASH_HOOK='
function f() {
  local dir
  dir=$("$HOME/.local/bin/f" "$@")
  [ -n "$dir" ] && cd "$dir" && "$HOME/.local/bin/f" boost "$dir" 2>/dev/null
}
function fb() {
  local dir
  dir=$("$HOME/.local/bin/f" back "$@")
  [ -n "$dir" ] && cd "$dir"
}'

FISH_HOOK='
function f
  set -l dir ("$HOME/.local/bin/f" $argv)
  if test -n "$dir"
    cd "$dir"
    "$HOME/.local/bin/f" boost "$dir" 2>/dev/null
  end
end
function fb
  set -l dir ("$HOME/.local/bin/f" back $argv)
  if test -n "$dir"
    cd "$dir"
  end
end'

add_to_shell() {
  [ -f "$1" ] || return 0
  grep -v '\.local/bin/f' "$1" > "$1.tmp" 2>/dev/null || true
  mv "$1.tmp" "$1"
  echo "$2" >> "$1"
}

add_to_shell "$HOME/.zshrc" "$ZSH_HOOK"
add_to_shell "$HOME/.bashrc" "$BASH_HOOK"

mkdir -p "$HOME/.config/fish/functions"
echo "$FISH_HOOK" > "$HOME/.config/fish/functions/f.fish"

if ! echo "$PATH" | grep -q "$HOME/.local/bin"; then
  for rc in "$HOME/.zshrc" "$HOME/.bashrc"; do
    if [ -f "$rc" ] && ! grep -q 'export PATH=.*\.local/bin' "$rc"; then
      echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$rc"
    fi
  done
  if [ -f "$HOME/.config/fish/config.fish" ]; then
    if ! grep -q '\.local/bin' "$HOME/.config/fish/config.fish"; then
      echo 'fish_add_path "$HOME/.local/bin"' >> "$HOME/.config/fish/config.fish"
    fi
  fi
fi

echo "done"
