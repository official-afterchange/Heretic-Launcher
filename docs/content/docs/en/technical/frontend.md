---
title: Frontend
description: Structure and behavior of the Heretic Launcher interface
---

The Heretic Launcher interface is a **React** application written in **TypeScript**. It runs inside Tauri's web window and contains no sensitive logic: everything that touches the system goes through the Rust backend.

## Structure

```text
src/
├── pages/
│   ├── home.tsx
│   ├── account.tsx
│   ├── browser.tsx
│   ├── info.tsx
│   └── settings.tsx
├── components/      ← shared components
└── main.tsx         ← entry point
```

## Pages

| Page | File | Purpose |
| --- | --- | --- |
| Home | `home.tsx` | Launch the game and see information about the server the launcher belongs to |
| Account | `account.tsx` | Manage the player's account |
| Browser | `browser.tsx` | TO FIX: browsing the launcher's content |
| Info | `info.tsx` | Information about the developer |
| Settings | `settings.tsx` | Launcher settings |

## Navigation

Pages are displayed in a single window, without reloading. A navigation bar lets the player switch between them, and the Home page is shown at startup.

## Communicating with the backend

The frontend calls Rust commands with Tauri's `invoke` function:

```ts
import { invoke } from '@tauri-apps/api/core';

async function launchGame() {
  try {
    await invoke('launch_game');
  } catch (error) {
    console.error('Launch failed:', error);
  }
}
```

Commands are asynchronous: they return a promise, and an error on the Rust side becomes a rejection on the TypeScript side. The backend can also send events to the frontend (download progress, game logs) that the interface listens to in order to update itself.

## Interface state

State is managed with React hooks (`useState`, `useEffect`). Values shared between several pages, such as the logged-in account or the configuration, are centralized rather than duplicated in each page.