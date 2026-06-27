#!/usr/bin/env bash
set -euo pipefail

APP_NAME="Reading Ruler"
APP_PATH="src-tauri/target/release/bundle/macos/${APP_NAME}.app"
EXECUTABLE_PATH="${APP_PATH}/Contents/MacOS/reading-ruler"
INFO_PLIST="${APP_PATH}/Contents/Info.plist"
BUNDLE_DIR="src-tauri/target/release/bundle"
DMG_DIR="${BUNDLE_DIR}/dmg"
MACOS_DIR="${BUNDLE_DIR}/macos"
VERSION="$(node -p "require('./package.json').version")"
ARCH="$(uname -m)"
ZIP_PATH="${MACOS_DIR}/Reading Ruler_${VERSION}_${ARCH}.app.zip"
DMG_PATH="${DMG_DIR}/Reading Ruler_${VERSION}_${ARCH}.dmg"
CHECKSUM_PATH="${BUNDLE_DIR}/Reading Ruler_${VERSION}_${ARCH}.sha256"

usage() {
  cat <<'USAGE'
Usage:
  APPLE_SIGNING_IDENTITY="Developer ID Application: Name (TEAMID)" \
    ./script/package_macos_distribution.sh [--notarize KEYCHAIN_PROFILE]

Required:
  APPLE_SIGNING_IDENTITY  Developer ID Application identity for public distribution.

Optional:
  --notarize PROFILE      Submit both the app zip and DMG with xcrun notarytool.
                          PROFILE must already exist in the keychain:
                          xcrun notarytool store-credentials PROFILE

Outputs:
  src-tauri/target/release/bundle/macos/Reading Ruler_<version>_<arch>.app.zip
  src-tauri/target/release/bundle/dmg/Reading Ruler_<version>_<arch>.dmg
  src-tauri/target/release/bundle/Reading Ruler_<version>_<arch>.sha256
USAGE
}

NOTARY_PROFILE=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --help|-h)
      usage
      exit 0
      ;;
    --notarize)
      if [[ $# -lt 2 ]]; then
        echo "--notarize requires a keychain profile name." >&2
        exit 2
      fi
      NOTARY_PROFILE="$2"
      shift 2
      ;;
    *)
      echo "Unknown argument: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if [[ -z "${APPLE_SIGNING_IDENTITY:-}" ]]; then
  echo "APPLE_SIGNING_IDENTITY is required for distribution packaging." >&2
  echo "Expected a Developer ID Application certificate, not an Apple Development certificate." >&2
  exit 2
fi

if [[ "$APPLE_SIGNING_IDENTITY" != Developer\ ID\ Application:* ]]; then
  echo "APPLE_SIGNING_IDENTITY must be a Developer ID Application identity for public distribution." >&2
  echo "Received: $APPLE_SIGNING_IDENTITY" >&2
  exit 2
fi

if ! security find-identity -v -p codesigning | grep -F "$APPLE_SIGNING_IDENTITY" >/dev/null; then
  echo "Signing identity was not found in the keychain: $APPLE_SIGNING_IDENTITY" >&2
  exit 1
fi

npm run app:build

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

echo "Signing app bundle with hardened runtime: $APP_PATH"
codesign \
  --force \
  --deep \
  --options runtime \
  --timestamp \
  --sign "$APPLE_SIGNING_IDENTITY" \
  "$APP_PATH"

codesign --verify --deep --strict --verbose=2 "$APP_PATH"
if spctl --assess --type execute --verbose=4 "$APP_PATH"; then
  echo "Gatekeeper assessment passed before notarization."
else
  echo "Gatekeeper assessment did not pass before notarization; this is expected for unsigned-notarization workflows."
fi

mkdir -p "$MACOS_DIR" "$DMG_DIR"
ditto -c -k --sequesterRsrc --keepParent "$APP_PATH" "$ZIP_PATH"

if [[ -n "$NOTARY_PROFILE" ]]; then
  echo "Submitting app zip for notarization: $ZIP_PATH"
  xcrun notarytool submit "$ZIP_PATH" --keychain-profile "$NOTARY_PROFILE" --wait
  xcrun stapler staple "$APP_PATH"
  xcrun stapler validate "$APP_PATH"
  spctl --assess --type execute --verbose=4 "$APP_PATH"
  ditto -c -k --sequesterRsrc --keepParent "$APP_PATH" "$ZIP_PATH"
fi

echo "Creating signed DMG: $DMG_PATH"
hdiutil create \
  -volname "$APP_NAME" \
  -srcfolder "$APP_PATH" \
  -ov \
  -format UDZO \
  "$DMG_PATH"

codesign --force --timestamp --sign "$APPLE_SIGNING_IDENTITY" "$DMG_PATH"
codesign --verify --verbose=2 "$DMG_PATH"
hdiutil verify "$DMG_PATH"

if [[ -n "$NOTARY_PROFILE" ]]; then
  echo "Submitting DMG for notarization: $DMG_PATH"
  xcrun notarytool submit "$DMG_PATH" --keychain-profile "$NOTARY_PROFILE" --wait
  xcrun stapler staple "$DMG_PATH"
  xcrun stapler validate "$DMG_PATH"
  spctl --assess --type open --context context:primary-signature --verbose=4 "$DMG_PATH"
fi

shasum -a 256 "$DMG_PATH" "$ZIP_PATH" > "$CHECKSUM_PATH"

echo "Distribution artifacts:"
echo "  $DMG_PATH"
echo "  $ZIP_PATH"
echo "  $CHECKSUM_PATH"
