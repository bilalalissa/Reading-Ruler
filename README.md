# Reading Ruler

Reading Ruler is a cross-platform desktop reading ruler overlay with multiple customizable rulers, window targeting, and macOS packaging.

It helps keep your place while reading dense text, documentation, spreadsheets, PDFs, browser pages, or side-by-side windows. Local unsigned install paths are provided for macOS Apple Silicon, macOS Intel, and Windows.

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
- Unsigned local `.app` zip/install packaging for macOS Apple Silicon, macOS Intel, and universal macOS builds.
- Unsigned Windows local installer packaging helper for NSIS or MSI builds.
- Local DMG packaging remains available for macOS testing, but DMGs are not the primary release path while Developer ID signing is unavailable.
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

Regular users should install already-built files from the GitHub release or the `Local Packaging` workflow artifacts. You do not need Rust, Node.js, Cargo, Xcode Command Line Tools, or Visual Studio Build Tools unless you are building from source.

### macOS Apple Silicon Install

Use this on M1, M2, M3, or newer Apple Silicon Macs.

Download from the GitHub release:

- [Reading.Ruler_0.1.0_aarch64.app.zip](https://github.com/bilalalissa/Reading-Ruler/releases/download/v0.1.0/Reading.Ruler_0.1.0_aarch64.app.zip)

Install from the downloaded file:

1. Unzip `Reading.Ruler_0.1.0_aarch64.app.zip`.
2. Move `Reading Ruler.app` to `Applications` or `~/Applications`.
3. Open `Reading Ruler.app`.
4. If macOS blocks the unsigned app, Control-click the app, choose `Open`, then confirm. For local testing you can also remove quarantine:

```sh
xattr -dr com.apple.quarantine "$HOME/Applications/Reading Ruler.app"
```

For source builds and developer runs, see [Development](docs/DEVELOPMENT.md).

### macOS Intel Install

Use this on Intel Macs.

No Intel release file is published yet. Use the GitHub Actions `Local Packaging` workflow, choose `macos` and `x64`, then download the generated `.app.zip` artifact. Unzip it, move `Reading Ruler.app` to `Applications` or `~/Applications`, and open it.

### Universal macOS Install

Use this when one local app bundle should run on both Apple Silicon and Intel Macs.

No universal release file is published yet. Use the GitHub Actions `Local Packaging` workflow, choose `macos` and `universal`, then download the generated `.app.zip` artifact. Unzip it, move `Reading Ruler.app` to `Applications` or `~/Applications`, and open it.

### Windows Install

Use this on Windows for a local unsigned installer.

No Windows release file is published yet. Use the GitHub Actions `Local Packaging` workflow, choose `windows`, choose `nsis` or `msi`, then download the generated installer artifact. Run the `.exe` or `.msi` installer. If Windows SmartScreen warns that the installer is unsigned, choose the local/internal install option to continue.

### Available Installation Files

Download from the current GitHub release:

- [Reading.Ruler_0.1.0_aarch64.app.zip](https://github.com/bilalalissa/Reading-Ruler/releases/download/v0.1.0/Reading.Ruler_0.1.0_aarch64.app.zip)
- [Reading.Ruler_0.1.0_aarch64.sha256](https://github.com/bilalalissa/Reading-Ruler/releases/download/v0.1.0/Reading.Ruler_0.1.0_aarch64.sha256)

For Apple Silicon, download `Reading.Ruler_0.1.0_aarch64.app.zip`. The checksum file is optional and is used to verify the download. DMGs are kept as local test artifacts until Developer ID signing is available.

The manual GitHub Actions workflow `Local Packaging` can also build unsigned local macOS `.app.zip` artifacts and unsigned Windows NSIS/MSI installer artifacts without Apple Developer ID credentials.

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
- native target-window listing on macOS and Windows, including process/window labels where available
- direct drag/resize behavior with persistence
- menu and shortcut control for the active ruler
- settings migration and reset support
- bundled Help menu with feature explanations and screenshots

Details: [Multi-ruler implementation](docs/MULTI_RULER_IMPLEMENTATION.md)

### Local Platform Packaging

Unsigned local platform packaging is implemented:

- manual GitHub Actions workflow `Local Packaging` for unsigned macOS and Windows artifacts
- Tauri bundling is enabled for macOS `.app` and local DMG targets.
- A generated app icon set is included.
- `script/build_macos_app.sh` builds and validates the package.
- The generated executable is verified as `arm64`.
- DMG verification is performed with `hdiutil verify`.

Details: [Installation](docs/INSTALLATION.md) and [Development](docs/DEVELOPMENT.md)

### Signed Distribution Packaging

Signed macOS distribution packaging is implemented:

- requires `APPLE_SIGNING_IDENTITY` set to a `Developer ID Application` certificate
- signs with hardened runtime and timestamp
- verifies signatures with `codesign` and Gatekeeper assessment with `spctl`
- optionally notarizes and staples the app and DMG through `xcrun notarytool`
- creates release-ready DMG, app zip, and SHA-256 checksum files
- includes a manual GitHub Actions workflow that can upload signed/notarized artifacts to a release

Details: [Development](docs/DEVELOPMENT.md)

## Icons

The source icon is `src-tauri/icons/icon-source.png`. Regenerate the Tauri icon set with:

```sh
npx tauri icon src-tauri/icons/icon-source.png
```

The generated files include `icon.icns`, `icon.ico`, bundle PNGs, and platform icon assets.

## Related Files

See [Related files](docs/RELATED_FILES.md) for the main controller, overlay, backend, packaging, icon, and metadata files.

Developer-only run instructions are in [Development](docs/DEVELOPMENT.md).

## Current Limitations

- macOS Apple Silicon local packaging is verified on this machine. Intel and universal macOS packaging require installing the `x86_64-apple-darwin` Rust target. Windows packaging must be run on Windows.
- Targeted window mode is best-effort. If the selected window closes, minimizes, moves to another Space, or is not frontmost, the overlay hides and reports the target state.
- Target window listing may be limited by macOS privacy protections or by Windows apps that do not expose normal top-level window titles.
- Clipboard image import depends on WebView/macOS clipboard access; normal paste in the control panel is also supported.
- Click-through disables direct overlay interaction until edit mode is enabled again.
- Public distribution requires a Developer ID certificate and notarization profile; local install paths do not.
- Public signed installers and auto-update can be added after Developer ID signing is available.
