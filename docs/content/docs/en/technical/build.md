---
title: Build
description: Building Heretic Launcher executables
---

Heretic Launcher is built with the Tauri CLI. It produces an executable and installers for the system you run the command on.

## Prerequisites

- Node.js and npm
- Rust (via [rustup](https://rustup.rs))
- The Tauri v2 system dependencies for your OS:
  - **Windows**: Microsoft C++ Build Tools and WebView2 (already included on Windows 11)
  - **macOS**: Xcode Command Line Tools
  - **Linux**: WebKitGTK and the related development libraries (see Tauri's prerequisites guide for your distribution)

## Running the build

From the project root:

```bash
npm install
npm run build
```

The build compiles the frontend, compiles the Rust backend in release mode, then generates the installers.

## Output

The files are created in `src-tauri/target/release/bundle/`, in one subfolder per format:

| System | Typical formats |
| --- | --- |
| Windows | `.msi`, `.exe` (NSIS installer) |
| macOS | `.app`, `.dmg` |
| Linux | `.deb`, `.rpm`, `.AppImage` |

The standalone executable is located in `src-tauri/target/release/`.

## One build per system

Tauri does not reliably support cross-compilation between systems: to get the Windows executable, you have to build on Windows, and likewise for macOS and Linux. To produce all three, the most practical option is a GitHub Action that runs the build on three machines, using `tauri-apps/tauri-action`.

## Development builds

- `npm run tauri dev` starts the launcher in development mode, with hot reloading for the frontend.
- `npm run tauri build -- --debug` produces a build with debug symbols, which compiles faster.

## Signing

Unsigned executables trigger warnings for the player (SmartScreen on Windows, Gatekeeper on macOS). Code signing is optional for testing, but recommended before distributing the launcher.