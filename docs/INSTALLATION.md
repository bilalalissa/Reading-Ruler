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
