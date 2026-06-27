# Reading Ruler

Reading Ruler is a cross-platform desktop reading ruler overlay with multiple customizable rulers, window targeting, and macOS packaging.

Stage 6 targets macOS Apple Silicon packaging for the multi-ruler overlay prototype:

- control panel window
- multiple independent draggable translucent ruler overlays
- show/hide control
- default `Control+Alt+R` shortcut, when the global shortcut plugin can register it
- border thickness and color controls
- background color and transparency controls
- solid, dotted, striped, and grid pattern controls with adjustable spacing
- transparent and edge-only ruler body modes
- persisted image backgrounds imported from file or pasted from clipboard
- exact width, height, X, and Y fields
- direct on-screen resize handles on the overlay
- visible in-ruler grips and rail resize affordances
- whole-screen and targeted app/window modes
- targeted mode follows the selected window using saved user offsets
- active ruler selector with add, duplicate, rename, and delete controls
- per-ruler geometry, color, pattern, image, click-through, and target settings
- configurable show/hide shortcut
- macOS app menu actions to recover the control panel and show/hide the ruler
- click-through mode with a separate edit mode for moving/resizing
- reset-to-defaults
- basic monitor selection for placing the overlay on a display
- persisted settings restored on app restart
- unsigned local macOS `.app` and DMG packaging workflow
- custom generated Tauri icon set

Later stages add Developer ID signing/notarization, macOS Intel support, and Windows support.

Repository: <https://github.com/bilalalissa/Reading-Ruler.git>

License: MIT

## Run

```sh
./script/build_and_run.sh
```

Use `./script/build_and_run.sh --verify` to build, launch, and confirm that the process is running.

Settings are stored in the OS app config directory for `com.readingruler.prototype`.

## Package

```sh
npm run app:package:mac
```

The packaging script builds with Tauri, validates the generated app bundle, reports the executable architecture, checks code-signing status, and verifies any generated DMG.

Expected local artifacts:

- `src-tauri/target/release/bundle/macos/Reading Ruler.app`
- `src-tauri/target/release/bundle/dmg/*.dmg`

## Docs

- [Stage 5 implementation](docs/STAGE_5_IMPLEMENTATION.md)
- [Installation and packaging](docs/INSTALLATION.md)
- [Related files](docs/RELATED_FILES.md)

## Icons

The source icon is `src-tauri/icons/icon-source.png`. Regenerate the Tauri icon set with:

```sh
npx tauri icon src-tauri/icons/icon-source.png
```

## Stage 6 Limitations

- The control panel edits the active selected ruler only.
- The global shortcut toggles the active selected ruler only.
- Dragging is enabled by clicking and dragging the ruler itself, and the updated position is saved.
- Resizing is enabled by dragging the ruler's edge/corner handles or the visible grips/rail, and the updated size is saved.
- Targeted window mode is best-effort. If the selected window closes, minimizes, moves to another Space, or is not frontmost, the overlay hides and reports the target state.
- When an app has multiple windows, only the selected target window is tracked; other windows from the same app do not keep the overlay visible.
- In targeted mode, manual size and position edits are saved as offsets from the target window.
- Target window listing may be limited by macOS privacy protections.
- Clipboard image import depends on WebView/macOS clipboard access; normal paste in the control panel is also supported.
- Click-through disables direct overlay interaction until Edit overlay is turned back on in the control panel.
- Local unsigned packaging is available for Apple Silicon testing.
- Public distribution still needs Developer ID signing, hardened runtime, and notarization credentials.
