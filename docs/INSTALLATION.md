# Installation And Packaging

Reading Ruler currently targets local macOS Apple Silicon testing.

## Development Run

```sh
./script/build_and_run.sh
```

Use the verification mode to build, launch, and confirm the process is running:

```sh
./script/build_and_run.sh --verify
```

## Unsigned macOS Package

```sh
npm run app:package:mac
```

The packaging script builds with Tauri, validates the generated app bundle, reports executable architecture, checks code-signing status, and verifies generated DMGs.

Expected artifacts:

- `src-tauri/target/release/bundle/macos/Reading Ruler.app`
- `src-tauri/target/release/bundle/dmg/*.dmg`

Local unsigned builds are suitable for development and local testing. Public distribution still needs Developer ID signing, hardened runtime, and notarization.

## Signed macOS Distribution Package

Use the distribution package script when a public macOS build is needed:

```sh
APPLE_SIGNING_IDENTITY="Developer ID Application: Your Name (TEAMID)" \
  npm run app:package:mac:distribution
```

The signing identity must be a `Developer ID Application` certificate. `Apple Development` certificates are only suitable for development and do not produce a public Gatekeeper-clean distribution build.

To notarize, store a notary profile once:

```sh
xcrun notarytool store-credentials reading-ruler-notary
```

Then run:

```sh
APPLE_SIGNING_IDENTITY="Developer ID Application: Your Name (TEAMID)" \
  npm run app:package:mac:distribution -- --notarize reading-ruler-notary
```

The script validates the app bundle, signs the app with hardened runtime and timestamp, verifies the signature, creates an app zip, optionally notarizes and staples the app, creates a signed DMG, optionally notarizes and staples the DMG, verifies the DMG, and writes SHA-256 checksums.

Distribution outputs:

- `src-tauri/target/release/bundle/macos/Reading Ruler_0.1.0_arm64.app.zip`
- `src-tauri/target/release/bundle/dmg/Reading Ruler_0.1.0_arm64.dmg`
- `src-tauri/target/release/bundle/Reading Ruler_0.1.0_arm64.sha256`

Current credential status on this machine: only an `Apple Development` identity is installed. Install a `Developer ID Application` certificate and create a notary profile before running the signed distribution path for release.

## Available Installation Files

The latest Apple Silicon installation files are attached to the `v0.1.0` GitHub release:

- [Reading.Ruler_0.1.0_aarch64.dmg](https://github.com/bilalalissa/Reading-Ruler/releases/download/v0.1.0/Reading.Ruler_0.1.0_aarch64.dmg)
- [Reading.Ruler_0.1.0_aarch64.app.zip](https://github.com/bilalalissa/Reading-Ruler/releases/download/v0.1.0/Reading.Ruler_0.1.0_aarch64.app.zip)
- [Reading.Ruler_0.1.0_aarch64.sha256](https://github.com/bilalalissa/Reading-Ruler/releases/download/v0.1.0/Reading.Ruler_0.1.0_aarch64.sha256)

After a successful package build, the same local installation files are available:

- `src-tauri/target/release/bundle/macos/Reading Ruler.app`
- `src-tauri/target/release/bundle/dmg/Reading Ruler_0.1.0_aarch64.dmg`

Use the DMG for drag-and-drop installation testing. Use the `.app` bundle or `.app.zip` for direct launch testing. The current package is unsigned/ad-hoc; public distribution still needs Developer ID signing and notarization.

The local Apple Silicon package is finished when `npm run app:package:mac` reports:

- app bundle generated
- `Info.plist` lint passes
- executable architecture is `arm64`
- DMG verification is valid

## GitHub Repository

The project repository is:

```text
https://github.com/bilalalissa/Reading-Ruler.git
```

The repository description should be:

```text
Cross-platform desktop reading ruler overlay with multiple customizable rulers, window targeting, and macOS packaging.
```
