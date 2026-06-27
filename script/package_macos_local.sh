#!/usr/bin/env bash
set -euo pipefail

APP_NAME="Reading Ruler"
VERSION="$(node -p "require('./package.json').version")"
TARGET="aarch64-apple-darwin"
INSTALL=false
INSTALL_DIR="${HOME}/Applications"

usage() {
  cat <<'USAGE'
Usage:
  ./script/package_macos_local.sh [--target arm64|x64|universal] [--install] [--install-dir DIR]

Builds an unsigned local macOS .app bundle and .app.zip without Developer ID,
notarization, or a public DMG.

Targets:
  arm64      Apple Silicon, requires rust target aarch64-apple-darwin
  x64        Intel Mac, requires rust target x86_64-apple-darwin
  universal  Universal macOS app, requires both arm64 and x64 Rust targets

Outputs:
  src-tauri/target/<target>/release/bundle/macos/Reading Ruler.app
  src-tauri/target/<target>/release/bundle/macos/Reading Ruler_<version>_<target>.app.zip
  src-tauri/target/<target>/release/bundle/Reading Ruler_<version>_<target>.sha256

Install:
  --install copies the app to ~/Applications by default and removes quarantine
  from that local copy when xattr is available.
USAGE
}

target_label() {
  case "$TARGET" in
    aarch64-apple-darwin) echo "arm64" ;;
    x86_64-apple-darwin) echo "x64" ;;
    universal-apple-darwin) echo "universal" ;;
    *) echo "$TARGET" ;;
  esac
}

target_dir() {
  echo "src-tauri/target/${TARGET}/release/bundle"
}

require_rust_target() {
  local rust_target="$1"
  if ! rustup target list --installed | grep -Fx "$rust_target" >/dev/null; then
    echo "Missing Rust target: $rust_target" >&2
    echo "Install it with: rustup target add $rust_target" >&2
    exit 2
  fi
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --help|-h)
      usage
      exit 0
      ;;
    --target)
      if [[ $# -lt 2 ]]; then
        echo "--target requires arm64, x64, or universal." >&2
        exit 2
      fi
      case "$2" in
        arm64|aarch64|aarch64-apple-darwin)
          TARGET="aarch64-apple-darwin"
          ;;
        x64|x86_64|intel|x86_64-apple-darwin)
          TARGET="x86_64-apple-darwin"
          ;;
        universal|universal-apple-darwin)
          TARGET="universal-apple-darwin"
          ;;
        *)
          echo "Unknown macOS target: $2" >&2
          usage >&2
          exit 2
          ;;
      esac
      shift 2
      ;;
    --install)
      INSTALL=true
      shift
      ;;
    --install-dir)
      if [[ $# -lt 2 ]]; then
        echo "--install-dir requires a directory." >&2
        exit 2
      fi
      INSTALL_DIR="$2"
      shift 2
      ;;
    *)
      echo "Unknown argument: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if [[ "$(uname -s)" != "Darwin" ]]; then
  echo "macOS local packaging must run on macOS." >&2
  exit 2
fi

if [[ "$TARGET" == "universal-apple-darwin" ]]; then
  require_rust_target "aarch64-apple-darwin"
  require_rust_target "x86_64-apple-darwin"
else
  require_rust_target "$TARGET"
fi

LABEL="$(target_label)"
BUNDLE_DIR="$(target_dir)"
MACOS_DIR="${BUNDLE_DIR}/macos"
APP_PATH="${MACOS_DIR}/${APP_NAME}.app"
EXECUTABLE_PATH="${APP_PATH}/Contents/MacOS/reading-ruler"
INFO_PLIST="${APP_PATH}/Contents/Info.plist"
ZIP_PATH="${MACOS_DIR}/Reading Ruler_${VERSION}_${LABEL}.app.zip"
CHECKSUM_PATH="${BUNDLE_DIR}/Reading Ruler_${VERSION}_${LABEL}.sha256"

npx tauri build --target "$TARGET" --bundles app --no-sign

if [[ ! -d "$APP_PATH" ]]; then
  echo "Expected app bundle was not created: $APP_PATH" >&2
  exit 1
fi

if [[ ! -x "$EXECUTABLE_PATH" ]]; then
  echo "Expected executable is missing or not executable: $EXECUTABLE_PATH" >&2
  exit 1
fi

if [[ ! -f "$INFO_PLIST" ]]; then
  echo "Expected Info.plist is missing: $INFO_PLIST" >&2
  exit 1
fi

plutil -lint "$INFO_PLIST"
echo "Executable architecture: $(lipo -archs "$EXECUTABLE_PATH")"

ditto -c -k --sequesterRsrc --keepParent "$APP_PATH" "$ZIP_PATH"
shasum -a 256 "$ZIP_PATH" > "$CHECKSUM_PATH"

if [[ "$INSTALL" == true ]]; then
  mkdir -p "$INSTALL_DIR"
  INSTALL_PATH="${INSTALL_DIR}/${APP_NAME}.app"
  rm -rf "$INSTALL_PATH"
  ditto "$APP_PATH" "$INSTALL_PATH"
  if command -v xattr >/dev/null 2>&1; then
    xattr -dr com.apple.quarantine "$INSTALL_PATH" 2>/dev/null || true
  fi
  echo "Installed local unsigned app: $INSTALL_PATH"
fi

echo "Local macOS artifacts:"
echo "  $APP_PATH"
echo "  $ZIP_PATH"
echo "  $CHECKSUM_PATH"
