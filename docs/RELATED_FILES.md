# Related Files

## Application

- `app/index.html` and `app/styles.css`: controller window UI and styling.
- `app/help.html`, `app/help.css`, and `app/help-assets/`: bundled Help window content and screenshots.
- `app/main.js`: controller logic for active ruler editing, target polling, imports, shortcuts, and settings sync.
- `app/overlay.html`, `app/overlay.css`, and `app/overlay.js`: per-ruler overlay UI, resize affordances, drag/resize persistence, and visual modes.
- `src-tauri/src/lib.rs`: Rust backend for settings, overlay windows, menus, shortcuts, image storage, and target-window tracking.

## Packaging

- `src-tauri/tauri.conf.json`: Tauri app, window, bundle, icon, and DMG configuration.
- `script/check_macos_deps.sh`: macOS dependency checker/optional installer for Xcode Command Line Tools, Rust/Cargo, Node.js/npm, and the Intel Rust target.
- `script/check_windows_deps.ps1`: Windows dependency checker/optional installer for Node.js/npm, Rust/Cargo, Visual Studio Build Tools, and WebView2 Runtime.
- `script/build_and_run.sh`: development build/run helper.
- `script/build_macos_app.sh`: unsigned macOS `.app` and DMG packaging validation.
- `script/package_macos_local.sh`: unsigned local macOS `.app` zip/install helper for Apple Silicon, Intel, and universal builds when the matching Rust targets are installed.
- `script/package_windows_local.ps1`: unsigned Windows NSIS/MSI local packaging helper.
- `script/package_macos_distribution.sh`: Developer ID signing, hardened runtime, optional notarization, stapling, DMG creation, and checksum generation for macOS distribution.
- `.github/workflows/macos-distribution.yml`: manual GitHub Actions workflow for signed/notarized macOS release artifacts.
- `package.json` and `package-lock.json`: npm scripts and local Tauri CLI dependency.
- `src-tauri/Cargo.toml` and `src-tauri/Cargo.lock`: Rust package metadata and dependency lock.

## Icons

- `src-tauri/icons/icon-source.png`: 1024px source icon.
- `src-tauri/icons/icon.png`: generated 512px Tauri icon.
- `src-tauri/icons/icon.icns`: generated macOS icon bundle.
- `src-tauri/icons/icon.ico`: generated Windows icon.
- `src-tauri/icons/32x32.png`, `128x128.png`, and `128x128@2x.png`: generated bundle icons referenced by Tauri.

## Project Metadata

- `README.md`: project description, run/package instructions, implementation status, and limitations.
- `LICENSE`: MIT License.
- `.gitignore`: excludes generated build outputs and local dependencies.
