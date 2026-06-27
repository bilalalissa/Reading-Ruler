#!/usr/bin/env bash
set -euo pipefail

APP_NAME="Reading Ruler"
APP_PATH="src-tauri/target/release/bundle/macos/${APP_NAME}.app"
DMG_DIR="src-tauri/target/release/bundle/dmg"
EXECUTABLE_PATH="${APP_PATH}/Contents/MacOS/reading-ruler"
INFO_PLIST="${APP_PATH}/Contents/Info.plist"

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

echo "App bundle: $APP_PATH"
echo "Executable architecture: $(lipo -archs "$EXECUTABLE_PATH")"

if codesign --verify --deep --strict "$APP_PATH" >/dev/null 2>&1; then
  echo "Code signing: valid"
else
  echo "Code signing: unsigned or ad-hoc only; local testing is OK, distribution still needs Developer ID signing and notarization."
fi

if [[ -d "$DMG_DIR" ]]; then
  found_dmg=false
  while IFS= read -r dmg_path; do
    found_dmg=true
    echo "DMG artifact: $dmg_path"
    hdiutil verify "$dmg_path"
  done < <(find "$DMG_DIR" -maxdepth 1 -name "*.dmg" -type f | sort)

  if [[ "$found_dmg" == false ]]; then
    echo "No DMG artifact found in $DMG_DIR."
  fi
else
  echo "No DMG artifact directory found: $DMG_DIR"
fi
