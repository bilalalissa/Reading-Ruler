# Installation

Reading Ruler currently uses unsigned local installation files. DMGs remain local test artifacts until Developer ID signing is available.

Regular users do not need Rust, Node.js, Cargo, Xcode Command Line Tools, or Visual Studio Build Tools. Those are only needed for source builds and developer runs, which are documented separately in [Development](DEVELOPMENT.md).

## Choose The Right File

- Apple Silicon Mac, such as M1, M2, M3, or newer: download `Reading.Ruler_0.1.0_aarch64.app.zip` from the release.
- Intel Mac: use the `Local Packaging` GitHub Actions workflow and download the `macos x64` `.app.zip` artifact.
- One Mac app for both Apple Silicon and Intel: use the `Local Packaging` workflow and download the `macos universal` `.app.zip` artifact.
- Windows: use the `Local Packaging` workflow and download the `windows nsis` `.exe` artifact or the `windows msi` `.msi` artifact.

## macOS Apple Silicon

1. Open the GitHub release page:
   <https://github.com/bilalalissa/Reading-Ruler/releases/tag/v0.1.0>
2. Download `Reading.Ruler_0.1.0_aarch64.app.zip`.
3. Optional: download `Reading.Ruler_0.1.0_aarch64.sha256` if you want to verify the file checksum.
4. Unzip `Reading.Ruler_0.1.0_aarch64.app.zip`.
5. Move `Reading Ruler.app` to `Applications` or `~/Applications`.
6. Open `Reading Ruler.app`.
7. If macOS blocks the unsigned app, Control-click `Reading Ruler.app`, choose `Open`, then confirm.

If the app is still quarantined, open the macOS `Terminal` app and run:

```sh
xattr -dr com.apple.quarantine "$HOME/Applications/Reading Ruler.app"
```

Use `/Applications/Reading Ruler.app` instead if you moved the app to the system Applications folder.

## macOS Intel

No Intel release file is published yet. Use GitHub Actions to create a local unsigned Intel app artifact.

1. Open the repository:
   <https://github.com/bilalalissa/Reading-Ruler>
2. Click `Actions`.
3. Select `Local Packaging`.
4. Click `Run workflow`.
5. Set `platform` to `macos`.
6. Set `macos_target` to `x64`.
7. Click `Run workflow`.
8. When the run finishes, open the run and download the `reading-ruler-macos-x64-local` artifact.
9. Unzip the artifact, then unzip the `.app.zip` inside it.
10. Move `Reading Ruler.app` to `Applications` or `~/Applications`.
11. Open `Reading Ruler.app`.
12. If macOS blocks the unsigned app, Control-click the app, choose `Open`, then confirm.

## Universal macOS

Use this when one downloaded app should run on both Apple Silicon and Intel Macs.

1. Open the repository:
   <https://github.com/bilalalissa/Reading-Ruler>
2. Click `Actions`.
3. Select `Local Packaging`.
4. Click `Run workflow`.
5. Set `platform` to `macos`.
6. Set `macos_target` to `universal`.
7. Click `Run workflow`.
8. When the run finishes, open the run and download the `reading-ruler-macos-universal-local` artifact.
9. Unzip the artifact, then unzip the `.app.zip` inside it.
10. Move `Reading Ruler.app` to `Applications` or `~/Applications`.
11. Open `Reading Ruler.app`.
12. If macOS blocks the unsigned app, Control-click the app, choose `Open`, then confirm.

## Windows

No Windows release file is published yet. Use GitHub Actions to create a local unsigned Windows installer artifact.

1. Open the repository:
   <https://github.com/bilalalissa/Reading-Ruler>
2. Click `Actions`.
3. Select `Local Packaging`.
4. Click `Run workflow`.
5. Set `platform` to `windows`.
6. Set `windows_bundle` to `nsis` for a `.exe` installer, or `msi` for an `.msi` installer.
7. Click `Run workflow`.
8. When the run finishes, open the run and download the `reading-ruler-windows-nsis-local` or `reading-ruler-windows-msi-local` artifact.
9. Unzip the artifact.
10. Run the downloaded `.exe` or `.msi` installer.
11. If Windows SmartScreen warns that the installer is unsigned, choose the local/internal install option to continue.

Windows may require Microsoft WebView2 Runtime. Most current Windows systems already have it. If Reading Ruler does not launch and WebView2 is missing, install it from:
<https://developer.microsoft.com/microsoft-edge/webview2/>

## Available Installation Files

Current release downloads:

- `Reading.Ruler_0.1.0_aarch64.app.zip`
- `Reading.Ruler_0.1.0_aarch64.sha256`

Current GitHub Actions `Local Packaging` artifacts:

- `reading-ruler-macos-arm64-local`
- `reading-ruler-macos-x64-local`
- `reading-ruler-macos-universal-local`
- `reading-ruler-windows-nsis-local`
- `reading-ruler-windows-msi-local`

Workflow artifacts expire after GitHub's retention period. If an artifact is missing, run the `Local Packaging` workflow again.

## DMG Status

DMG files are kept as local test artifacts while Developer ID signing is unavailable. Use `.app.zip` for unsigned macOS sharing.

The most recent local DMG path on this machine was:

```text
/Users/ba/Code/Reading-Ruler/src-tauri/target/release/bundle/dmg/Reading Ruler_0.1.0_aarch64.dmg
```

## Source Builds

Source builds, local package generation commands, dependency checker scripts, optional development runs, and signed distribution packaging are documented in [Development](DEVELOPMENT.md).
