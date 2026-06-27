# Development

This page is for developers who want to run Reading Ruler directly from source or generate local package files from source. Regular user installation instructions are in [Installation](INSTALLATION.md).

## Get The Repo

### macOS

1. Open the macOS `Terminal` app.
2. Choose where you want the project folder:

```sh
cd "$HOME/Downloads"
```

3. Clone the repo:

```sh
git clone https://github.com/bilalalissa/Reading-Ruler.git
cd Reading-Ruler
```

If Git is not installed, open <https://github.com/bilalalissa/Reading-Ruler>, choose `Code` > `Download ZIP`, unzip it, then in Terminal run `cd` into the unzipped `Reading-Ruler` folder.

### Windows

1. Open `PowerShell`.
2. Choose where you want the project folder:

```powershell
cd $HOME\Downloads
```

3. Clone the repo:

```powershell
git clone https://github.com/bilalalissa/Reading-Ruler.git
cd Reading-Ruler
```

If Git is not installed, open <https://github.com/bilalalissa/Reading-Ruler>, choose `Code` > `Download ZIP`, unzip it, then in PowerShell run `cd` into the unzipped `Reading-Ruler` folder.

## Install Build Dependencies

Run these commands from inside the `Reading-Ruler` repo folder.

### macOS Apple Silicon

In Terminal:

```sh
./script/check_macos_deps.sh
```

If the script reports missing tools, let it try to install them:

```sh
./script/check_macos_deps.sh --install
```

The script checks Xcode Command Line Tools, Rust/Cargo/rustup, and Node.js/npm. It can open the Xcode Command Line Tools installer and use Homebrew for Node.js or rustup if Homebrew is installed.

### macOS Intel Or Universal

In Terminal:

```sh
./script/check_macos_deps.sh --with-intel-target
```

If the script reports missing tools, let it try to install them:

```sh
./script/check_macos_deps.sh --install --with-intel-target
```

This checks the normal macOS dependencies plus the `x86_64-apple-darwin` Rust target needed for Intel/universal builds.

### Windows

In PowerShell:

```powershell
powershell -ExecutionPolicy Bypass -File script/check_windows_deps.ps1
```

If the script reports missing tools, let it try to install them with `winget`:

```powershell
powershell -ExecutionPolicy Bypass -File script/check_windows_deps.ps1 -Install
```

The script checks Rust/Cargo/rustup with the MSVC toolchain, Node.js/npm, Visual Studio Build Tools with MSVC, and Microsoft WebView2 Runtime.

Manual dependency links:

- Rust/Cargo: <https://rustup.rs/>
- Node.js/npm: <https://nodejs.org/>
- Xcode Command Line Tools: run `xcode-select --install` in macOS Terminal.
- Visual Studio Build Tools: <https://visualstudio.microsoft.com/visual-cpp-build-tools/>
- Microsoft WebView2 Runtime: <https://developer.microsoft.com/microsoft-edge/webview2/>

## Project Dependencies

In Terminal on macOS or PowerShell on Windows, run this from the repo folder:

```sh
npm install
```

## Optional Development Run

Use this only when you want to test code changes without installing an app bundle.

In Terminal on macOS:

```sh
./script/build_and_run.sh
```

Optional verification mode:

```sh
./script/build_and_run.sh --verify
```

## Local Package Builds

These commands generate installable local artifacts from source.

### macOS Apple Silicon

In Terminal:

```sh
npm run app:package:mac:local -- --target arm64 --install
```

The app is copied to `~/Applications/Reading Ruler.app`. The generated shareable zip is under:

```text
src-tauri/target/aarch64-apple-darwin/release/bundle/macos/
```

### macOS Intel

In Terminal:

```sh
npm run app:package:mac:local -- --target x64 --install
```

### Universal macOS

In Terminal:

```sh
npm run app:package:mac:local -- --target universal --install
```

### Windows NSIS

In PowerShell:

```powershell
npm run app:package:windows:local
```

The generated installer is under:

```text
src-tauri\target\release\bundle\
```

### Windows MSI

In PowerShell:

```powershell
npm run app:package:windows:local -- -Bundle msi
```

## Local macOS DMG

DMGs are local-only for now. Use this only when you specifically need a local DMG test artifact.

In Terminal:

```sh
npm run app:package:mac
```

Expected local artifacts:

- `src-tauri/target/release/bundle/macos/Reading Ruler.app`
- `src-tauri/target/release/bundle/dmg/*.dmg`

The packaging script builds with Tauri, validates the app bundle, reports executable architecture, checks code-signing status, and verifies generated DMGs.

## Signed macOS Distribution

This path is optional and blocked until a Developer ID certificate and notarization profile are available.

In Terminal:

```sh
npm run app:package:mac:distribution -- --check-prereqs
```

```sh
APPLE_SIGNING_IDENTITY="Developer ID Application: Your Name (TEAMID)" \
  npm run app:package:mac:distribution -- --notarize reading-ruler-notary
```

The signing identity must be a `Developer ID Application` certificate. `Apple Development` certificates are only suitable for development and do not produce a public Gatekeeper-clean distribution build.

## GitHub Actions Packaging

The manual `Local Packaging` workflow builds unsigned local artifacts without Apple Developer ID credentials:

1. Open the repository on GitHub.
2. Go to `Actions`.
3. Select `Local Packaging`.
4. Click `Run workflow`.
5. Choose `macos` or `windows`.
6. For macOS, choose `arm64`, `x64`, or `universal`.
7. For Windows, choose `nsis` or `msi`.
8. Download the generated workflow artifact after the run finishes.

The manual `macOS Distribution` workflow signs, notarizes, staples, checksums, and uploads package files to a GitHub release after Apple credentials are configured.
