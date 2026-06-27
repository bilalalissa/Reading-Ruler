# Development

This page is for developers who want to run Reading Ruler directly from source. Regular installation instructions are in [Installation and packaging](INSTALLATION.md).

## Optional Development Run

Use this when you want to test code changes without installing an app bundle.

1. Get the repo and install build dependencies using the platform steps in [Installation and packaging](INSTALLATION.md).
2. Install project dependencies:

```sh
npm install
```

3. Build and run:

```sh
./script/build_and_run.sh
```

4. Optional: verify the app stays running:

```sh
./script/build_and_run.sh --verify
```

