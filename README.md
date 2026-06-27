# Reading Ruler

Reading Ruler is a cross-platform desktop reading ruler overlay with multiple customizable rulers, window targeting, and macOS packaging.

It helps keep your place while reading dense text, documentation, spreadsheets, PDFs, browser pages, or side-by-side windows. The current implementation targets macOS Apple Silicon first.

![Reading Ruler whole-screen overlay scenario](docs/screenshots/whole-screen-reading.png)

Repository: <https://github.com/bilalalissa/Reading-Ruler.git>

License: MIT

## Features

- Multiple independent rulers, each with its own geometry, style, target, image, click-through state, and visibility.
- Active-ruler controller with add, duplicate, rename, delete, select, show, and hide controls.
- Direct on-screen movement and resizing by dragging the ruler body, edges, or corners.
- Whole-screen mode for a selected/saved display position.
- Targeted app/window mode that tracks the selected exact window with saved offsets.
- Style controls for border thickness/color, background color, opacity, pattern spacing, and body mode.
- Body modes: solid, dotted, striped, grid, transparent, edge-only variants, and image background.
- Image backgrounds from imported files or clipboard paste.
- Click-through mode with a separate edit mode so the ruler can stop intercepting mouse input during reading.
- Global shortcut, default `Control+Alt+R`, for toggling the active ruler.
- macOS app menu actions for recovering the control panel and showing, hiding, toggling, or resetting the active ruler.
- Settings are persisted in the OS user config directory and restored on launch.
- Unsigned local `.app` and DMG packaging for macOS Apple Silicon testing.
- Developer ID distribution packaging workflow with hardened runtime, optional notarization, stapling, and checksum generation.

## Screenshots

### Control Panel

Use the control panel to choose the active ruler, edit its properties, target a window, import/paste images, configure click-through, and package the same settings across restarts.

![Reading Ruler control panel](docs/screenshots/control-panel.png)

### Whole-Screen Reading

Use whole-screen mode when you want a ruler that floats over the current display and stays where you place it.

![Whole-screen reading ruler scenario](docs/screenshots/whole-screen-reading.png)

### Multiple Rulers

Use multiple rulers when you want separate overlays for different regions, windows, monitors, colors, or reading tasks.

![Multiple independent rulers scenario](docs/screenshots/multiple-rulers.png)

## Install

### Requirements

- macOS on Apple Silicon for the current packaged build path.
- Rust/Cargo.
- Node.js and npm.
- Xcode Command Line Tools.

### Development Run

```sh
npm install
./script/build_and_run.sh
```

Use verification mode to build, launch, and confirm the process stays running:

```sh
./script/build_and_run.sh --verify
```

### Unsigned macOS App And DMG

```sh
npm run app:package:mac
```

Expected local artifacts:

- `src-tauri/target/release/bundle/macos/Reading Ruler.app`
- `src-tauri/target/release/bundle/dmg/*.dmg`

The packaging script validates the generated app bundle, reports executable architecture, checks code-signing status, and verifies generated DMGs. Local unsigned builds are suitable for development and local testing.

Public distribution still needs Developer ID signing, hardened runtime, and notarization credentials.

### Signed macOS Distribution Package

Public macOS distribution uses a separate packaging script so local unsigned testing stays simple:

```sh
npm run app:package:mac:distribution -- --check-prereqs
```

```sh
APPLE_SIGNING_IDENTITY="Developer ID Application: Your Name (TEAMID)" \
  npm run app:package:mac:distribution
```

To notarize and staple the app and DMG, first store an Apple notarization profile:

```sh
xcrun notarytool store-credentials reading-ruler-notary
```

Then run:

```sh
APPLE_SIGNING_IDENTITY="Developer ID Application: Your Name (TEAMID)" \
  npm run app:package:mac:distribution -- --notarize reading-ruler-notary
```

Distribution outputs:

- `src-tauri/target/release/bundle/macos/Reading Ruler_0.1.0_arm64.app.zip`
- `src-tauri/target/release/bundle/dmg/Reading Ruler_0.1.0_arm64.dmg`
- `src-tauri/target/release/bundle/Reading Ruler_0.1.0_arm64.sha256`

This machine currently has an `Apple Development` certificate only. A `Developer ID Application` certificate is required before this script can produce a public Gatekeeper-clean build.

### Available Installation Files

The latest Apple Silicon installation files are attached to the `v0.1.0` GitHub release:

- [Reading.Ruler_0.1.0_aarch64.dmg](https://github.com/bilalalissa/Reading-Ruler/releases/download/v0.1.0/Reading.Ruler_0.1.0_aarch64.dmg)
- [Reading.Ruler_0.1.0_aarch64.app.zip](https://github.com/bilalalissa/Reading-Ruler/releases/download/v0.1.0/Reading.Ruler_0.1.0_aarch64.app.zip)
- [Reading.Ruler_0.1.0_aarch64.sha256](https://github.com/bilalalissa/Reading-Ruler/releases/download/v0.1.0/Reading.Ruler_0.1.0_aarch64.sha256)

After `npm run app:package:mac`, the same local installation files are available at:

- `src-tauri/target/release/bundle/macos/Reading Ruler.app`
- `src-tauri/target/release/bundle/dmg/Reading Ruler_0.1.0_aarch64.dmg`

Use the DMG for drag-and-drop installation testing. Use the `.app` bundle or `.app.zip` for direct launch testing. The current package is unsigned/ad-hoc; public distribution still needs Developer ID signing and notarization.

## How To Use

### Scenario: Read A Long Article Or PDF

1. Open Reading Ruler.
2. Keep `Mode` set to `Whole screen`.
3. Click `Show ruler`.
4. Drag the ruler body over the line or paragraph you are reading.
5. Drag an edge or corner to resize it.
6. Adjust background opacity, body mode, and pattern until the text remains readable.

### Scenario: Track One Browser Or App Window

1. Set `Mode` to `Target app/window`.
2. Click `Refresh targets`.
3. Pick the exact target window from the target list.
4. Show the ruler and position it over the reading area.
5. Move or resize the target window. The ruler follows using the saved offsets.

Targeting is exact-window based. If an app has multiple windows, only the selected window should keep the ruler active.

### Scenario: Use Multiple Rulers

1. Click `Add` to create a second ruler, or `Duplicate` to copy the active ruler.
2. Select a ruler in the `Selected ruler` dropdown.
3. Change size, position, style, image, target, or visibility.
4. Switch back to another ruler. Its saved properties are restored without changing the other rulers.

The global shortcut toggles only the active selected ruler.

### Scenario: Read Without Blocking Mouse Clicks

1. Turn on `Click-through`.
2. Turn off `Edit overlay` while reading so mouse input passes through the ruler.
3. Turn `Edit overlay` back on when you need to drag or resize the ruler again.

### Scenario: Use A Custom Image Background

1. Choose `Image` as the body mode, or import/paste an image.
2. Use `Import image` for a file or `Paste image from clipboard` for clipboard content.
3. The image is copied into the app config directory and restored on restart.
4. Use `Clear image` to return to non-image body modes.

## Implementation Status

### Multi-Ruler Overlay

The macOS Apple Silicon multi-ruler overlay is implemented:

- independent ruler settings and overlay windows
- active-ruler controller behavior
- per-ruler geometry, style, image, target, click-through, edit mode, and visibility
- exact-window targeted mode with offsets
- direct drag/resize behavior with persistence
- menu and shortcut control for the active ruler
- settings migration and reset support
- bundled Help menu with feature explanations and screenshots

Details: [Multi-ruler implementation](docs/MULTI_RULER_IMPLEMENTATION.md)

### Local Apple Silicon Packaging

Unsigned local macOS Apple Silicon packaging is implemented:

- Tauri bundling is enabled for `.app` and DMG targets.
- A generated app icon set is included.
- `script/build_macos_app.sh` builds and validates the package.
- The generated executable is verified as `arm64`.
- DMG verification is performed with `hdiutil verify`.

Details: [Installation and packaging](docs/INSTALLATION.md)

### Signed Distribution Packaging

Signed macOS distribution packaging is implemented:

- `npm run app:package:mac:distribution`
- preflight check with `npm run app:package:mac:distribution -- --check-prereqs`
- requires `APPLE_SIGNING_IDENTITY` set to a `Developer ID Application` certificate
- signs with hardened runtime and timestamp
- verifies signatures with `codesign` and Gatekeeper assessment with `spctl`
- optionally notarizes and staples the app and DMG through `xcrun notarytool`
- creates release-ready DMG, app zip, and SHA-256 checksum files

Details: [Installation and packaging](docs/INSTALLATION.md)

## Icons

The source icon is `src-tauri/icons/icon-source.png`. Regenerate the Tauri icon set with:

```sh
npx tauri icon src-tauri/icons/icon-source.png
```

The generated files include `icon.icns`, `icon.ico`, bundle PNGs, and platform icon assets.

## Related Files

See [Related files](docs/RELATED_FILES.md) for the main controller, overlay, backend, packaging, icon, and metadata files.

## Current Limitations

- macOS Apple Silicon is the only verified package target.
- Targeted window mode is best-effort. If the selected window closes, minimizes, moves to another Space, or is not frontmost, the overlay hides and reports the target state.
- Target window listing may be limited by macOS privacy protections.
- Clipboard image import depends on WebView/macOS clipboard access; normal paste in the control panel is also supported.
- Click-through disables direct overlay interaction until edit mode is enabled again.
- Public distribution requires a Developer ID certificate and notarization profile; this machine currently has an Apple Development certificate only.
- Future work can add macOS Intel and Windows installers.
