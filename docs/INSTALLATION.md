# Installation And Packaging

Reading Ruler uses local unsigned installation paths for macOS Apple Silicon, macOS Intel, universal macOS, and Windows while Developer ID signing is unavailable. DMG creation remains available for local macOS testing, but `.app.zip` is the preferred macOS sharing artifact for now.

## Development Run

Use this when you want to run from source instead of installing an app bundle.

1. Install dependencies:

```sh
npm install
```

2. Build and run:

```sh
./script/build_and_run.sh
```

3. Optional: verify the app stays running:

```sh
./script/build_and_run.sh --verify
```

## macOS Apple Silicon Install

Use this on M1, M2, M3, or newer Apple Silicon Macs.

### Download From GitHub

Download this file from the current release:

- [Reading.Ruler_0.1.0_aarch64.app.zip](https://github.com/bilalalissa/Reading-Ruler/releases/download/v0.1.0/Reading.Ruler_0.1.0_aarch64.app.zip)

Install it:

1. Unzip `Reading.Ruler_0.1.0_aarch64.app.zip`.
2. Move `Reading Ruler.app` to `Applications` or `~/Applications`.
3. Open `Reading Ruler.app`.
4. If macOS blocks the unsigned app, Control-click the app, choose `Open`, then confirm.

For local testing, you can also remove quarantine:

```sh
xattr -dr com.apple.quarantine "$HOME/Applications/Reading Ruler.app"
```

Optional checksum file:

- [Reading.Ruler_0.1.0_aarch64.sha256](https://github.com/bilalalissa/Reading-Ruler/releases/download/v0.1.0/Reading.Ruler_0.1.0_aarch64.sha256)

### Build From Source

1. Install prerequisites:

- Rust/Cargo
- Node.js and npm
- Xcode Command Line Tools

2. Open the macOS `Terminal` app.
3. Go to the project folder. If you downloaded the repo ZIP, use the unzipped folder. If you cloned the repo, run:

```sh
cd Reading-Ruler
```

4. Install npm dependencies in Terminal:

```sh
npm install
```

5. Build and install the app locally from Terminal:

```sh
npm run app:package:mac:local -- --target arm64 --install
```

6. Open the app:

```sh
open "$HOME/Applications/Reading Ruler.app"
```

The app is copied to `~/Applications/Reading Ruler.app`. The generated shareable zip is:

- `src-tauri/target/aarch64-apple-darwin/release/bundle/macos/Reading Ruler_0.1.0_arm64.app.zip`

## macOS Intel Install

Use this on Intel Macs.

No Intel release download is published yet. Build the Intel local app from the repo.

1. Download the repo from <https://github.com/bilalalissa/Reading-Ruler> using `Code` > `Download ZIP`, or clone it:
2. Open the macOS `Terminal` app.
3. If you chose `Download ZIP`, go to the unzipped project folder in Terminal. If you use Git, run:

```sh
git clone https://github.com/bilalalissa/Reading-Ruler.git
cd Reading-Ruler
```

4. Install prerequisites:

- Rust/Cargo
- Node.js and npm
- Xcode Command Line Tools

5. Install npm dependencies and the Intel Rust target in Terminal:

```sh
npm install
rustup target add x86_64-apple-darwin
```

6. Build and install the app locally from Terminal:

```sh
npm run app:package:mac:local -- --target x64 --install
```

7. Open the app:

```sh
open "$HOME/Applications/Reading Ruler.app"
```

The app is copied to `~/Applications/Reading Ruler.app`. The generated shareable zip uses `x64` in the file name.

## Universal macOS Install

Use this when one local app bundle should run on both Apple Silicon and Intel Macs.

No universal macOS release download is published yet. Build the universal local app from the repo.

1. Download the repo from <https://github.com/bilalalissa/Reading-Ruler> using `Code` > `Download ZIP`, or clone it:
2. Open the macOS `Terminal` app.
3. If you chose `Download ZIP`, go to the unzipped project folder in Terminal. If you use Git, run:

```sh
git clone https://github.com/bilalalissa/Reading-Ruler.git
cd Reading-Ruler
```

4. Install prerequisites:

- Rust/Cargo
- Node.js and npm
- Xcode Command Line Tools

5. Install npm dependencies and the Intel Rust target in Terminal:

```sh
npm install
rustup target add x86_64-apple-darwin
```

6. Build and install the universal app locally from Terminal:

```sh
npm run app:package:mac:local -- --target universal --install
```

7. Open the app:

```sh
open "$HOME/Applications/Reading Ruler.app"
```

The app is copied to `~/Applications/Reading Ruler.app`. The generated shareable zip uses `universal` in the file name.

## Windows Install

Use this on Windows for a local unsigned installer.

No Windows release download is published yet. Build the Windows local installer from the repo.

1. Download the repo from <https://github.com/bilalalissa/Reading-Ruler> using `Code` > `Download ZIP`, or clone it:
2. Open `PowerShell`.
3. If you chose `Download ZIP`, go to the unzipped project folder in PowerShell. If you use Git, run:

```powershell
git clone https://github.com/bilalalissa/Reading-Ruler.git
cd Reading-Ruler
```

4. Install prerequisites:

- Rust/Cargo with the MSVC toolchain
- Node.js and npm
- Microsoft WebView2 Runtime
- Visual Studio Build Tools

5. Install npm dependencies in PowerShell:

```powershell
npm install
```

6. Build the default NSIS installer in PowerShell:

```powershell
npm run app:package:windows:local
```

7. Run the generated installer from:

```text
src-tauri\target\release\bundle\
```

8. If Windows SmartScreen warns that the installer is unsigned, choose the local/internal install option to continue.

To build MSI instead of NSIS:

```powershell
npm run app:package:windows:local -- -Bundle msi
```

The script writes a SHA-256 checksum file next to the generated installer.

## Local macOS Artifact Notes

The macOS local install script:

- builds an unsigned `.app`
- creates a shareable `.app.zip`
- writes a SHA-256 checksum
- copies the app to `~/Applications` when `--install` is used
- removes the quarantine attribute from that local copy when `xattr` is available

## Local macOS DMG

DMGs are local-only for now. Use this only when you specifically need a local DMG test artifact:

```sh
npm run app:package:mac
```

Expected local artifacts:

- `src-tauri/target/release/bundle/macos/Reading Ruler.app`
- `src-tauri/target/release/bundle/dmg/*.dmg`

The packaging script builds with Tauri, validates the app bundle, reports executable architecture, checks code-signing status, and verifies generated DMGs.

## Signed macOS Distribution Package

This path is optional and blocked until a Developer ID certificate and notarization profile are available:

```sh
npm run app:package:mac:distribution -- --check-prereqs
```

```sh
APPLE_SIGNING_IDENTITY="Developer ID Application: Your Name (TEAMID)" \
  npm run app:package:mac:distribution -- --notarize reading-ruler-notary
```

The signing identity must be a `Developer ID Application` certificate. `Apple Development` certificates are only suitable for development and do not produce a public Gatekeeper-clean distribution build.

## GitHub Actions Distribution Release

The manual `macOS Distribution` workflow signs, notarizes, staples, checksums, and uploads package files to a GitHub release after Apple credentials are configured.

Required repository secrets:

- `APPLE_SIGNING_IDENTITY`: full Developer ID Application identity name.
- `APPLE_CERTIFICATE_P12_BASE64`: base64-encoded `.p12` Developer ID certificate.
- `APPLE_CERTIFICATE_PASSWORD`: password for the `.p12` certificate.
- `KEYCHAIN_PASSWORD`: temporary CI keychain password.
- `APPLE_ID`: Apple ID used for notarization.
- `APPLE_TEAM_ID`: Apple Developer Team ID.
- `APPLE_APP_SPECIFIC_PASSWORD`: app-specific password for the Apple ID.

## Available Installation Files

Download from the current GitHub release:

- [Reading.Ruler_0.1.0_aarch64.app.zip](https://github.com/bilalalissa/Reading-Ruler/releases/download/v0.1.0/Reading.Ruler_0.1.0_aarch64.app.zip)
- [Reading.Ruler_0.1.0_aarch64.sha256](https://github.com/bilalalissa/Reading-Ruler/releases/download/v0.1.0/Reading.Ruler_0.1.0_aarch64.sha256)

For Apple Silicon, download `Reading.Ruler_0.1.0_aarch64.app.zip`. The checksum file is optional and is used to verify the download. DMGs are kept as local test artifacts until Developer ID signing is available.

New local installation files are generated with:

- `npm run app:package:mac:local -- --target arm64 --install`
- `npm run app:package:mac:local -- --target x64 --install`
- `npm run app:package:mac:local -- --target universal --install`
- `npm run app:package:windows:local`

Use `.app.zip` for macOS local sharing and the generated NSIS/MSI installer for Windows local sharing. Keep DMGs local until Developer ID signing is available.

## GitHub Repository

The project repository is:

```text
https://github.com/bilalalissa/Reading-Ruler.git
```

The repository description should be:

```text
Cross-platform desktop reading ruler overlay with multiple customizable rulers, window targeting, and macOS packaging.
```
