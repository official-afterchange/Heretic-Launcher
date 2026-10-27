---
title: Installation
description: Setting up the Heretic Launcher development environment
---

This page explains how to clone the project and run it in development mode. To generate the executables, see the [Build](/en/technical/build/) page.

## Prerequisites

- **Node.js** and npm
- **Rust**, installed with [rustup](https://rustup.rs)
- The **Tauri v2 system dependencies** for your OS (see the [Build](/en/technical/build/) page for details)

Check your installations:

```bash
node --version
npm --version
rustc --version
cargo --version
```

## Getting the project

```bash
git clone https://github.com/official-afterchange/heretic-launcher.git
cd heretic-launcher
npm install
```

The first Rust compilation downloads and compiles all the backend dependencies, which can take several minutes. Subsequent builds are much faster.

## Running the launcher in development

```bash
npm run dev
```

This command starts the frontend development server (Vite) and opens the launcher in its Tauri window. Frontend changes are applied instantly through hot reloading. Changes to the Rust code trigger a recompilation and a restart of the application.

## Running the documentation

The documentation is a Starlight site in the `docs/` folder. To view it locally:

```bash
npm run docs:dev
```

It is then available at `http://localhost:4321`.

## Common issues

- **The window does not open or Rust fails to compile**: check that the Tauri v2 system dependencies are installed for your OS.
- **`cargo` not found**: close and reopen the terminal after installing Rust so the `PATH` is updated.
- **The development server port is already in use**: close the other instance of the launcher or Vite that is still running.