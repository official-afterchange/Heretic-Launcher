---
title: Fabric API Routes (Loader)
description: Fabric Meta API routes to retrieve the Fabric loader, its launch profile, and its libraries for a given Minecraft version.
---

[Fabric](https://fabricmc.net) is a mod loader. To launch Minecraft with Fabric, the launcher must retrieve a **launch profile** that layers on top of the vanilla version: main class, arguments, and additional libraries. This information is provided by the **Fabric Meta** API.

:::note
As with the Mojang routes, these routes are **public** and require no authentication.
:::

## Overview

```text
List of loaders for the game version
        │  (pick a stable loader)
        ▼
Launch profile (profile/json)
        ├── inheritsFrom  → vanilla version to merge with
        ├── mainClass     → Fabric's main class
        ├── arguments     → additional arguments
        └── libraries[]   → Fabric libraries (Maven)
```

## Domains to Allow

| Domain | Role |
|---|---|
| `meta.fabricmc.net` | Fabric Meta API (loader versions, launch profile) |
| `maven.fabricmc.net` | Download of the Fabric libraries (JARs) |

## 1. List the Available Loaders

```text
https://meta.fabricmc.net/v2/versions/loader/{game_version}
```

Example for Minecraft 26.2:

```text
https://meta.fabricmc.net/v2/versions/loader/26.2
```

The response is an array, from newest to oldest. Each entry contains a `loader` object:

```json
"loader": {
    "separator": ".",
    "build": 1,
    "maven": "net.fabricmc:fabric-loader:0.20.1",
    "version": "0.20.1",
    "stable": true
}
```

| Field | Description |
|---|---|
| `version` | Loader version (e.g., `0.20.1`), to be reused in the next step |
| `stable` | `true` for a stable version, `false` for a pre-release |
| `maven` | Maven coordinates of the loader |
| `build` | Build number |
| `separator` | Separator between the name and the build number |

### Choosing the Right Loader

To get the latest stable version, simply **iterate through the array and take the first entry where `loader.stable` is `true`**.

:::caution
The values `26.2` and `0.20.1` are examples. Do not hardcode them: the loader version changes regularly, so you should always query it.
:::

If the array is empty (or if the API returns an error), Fabric does not support this game version yet: the launcher should display a clear message to the user.

## 2. Retrieve the Launch Profile

```text
https://meta.fabricmc.net/v2/versions/loader/{game_version}/{loader_version}/profile/json
```

Example:

```text
https://meta.fabricmc.net/v2/versions/loader/26.2/0.20.1/profile/json
```

The response is a JSON in the same format as a Mojang version JSON, but **incomplete**: it only describes what Fabric adds to the vanilla version.

| Field | Content |
|---|---|
| `id` | Profile identifier (e.g., `fabric-loader-0.20.1-26.2`) |
| `inheritsFrom` | Vanilla version the profile inherits from (e.g., `26.2`) |
| `mainClass` | Fabric's main class, which **replaces** Mojang's |
| `arguments` | Additional JVM and/or game arguments |
| `libraries[]` | Fabric libraries to add to the classpath |

## 3. Merge with the Vanilla Version

The Fabric profile does not replace the Mojang version JSON, it **extends** it. The launcher must therefore:

1. retrieve the vanilla version JSON (see [Mojang Routes and Domains](/Heretic-Launcher/en/api/mojang));
2. retrieve the Fabric profile;
3. merge the two:
   * `libraries`: **add** the Fabric libraries to Mojang's;
   * `arguments`: **add** the Fabric arguments to Mojang's;
   * `mainClass`: **use Fabric's**.

The `client.jar`, the assets, and the Java version remain those of the vanilla version.

## 4. Download the Libraries

Unlike Mojang, the entries in `libraries[]` do not always provide a full URL: they give **Maven coordinates** and the repository address.

```json
{
  "name": "net.fabricmc:fabric-loader:0.20.1",
  "url": "https://maven.fabricmc.net/"
}
```

The file URL must be rebuilt from the `group:artifact:version` coordinates:

```text
{url}{group with /}/{artifact}/{version}/{artifact}-{version}.jar
```

Which gives, for the example above:

```text
https://maven.fabricmc.net/net/fabricmc/fabric-loader/0.20.1/fabric-loader-0.20.1.jar
```

Then save the file in `libraries/` following the same path (`net/fabricmc/fabric-loader/0.20.1/fabric-loader-0.20.1.jar`), as with the Mojang libraries.

:::tip
Some entries may also provide a `sha1` or a size. When present, use them to verify the integrity of the downloaded file.
:::

## Best Practices

* **Cache** the API response to avoid querying it at every launch.
* **Let the user choose** between a stable loader (default) and a pre-release.
* **Handle the absence of a loader** for an unsupported game version.
* **Parallelize** library downloads and retry on network errors.

:::note
This page only covers the **loader**. The **Fabric API**, which is a mod and not the loader, is downloaded separately, for example via Modrinth or CurseForge.
:::