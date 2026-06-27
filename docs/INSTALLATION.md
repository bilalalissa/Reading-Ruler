# Installation And Packaging

Reading Ruler uses local unsigned installation paths for macOS Apple Silicon, macOS Intel, universal macOS, and Windows while Developer ID signing is unavailable. DMG creation remains available for local macOS testing, but `.app.zip` is the preferred macOS sharing artifact for now.

## Development Run

```sh
./script/build_and_run.sh
```

Use verification mode to build, launch, and confirm the process is running:

```sh
./script/build_and_run.sh --verify
```

## macOS Local Install Without Developer ID

Apple Silicon:

```sh
npm install
npm run app:package:mac:local -- --target arm64 --install
```

Intel Mac:

```sh
npm install
rustup target add x86_64-apple-darwin
npm run app:package:mac:local -- --target x64 --install
```

Universal macOS app:

```sh
npm install
rustup target add x86_64-apple-darwin
npm run app:package:mac:local -- --target universal --install
```

The `--install` option copies `Reading Ruler.app` to `~/Applications` and removes the quarantine attribute from that local copy when `xattr` is available.

Expected Apple Silicon local artifacts:

- `src-tauri/target/aarch64-apple-darwin/release/bundle/macos/Reading Ruler.app`
- `src-tauri/target/aarch64-apple-darwin/release/bundle/macos/Reading Ruler_0.1.0_arm64.app.zip`
- `src-tauri/target/aarch64-apple-darwin/release/bundle/Reading Ruler_0.1.0_arm64.sha256`

Use `x64` or `universal` in the file names for Intel or universal builds.

## Windows Local Install Without Code Signing

Run on Windows:

```powershell
npm install
npm run app:package:windows:local
```

By default this builds an unsigned NSIS installer. To build MSI instead:

```powershell
npm run app:package:windows:local -- -Bundle msi
```

Windows artifacts are written under:

- `src-tauri\target\release\bundle\`

The script also writes a SHA-256 checksum file next to the generated installer. Windows may show a SmartScreen warning because the installer is unsigned; this path is intended for local testing and internal installs.

## Local macOS DMG

Keep DMGs local until Developer ID signing is available:

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

The current GitHub release keeps the original unsigned Apple Silicon artifacts:

- [Reading.Ruler_0.1.0_aarch64.dmg](https://github.com/bilalalissa/Reading-Ruler/releases/download/v0.1.0/Reading.Ruler_0.1.0_aarch64.dmg)
- [Reading.Ruler_0.1.0_aarch64.app.zip](https://github.com/bilalalissa/Reading-Ruler/releases/download/v0.1.0/Reading.Ruler_0.1.0_aarch64.app.zip)
- [Reading.Ruler_0.1.0_aarch64.sha256](https://github.com/bilalalissa/Reading-Ruler/releases/download/v0.1.0/Reading.Ruler_0.1.0_aarch64.sha256)

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
