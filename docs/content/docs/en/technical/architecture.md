---
title: Architecture
description: Technical overview of Heretic Launcher
---

Heretic Launcher is a custom Minecraft launcher built with **Tauri v2**. Each launcher is tied to **a single server**. There is no multi-instance support.

## Overview

```text
┌──────────────────────────────────────────────┐
│               Heretic Launcher               │
│                                              │
│  ┌────────────────┐      ┌────────────────┐  │
│  │ Frontend       │      │ Backend        │  │
│  │ React + TS     │◄────►│ Rust (Tauri)   │  │
│  │                │ IPC  │                │  │
│  └────────────────┘      └───────┬────────┘  │
└──────────────────────────────────┼───────────┘
                                   │ launches
                                   ▼
                           ┌──────────────┐      ┌─────────────┐
                           │ Java         │─────►│ Minecraft   │
                           │ process      │      │ server      │
                           │ (Minecraft)  │      └─────────────┘
                           └──────────────┘
```

## Layers

### Frontend (React + TypeScript)

The interface has four pages:

| Page | Purpose |
| --- | --- |
| Home | Launch the game and see information about the server the launcher belongs to |
| Account | Manage the player's account |
| Info | Information about the developer |
| Settings | Launcher settings |

The frontend contains no sensitive logic. It calls the backend through Tauri commands and displays the returned state (progress, errors, game status).

### Backend (Rust)

The backend handles everything that touches the system:

- **checking and downloading** the game files (client, libraries, assets);
- **detecting Java** and choosing a suitable version;
- **building the Java command** and injecting the JVM arguments;
- **launching and monitoring** the Minecraft process, forwarding its logs to the frontend;
- **reading and writing** the launcher configuration (allocated memory, Java path, language).

## Game launch cycle

1. The player clicks **Play** on the Home page.
2. The frontend calls the launch Tauri command.
3. The backend checks that the game files are present and up to date, and downloads any that are missing.
4. It locates Java and assembles the launch command with the JVM arguments.
5. It starts the Minecraft process and sends its status to the frontend.
6. When the game closes, the backend releases the process and resets the interface to its initial state.

## Repository structure

```text
heretic-launcher/
├── src/                  ← React frontend
│   ├── pages/            ← Home, Account, Info, Settings
│   └── components/       ← shared components
├── src-tauri/            ← Rust backend
│   ├── src/
│   │   ├── main.rs       ← entry point
│   │   ├── commands.rs   ← commands exposed to the frontend
│   │   ├── launcher.rs   ← command building and launch
│   │   ├── download.rs   ← game file downloads
│   │   └── config.rs     ← launcher configuration
│   ├── Cargo.toml
│   └── tauri.conf.json
└── docs/                 ← this documentation
```

## Technical choices

- **Tauri v2 over Electron**: a much lighter executable, since it uses the system's web engine instead of bundling Chromium, with a Rust backend.
- **Rust for the backend**: reliable process and file handling, with no extra runtime for the player to install.
- **React + TypeScript for the frontend**: reusable components and typed exchanges with the backend.
- **One server per launcher**: no profile or instance management, which keeps the interface and the configuration simple.