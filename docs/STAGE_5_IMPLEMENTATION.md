# Stage 5 Implementation

Reading Ruler is a cross-platform desktop reading ruler overlay with multiple customizable rulers, window targeting, and macOS packaging.

Stage 5 focuses on the macOS Apple Silicon multi-ruler prototype: each ruler owns its own geometry, colors, body mode, image background, click-through/edit state, target window, offsets, and visibility. The controller edits only the active ruler, and clicking or dragging a ruler selects it.

## Implemented Behavior

- Multiple independent overlay windows are created from per-ruler settings.
- The controller can add, duplicate, rename, delete, select, show, and hide rulers.
- Geometry changes from fields, dragging, and resizing persist per ruler.
- Targeted mode tracks the selected exact window using saved offsets.
- Global shortcut and app menu actions apply to the active ruler.
- Body modes include solid, patterns, transparent modes, edge-only modes, and image backgrounds.
- Click-through can be enabled while edit mode keeps overlays draggable/resizable.
- Settings migrate from the previous flat settings shape into the multi-ruler document.

## Verification

Stage 5 is considered satisfied when these checks pass:

- `node --check app/main.js`
- `node --check app/overlay.js`
- `npm run build`
- `./script/build_and_run.sh --verify`

The README includes scenario screenshots for the controller, whole-screen reading, and multiple rulers.

## Related Files

- `src-tauri/src/lib.rs` owns settings, ruler window creation, target tracking, global shortcut, menu commands, and image persistence.
- `app/main.js` owns controller state, active-ruler editing, target polling, image import/paste, and settings-event synchronization.
- `app/overlay.js` owns per-overlay selection, drag/resize handling, style application, and geometry persistence.
- `app/index.html`, `app/styles.css`, `app/overlay.html`, and `app/overlay.css` define the controller and overlay UI.
- `src-tauri/icons/icon-source.png` is the 1024px source for the generated Tauri icon set.
- `docs/screenshots/` contains README screenshots and scenario images.
