#!/usr/bin/env bash
set -euo pipefail

INSTALL=false
ADD_INTEL_TARGET=false

usage() {
  cat <<'USAGE'
Usage:
  ./script/check_macos_deps.sh [--install] [--with-intel-target]

Checks macOS build dependencies for Reading Ruler:
  - Xcode Command Line Tools
  - Rust/Cargo
  - Node.js/npm
  - optional x86_64-apple-darwin Rust target for Intel/universal builds

Options:
  --install            Try to install missing tools.
                       Uses xcode-select for Command Line Tools.
                       Uses Homebrew for node and rustup when brew exists.
  --with-intel-target  Also check/install x86_64-apple-darwin.
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --help|-h)
      usage
      exit 0
      ;;
    --install)
      INSTALL=true
      shift
      ;;
    --with-intel-target)
      ADD_INTEL_TARGET=true
      shift
      ;;
    *)
      echo "Unknown argument: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if [[ "$(uname -s)" != "Darwin" ]]; then
  echo "This script must run on macOS." >&2
  exit 2
fi

missing=()

has_command() {
  command -v "$1" >/dev/null 2>&1
}

check_xcode_tools() {
  if xcode-select -p >/dev/null 2>&1 && has_command clang; then
    echo "ok: Xcode Command Line Tools"
  else
    echo "missing: Xcode Command Line Tools"
    missing+=("xcode-tools")
  fi
}

check_node() {
  if has_command node && has_command npm; then
    echo "ok: Node.js $(node --version), npm $(npm --version)"
  else
    echo "missing: Node.js/npm"
    missing+=("node")
  fi
}

check_rust() {
  if has_command rustc && has_command cargo && has_command rustup; then
    echo "ok: rustc $(rustc --version), cargo $(cargo --version)"
  else
    echo "missing: Rust/Cargo/rustup"
    missing+=("rust")
  fi
}

check_intel_target() {
  if ! has_command rustup; then
    echo "skip: x86_64-apple-darwin target check needs rustup"
    return
  fi

  if rustup target list --installed | grep -Fx "x86_64-apple-darwin" >/dev/null; then
    echo "ok: Rust target x86_64-apple-darwin"
  else
    echo "missing: Rust target x86_64-apple-darwin"
    missing+=("intel-target")
  fi
}

install_missing() {
  local item
  for item in "${missing[@]}"; do
    case "$item" in
      xcode-tools)
        echo "Installing Xcode Command Line Tools..."
        xcode-select --install || true
        echo "If an installer window opened, finish it before rerunning this script."
        ;;
      node)
        if has_command brew; then
          echo "Installing Node.js with Homebrew..."
          brew install node
        else
          echo "Homebrew is not installed. Install Node.js from https://nodejs.org/ or install Homebrew, then rerun."
        fi
        ;;
      rust)
        if has_command brew; then
          echo "Installing rustup with Homebrew..."
          brew install rustup-init
          rustup-init -y
          echo "Restart Terminal or run: source \"$HOME/.cargo/env\""
        else
          echo "Homebrew is not installed. Install Rust with:"
          echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        fi
        ;;
      intel-target)
        echo "Installing Rust target x86_64-apple-darwin..."
        rustup target add x86_64-apple-darwin
        ;;
      *)
        echo "Unknown missing dependency: $item" >&2
        ;;
    esac
  done
}

echo "Checking macOS dependencies..."
check_xcode_tools
check_node
check_rust
if [[ "$ADD_INTEL_TARGET" == true ]]; then
  check_intel_target
fi

if [[ "${#missing[@]}" -eq 0 ]]; then
  echo "All requested macOS dependencies are installed."
  exit 0
fi

echo
echo "Missing dependencies: ${missing[*]}"

if [[ "$INSTALL" == true ]]; then
  install_missing
  echo
  suffix=""
  if [[ "$ADD_INTEL_TARGET" == true ]]; then
    suffix=" --with-intel-target"
  fi
  echo "Rerun this script after installers finish:"
  echo "  ./script/check_macos_deps.sh${suffix}"
else
  suffix=""
  if [[ "$ADD_INTEL_TARGET" == true ]]; then
    suffix=" --with-intel-target"
  fi
  echo "To try automatic install, run:"
  echo "  ./script/check_macos_deps.sh --install${suffix}"
fi

exit 1
