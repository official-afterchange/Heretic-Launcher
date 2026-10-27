# Heretic Launcher

Heretic Launcher is the custom Minecraft client/launcher for the Heretic ecosystem, bypassing official Microsoft/Mojang authentication via Heretic IDP.

## Tech Stack

- **App Framework:** Tauri (v2)
- **Frontend:** TypeScript, HTML, CSS
- **Backend:** Rust (Java process management, JVM argument injection)
- **Auth Protocol:** Custom Yggdrasil implementation

## Documentation

Full project documentation, setup guides, and architecture specifications can be found in the [docs/](./docs) directory:

- [Architecture & Auth Flow](./docs/architecture.md)
- [Development Setup Guide](./docs/setup.md)

## Development Setup

```bash
git clone https://github.com/official-afterchange/heretic-launcher.git
cd heretic-launcher
npm install
npm run tauri dev
```

## Heretic Ecosystem
- [Heretic IDP](https://github.com/official-afterchange/heretic-idp) - The Identity Provider Backend