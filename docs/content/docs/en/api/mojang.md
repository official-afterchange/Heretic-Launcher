---
title: Mojang Routes and Domains (Assets)
description: Domains and routes used by a launcher to download Minecraft versions, libraries, the client, and assets.
---

A custom launcher needs to access Mojang's official content servers to download the files required to launch the game: `client.jar`, Java libraries, the asset index, and resources (sounds, textures, languages).

:::note
Downloading these files requires **no authentication**. These routes are public and accessible even if Microsoft authentication is skipped or simulated in your launcher.
:::

## Overview

Downloading a version works as a cascade: each step provides the URLs for the next one.

```text
Version manifest
        │  (url of the chosen version)
        ▼
Version JSON
        ├── downloads.client.url  → client.jar
        ├── libraries[]           → Java libraries
        ├── assetIndex.url        → asset index
        │                               │  (hash of each file)
        │                               ▼
        │                          asset files
        └── javaVersion           → required Java version
```

## Domains to Allow

If you filter network traffic (firewall, proxy, your application's CSP), allow these domains:

| Domain | Role |
|---|---|
| `piston-meta.mojang.com` | Version manifest, version JSON, asset index |
| `piston-data.mojang.com` | `client.jar` and other version files |
| `libraries.minecraft.net` | Java libraries (`libraries`) |
| `resources.download.minecraft.net` | Asset files (sounds, textures, languages) |
| `launchermeta.mojang.com` | Manifest of the Java runtimes provided by Mojang (optional) |

## 1. Version Manifest

```text
https://piston-meta.mojang.com/mc/game/version_manifest_v2.json
```

This file lists all available versions in the `versions` array. Look for the entry whose `id` matches the desired version (e.g., `26.2`) and retrieve its `url` field.

```json
{
  "id": "26.2",
  "type": "release",
  "url": "https://piston-meta.mojang.com/v1/packages/<sha1>/26.2.json",
  "sha1": "<sha1>"
}
```

## 2. Version JSON

This is the URL retrieved in the previous step, for example:

```text
https://piston-meta.mojang.com/v1/packages/ea228cdd1379853393fd73ad0a562c3f5eb7fd9d/26.2.json
```

:::caution
Do not hardcode this URL: the hash changes if Mojang republishes the file. Always retrieve it from the manifest.
:::

This JSON is the central file. It contains everything needed to download and launch the game:

| Field | Content |
|---|---|
| `downloads.client` | URL, `sha1`, and size of the `client.jar` |
| `libraries[]` | List of libraries to download |
| `assetIndex` | `id`, `sha1`, and `url` of the asset index |
| `javaVersion` | Required major Java version (`component` and `majorVersion`) |
| `arguments` | JVM arguments and game arguments |
| `mainClass` | Main class to launch |
| `logging` | Logging configuration (optional) |

## 3. Game Client (`client.jar`)

The URL is found in `downloads.client.url` of the version JSON:

```text
https://piston-data.mojang.com/v1/objects/<sha1>/client.jar
```

## 4. Libraries (`libraries`)

Each entry in `libraries[]` directly provides its URL:

```json
{
  "name": "group:artifact:version",
  "downloads": {
    "artifact": {
      "path": "group/artifact/version/artifact-version.jar",
      "url": "https://libraries.minecraft.net/group/artifact/version/artifact-version.jar",
      "sha1": "<sha1>",
      "size": 12345
    }
  },
  "rules": [
    { "action": "allow", "os": { "name": "windows" } }
  ]
}
```

* **Rules (`rules`)**: some libraries only apply to a specific OS or architecture. Evaluate these rules and only download the ones that apply to the user's machine.
* **Natives**: in recent versions, native libraries are regular entries in `libraries[]`, filtered by `rules`.
* **Storage**: save each file according to its `path`, inside `libraries/`.

## 5. Assets (sounds, textures, languages)

### Asset Index

The URL is in `assetIndex.url` of the version JSON. The file contains an `objects` object that maps each resource name to its hash:

```json
{
  "objects": {
    "minecraft/lang/fr_fr.json": {
      "hash": "ab12cd34...",
      "size": 123456
    }
  }
}
```

### Downloading an Asset

```text
https://resources.download.minecraft.net/{first 2 characters of the hash}/{hash}
```

Example for the hash `ab12cd34...`:

```text
https://resources.download.minecraft.net/ab/ab12cd34...
```

### On-Disk Layout

Reproduce the official launcher's structure:

```text
assets/
├── indexes/
│   └── {assetIndex.id}.json
└── objects/
    └── ab/
        └── ab12cd34...
```

## 6. Java (optional)

The `javaVersion.majorVersion` field indicates the required Java version (Java 25 for Minecraft 26.2). There are two options:

* **Adoptium (Temurin)**: see the [Adoptium API Routes and Parameters](/Heretic-Launcher/en/api/adoptium) page.
* **Runtime provided by Mojang**: via the runtime manifest.

```text
https://launchermeta.mojang.com/v1/products/java-runtime/2ec0cc96c44e5a76b9c8b7c39df7210883d12871/all.json
```

## Best Practices

* **Verify integrity**: compare the `sha1` of each downloaded file with the one provided by the API.
* **Avoid redownloads**: only download files that are missing or whose hash differs.
* **Parallelize**: an asset index contains thousands of files, so downloading them sequentially would be very slow.
* **Handle network errors**: plan for multiple attempts (retries) for assets.