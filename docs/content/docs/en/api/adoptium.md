---
title: Adoptium API Routes and Parameters (Java)
description: Detailed guide to the Eclipse Adoptium API for automatically downloading the Java runtimes (JRE) required by Minecraft.
---

To run Minecraft, your launcher needs a suitable Java runtime environment (usually a JRE). The official **Eclipse Adoptium (Temurin)** API lets you dynamically retrieve download links for up-to-date binaries.

The main endpoint is a `GET` request on stable major versions (`ga`).

## Base URL Structure

```text
https://api.adoptium.net/v3/assets/feature_releases/{version}/ga?{parameters}
```

* `{version}`: The major Java version number (e.g., `25` for Java 25).

* `/ga`: Stands for *General Availability* (stable, production-ready releases).

| Parameter | Example value | Description |
|---|---|---|
| architecture | x64 | The target processor architecture (`x64` for standard 64-bit PCs, `aarch64` for ARM/Apple Silicon). |
| image_type | jre | The image type. A JRE (Java Runtime Environment) is preferred over a JDK because it is lighter and contains only what is needed to run the game. |
| os | `windows`, `linux`, `mac` | The target operating system for the binary. |
| page | 0 | The pagination page of the results (starts at 0). |
| page_size | 1 | The number of results returned per page. Set to `1` to retrieve only the most recent result matching the criteria. |
| project | jdk | The source project of the release at Adoptium (left as `jdk` even when requesting a JRE). |
| sort_method | DEFAULT | The sorting method for the results. |
| sort_order | DESC | The sort order (`DESC` sorts from newest to oldest, ensuring the latest update comes first). |
| vendor | eclipse | The binary provider (here `eclipse` for Adoptium Temurin). |

## Examples by Operating System

Here are ready-to-use full URLs depending on the user's OS:

**Windows (64-bit)**
```text
https://api.adoptium.net/v3/assets/feature_releases/25/ga?architecture=x64&image_type=jre&os=windows&page=0&page_size=1&project=jdk&sort_method=DEFAULT&sort_order=DESC&vendor=eclipse
```

**Linux (64-bit)**
```text
https://api.adoptium.net/v3/assets/feature_releases/25/ga?architecture=x64&image_type=jre&os=linux&page=0&page_size=1&project=jdk&sort_method=DEFAULT&sort_order=DESC&vendor=eclipse
```

**macOS (Intel / Apple Silicon depending on the architecture)**
```text
https://api.adoptium.net/v3/assets/feature_releases/25/ga?architecture=x64&image_type=jre&os=mac&page=0&page_size=1&project=jdk&sort_method=DEFAULT&sort_order=DESC&vendor=eclipse
```
> **Note for macOS**: *Remember to adjust the `architecture` parameter (`aarch64` for M1/M2/M3 chips and `x64` for older Intel Macs).*